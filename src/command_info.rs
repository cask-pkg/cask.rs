#![deny(warnings)]

use crate::cask;
use crate::formula;

use eyre::Report;

pub async fn info(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let packages = cask.list_formula()?;

    let package = packages
        .iter()
        .find(|p| p.package.name == package_name)
        .or_else(|| packages.iter().find(|p| p.package.bin == package_name));

    if let Some(package_formula) = package {
        let cask_info = &package_formula.cask.as_ref().ok_or_else(|| {
            eyre::format_err!("can not parse cask property of file '{}'", package_name)
        })?;

        let msg = format!(
            r#"{}
            Package: {}
            Version: {}
            Repository: {}
            Location: {}
            Installed: true
            "#,
            package_formula.package.description,
            cask_info.name,
            cask_info.version,
            package_formula.package.repository,
            package_formula
                .filepath
                .parent()
                .ok_or_else(|| eyre::format_err!(
                    "can not get parent folder of '{}'",
                    package_formula.filepath.display()
                ))?
                .display()
        )
        .lines()
        .map(|s| s.trim_start().to_owned())
        .collect::<Vec<String>>()
        .join("\n");

        print!("{}", msg);

        let remote_versions = &package_formula.get_versions()?;

        println!("Remote Versions:");

        for v in remote_versions {
            println!("{}", v);
        }

        Ok(())
    } else {
        let package_formula = formula::fetch(cask, package_name, true)?;

        let msg = format!(
            r#"{}
            Package: {}
            Repository: {}
            Installed: false
            "#,
            package_formula.package.description,
            package_formula.package.name,
            package_formula.package.repository
        )
        .lines()
        .map(|s| s.trim_start().to_owned())
        .collect::<Vec<String>>()
        .join("\n");

        print!("{}", msg);

        let remote_versions = &package_formula.get_versions()?;

        println!("Remote Versions:");

        for v in remote_versions {
            println!("{}", v);
        }

        Ok(())
    }
}
