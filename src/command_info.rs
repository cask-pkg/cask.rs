#![deny(warnings)]

use crate::cask;

use eyre::Report;

pub async fn info(cask: &cask::Cask, package_name: &str) -> Result<(), Report> {
    let package_dir = cask.package_dir(package_name);
    let package_formula = &cask.package_formula(package_name)?;

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
        package_dir.display()
    );

    for v in remote_versions {
        msg.push_str(v);
        msg.push('\n');
    }

    print!("{}", msg);

    Ok(())
}
