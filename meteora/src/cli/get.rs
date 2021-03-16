use clap::ArgMatches;

use meteora_client::kv::client::KVClient;

use crate::log::set_logger;

pub fn run_get_cli(matches: &ArgMatches) -> Result<(), std::io::Error> {
    set_logger();

    let address = matches.value_of("ADDRESS").unwrap();
    let key = matches.value_of("KEY").unwrap();

    let mut kv_client = KVClient::new(address);

    match kv_client.get(key.as_bytes().to_vec()) {
        Ok(v) => {
            println!("{:?}", String::from_utf8(v).unwrap());
            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}
