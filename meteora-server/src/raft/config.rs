use std::collections::HashMap;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use bincode::{deserialize, serialize};
use grpcio::{ChannelBuilder, EnvBuilder};
use log::*;
use protobuf::Message as PMessage;
use raft::prelude::*;
use raft::storage::MemStorage;

use meteora_proto::proto::common::NodeAddress;
use meteora_proto::proto::raft::AddressState;
use meteora_proto::proto::raft_grpc::RaftServiceClient;

use crate::kv::server::Op;

type ProposeCallback = Box<dyn Fn(i32, HashMap<u64, NodeAddress>) + Send>;

pub enum Msg {
    Propose {
        seq: u64,
        op: Op,
        cb: ProposeCallback,
    },
    ConfigChange {
        seq: u64,
        change: ConfChange,
        cb: ProposeCallback,
    },
    Address(AddressState),
    // Here we don't use Raft Message, so use dead_code to
    // avoid the compiler warning.
    #[allow(dead_code)]
    Raft(Message),
}

pub fn init_and_run(
    storage: MemStorage,
    receiver: Receiver<Msg>,
    apply_sender: Sender<Op>,
    node_id: u64,
    node_address: NodeAddress,
    addresses: HashMap<u64, NodeAddress>,
) {
    let mut peers = vec![];
    let mut addresses = addresses;
    let mut rpc_clients = HashMap::new();
    for (id, address) in &addresses {
        peers.push(id.clone());
        insert_client(id.clone(), address.raft_address.as_str(), &mut rpc_clients);
    }
    if peers.is_empty() {
        addresses.insert(node_id, node_address);
        peers.push(node_id);
    }
    debug!("{:?}", peers);

    // Create the configuration for the Raft node.
    let cfg = Config {
        // The unique ID for the Raft node.
        id: node_id,
        // The Raft node list.
        // Mostly, the peers need to be saved in the storage
        peers,
        // Election tick is for how long the follower may campaign again after
        // it doesn't receive any message from the leader.
        election_tick: 10,
        // Heartbeat tick is for how long the leader needs to send
        // a heartbeat to keep alive.
        heartbeat_tick: 3,
        // The max size limits the max size of each appended message. Mostly, 1 MB is enough.
        max_size_per_msg: 1024 * 1024 * 1024,
        // Max inflight msgs that the leader sends messages to follower without
        // receiving ACKs.
        max_inflight_msgs: 256,
        // The Raft applied index.
        // You need to save your applied index when you apply the committed Raft logs.
        applied: 0,
        // Just for log
        tag: format!("[{}]", 1),
        ..Default::default()
    };

    // Create the Raft node.
    let mut r = RawNode::new(&cfg, storage, vec![]).unwrap();

    // Loop forever to drive the Raft.
    let mut t = Instant::now();
    let mut timeout = Duration::from_millis(100);

    // Use a HashMap to hold the `propose` callbacks.
    let mut callbacks = HashMap::new();

    loop {
        match receiver.recv_timeout(timeout) {
            Ok(Msg::Propose {
                seq,
                op,
                cb: callback,
            }) => {
                debug!("receive propose message");
                let leader_id = r.raft.leader_id;
                if r.raft.leader_id != r.raft.id {
                    // not leader, callback to notify client
                    debug!("not a leader");
                    callback(leader_id as i32, addresses.clone());
                    continue;
                }
                let serialized_op = serialize(&op).unwrap();
                callbacks.insert(seq, callback);
                debug!("propose");
                r.propose(serialize(&seq).unwrap(), serialized_op).unwrap();
            }
            Ok(Msg::ConfigChange {
                seq,
                change,
                cb: callback,
            }) => {
                debug!("receive config change message");
                let leader_id = r.raft.leader_id;
                if r.raft.leader_id != r.raft.id {
                    // not leader, callback to notify client
                    debug!("not a leader");
                    callback(leader_id as i32, addresses.clone());
                    continue;
                } else {
                    // TODO add address to map
                    callbacks.insert(seq, callback);
                    debug!("propose config change");
                    r.propose_conf_change(serialize(&seq).unwrap(), change)
                        .unwrap();
                }
            }
            Ok(Msg::Raft(m)) => {
                debug!("receive raft message");
                if let Ok(_a) = r.step(m) {};
            }
            Ok(Msg::Address(address_state)) => {
                debug!("receive address message");
                let new_addresses = address_state.get_address_map();
                for (id, address) in new_addresses {
                    let address_changed = match addresses.get(id) {
                        Some(a) => {
                            if a.raft_address == address.raft_address {
                                false
                            } else {
                                true
                            }
                        }
                        None => true,
                    };
                    if address_changed {
                        insert_client(id.clone(), address.raft_address.as_str(), &mut rpc_clients);
                    }
                }
                addresses = new_addresses.clone();
            }
            Err(RecvTimeoutError::Timeout) => {
                debug!("timeout");
            }
            Err(RecvTimeoutError::Disconnected) => {
                debug!("disconnected");
            }
        }

        let d = t.elapsed();
        if d >= timeout {
            t = Instant::now();
            timeout = Duration::from_millis(100);
            // We drive Raft every 100ms.
            r.tick();
        } else {
            timeout -= d;
        }

        on_ready(
            &mut r,
            &mut callbacks,
            &mut addresses,
            &mut rpc_clients,
            apply_sender.clone(),
        );
    }
}

