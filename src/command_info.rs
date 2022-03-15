#![deny(warnings)]

use crate::cask;

use eyre::Report;

pub async fn info(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let package_dir = cask.package_dir(package_name);
    let package_formula = cask.package_formula(package_name)?;

    let cask_info = package_formula.cask.ok_or_else(|| {
        eyre::format_err!("can not parse cask property of file '{}'", package_name)
    })?;

    let msg = format!(
        r#"{}
Package: {}
Version: {}
Repository: {}
Location: {}"#,
        package_formula.package.description,
        cask_info.name,
        cask_info.version,
        package_formula.package.repository,
        package_dir.display()
    );

    print!("{}", msg);

    Ok(())
}
