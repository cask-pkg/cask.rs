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

pub async fn check_updates(
    cask: &cask::Cask,
    is_check_only: bool,
    is_verbose: bool,
) -> Result<(), Report> {
    let mut packages: Vec<PackageInfo> = vec![];

    let package_list = match cask.list_formula() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error listing formulas: {}", e);
            return Err(e);
        }
    };

    for package in package_list {
        eprintln!("Checking {} for update...", package.package.name);

        let latest_version_op = match package.get_latest_version() {
            Ok(ver) => ver,
            Err(e) => {
                eprintln!(
                    "Error getting latest version for {}: {}",
                    package.package.name, e
                );
                continue;
            }
        };

        if latest_version_op.is_none() {
            continue;
        }

        let latest_version_str = latest_version_op.unwrap();

        let cask_info = match package.cask {
            Some(info) => info,
            None => {
                eprintln!(
                    "No cask info available for package {}",
                    package.package.name
                );
                continue;
            }
        };

        let current = match Version::parse(&cask_info.version) {
            Ok(ver) => ver,
            Err(e) => {
                eprintln!(
                    "Error parsing current version for {}: {}",
                    package.package.name, e
                );
                continue;
            }
        };

        let latest = match Version::parse(&latest_version_str) {
            Ok(ver) => ver,
            Err(e) => {
                eprintln!(
                    "Error parsing latest version for {}: {}",
                    package.package.name, e
                );
                continue;
            }
        };

        if latest > current {
            packages.push(PackageInfo {
                name: package.package.name,
                bin: package.package.bin,
                current_version: cask_info.version,
                latest_version: latest_version_str,
            });
        }
    }

    for package in packages {
        eprintln!(
            "{}@{} got an update to {}",
            package.name, package.current_version, package.latest_version
        );

        if !is_check_only {
            if let Err(e) = command_install::install(
                cask,
                &package.name,
                Some(&package.latest_version),
                is_verbose,
            )
            .await
            {
                eprintln!("Error installing package {}: {}", package.name, e);
            }
        }
    }

    Ok(())
}
