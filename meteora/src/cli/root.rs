use clap::{App, AppSettings, Arg, SubCommand};

use crate::cli::delete::run_delete_cli;
use crate::cli::get::run_get_cli;
use crate::cli::set::run_set_cli;
use crate::cli::start::run_start_cli;

pub fn run_root_cli() -> Result<(), std::io::Error> {
    let app = App::new(crate_name!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about("Key-value store.")
        .help_message("Prints help information.")
        .version_message("Prints version information.")
        .version_short("v")
        .subcommand(
            SubCommand::with_name("start")
                .name("start")
                .setting(AppSettings::DeriveDisplayOrder)
                .version(crate_version!())
                .author(crate_authors!())
                .about("Start key-value store.")
                .help_message("Prints help information.")
                .version_message("Prints version information.")
                .version_short("v")
                .arg(
                    Arg::with_name("ID")
                        .help("Node ID.")
                        .value_name("ID")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("HOST")
                        .help("Node address.")
                        .short("H")
                        .long("host")
                        .value_name("HOST")
                        .default_value("0.0.0.0")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("RAFT_PORT")
                        .help("Raft service port number.")
                        .short("r")
                        .long("raft-port")
                        .value_name("RAFT_PORT")
                        .default_value("7000")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("KV_PORT")
                        .help("Key-value service port number")
                        .short("k")
                        .long("kv-port")
                        .value_name("KV_PORT")
                        .default_value("5000")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("PEER_RAFT_ADDRESS")
                        .help("Raft address of a peer node running in an existing cluster.")
                        .short("p")
                        .long("peer-raft-address")
                        .value_name("IP:PORT")
                        // .default_value("127.0.0.1:7000")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("DATA_DIRECTORY")
                        .help("Data directory. Stores index, snapshots, and raft logs. If not specified, use the default directory.")
                        .short("d")
                        .long("data-directory")
                        .value_name("DATA_DIRECTORY")
                        .default_value("./data")
                        .takes_value(true),
                )
        )
        .subcommand(
            SubCommand::with_name("set")
                .name("set")
                .setting(AppSettings::DeriveDisplayOrder)
                .version(crate_version!())
                .author(crate_authors!())
                .about("Set data to key-value store")
                .help_message("Prints help information.")
                .version_message("Prints version information.")
                .version_short("v")
                .arg(
                    Arg::with_name("SERVER")
                        .help("Key-value service address.")
                        .short("s")
                        .long("server")
                        .value_name("IP:PORT")
                        .default_value("127.0.0.1:5000")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("A unique key that identifies the value in the key-value tore.")
                        .value_name("KEY")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .help("Value in the key-value tore.")
                        .value_name("VALUE")
                        .takes_value(true),
                )
        )
        .subcommand(
            SubCommand::with_name("get")
                .name("get")
                .setting(AppSettings::DeriveDisplayOrder)
                .version(crate_version!())
                .author(crate_authors!())
                .about("Get data from key-value store")
                .help_message("Prints help information.")
                .version_message("Prints version information.")
                .version_short("v")
                .arg(
                    Arg::with_name("SERVER")
                        .help("Key-value service address.")
                        .short("s")
                        .long("server")
                        .value_name("IP:PORT")
                        .default_value("127.0.0.1:5000")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("A unique key that identifies the value in the key-value tore.")
                        .value_name("KEY")
                        .takes_value(true),
                )
        )
        .subcommand(
            SubCommand::with_name("delete")
                .name("delete")
                .setting(AppSettings::DeriveDisplayOrder)
                .version(crate_version!())
                .author(crate_authors!())
                .about("Delete data from key-value store")
                .help_message("Prints help information.")
                .version_message("Prints version information.")
                .version_short("v")
                .arg(
                    Arg::with_name("SERVER")
                        .help("Key-value service address.")
                        .short("s")
                        .long("server")
                        .value_name("IP:PORT")
                        .default_value("127.0.0.1:5000")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("A unique key that identifies the value in the key-value tore.")
                        .value_name("KEY")
                        .takes_value(true),
                )
        )
        .get_matches();

    let (subcommand, some_options) = app.subcommand();
    let options = some_options.unwrap();
    let run_cli = match subcommand {
        "start" => run_start_cli,
        "set" => run_set_cli,
        "get" => run_get_cli,
        "delete" => run_delete_cli,
        _ => panic!("Subcommand {} is unknown", subcommand),
    };

    run_cli(options)
}
