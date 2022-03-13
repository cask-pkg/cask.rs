#![deny(warnings)]

use crate::formula;

use eyre::Report;

pub async fn search(package_name: &str) -> Result<(), Report> {
    let package_formula = formula::fetch(package_name)?;

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
