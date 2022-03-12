#![deny(warnings)]

extern crate flate2;
extern crate tar;

use crate::formula;
use crate::git;

use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use eyre::Report;

pub async fn info(package_name: &str) -> Result<(), Report> {
    let url = format!("https://{}-cask.git", package_name);

    let unix_time = {
        let start = SystemTime::now();

        let t = start.duration_since(UNIX_EPOCH)?;

        t.as_secs()
    };

    let formula_cloned_dir = env::temp_dir().join(format!("cask_{}", unix_time));
    let cask_file_path = formula_cloned_dir.join("Cask.toml");

    let package_formula = match git::clone(&url, &formula_cloned_dir, vec!["--depth", "1"]) {
        Ok(()) => {
            if !cask_file_path.exists() {
                // remove cloned repo
                fs::remove_dir_all(formula_cloned_dir)?;
                return Err(eyre::format_err!(
                    "{} is not a valid formula!",
                    package_name
                ));
            }

            let f = formula::new(&cask_file_path)?;

            Ok(f)
        }
        Err(e) => Err(e),
    }?;

    // remove cloned repo
    fs::remove_dir_all(formula_cloned_dir)?;

    let msg = format!(
        r#"{}
Package: {}
Repository: {}"#,
        package_formula.package.description,
        package_formula.package.name,
        package_formula.package.repository
    );

    print!("{}", msg);

    Ok(())
}
