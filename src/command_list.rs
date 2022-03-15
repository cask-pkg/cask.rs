#![deny(warnings)]

use std::fs;

use crate::cask;
use crate::formula;

use eyre::Report;

pub async fn list(cask: &cask::Cask) -> Result<(), Report> {
    cask.init()?;

    let formula_dir = cask.formula_dir();

    let dir = fs::read_dir(formula_dir)?;

    for entry in dir {
        let file = entry?;
        let path = file.path();

        if !path.is_dir() {
            continue;
        }

        let cask_file_path = path.join("Cask.toml");

        let f = match formula::new(&cask_file_path) {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!("Cannot read Cask.toml in '{}': {}", path.display(), e);
                None
            }
        };

        if let Some(f) = f {
            let cask_info = f.cask.ok_or_else(|| {
                eyre::format_err!(
                    "can not parse cask property of file '{}'",
                    cask_file_path.display()
                )
            })?;

            println!("{} {}", cask_info.name, cask_info.version);
        }
    }

    Ok(())
}
