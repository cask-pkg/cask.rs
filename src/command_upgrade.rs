#![deny(warnings)]

use crate::cask;
use crate::command_install;
use crate::formula;

use colored::Colorize;
use eyre::Report;
use semver::Version;

pub async fn upgrade(
    cask: &cask::Cask,
    package_name: &str,
    is_check_only: bool,
) -> Result<(), Report> {
    let packages = cask.list_formula()?;

    let package_formula = packages
        .iter()
        .find(|p| p.package.name == package_name)
        .or_else(|| packages.iter().find(|p| p.package.bin == package_name))
        .ok_or_else(|| {
            eyre::format_err!("can not found the installed package '{}'", package_name)
        })?;

    let cask_info = package_formula.cask.as_ref().ok_or_else(|| {
        eyre::format_err!(
            "can not parse cask property of file '{}'",
            &package_formula.package.name
        )
    })?;

    let current = Version::parse(&cask_info.version)
        .map_err(|e| eyre::format_err!("invalid semver version '{}': {}", &cask_info.version, e))?;

    let remote_formula = formula::fetch(cask, &package_formula.package.name, true)?;

    let remote_versions = remote_formula.get_versions()?;

    let err_not_found_release = eyre::format_err!(
        "can not found any version on '{}' remote",
        &package_formula.package.name.underline()
    );

    if remote_versions.is_empty() {
        return Err(err_not_found_release);
    }

    let latest_str = &remote_versions.first().ok_or(err_not_found_release)?;

    let latest = Version::parse(latest_str)
        .map_err(|e| eyre::format_err!("invalid semver version '{}': {}", latest_str, e))?;

    if latest <= current {
        eprintln!(
            "You are using the latest version of {}",
            &package_formula.package.name.green()
        );
        return Ok(());
    }

    if is_check_only {
        eprintln!(
            "Found latest version {} of {}, but using {} currently",
            latest.to_string().green(),
            &package_formula.package.name.underline(),
            cask_info.version.green()
        );
    } else {
        command_install::install(cask, &package_formula.package.name, Some(latest_str)).await?;

        eprintln!(
            "Upgrade {}@{} from  to '{}' finish!",
            &package_formula.package.name.underline(),
            cask_info.version.red(),
            latest.to_string().green()
        );
    }

    Ok(())
}
