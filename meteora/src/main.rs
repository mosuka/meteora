use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand};

use meteora::cli::delete::run_delete_cli;
use meteora::cli::get::run_get_cli;
use meteora::cli::leave::run_leave_cli;
use meteora::cli::put::run_put_cli;
use meteora::cli::start::run_start_cli;
use meteora::cli::status::run_status_cli;

fn main() -> Result<(), std::io::Error> {
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
                        .help("A number that is unique ID in the cluster. It must be greater than or equal to 1.")
                        .short("i")
                        .long("id")
                        .value_name("ID")
                        .env("METEORA_ID")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("ADDRESS")
                        .help("An IP address or a hostname that runs the server.")
                        .short("a")
                        .long("address")
                        .value_name("ADDRESS")
                        .env("METEORA_ADDRESS")
                        .default_value("0.0.0.0")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("RAFT_PORT")
                        .help("A port number that provides the Raft service.")
                        .short("r")
                        .long("raft-port")
                        .value_name("RAFT_PORT")
                        .env("METEORA_RAFT_PORT")
                        .default_value("7000")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("KV_PORT")
                        .help("A port number that provides the Key-Value service.")
                        .short("k")
                        .long("kv-port")
                        .value_name("KV_PORT")
                        .env("METEORA_KV_PORT")
                        .default_value("5000")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("DATA_DIRECTORY")
                        .help("A directory path that stores data.")
                        .short("d")
                        .long("data-directory")
                        .value_name("DATA_DIRECTORY")
                        .env("METEORA_DATA_DIRECTORY")
                        .default_value("./data")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("PEER_RAFT_ADDRESS")
                        .help("Join a cluster in which a peer node with the specified raft address is joined.")
                        .short("p")
                        .long("peer-raft-address")
                        .value_name("ADDRESS:RAFT_PORT")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("ENABLE_AUTO_LEAVING")
                        .help("Automatically delete the node from the cluster when stopped.")
                        .short("l")
                        .long("enable-auto-leaving"),
                )
        )
        .subcommand(
            SubCommand::with_name("put")
                .name("put")
                .setting(AppSettings::DeriveDisplayOrder)
                .version(crate_version!())
                .author(crate_authors!())
                .about("Put data to key-value store")
                .help_message("Prints help information.")
                .version_message("Prints version information.")
                .version_short("v")
                .arg(
                    Arg::with_name("ADDRESS")
                        .help("An address that provides the raft service.")
                        .short("a")
                        .long("address")
                        .value_name("ADDRESS:KV_PORT")
                        .default_value("127.0.0.1:7000")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("A unique key that identifies the value in the key-value store.")
                        .value_name("KEY")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .help("Value in the key-value tore.")
                        .value_name("VALUE")
                        .required(true)
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
                    Arg::with_name("ADDRESS")
                        .help("An address that provides the raft service.")
                        .short("a")
                        .long("address")
                        .value_name("ADDRESS:KV_PORT")
                        .default_value("127.0.0.1:7000")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("A unique key that identifies the value in the key-value store.")
                        .value_name("KEY")
                        .required(true)
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
                    Arg::with_name("ADDRESS")
                        .help("An address that provides the raft service.")
                        .short("a")
                        .long("address")
                        .value_name("ADDRESS:KV_PORT")
                        .default_value("127.0.0.1:7000")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("KEY")
                        .help("A unique key that identifies the value in the key-value store.")
                        .value_name("KEY")
                        .required(true)
                        .takes_value(true),
                )
        )
        .subcommand(
            SubCommand::with_name("status")
                .name("status")
                .setting(AppSettings::DeriveDisplayOrder)
                .version(crate_version!())
                .author(crate_authors!())
                .about("Get status")
                .help_message("Prints help information.")
                .version_message("Prints version information.")
                .version_short("v")
                .arg(
                    Arg::with_name("ADDRESS")
                        .help("An address that provides the raft service.")
                        .short("a")
                        .long("address")
                        .value_name("ADDRESS:KV_PORT")
                        .default_value("127.0.0.1:7000")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("leave")
                .name("leave")
                .setting(AppSettings::DeriveDisplayOrder)
                .version(crate_version!())
                .author(crate_authors!())
                .about("Delete the node from the cluster")
                .help_message("Prints help information.")
                .version_message("Prints version information.")
                .version_short("v")
                .arg(
                    Arg::with_name("ADDRESS")
                        .help("An address that provides the raft service.")
                        .short("a")
                        .long("address")
                        .value_name("ADDRESS:KV_PORT")
                        .default_value("127.0.0.1:7000")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("ID")
                        .help("A number that is unique ID in the cluster. It must be greater than or equal to 1.")
                        .short("i")
                        .long("id")
                        .value_name("ID")
                        .required(true)
                        .takes_value(true),
                )
        )
        .get_matches();

    let (subcommand, some_options) = app.subcommand();
    let options = some_options.unwrap();
    let run_cli = match subcommand {
        "start" => run_start_cli,
        "put" => run_put_cli,
        "get" => run_get_cli,
        "delete" => run_delete_cli,
        "status" => run_status_cli,
        "leave" => run_leave_cli,
        _ => panic!("Subcommand {} is unknown", subcommand),
    };

    run_cli(options)
}
