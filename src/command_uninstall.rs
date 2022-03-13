#![deny(warnings)]

use crate::formula;

use std::fs;

use eyre::Report;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
struct URLTemplateContext {
    name: String,
    bin: String,
    version: String,
}

pub async fn uninstall(package_name: &str) -> Result<(), Report> {
    let hash_of_package = {
        let mut hasher = Sha256::new();

        hasher.update(package_name);
        format!("{:x}", hasher.finalize())
    };

    let home_dir = match dirs::home_dir() {
        Some(p) => Ok(p),
        None => Err(eyre::format_err!("can not get $HOME dir")),
    }?;

    let cask_dir = home_dir.join(".cask");
    let cask_bin_dir = cask_dir.join("bin");
    let cask_formula_dir = cask_dir.join("formula");
    let package_dir = cask_formula_dir.join(hash_of_package);
    let formula_file_path = package_dir.join("Cask.toml");

    if formula_file_path.exists() {
        let package_formula = formula::new(&formula_file_path)?;

        let bin_name = if cfg!(target_os = "windows") {
            format!("{}.exe", &package_formula.package.bin)
        } else {
            package_formula.package.bin
        };

        let symlink_file = cask_bin_dir.join(bin_name);

        if symlink_file.exists() {
            fs::remove_file(symlink_file)?;
        }
    }

    if package_dir.exists() {
        fs::remove_dir_all(package_dir)?;
    }

    Ok(())
}
