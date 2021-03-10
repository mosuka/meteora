use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::Arc;

use bincode::deserialize;
use grpcio::{ChannelBuilder, EnvBuilder};
use log::*;

use meteora_proto::proto::common::State;
use meteora_proto::proto::kv::{DeleteReq, GetReq, SetReq};
use meteora_proto::proto::kv_grpc::KvServiceClient;
use meteora_server::raft::config::NodeAddress;

fn create_client(address: String) -> KvServiceClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&address);
    let client = KvServiceClient::new(ch);
    client
}

pub struct KVClient {
    leader_id: u64, // leader's node id
    clients: HashMap<u64, Arc<KvServiceClient>>,
    addresses: HashMap<u64, String>,
    next_index: usize,
    node_id: u64, // node id
}

impl KVClient {
    pub fn new(server: &str) -> KVClient {
        let initial_node_id = 0;

        let mut addresses = HashMap::new();
        addresses.insert(initial_node_id, server.to_string());

        let mut clients = HashMap::new();
        let client = create_client(server.to_string());
        clients.insert(initial_node_id, Arc::new(client));

        KVClient {
            leader_id: initial_node_id,
            clients,
            addresses,
            next_index: 0,
            node_id: initial_node_id,
        }
    }

    pub fn get(&mut self, key: String) -> Result<String, std::io::Error> {
        let mut req = GetReq::new();
        req.set_key(key);

        let max_retry = 10;
        let mut cnt_retry = 0;

        loop {
            if max_retry < cnt_retry {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("max retry count has been exceeded: max_retry={}", max_retry),
                ));
            }

