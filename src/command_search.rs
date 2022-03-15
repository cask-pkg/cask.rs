#![deny(warnings)]

use crate::cask;
use crate::formula;

use eyre::Report;

pub async fn search(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let package_formula = formula::fetch(cask, package_name, true)?;

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
