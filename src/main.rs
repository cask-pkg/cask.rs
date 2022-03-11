#![deny(warnings)]

mod command_install;
mod formula;
mod git;
mod util;

use clap::{arg, Command};
use futures::executor;
use std::process;

#[tokio::main]
async fn main() {
    let mut app = Command::new("cask")
        .version("v0.1.0")
        .author("Axetroy <axetroy.dev@gmail.com>")
        .about("General binary package management, written in Rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("install")
                .about("Install package")
                .arg(arg!(<PACKAGE> "The package address"))
                .arg_required_else_help(true),
        );

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");

            let f = command_install::install(package_name);

            executor::block_on(f).expect("install package fail!");
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            println!("Unknown the command {:?} with argument {:?}", ext, args);
            app.print_help().unwrap();
            process::exit(0x1);
        }
        _ => unreachable!(),
    }

    // Continued program logic goes here...
}
