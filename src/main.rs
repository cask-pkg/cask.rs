#![deny(warnings)]

mod cask;
mod command_check_updates;
mod command_clean;
mod command_homepage;
mod command_info;
mod command_install;
mod command_list;
mod command_relink;
mod command_remote_list;
mod command_remote_sync;
mod command_self_uninstall;
mod command_self_update;
mod command_uninstall;
mod command_update;
mod formula;
mod hooker;
mod symlink;
mod util;

use std::process;

use atty::{is, Stream};
use clap::{arg, crate_version, Arg, Command};

#[tokio::main]
async fn main() {
    let mut app = Command::new(env!("CARGO_BIN_NAME"))
        .version(crate_version!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("install")
                .visible_alias("i")
                .about("Install package")
                .arg(
                    Arg::new("PACKAGE")
                        .required(is(Stream::Stdin))
                        .num_args(1)
                        .help("The package name or repository url"),
                )
                .arg(
                    Arg::new("VERSION")
                        .required(false)
                        .num_args(0..=1)
                        .help("Install specified version."),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Print verbose information")
                        .num_args(0..=1),
                )
                .arg_required_else_help(is(Stream::Stdin)),
        )
        .subcommand(
            Command::new("uninstall")
                .visible_alias("rm")
                .about("Uninstall package")
                .arg(arg!(<PACKAGE> "The package name or the executable file name of the package"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("list")
                .visible_alias("ls")
                .about("List installed package")
                .arg(
                    Arg::new("json")
                        .short('j')
                        .long("json")
                        .help("Print json format instead of pretty format")
                        .num_args(0..=1),
                ),
        )
        .subcommand(
            Command::new("info")
                .about("Show information of package")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("update")
                .visible_alias("upgrade")
                .about("Upgrade package to latest")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg(
                    Arg::new("check-only")
                        .short('c')
                        .long("check-only")
                        .help("Check update only")
                        .num_args(0..=1),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Print verbose information")
                        .num_args(0..=1),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("homepage")
                .visible_alias("home")
                .about("Open homepage of package")
                .arg(arg!(<PACKAGE> "The package name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("check-updates")
                .visible_alias("check-upgrades")
                .about("Check and update packages to latest")
                .arg(
                    Arg::new("check-only")
                        .short('c')
                        .long("check-only")
                        .help("Check update only")
                        .num_args(0..=1),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Print verbose information")
                        .num_args(0..=1),
                ),
        )
        .subcommand(
            Command::new("self-update")
                .visible_alias("self-upgrade")
                .about("Update Cask to the newest version"),
        )
        .subcommand(
            Command::new("self-uninstall").about("Uninstall cask itself and installed package"),
        )
        .subcommand(
            Command::new("clean")
                .visible_alias("clear")
                .about("Clear residual data"),
        )
        .subcommand(Command::new("relink").about("Relink installed packages"))
        .subcommand(
            Command::new("remote")
                .about("Operation for build-in formula")
                .subcommand(
                    Command::new("sync")
                        .about("Sync build-in formula from remote to local")
                        .arg(
                            Arg::new("verbose")
                                .short('v')
                                .long("verbose")
                                .help("Print verbose information")
                                .num_args(0..=1),
                        ),
                )
                .subcommand(
                    Command::new("list")
                        .visible_alias("ls")
                        .about("List build-in formula on remote")
                        .arg(
                            Arg::new("verbose")
                                .short('v')
                                .long("verbose")
                                .help("Print verbose information")
                                .num_args(0..=1),
                        ),
                ),
        );

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
            let package_name = sub_matches.get_one::<String>("PACKAGE").expect("required");

            let version = sub_matches.get_one::<String>("VERSION").map(|x| x.as_str());
            let is_verbose = sub_matches.contains_id("verbose");

            command_install::install(&cask, package_name, version, is_verbose)
                .await
                .expect("install package fail!");
        }
        Some(("uninstall", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("PACKAGE").expect("required");

            command_uninstall::uninstall(&cask, package_name)
                .await
                .expect("uninstall package fail!");
        }
        Some(("list", sub_matches)) => {
            let is_print_as_json = sub_matches.contains_id("json");
            command_list::list(&cask, is_print_as_json)
                .await
                .expect("list packages fail!");
        }
        Some(("info", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("PACKAGE").expect("required");

            command_info::info(&cask, package_name)
                .await
                .expect("info installed package fail!");
        }
        Some(("update", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("PACKAGE").expect("required");
            let is_check_only = sub_matches.contains_id("check-only");
            let is_verbose = sub_matches.contains_id("verbose");

            command_update::update(&cask, package_name, is_check_only, is_verbose)
                .await
                .expect("update package fail!");
        }
        Some(("homepage", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("PACKAGE").expect("required");

            command_homepage::homepage(&cask, package_name)
                .await
                .expect("open homepage of package fail!");
        }
        Some(("check-updates", sub_matches)) => {
            let is_check_only = sub_matches.contains_id("check-only");
            let is_verbose = sub_matches.contains_id("verbose");

            command_check_updates::check_updates(&cask, is_check_only, is_verbose)
                .await
                .expect("check-updates of packages fail!");
        }
        Some(("clean", _sub_matches)) => {
            command_clean::clean(&cask).await.expect("clean fail!");
        }
        Some(("relink", _sub_matches)) => {
            command_relink::relink(&cask).await.expect("relink fail!");
        }
        Some(("self-update", _sub_matches)) => {
            command_self_update::self_update(&cask)
                .await
                .expect("self-update fail!");
        }
        Some(("self-uninstall", _sub_matches)) => {
            command_self_uninstall::self_uninstall(&cask)
                .await
                .expect("self-uninstall fail!");
        }
        Some(("remote", sub_matches)) => match sub_matches.subcommand() {
            Some(("sync", sync_sub_matches)) => {
                let is_verbose = sync_sub_matches.contains_id("verbose");
                command_remote_sync::sync(&cask, is_verbose).expect("sync build-in formula fail!");
            }
            Some(("list", sync_sub_matches)) => {
                let is_verbose = sync_sub_matches.contains_id("verbose");
                command_remote_list::list(&cask, is_verbose).expect("list build-in formula fail!");
            }
            _ => {
                let sub_cmd = app.find_subcommand_mut("remote").unwrap();
                sub_cmd.print_help().unwrap();
                process::exit(0x1);
            }
        },
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<String>("")
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
