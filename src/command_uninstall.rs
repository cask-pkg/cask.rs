#![deny(warnings)]

use crate::cask;
use crate::formula;

use std::fs;

use eyre::Report;

pub async fn uninstall(cask: cask::Cask, package_name: &str) -> Result<(), Report> {
    let formula_file_path = cask.package_dir(package_name).join("Cask.toml");

    // remove symlink file
    if formula_file_path.exists() {
        let package_formula = formula::new(&formula_file_path)?;

        #[cfg(target_family = "unix")]
        let executable_name = package_formula.package.bin;
        #[cfg(target_family = "windows")]
        let executable_name = format!("{}.exe", &package_formula.package.bin);

        let symlink_file = cask.bin_dir().join(executable_name);

        if symlink_file.exists() {
            fs::remove_file(symlink_file)?;
        }
    }

    if cask.package_dir(package_name).exists() {
        fs::remove_dir_all(cask.package_dir(package_name))?;
    } else {
        return Err(eyre::format_err!(
            "can not found the installed package '{}'",
            package_name
        ));
    }

    println!("uninstall {} success!", package_name);

    Ok(())
}
