// #![deny(warnings)]

mod cask;
mod command_clean;
mod command_info;
mod command_install;
mod command_list;
mod command_self_update;
mod command_uninstall;
mod command_upgrade;
mod formula;
mod git;
mod hooker;
mod symlink;
mod util;

use std::{fs, process};

use clap::{arg, Arg, Command};
use futures::executor;

#[tokio::main]
async fn main() {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));

    let mut app = Command::new(env!("CARGO_BIN_NAME"))
        .version(version.as_str())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("install")
                .alias("i")
                .about("Install package")
                .arg(arg!(<PACKAGE> "The package name or repository url"))
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
                .alias("rm")
                .about("Uninstall package")
                .arg(arg!(<PACKAGE> "The package name or the executable file name of the package"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("List installed package")
                .arg(
                    Arg::new("json")
                        .short('j')
                        .long("json")
                        .help("Print json format instead of pretty format")
                        .takes_value(false),
                ),
        )
        .subcommand(
            Command::new("info")
                .about("Show information of package")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("upgrade")
                .alias("up")
                .about("Upgrade package to latest")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg(
                    Arg::new("check-only")
                        .short('c')
                        .long("check-only")
                        .help("Check update only")
                        .takes_value(false),
                )
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("self-update").about("Update Cask to the newest version"))
        .subcommand(Command::new("clean").about("Clear residual data"));

    let matches = app.clone().get_matches();

    let home_dir = dirs::home_dir().expect("can not get home dir");

    let cask = cask::new(&home_dir.join(".cask"));

    cask.init().expect("init cask fail");

    cask.check_bin_path().unwrap_or_else(|e| {
        eprint!("{}", e);
        process::exit(1);
    });

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");
            let version = sub_matches.value_of("VERSION");

            executor::block_on(command_install::install(&cask, package_name, version))
                .map_err(|err| {
                    let dir = cask.package_dir(package_name);
                    fs::remove_dir_all(dir).ok();
                    err
                })
                .expect("install package fail!");
        }
        Some(("uninstall", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");

            let f = command_uninstall::uninstall(&cask, package_name);

            executor::block_on(f).expect("uninstall package fail!");
        }
        Some(("list", sub_matches)) => {
            let is_print_as_json = sub_matches.is_present("json");
            let f = command_list::list(&cask, is_print_as_json);

            executor::block_on(f).expect("list packages fail!");
        }
        Some(("info", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");

            let f = command_info::info(&cask, package_name);

            executor::block_on(f).expect("info installed package fail!");
        }
        Some(("upgrade", sub_matches)) => {
            let package_name = sub_matches.value_of("PACKAGE").expect("required");
            let is_check_only = sub_matches.is_present("check-only");

            let f = command_upgrade::upgrade(&cask, package_name, is_check_only);

            executor::block_on(f).expect("info installed package fail!");
        }
        Some(("clean", _sub_matches)) => {
            let f = command_clean::clean(&cask);

            executor::block_on(f).expect("info installed package fail!");
        }
        Some(("self-update", _sub_matches)) => {
            let f = command_self_update::self_update(&cask);

            executor::block_on(f).expect("self-update fail!");
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
