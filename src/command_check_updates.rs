#![deny(warnings)]

use crate::{cask, command_install};

use eyre::Report;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PackageInfo {
    name: String,
    bin: String,
    current_version: String,
    latest_version: String,
}

pub async fn check_updates(cask: &cask::Cask, is_check_only: bool) -> Result<(), Report> {
    let mut packages: Vec<PackageInfo> = vec![];

    for package in cask.list_formula()? {
        eprintln!("Checking {} for update...", package.package.name);

        let latest_version_op = package.get_latest_version()?;

        if latest_version_op.is_none() {
            continue;
        }

        let latest_version_str = latest_version_op.unwrap();

        let cask_info = package.cask.unwrap();

        let current = Version::parse(&cask_info.version).unwrap();
        let latest = Version::parse(&latest_version_str).unwrap();

        if latest > current {
            packages.push(PackageInfo {
                name: package.package.name,
                bin: package.package.bin,
                current_version: cask_info.version,
                latest_version: latest_version_str,
            })
        }
    }

    for package in packages {
        eprintln!(
            "{}@{} get a update to {}",
            package.name, package.current_version, package.latest_version
        );

        if !is_check_only {
            command_install::install(cask, &package.name, Some(&package.latest_version)).await?;
        }
    }

    Ok(())
}
