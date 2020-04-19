use clap::ArgMatches;

use meteora_client::kv::client::KVClient;

use crate::log::set_logger;

pub fn run_get_cli(matches: &ArgMatches) -> Result<(), std::io::Error> {
    set_logger();

    let server = matches.value_of("SERVER").unwrap();
    let key = matches.value_of("KEY").unwrap();

    let mut kv_client = KVClient::new(server);

    match kv_client.get(key.to_string()) {
        Ok(v) => {
            println!("{}", v);
            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}
