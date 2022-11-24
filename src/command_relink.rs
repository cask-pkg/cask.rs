#![deny(warnings)]

use crate::{cask, symlink};

use std::fs;

use eyre::Report;

pub async fn relink(cask: &cask::Cask) -> Result<(), Report> {
    let list = cask.list_formula()?;

    for package_formula in list {
        let symlink_file = cask.bin_dir().join(&package_formula.package.bin);

        let package_dir = cask.package_dir(&package_formula.package.name);

        #[cfg(target_family = "unix")]
        let executable_name = package_formula.package.bin.clone();
        #[cfg(target_family = "windows")]
        let executable_name = format!("{}.exe", &package_formula.package.bin);

        let output_file_path = package_dir.join("bin").join(executable_name);

        // unlink before symlink
        {
            fs::remove_file(&symlink_file).ok();

            #[cfg(target_family = "windows")]
            fs::remove_file(format!("{}.bat", &symlink_file.display())).ok();
        }

        symlink::symlink(
            &output_file_path,
            &symlink_file,
            &package_formula.package.name,
        )?;
    }

    Ok(())
}
