#![deny(warnings)]

use crate::cask;

use eyre::Report;

pub async fn homepage(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let packages = cask.list_formula()?;

    let package_formula = packages
        .iter()
        .find(|p| p.package.name == package_name)
        .or_else(|| packages.iter().find(|p| p.package.bin == package_name))
        .ok_or_else(|| {
            eyre::format_err!("can not found the installed package '{}'", package_name)
        })?;

    if let Some(homepage) = &package_formula.package.homepage {
        opener::open(homepage)?;
    } else {
        opener::open(package_formula.package.repository.clone())?;
    }

    Ok(())
}
