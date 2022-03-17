#![deny(warnings)]

use crate::cask;
use crate::formula;

use eyre::Report;

pub async fn search(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let package_formula = formula::fetch(cask, package_name, true)?;

    let remote_versions = &package_formula.get_versions()?;

    let mut msg = format!(
        r#"{}
Package: {}
Repository: {}
Remote Versions:
"#,
        package_formula.package.description,
        package_formula.package.name,
        package_formula.package.repository
    );

    for v in remote_versions {
        msg.push_str(v);
        msg.push('\n');
    }

    print!("{}", msg);

    Ok(())
}
