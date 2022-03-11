use crate::config;
use crate::download;
use crate::git;
use eyre::Report;
use std::env;
use std::fs;
use std::process;

pub async fn install(package_name: &str) -> Result<(), Report> {
    let url = format!("https://{}-cask.git", package_name);

    let cwd = env::current_dir()?;

    // TODO:: use custom folder
    let dest_dir = &cwd.join("dist").join(package_name);

    if dest_dir.exists() {
        fs::remove_dir_all(dest_dir)?;
    }

    let option_target = match git::clone(&url, dest_dir, vec![]) {
        Ok(()) => {
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
        target.x86
    } else if cfg!(target_arch = "x86_64") {
        target.x86_64
    } else if cfg!(target_arch = "arm") {
        target.arm
    } else if cfg!(target_arch = "aarch64") {
        target.aarch64
    } else if cfg!(target_arch = "mips") {
        target.mips
    } else if cfg!(target_arch = "mips64") {
        target.mips64
    } else if cfg!(target_arch = "mips64el") {
        target.mips64el
    } else {
        None
    };

    let arch = match option_arch {
        Some(a) => Ok(a),
        None => Err(eyre::format_err!("{} not support your arch", package_name)),
    }?;

    let dest = cwd.join("gpm.tar.gz");

    download::download(&arch.url, &dest).await?;

    Ok(())
}
