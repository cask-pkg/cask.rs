#![deny(warnings)]

use std::{collections::HashMap, io};

use crate::cask;

use eyre::Report;

pub fn sync(cask: &cask::Cask, is_verbose: bool) -> Result<(), Report> {
    let mirror_dir = cask.build_in_formula_dir();

    if mirror_dir.exists() {
        eprintln!("Updating build-in formula...");

        if is_verbose {
            let mut stderr = io::stderr();
            let mut output = shell::Output::Writer(&mut stderr);
            shell::run(&mirror_dir, "git fetch", &mut output, HashMap::from([]))?;
            shell::run(
                &mirror_dir,
                "git checkout main",
                &mut output,
                HashMap::from([]),
            )?;
            shell::run(&mirror_dir, "git clean -df", &mut output, HashMap::from([]))?;
            shell::run(
                &mirror_dir,
                "git pull --rebase",
                &mut output,
                HashMap::from([]),
            )?;
        } else {
            let mut output = shell::Output::None;
            shell::run(
                &mirror_dir,
                "git checkout ./",
                &mut output,
                HashMap::from([]),
            )?;
            shell::run(&mirror_dir, "git clean -df", &mut output, HashMap::from([]))?;
            shell::run(
                &mirror_dir,
                "git pull --rebase",
                &mut output,
                HashMap::from([]),
            )?;
        };
    } else {
        eprintln!("Pulling build-in formula...");

        let client = git::new("https://github.com/cask-pkg/cask-core")?;

        client.clone(
            &mirror_dir,
            git::CloneOption {
                depth: Some(1),
                quiet: Some(true),
                verbose: Some(is_verbose),
                progress: Some(true),
                single_branch: Some(true),
                dissociate: Some(true),
                filter: Some("tree:0".to_string()),
            },
        )?
    }

    eprintln!("Sync remote build-in formula success");

    Ok(())
}
