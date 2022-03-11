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
                fs::remove_dir_all(dest_dir)?;
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

    let arch = option_arch.unwrap_or_else(|| panic!("{} not support your arch", package_name));

    println!("{}", arch.url);

    let dest = cwd.join("gpm.tar.gz");

    download::download(&arch.url, &dest)?;

    Ok(())
}
