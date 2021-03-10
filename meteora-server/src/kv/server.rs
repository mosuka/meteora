use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use futures::Future;
use grpcio::{RpcContext, UnarySink};
use log::*;
use raft::storage::MemStorage;
use rocksdb::DB;
use serde::{Deserialize, Serialize};

use meteora_proto::proto::common::State;
use meteora_proto::proto::kv::{DeleteReply, DeleteReq, GetReply, GetReq, SetReply, SetReq};
use meteora_proto::proto::kv_grpc::KvService;

use crate::raft::config;
use crate::raft::config::NodeAddress;
use crate::raft::server::RaftServer;

#[derive(Clone)]
pub struct KVServer {
    db: Arc<DB>,
    sender: Sender<config::Msg>,
    seq: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Op {
    Put { key: String, val: String },
    Get { key: String },
    Delete { key: String },
}

impl KVServer {
    pub fn new(
        db_path: String,
        raft_storage: MemStorage,
        server_id: u64,
        node_address: NodeAddress,
        addresses: HashMap<u64, NodeAddress>,
    ) -> (KVServer, RaftServer) {
        let db = DB::open_default(&db_path).unwrap();

        let (rs, rr) = mpsc::channel();
        let (apply_s, apply_r) = mpsc::channel();
        thread::spawn(move || {
            config::init_and_run(
                raft_storage,
                rr,
                apply_s,
                server_id,
                node_address,
                addresses,
            );
        });

        let kv_server = KVServer {
            db: Arc::new(db),
            sender: rs.clone(),
            seq: 0,
        };
        let raft_server = RaftServer::new(rs);

        let db = kv_server.db.clone();
        thread::spawn(move || {
            apply_daemon(apply_r, db);
        });

        return (kv_server, raft_server);
    }
}

impl KvService for KVServer {
    fn get(&mut self, ctx: RpcContext, req: GetReq, sink: UnarySink<GetReply>) {
        let (s1, r1) = mpsc::channel();
        let db = Arc::clone(&self.db);
        let sender = self.sender.clone();
        let op = Op::Get {
            key: String::from(req.get_key()),
        };
        let seq = self.seq;
        self.seq += 1;

        sender
            .send(config::Msg::Propose {
                seq,
                op,
                cb: Box::new(move |leader_id: i32, addresses: Vec<u8>| {
                    // Get
                    let mut reply = GetReply::new();
                    let (state, value) = match db.get(req.get_key().as_bytes()) {
                        Ok(Some(v)) => (State::OK, String::from_utf8(v).unwrap()),
                        Ok(None) => (State::NOT_FOUND, "".to_string()),
                        Err(e) => (State::IO_ERROR, String::from(e)),
                    };
                    reply.set_state(state);
                    reply.set_leader_id(leader_id as u64);
                    reply.set_value(value);
                    reply.set_address_map(addresses);
                    s1.send(reply).expect("cb channel closed");
                }),
            })
            .unwrap();

        let reply = match r1.recv_timeout(Duration::from_secs(2)) {
            Ok(r) => r,
            Err(_e) => {
                let mut r = GetReply::new();
                r.set_state(State::IO_ERROR);
                r
            }
        };

        let f = sink
            .success(reply.clone())
            .map_err(move |err| error!("Failed to reply get: {:?}", err));
        ctx.spawn(f);
    }

    fn set(&mut self, ctx: RpcContext, req: SetReq, sink: UnarySink<SetReply>) {
        let (s1, r1) = mpsc::channel();
        let sender = self.sender.clone();
        let op = Op::Put {
            key: String::from(req.get_key()),
            val: String::from(req.get_value()),
        };
        let seq = self.seq;
        self.seq += 1;

        sender
            .send(config::Msg::Propose {
                seq,
                op,
                cb: Box::new(move |leader_id: i32, addresses: Vec<u8>| {
                    let mut reply = SetReply::new();
                    if leader_id >= 0 {
                        reply.set_state(State::WRONG_LEADER);
                        reply.set_leader_id(leader_id as u64);
                    } else {
                        reply.set_state(State::OK);
                    }
                    reply.set_address_map(addresses);
                    // done job, wake
                    s1.send(reply).expect("cb channel closed");
                }),
            })
            .unwrap();

        let reply = match r1.recv_timeout(Duration::from_secs(2)) {
            Ok(r) => r,
            Err(_e) => {
                let mut r = SetReply::new();
                r.set_state(State::IO_ERROR);
                r
            }
        };

        let f = sink
            .success(reply.clone())
            .map_err(move |err| error!("Failed to reply set: {:?}", err));
        ctx.spawn(f);
    }

    fn delete(&mut self, ctx: RpcContext, req: DeleteReq, sink: UnarySink<DeleteReply>) {
        let (s1, r1) = mpsc::channel();
        let sender = self.sender.clone();
        let op = Op::Delete {
            key: String::from(req.get_key()),
        };
        let seq = self.seq;
        self.seq += 1;

        sender
            .send(config::Msg::Propose {
                seq,
                op,
                cb: Box::new(move |leader_id: i32, addresses: Vec<u8>| {
                    let mut reply = DeleteReply::new();
                    if leader_id >= 0 {
                        reply.set_state(State::WRONG_LEADER);
                        reply.set_leader_id(leader_id as u64);
                    } else {
                        reply.set_state(State::OK);
                    }
                    reply.set_address_map(addresses);
                    // done job, wake
                    s1.send(reply).expect("cb channel closed");
                }),
            })
            .unwrap();

        let reply = match r1.recv_timeout(Duration::from_secs(2)) {
            Ok(r) => r,
            Err(_e) => {
                let mut r = DeleteReply::new();
                r.set_state(State::IO_ERROR);
                r
            }
        };

        let f = sink
            .success(reply.clone())
            .map_err(move |err| error!("Failed to reply delete: {:?}", err));
        ctx.spawn(f);
    }
}

fn apply_daemon(receiver: Receiver<Op>, db: Arc<DB>) {
    loop {
        let op = match receiver.recv() {
            Ok(o) => o,
            _ => {
                debug!("apply dammon return");
                return;
            }
        };
        match op {
            Op::Get { key: _k } => {
                // noop
            }
            Op::Put { key, val } => {
                db.put(key.as_bytes(), val.as_bytes()).unwrap();
            }
            Op::Delete { key } => {
                db.delete(key.as_bytes()).unwrap();
            }
        }
    }
}
