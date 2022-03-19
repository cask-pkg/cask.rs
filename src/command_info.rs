#![deny(warnings)]

use crate::cask;

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

        let remote_versions = &package_formula.get_versions()?;

        let mut msg = format!(
            r#"{}
Package: {}
Version: {}
Repository: {}
Location: {}
Remote Versions:
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
        );

        for v in remote_versions {
            msg.push_str(v);
            msg.push('\n');
        }

        print!("{}", msg);

        Ok(())
    } else {
        return Err(eyre::format_err!(
            "can not found the installed package '{}'",
            package_name
        ));
    }
}
