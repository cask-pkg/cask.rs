#![deny(warnings)]

use crate::cask;

use chrono::prelude::*;
use eyre::Report;
use serde::{Deserialize, Serialize};
use tabled::{Style, Table, Tabled};

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct PackageInfo {
    name: String,
    bin: String,
    version: String,
    #[serde(skip)]
    install_at: String,
    #[header(hidden = true)]
    create_at: String,
}

pub async fn list(cask: &cask::Cask, is_print_as_json: bool) -> Result<(), Report> {
    let mut packages: Vec<PackageInfo> = vec![];

    for package in cask.list_formula()? {
        let cask_info = package.cask.ok_or_else(|| {
            eyre::format_err!(
                "can not parse cask property of package '{}'",
                package.package.name
            )
        })?;

        let create_at = DateTime::parse_from_str(&cask_info.created_at, "%+")
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        packages.push(PackageInfo {
            name: cask_info.name,
            bin: package.package.bin,
            version: cask_info.version,
            install_at: create_at,
            create_at: cask_info.created_at,
        });
    }

    packages.sort_by(|a, b| {
        let t1 = DateTime::parse_from_str(&a.create_at, "%+").unwrap();
        let t2 = DateTime::parse_from_str(&b.create_at, "%+").unwrap();

        t2.cmp(&t1)
    });

    let table = Table::new(&packages).with(Style::psql()).to_string();

    if is_print_as_json {
        let serialized = serde_json::to_string(&packages).unwrap();
        println!("{}", serialized);
    } else {
        print!("{}", table);
    }

    Ok(())
}