            let client = match self.clients.get(&self.node_id) {
                Some(c) => c,
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("failed to get client for node: id={}", self.node_id),
                    ));
                }
            };

            let reply = match client.get(&req) {
                Ok(r) => r,
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("failed to get value: key={}", req.get_key()),
                    ));
                }
            };

            // update address list and clients
            if reply.get_address_map().len() > 0 {
                let address_map: HashMap<u64, NodeAddress> =
                    deserialize(&reply.get_address_map()).unwrap();
                // add new ids
                for (id, address) in &address_map {
                    if let Some(grpc_address) = self.addresses.get(&id) {
                        if grpc_address == address.kv_address.as_str() {
                            debug!(
                                "node has not been changed: id={}, address={}",
                                id, grpc_address
                            );
                        } else {
                            debug!("update node: id={}, address={}", id, address.kv_address);
                            self.addresses
                                .insert(id.clone(), address.kv_address.clone());
                            self.clients.insert(
                                id.clone(),
                                Arc::new(create_client(address.kv_address.clone())),
                            );
                        }
                    } else {
                        debug!("add node: id={}, address={}", id, address.kv_address);
                        self.addresses
                            .insert(id.clone(), address.kv_address.clone());
                        self.clients.insert(
                            id.clone(),
                            Arc::new(create_client(address.kv_address.clone())),
                        );
                    }
                }

                // remove unused ids
                for (id, address) in &self.addresses.clone() {
                    if let Some(_) = address_map.get(&id) {
                        debug!("node is in use: id={}, address={}", id, address);
                    } else {
                        debug!("node is not in use: id={}, address={}", id, address);
                        self.addresses.remove(id);
                        self.clients.remove(id);
                    }
                }

                debug!("addresses={:?}", self.addresses);
            }

            // change node id
            let keys: Vec<u64> = self.addresses.keys().map(|i| i.clone()).collect();
            self.next_index = (self.next_index + 1) % self.addresses.len();
            self.node_id = keys.get(self.next_index).unwrap().clone();

            match reply.get_state() {
                State::OK => {
                    return Ok(String::from(reply.get_value()));
                }
                State::NOT_FOUND => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("not found: key={}", req.get_key()),
                    ));
                }
                _ => {
                    cnt_retry += 1;
                    warn!("failed to get value: key={}", req.get_key());
                }
            }
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), std::io::Error> {
        let mut req = SetReq::new();
        req.set_key(key);
        req.set_value(value);

        let max_retry = 10;
        let mut cnt_retry = 0;

        loop {
            if max_retry < cnt_retry {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("max retry count has been exceeded: max_retry={}", max_retry),
                ));
            }

            let client = match self.clients.get(&self.leader_id) {
                Some(c) => c,
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("failed to get client for node: id={}", self.leader_id),
                    ));
                }
            };

            let reply = match client.set(&req) {
                Ok(r) => r,
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("failed to set value: key={}", req.get_key()),
                    ));
                }
            };

            // update address list and clients
            if reply.get_address_map().len() > 0 {
                let address_map: HashMap<u64, NodeAddress> =
                    deserialize(&reply.get_address_map()).unwrap();
                // add new ids
                for (id, address) in &address_map {
                    if let Some(grpc_address) = self.addresses.get(&id) {
                        if grpc_address == address.kv_address.as_str() {
                            debug!(
                                "node has not been changed: id={}, address={}",
                                id, grpc_address
                            );
                        } else {
                            debug!("update node: id={}, address={}", id, address.kv_address);
                            self.addresses
                                .insert(id.clone(), address.kv_address.clone());
                            self.clients.insert(
                                id.clone(),
                                Arc::new(create_client(address.kv_address.clone())),
                            );
                        }
                    } else {
                        debug!("add node: id={}, address={}", id, address.kv_address);
                        self.addresses
                            .insert(id.clone(), address.kv_address.clone());
                        self.clients.insert(
                            id.clone(),
                            Arc::new(create_client(address.kv_address.clone())),
                        );
                    }
                }

                // remove unused ids
                for (id, address) in &self.addresses.clone() {
                    if let Some(_) = address_map.get(&id) {
                        debug!("node is in use: id={}, address={}", id, address);
                    } else {
                        debug!("node is not in use: id={}, address={}", id, address);
                        self.addresses.remove(id);
                        self.clients.remove(id);
                    }
                }

                debug!("addresses={:?}", self.addresses);
            }

            match reply.get_state() {
                State::OK => {
                    return Ok(());
                }
                State::WRONG_LEADER => {
                    warn!(
                        "upddate leader id: current={}, new={}",
                        self.leader_id,
                        reply.get_leader_id()
                    );
                    self.leader_id = reply.get_leader_id();
                    cnt_retry += 1;
                    warn!("retry with a new leader: id={}", self.leader_id);
                    continue;
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("failed to set value: key={}", req.get_key()),
                    ));
                }
            };
        }
    }

    pub fn delete(&mut self, key: String) -> Result<(), std::io::Error> {
        let mut req = DeleteReq::new();
        req.set_key(key);

        let max_retry = 10;
        let mut cnt_retry = 0;

        loop {
            if max_retry < cnt_retry {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("max retry count has been exceeded: max_retry={}", max_retry),
                ));
            }

            let client = match self.clients.get(&self.leader_id) {
                Some(c) => c,
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("failed to get client for node: id={}", self.leader_id),
                    ));
                }
            };

            let reply = match client.delete(&req) {
                Ok(r) => r,
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("failed to delete value: key={}", req.get_key()),
                    ));
                }
            };

            // update address list and clients
            if reply.get_address_map().len() > 0 {
                let address_map: HashMap<u64, NodeAddress> =
                    deserialize(&reply.get_address_map()).unwrap();
                // add new ids
                for (id, address) in &address_map {
                    if let Some(grpc_address) = self.addresses.get(&id) {
                        if grpc_address == address.kv_address.as_str() {
                            debug!(
                                "node has not been changed: id={}, address={}",
                                id, grpc_address
                            );
                        } else {
                            debug!("update node: id={}, address={}", id, address.kv_address);
                            self.addresses
                                .insert(id.clone(), address.kv_address.clone());
                            self.clients.insert(
                                id.clone(),
                                Arc::new(create_client(address.kv_address.clone())),
                            );
                        }
                    } else {
                        debug!("add node: id={}, address={}", id, address.kv_address);
                        self.addresses
                            .insert(id.clone(), address.kv_address.clone());
                        self.clients.insert(
                            id.clone(),
                            Arc::new(create_client(address.kv_address.clone())),
                        );
                    }
                }

                // remove unused ids
                for (id, address) in &self.addresses.clone() {
                    if let Some(_) = address_map.get(&id) {
                        debug!("node is in use: id={}, address={}", id, address);
                    } else {
                        debug!("node is not in use: id={}, address={}", id, address);
                        self.addresses.remove(id);
                        self.clients.remove(id);
                    }
                }

                debug!("addresses={:?}", self.addresses);
            }

            match reply.get_state() {
                State::OK => {
                    return Ok(());
                }
                State::WRONG_LEADER => {
                    warn!(
                        "upddate leader id: current={}, new={}",
                        self.leader_id,
                        reply.get_leader_id()
                    );
                    self.leader_id = reply.get_leader_id();
                    cnt_retry += 1;
                    warn!("retry with a new leader: id={}", self.leader_id);
                    continue;
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("failed to delete value: key={}", req.get_key()),
                    ));
                }
            };
        }
    }
}
