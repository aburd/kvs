extern crate clap;
use clap::{App, Arg, SubCommand};

fn create_cli_app() -> clap::ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(Arg::with_name("VALUE").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches()
}

fn main() {
    let matches = create_cli_app();

    match matches.subcommand() {
        ("set", _matches) => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        ("get", _matches) => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        ("rm", _matches) => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        _ => unreachable!(),
    }
}
