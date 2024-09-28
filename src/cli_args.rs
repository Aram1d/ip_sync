use clap::{Arg, ArgMatches, Command};
use std::sync::OnceLock;

static ARGS: OnceLock<ArgMatches> = OnceLock::new();

fn load_args() -> ArgMatches {
    Command::new("IpSync")
        .version("0.1.0")
        .author("Wilfried SUGNIAUX <wsu287@gmail.com>")
        .about("Simple daemon to sync your IP with an AWS Route53 record")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path of your config file, default /etc/ipsync.conf"),
        )
        .get_matches()
}

pub fn get_args() -> &'static ArgMatches {
    ARGS.get_or_init(load_args)
}
