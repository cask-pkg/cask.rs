#![deny(warnings)]

use crate::cask;
use crate::formula;

use std::fs;

use eyre::Report;

pub async fn clean(cask: cask::Cask) -> Result<(), Report> {
    // clear formula dir
    let formula_dir = cask.formula_dir();

    let dir = fs::read_dir(formula_dir)?;

    for entry in dir {
        let file = entry?;
        let path = file.path();
        let bin_dir = path.join("bin");
        let version_dir = path.join("version");

        if !path.is_dir() {
            continue;
        }

        // clear version
        {
            for download_resource in fs::read_dir(version_dir)? {
                let resource_file_path = download_resource?;
                fs::remove_file(&resource_file_path.path())?;
            }
        }

        let f = formula::new(&path.join("Cask.toml"))?;

        #[cfg(unix)]
        let bin_name = f.package.bin.clone();
        #[cfg(windows)]
        let bin_name = f.package.bin.clone() + "exe";

        // clear bin of formula
        {
            for bin_entry in fs::read_dir(bin_dir)? {
                let entry = bin_entry?;
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                let filename = entry.file_name();

                if *filename.to_str().unwrap() != bin_name {
                    // remove extra executable
                    let symlink = cask.bin_dir().join(f.package.bin.clone());

                    if symlink.is_symlink() {
                        match fs::read_link(&symlink) {
                            Ok(p) => {
                                // if symlink is point to the binary file, then remove it
                                if p.as_os_str().to_str().unwrap() == path_str {
                                    fs::remove_file(&symlink).ok();
                                }
                            }
                            Err(_) => {
                                // if path does not exist. then remove the symlink
                                fs::remove_file(&symlink).ok();
                            }
                        };
                    } else if symlink.is_file() {
                        // shell script
                        {
                            let file_content = fs::read_to_string(&symlink)?;
                            if file_content.contains(path_str) {
                                fs::remove_file(symlink).ok();
                            }
                        }

                        // batch script
                        {
                            let bat_file_path =
                                path.parent().unwrap().join(f.package.bin.clone() + ".bat");
                            let file_content = fs::read_to_string(&bat_file_path)?;
                            if file_content.contains(path_str) {
                                fs::remove_file(bat_file_path).ok();
                            }
                        }
                    } else {
                        fs::remove_file(symlink).ok();
                    }
                }
            }
        }
    }

    Ok(())
}
