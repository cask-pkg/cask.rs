#![deny(warnings)]

use crate::cask;
use crate::command_install;
use crate::formula;

use eyre::Report;

pub async fn upgrade(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let package_formula = cask.package_formula(package_name)?;

    let cask_info = package_formula.cask.ok_or_else(|| {
        eyre::format_err!("can not parse cask property of file '{}'", package_name)
    })?;

    let remote_formula = formula::fetch(cask, package_name, true)?;

    if remote_formula.package.versions.is_empty() {
        return Err(eyre::format_err!(
            "can not found any version on '{}' remote",
            package_name
        ));
    }

    let latest = &remote_formula.package.versions[0];

    if latest == cask_info.version.as_str() {
        eprintln!("You have used the latest version of '{}'", package_name);
        return Ok(());
    }

    command_install::install(cask, package_name, Some(latest)).await?;

    eprintln!(
        "Upgrade {}@{} from  to '{}' finish!",
        package_name, cask_info.version, latest
    );

    Ok(())
}
