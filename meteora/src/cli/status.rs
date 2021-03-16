use clap::ArgMatches;

use meteora_client::raft::client::RaftClient;

use crate::log::set_logger;

pub fn run_status_cli(matches: &ArgMatches) -> Result<(), std::io::Error> {
    set_logger();

    let address = matches.value_of("ADDRESS").unwrap();

    let mut raft_client = RaftClient::new(address);

    match raft_client.status() {
        Ok(v) => {
            println!("{}", serde_json::to_string(&v).unwrap());
            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}
