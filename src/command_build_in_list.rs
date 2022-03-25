#![deny(warnings)]

use crate::{cask, command_build_in_sync};

use eyre::Report;

pub fn list(cask: &cask::Cask) -> Result<(), Report> {
    let mirror_dir = cask.build_in_formula_dir();

    if mirror_dir.exists() {
        command_build_in_sync::sync(cask)?;
    }

    // TODO

    Ok(())
}
