#![deny(warnings)]

use crate::cask;
use crate::command_install;
use crate::formula;

use eyre::Report;
use semver::Version;

pub async fn upgrade(
    cask: &cask::Cask,
    package_name: &str,
    is_check_only: bool,
) -> Result<(), Report> {
    let package_formula = cask.package_formula(package_name)?;

    let cask_info = package_formula.cask.ok_or_else(|| {
        eyre::format_err!("can not parse cask property of file '{}'", package_name)
    })?;

    let current = Version::parse(&cask_info.version)
        .map_err(|e| eyre::format_err!("invalid semver version '{}': {}", &cask_info.version, e))?;

    let remote_formula = formula::fetch(cask, package_name, true)?;

    let remote_versions = remote_formula.get_versions()?;

    if remote_versions.is_empty() {
        return Err(eyre::format_err!(
            "can not found any version on '{}' remote",
            package_name
        ));
    }

    let latest_str = &remote_versions[0];

    let latest = Version::parse(latest_str)
        .map_err(|e| eyre::format_err!("invalid semver version '{}': {}", latest_str, e))?;

    if latest <= current {
        eprintln!("You have used the latest version of '{}'", package_name);
        return Ok(());
    }

    if is_check_only {
        eprintln!(
            "Found latest version {} of {}, but current using {}",
            latest, package_name, cask_info.version
        );
    } else {
        command_install::install(cask, package_name, Some(latest_str)).await?;

        eprintln!(
            "Upgrade {}@{} from  to '{}' finish!",
            package_name, cask_info.version, latest
        );
    }

    Ok(())
}
