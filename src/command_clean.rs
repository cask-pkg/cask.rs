#![deny(warnings)]

use crate::cask;
use crate::formula;

use std::fs;
use std::io::ErrorKind;

use eyre::Report;

pub async fn clean(cask: &cask::Cask) -> Result<(), Report> {
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
                fs::remove_file(&resource_file_path.path()).map_err(|err| {
                    eyre::format_err!(
                        "can not remove file '{}': {}",
                        resource_file_path.path().display(),
                        err
                    )
                })?;
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
                let path_str = path.to_string_lossy().to_string();
                let filename = entry.file_name();

                // if the file is not package binary file
                // then is should be removed
                if *filename.to_string_lossy() != bin_name {
                    let symlink = cask.bin_dir().join(f.package.bin.clone());

                    if symlink.is_symlink() {
                        match fs::read_link(&symlink) {
                            Ok(p) => {
                                // if symlink is point to the binary file, then remove it
                                if p.as_os_str().to_string_lossy() == path_str {
                                    if let Ok(()) = fs::remove_file(&symlink) {
                                        eprintln!(
                                            "symlink file '{}' has been removed",
                                            symlink.display()
                                        );
                                    }
                                }
                            }
                            Err(err) => {
                                if err.kind() == ErrorKind::NotFound {
                                    // try to remove and ignore error
                                    if let Ok(()) = fs::remove_file(&symlink) {
                                        eprintln!(
                                            "symlink file '{}' has been removed",
                                            symlink.display()
                                        );
                                    }
                                }
                            }
                        };
                    } else if symlink.is_file() {
                        // shell script
                        {
                            let file_content = fs::read_to_string(&symlink).map_err(|err| {
                                eyre::format_err!(
                                    "can not read file '{}': {}",
                                    symlink.display(),
                                    err
                                )
                            })?;
                            if file_content.contains(&path_str) {
                                if let Ok(()) = fs::remove_file(&symlink) {
                                    eprintln!(
                                        "shell script '{}' has been removed",
                                        symlink.display()
                                    );
                                }
                            }
                        }

                        // batch script
                        {
                            let bat_file_path = path
                                .parent()
                                .ok_or_else(|| {
                                    eyre::format_err!(
                                        "cant not get parent folder of '{}'",
                                        path.display()
                                    )
                                })?
                                .join(f.package.bin.clone() + ".bat");

                            if bat_file_path.exists() {
                                let file_content =
                                    fs::read_to_string(&bat_file_path).map_err(|err| {
                                        eyre::format_err!(
                                            "can not read file '{}': {}",
                                            bat_file_path.display(),
                                            err
                                        )
                                    })?;

                                if file_content.contains(&path_str) {
                                    if let Ok(()) = fs::remove_file(&bat_file_path) {
                                        eprintln!(
                                            "batch script '{}' has been removed",
                                            bat_file_path.display()
                                        );
                                    }
                                }
                            }
                        }
                    } else if let Ok(()) = fs::remove_file(&symlink) {
                        eprintln!("unknown file '{}' has been removed", symlink.display());
                    }
                }
            }
        }
    }

    // remove broken symlink
    #[cfg(unix)]
    {
        let bin_dir = cask.bin_dir();

        let dir = fs::read_dir(bin_dir)?;

        for entry in dir {
            let file = entry?.path();

            if file.is_symlink() {
                match fs::read_link(&file) {
                    Ok(_) => (),
                    Err(err) => {
                        if err.kind() == ErrorKind::NotFound {
                            // try to remove and ignore error
                            if let Ok(()) = fs::remove_file(&file) {
                                eprintln!("broken symlink '{}' has been removed", file.display());
                            }
                        }
                    }
                }
            }
        }
    }

    eprintln!("clear!");

    Ok(())
}
