#![deny(warnings)]

use std::fs;

use crate::cask;
use crate::formula;

use eyre::Report;

pub async fn list(cask: cask::Cask) -> Result<(), Report> {
    cask.init()?;

    let formula_dir = cask.formula_dir();

    let dir = fs::read_dir(formula_dir)?;

    for entry in dir {
        let file = entry?;
        let path = file.path();

        if !path.is_dir() {
            continue;
        }

        let f = match formula::new(&path.join("Cask.toml")) {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!(
                    "Cannot read Cask.toml in '{}': {}",
                    path.as_os_str().to_str().unwrap(),
                    e
                );
                None
            }
        };

        if let Some(f) = f {
            let cask_info = f.cask.unwrap();
            println!("{} {}", cask_info.name, cask_info.version);
        }
    }

    Ok(())
}
