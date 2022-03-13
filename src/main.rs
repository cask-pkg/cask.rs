// #![deny(warnings)]

mod command_info;
mod command_install;
mod formula;
mod git;
mod util;
mod extractor;

use std::process;

use clap::{arg, Arg, Command};
use futures::executor;

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
                .arg(arg!(<PACKAGE> "The package name"))
                .arg(
                    Arg::new("VERSION")
                        .required(false)
                        .multiple_occurrences(false)
                        .help("Install specified version."),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("uninstall")
                .about("Uninstall package")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("list")
                .about("List installed package")
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("info")
                .about("Show information of package")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg_required_else_help(true),
        );

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");
            let version = sub_matches.value_of("VERSION");

            let f = command_install::install(package_name, version);

            executor::block_on(f).expect("install package fail!");
        }
        Some(("uninstall", _sub_matches)) => {
            // TODO
        }
        Some(("list", _sub_matches)) => {
            // TODO
        }
        Some(("info", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");

            let f = command_info::info(package_name);

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
