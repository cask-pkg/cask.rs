// #![deny(warnings)]

mod cask;
mod command_info;
mod command_install;
mod command_list;
mod command_search;
mod command_uninstall;
mod extractor;
mod formula;
mod git;
mod symlink;
mod util;

use std::process;

use clap::{arg, Arg, Command};
use futures::executor;

#[tokio::main]
async fn main() {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));

    let mut app = Command::new("cask")
        .version(version.as_str())
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
        .subcommand(Command::new("list").about("List installed package"))
        .subcommand(
            Command::new("search")
                .about("Show information of remote package")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("info")
                .about("Show information of installed package")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg_required_else_help(true),
        );

    let matches = app.clone().get_matches();

    let home_dir = dirs::home_dir().expect("can not get home dir");

    let cask = cask::new(&home_dir.join(".cask"));

    cask.init().expect("init cask fail");

    cask.check_bin_path().unwrap_or_else(|e| {
        eprintln!("{}", e);
        eprintln!("Manually add the directory to your $HOME/.bash_profile (or similar)");
        process::exit(1);
    });

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");
            let version = sub_matches.value_of("VERSION");

            let f = command_install::install(cask, package_name, version);

            executor::block_on(f).expect("install package fail!");
        }
        Some(("uninstall", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");

            let f = command_uninstall::uninstall(cask, package_name);

            executor::block_on(f).expect("uninstall package fail!");
        }
        Some(("list", _sub_matches)) => {
            let f = command_list::list(cask);

            executor::block_on(f).expect("list packages fail!");
        }
        Some(("search", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");

            let f = command_search::search(cask, package_name);

            executor::block_on(f).expect("search remote package fail!");
        }
        Some(("info", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");

            let f = command_info::info(cask, package_name);

            executor::block_on(f).expect("info installed package fail!");
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            eprintln!("Unknown the command {:?} with argument {:?}", ext, args);
            app.print_help().unwrap();
            process::exit(0x1);
        }
        _ => unreachable!(),
    }

    // Continued program logic goes here...
}
