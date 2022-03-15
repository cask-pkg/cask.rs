#![deny(warnings)]

use crate::cask;
use crate::formula;

use std::fs;

use eyre::Report;

pub async fn uninstall(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let package_dir = cask.package_dir(package_name);

    let formula_file_path = package_dir.join("Cask.toml");

    // remove symlink file
    if formula_file_path.exists() {
        let package_formula = formula::new(&formula_file_path)?;

        if cfg!(unix) {
            let symlink_file = cask.bin_dir().join(package_formula.package.bin);
            if symlink_file.exists() {
                fs::remove_file(symlink_file).ok();
            }
        } else {
            let bat_file_path = cask
                .bin_dir()
                .join(package_formula.package.bin.clone() + ".bat");
            let bash_file_path = cask.bin_dir().join(package_formula.package.bin);

            fs::remove_file(bat_file_path).ok();
            fs::remove_file(bash_file_path).ok();
        }
    }

    if package_dir.exists() {
        fs::remove_dir_all(package_dir)?;
    } else {
        return Err(eyre::format_err!(
            "can not found the installed package '{}'",
            package_name
        ));
    }

    eprintln!("The package '{}' has been uninstalled!", package_name);

    Ok(())
}