fn on_ready(
    r: &mut RawNode<MemStorage>,
    callbacks: &mut HashMap<u64, ProposeCallback>,
    addresses: &mut HashMap<u64, NodeAddress>,
    clients: &mut HashMap<u64, Arc<RaftServiceClient>>,
    apply_sender: Sender<Op>,
) {
    if !r.has_ready() {
        return;
    }

    // The Raft is ready, we can do something now.
    let mut ready = r.ready();

    let is_leader = r.raft.leader_id == r.raft.id;
    if is_leader {
        // If the peer is leader, the leader can send messages to other followers ASAP.
        let msgs = ready.messages.drain(..);
        for msg in msgs {
            let client = match clients.get(&msg.get_to()) {
                Some(c) => c.clone(),
                None => {
                    continue;
                }
            };
            let mut address_state = AddressState::new();
            address_state.set_address_map(addresses.clone());
            thread::spawn(move || {
                let msg = msg;
                let address_state = address_state;
                if let Ok(_) = client.send_msg(&msg) {};
                if let Ok(_) = client.send_address(&address_state) {};
            });
        }
    }

    if !raft::is_empty_snap(&ready.snapshot) {
        // This is a snapshot, we need to apply the snapshot at first.
        r.mut_store()
            .wl()
            .apply_snapshot(ready.snapshot.clone())
            .unwrap();
    }

    if !ready.entries.is_empty() {
        // Append entries to the Raft log
        r.mut_store().wl().append(&ready.entries).unwrap();
    }

    if let Some(ref hs) = ready.hs {
        // Raft HardState changed, and we need to persist it.
        r.mut_store().wl().set_hardstate(hs.clone());
    }

    if !is_leader {
        // If not leader, the follower needs to reply the messages to
        // the leader after appending Raft entries.
        let msgs = ready.messages.drain(..);
        for msg in msgs {
            // Send messages to other peers.
            let client = match clients.get(&msg.get_to()) {
                Some(c) => c.clone(),
                None => {
                    continue;
                }
            };
            thread::spawn(move || {
                let msg = msg;
                if let Ok(_) = client.send_msg(&msg) {};
            });
        }
    }

    if let Some(committed_entries) = ready.committed_entries.take() {
        let mut _last_apply_index = 0;
        for entry in committed_entries {
            // Mostly, you need to save the last apply index to resume applying
            // after restart. Here we just ignore this because we use a Memory storage.
            _last_apply_index = entry.get_index();

            if entry.get_data().is_empty() {
                debug!("empty entry");
                // Emtpy entry, when the peer becomes Leader it will send an empty entry.
                continue;
            }

            if entry.get_entry_type() == EntryType::EntryNormal {
                let op: Op = deserialize(entry.get_data()).unwrap();
                let seq: u64 = deserialize(entry.get_context()).unwrap();
                match apply_sender.send(op) {
                    _ => {}
                }
                if let Some(callback) = callbacks.remove(&seq) {
                    callback(-1, addresses.clone());
                }
            }

            // handle EntryConfChange
            if entry.get_entry_type() == EntryType::EntryConfChange {
                let mut change = ConfChange::new();
                change.merge_from_bytes(entry.get_data()).unwrap();
                let seq: u64 = deserialize(entry.get_context()).unwrap();
                let node_id = change.get_node_id();

                let change_type = change.get_change_type();
                if change_type == ConfChangeType::AddNode {
                    let node_address: NodeAddress = deserialize(change.get_context()).unwrap();
                    insert_client(node_id, node_address.raft_address.as_str(), clients);
                    addresses.insert(node_id, node_address);
                } else if change_type == ConfChangeType::RemoveNode {
                    if let Some(_client) = clients.remove(&node_id) {
                        addresses.remove(&node_id);
                    }
                }

                r.apply_conf_change(&change);
                if let Some(callback) = callbacks.remove(&seq) {
                    callback(-1, addresses.clone());
                }
            }
        }
    }

    // Advance the Raft
    r.advance(ready);
}

fn insert_client(
    node_id: u64,
    node_address: &str,
    clients: &mut HashMap<u64, Arc<RaftServiceClient>>,
) {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(node_address);
    let client = RaftServiceClient::new(ch);
    clients.insert(node_id.clone(), Arc::new(client));
}
