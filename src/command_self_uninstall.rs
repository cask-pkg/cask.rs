#![deny(warnings)]

use std::{env, error::Error, fs, path::PathBuf};

use crate::cask;

use eyre::Report;

pub async fn self_uninstall(cask: &cask::Cask) -> Result<(), Report> {
    let root_dir = cask.root_dir();

    let exe_path = env::current_exe()?;

    fs::remove_dir_all(root_dir)?;

    fn when_delete_fail(_e: impl Error, filepath: PathBuf) -> Report {
        eprintln!("self uninstall fail");
        eyre::format_err!("try delete file '{}' manually.", filepath.display())
    }

    if exe_path.is_symlink() {
        let real_exe_path = fs::read_link(&exe_path)?;

        fs::remove_file(&exe_path).map_err(|e| when_delete_fail(e, exe_path))?;
        fs::remove_file(&real_exe_path).map_err(|e| when_delete_fail(e, real_exe_path))?;
    } else {
        fs::remove_file(&exe_path).map_err(|e| when_delete_fail(e, exe_path))?;
    }

    eprintln!("self uninstall success");

    Ok(())
}
