use crate::config;
use crate::download;
use crate::git;
use eyre::Report;
use std::env;
use std::fs;
use std::process;

pub fn install(package_name: &str) -> Result<(), Report> {
    let url = format!("https://{}-cask.git", package_name);

    let cwd = env::current_dir()?;

    let dest_dir = &cwd.join("dist").join(package_name);

    let option_target = match git::clone(&url, dest_dir, vec![]) {
        Ok(()) => {
            println!("clone success");
            let config_file_path = dest_dir.join("Cask.toml");
            let config = config::new(&config_file_path)?;

            let target = if cfg!(target_os = "macos") {
                config.darwin
            } else if cfg!(target_os = "windows") {
                config.windows
            } else if cfg!(target_os = "linux") {
                config.linux
            } else {
                panic!("not support your system")
            };

            target
        }
        Err(_) => {
            if dest_dir.exists() {
                fs::remove_dir_all(dest_dir)?;
            }
            process::exit(0x1);
        }
    };

    let target = match option_target {
        Some(p) => Ok(p),
        None => Err(eyre::format_err!(
            "{} not support your system",
            package_name
        )),
    }?;

    let option_arch = if cfg!(target_arch = "x86") {
        target.ia32
    } else if cfg!(target_arch = "x86_64") {
        target.amd64
    } else {
        None
    };

    let arch = match option_arch {
        Some(a) => Ok(a),
        None => Err(eyre::format_err!("{} not support your arch", package_name)),
    }?;

    println!("{}", arch.url);

    let dest = cwd.join("gpm.tar.gz");

    download::download(&arch.url, &dest)?;

    Ok(())
}
