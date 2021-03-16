use clap::ArgMatches;

use meteora_client::kv::client::KVClient;

use crate::log::set_logger;

pub fn run_delete_cli(matches: &ArgMatches) -> Result<(), std::io::Error> {
    set_logger();

    let address = matches.value_of("ADDRESS").unwrap();
    let key = matches.value_of("KEY").unwrap();

    let mut kv_client = KVClient::new(address);

    kv_client.delete(key.as_bytes().to_vec())
}
