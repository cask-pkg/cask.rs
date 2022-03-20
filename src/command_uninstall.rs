#![deny(warnings)]

use crate::cask;

use std::fs;

use colored::Colorize;
use eyre::Report;

pub async fn uninstall(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let packages = cask.list_formula()?;

    let package_formula = packages
        .iter()
        .find(|p| p.package.name == package_name)
        .or_else(|| packages.iter().find(|p| p.package.bin == package_name))
        .ok_or_else(|| {
            eyre::format_err!("can not found the installed package '{}'", package_name)
        })?;

    let package_dir = package_formula.filepath.parent().unwrap();

    fs::remove_dir_all(package_dir)?;

    // remove symlink file
    if cfg!(unix) {
        let symlink_file = cask.bin_dir().join(&package_formula.package.bin);
        if symlink_file.exists() {
            fs::remove_file(symlink_file).ok();
        }
    } else {
        let bat_file_path = cask
            .bin_dir()
            .join(package_formula.package.bin.clone() + ".bat");
        let bash_file_path = cask.bin_dir().join(&package_formula.package.bin);

        fs::remove_file(bat_file_path).ok();
        fs::remove_file(bash_file_path).ok();
    }

    eprintln!(
        "{}",
        format!(
            "The package {} has been uninstalled and remove command {}!",
            package_formula.package.name.underline().red(),
            package_formula.package.bin.red()
        )
        .green()
    );

    Ok(())
}
