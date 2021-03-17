use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use futures::Future;
use grpcio::{RpcContext, UnarySink};
use log::*;
use raft::eraftpb::{ConfChange, Message};

use meteora_proto::proto::common::{NodeAddress, Null, State};
use meteora_proto::proto::raft::{AddressState, ChangeReply, StatusReply};
use meteora_proto::proto::raft_grpc::RaftService;

use crate::raft::config;

#[derive(Clone)]
pub struct RaftServer {
    pub sender: Sender<config::Msg>,
    seq: u64,
    node_id: u64,
}

impl RaftServer {
    pub fn new(sender: Sender<config::Msg>, node_id: u64) -> RaftServer {
        RaftServer {
            sender,
            seq: 0,
            node_id,
        }
    }
}

impl RaftService for RaftServer {
    fn status(&mut self, ctx: RpcContext, _req: Null, sink: UnarySink<StatusReply>) {
        let (s1, r1) = mpsc::channel();
        let sender = self.sender.clone();
        let node_id = self.node_id;

        sender
            .send(config::Msg::Read {
                cb: Box::new(
                    move |leader_id: i32, addresses: HashMap<u64, NodeAddress>| {
                        // Status
                        let mut reply = StatusReply::new();
                        reply.set_state(State::OK);
                        if leader_id >= 0 {
                            // follower
                            reply.set_leader_id(leader_id as u64);
                        } else {
                            // leader
                            reply.set_leader_id(node_id);
                        }
                        reply.set_address_map(addresses);
                        s1.send(reply).expect("callback channel closed");
                    },
                ),
            })
            .unwrap();

        let reply = match r1.recv_timeout(Duration::from_secs(2)) {
            Ok(r) => r,
            Err(e) => {
                error!("error: {:?}", e);
                let mut r = StatusReply::new();
                r.set_state(State::IO_ERROR);
                r
            }
        };

        let f = sink
            .success(reply.clone())
            .map_err(move |err| error!("failed to reply: {:?}", err));
        ctx.spawn(f);
    }

    fn change_config(&mut self, ctx: RpcContext, req: ConfChange, sink: UnarySink<ChangeReply>) {
        let (s1, r1) = mpsc::channel();
        let sender = self.sender.clone();
        let seq = self.seq;
        let node_id = self.node_id;

        self.seq += 1;

        sender
            .send(config::Msg::ConfigChange {
                seq,
                change: req,
                cb: Box::new(
                    move |leader_id: i32, addresses: HashMap<u64, NodeAddress>| {
                        let mut reply = ChangeReply::new();
                        if leader_id >= 0 {
                            // follower
                            reply.set_state(State::WRONG_LEADER);
                            reply.set_leader_id(leader_id as u64);
                        } else {
                            // leader
                            reply.set_state(State::OK);
                            reply.set_leader_id(node_id);
                        }
                        reply.set_address_map(addresses);
                        s1.send(reply).expect("callback channel closed");
                    },
                ),
            })
            .unwrap();

        let reply = match r1.recv_timeout(Duration::from_secs(2)) {
            Ok(r) => r,
            Err(e) => {
                error!("error: {:?}", e);
                let mut r = ChangeReply::new();
                r.set_state(State::IO_ERROR);
                r
            }
        };

        let f = sink
            .success(reply.clone())
            .map_err(move |err| error!("failed to reply: {:?}", err));
        ctx.spawn(f);
    }

    fn send_msg(&mut self, _ctx: RpcContext, req: Message, _sink: ::grpcio::UnarySink<Null>) {
        let sender = self.sender.clone();
        sender.send(config::Msg::Raft(req)).unwrap();
    }

    fn send_address(&mut self, _ctx: RpcContext, req: AddressState, _sink: UnarySink<Null>) {
        let sender = self.sender.clone();
        sender.send(config::Msg::Address(req)).unwrap();
    }
}
