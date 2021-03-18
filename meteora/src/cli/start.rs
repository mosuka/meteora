use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use clap::ArgMatches;
use crossbeam_channel::select;
use futures::Future;
use grpcio::{Environment, ServerBuilder};
use log::*;
use raft::storage::MemStorage;

use meteora_client::raft::client::RaftClient;
use meteora_proto::proto::common::NodeAddress;
use meteora_proto::proto::kv_grpc::create_kv_service;
use meteora_proto::proto::raft_grpc::create_raft_service;
use meteora_server::kv::server::KVServer;

use crate::log::set_logger;
use crate::signal::sigterm_channel;

pub fn run_start_cli(matches: &ArgMatches) -> Result<(), std::io::Error> {
    set_logger();

    let id = matches.value_of("ID").unwrap().parse::<u64>().unwrap();
    let address = matches.value_of("ADDRESS").unwrap();
    let raft_port = matches
        .value_of("RAFT_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let kv_port = matches.value_of("KV_PORT").unwrap().parse::<u16>().unwrap();
    let mut peer_address = "";
    let data_directory = matches.value_of("DATA_DIRECTORY").unwrap();
    if let Some(_peer_address) = matches.value_of("PEER_RAFT_ADDRESS") {
        peer_address = _peer_address;
    }

    let raft_address = format!("{}:{}", address, raft_port);
    let kv_address = format!("{}:{}", address, kv_port);

    let node_address = NodeAddress {
        kv_address,
        raft_address: raft_address.clone(),
        unknown_fields: Default::default(),
        cached_size: Default::default(),
    };

    let mut addresses = HashMap::new();

    // change config
    if peer_address != "" {
        let mut client = RaftClient::new(peer_address);
        match client.join(id, node_address.clone()) {
            Ok(_addresses) => addresses = _addresses,
            Err(e) => return Err(e),
        };
    }

    // new
    let env_kv = Arc::new(Environment::new(10));
    let env_raft = Arc::new(Environment::new(10));

    let kv_path = Path::new(data_directory)
        .join("kv")
        .to_str()
        .unwrap()
        .to_string();
    let raft_storage = MemStorage::new();

    let (kv, raft) = KVServer::new(kv_path, raft_storage, id, node_address, addresses);

    let kv_service = create_kv_service(kv);
    let raft_service = create_raft_service(raft);

    let mut kv_server = ServerBuilder::new(env_kv)
        .register_service(kv_service)
        .bind(address, kv_port)
        .build()
        .unwrap();
    let mut raft_server = ServerBuilder::new(env_raft)
        .register_service(raft_service)
        .bind(address, raft_port)
        .build()
        .unwrap();

    kv_server.start();
    raft_server.start();

    for &(ref h, p) in kv_server.bind_addrs() {
        info!("start key-value service on {}:{}", h, p);
    }

    for &(ref h, p) in raft_server.bind_addrs() {
        info!("start Raft service on {}:{}", h, p);
    }

    // Wait for signals for termination (SIGINT, SIGTERM).
    let sigterm_receiver = sigterm_channel().unwrap();
    loop {
        select! {
            recv(sigterm_receiver) -> _ => {
                debug!("receive signal");
                break;
            }
        }
    }

    if matches.is_present("ENABLE_AUTO_LEAVING") {
        info!("leaving from the cluster");
        let mut raft_client = RaftClient::new(&raft_address);

        match raft_client.leave(id) {
            Ok(v) => {
                info!("{}", serde_json::to_string(&v).unwrap());
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }

    match kv_server.shutdown().wait() {
        Ok(_) => {
            info!("stop key-value service on {}:{}", address, kv_port);
        }
        Err(e) => error!("{}", e),
    }
    match raft_server.shutdown().wait() {
        Ok(_) => {
            info!("stop Raft service on {}:{}", address, raft_port);
        }
        Err(e) => error!("{}", e),
    }

    Ok(())
}
