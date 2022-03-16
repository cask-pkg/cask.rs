#![deny(warnings)]

use crate::cask;

use eyre::Report;

pub async fn list(cask: &cask::Cask) -> Result<(), Report> {
    cask.init()?;

    for package in cask.list_formula()? {
        let cask_info = package.cask.ok_or_else(|| {
            eyre::format_err!(
                "can not parse cask property of package '{}'",
                package.package.name
            )
        })?;

        println!("{} {}", cask_info.name, cask_info.version);
    }

    Ok(())
}
