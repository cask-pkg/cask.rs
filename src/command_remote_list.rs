#![deny(warnings)]

use std::{fs, path::Path};

use crate::{cask, command_remote_sync, formula};

use eyre::Report;

fn print_formula(dir_path: &Path) -> Result<(), Report> {
    let dir = fs::read_dir(dir_path)?;

    for entry in dir.into_iter().filter_map(|f| f.ok()) {
        let p = entry.path();

        if p.is_dir() {
            print_formula(&p)?
        } else if entry.file_name().to_str().unwrap() == "Cask.toml" {
            let f = formula::new(&p, "")?;

            println!("{}", f.package.name)
        }
    }

    Ok(())
}

pub fn list(cask: &cask::Cask, is_verbose: bool) -> Result<(), Report> {
    let mirror_dir = cask.build_in_formula_dir();

    command_remote_sync::sync(cask, is_verbose)?;

    print_formula(&mirror_dir)
}
