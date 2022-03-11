mod config;
mod git;

use clap::{arg, Arg, Command, PossibleValue};
use config::Configure;
use inquire::{error::InquireError, Confirm, Select, Text};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::process;

fn main() {
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

            let url = format!("https://{}-cask.git", package_name);

            let cwd = env::current_dir().unwrap();

            let dest_dir = &cwd.join("dist").join(package_name);

            let mut config: Configure;

            let option_target = match git::clone(&url, &dest_dir, vec![]) {
                Ok(()) => {
                    println!("clone success");
                    let config_file_path = dest_dir.join("Cask.toml");
                    config = config::new(&config_file_path).unwrap();

                    #[cfg(target_os = "macos")]
                    let target = config.darwin;
                    #[cfg(target_os = "linux")]
                    let target = config.linux;
                    #[cfg(target_os = "windows")]
                    let target = config.windows;

                    target
                }
                Err(_) => {
                    if dest_dir.exists() {
                        fs::remove_dir_all(dest_dir).unwrap();
                    }
                    process::exit(0x1);
                }
            };

            let target =
                option_target.unwrap_or_else(|| panic!("{} not support your system", package_name));

            let option_arch = if cfg!(target_arch = "x86") {
                target.ia32
            } else if cfg!(target_arch = "x86_64") {
                target.amd64
            } else {
                None
            };

            let arch =
                option_arch.unwrap_or_else(|| panic!("{} not support your arch", package_name));

            println!("{}", arch.url);

            let resp = reqwest::blocking::get(arch.url).unwrap();
            let content = resp.text().unwrap();
            let mut dest = {
                let f_name = cwd.join("gpm.tar.gz");
                File::create(f_name).unwrap()
            };
            copy(&mut content.as_bytes(), &mut dest).unwrap();
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
