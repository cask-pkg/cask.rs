#![deny(warnings)]

use crate::cask;

use eyre::Report;

pub async fn sync(cask: &cask::Cask) -> Result<(), Report> {
    let mirror_dir = cask.build_in_formula_dir();

    if mirror_dir.exists() {
        shell::run(&mirror_dir, "git checkout ./")?;
        shell::run(&mirror_dir, "git clean -df")?;
        shell::run(&mirror_dir, "git pull --rebase")?;
    } else {
        let client = git::new("https://github.com/cask-pkg/cask-core")?;

        client.clone(
            &mirror_dir,
            git::CloneOption {
                depth: Some(1),
                quiet: Some(true),
                single_branch: Some(true),
                dissociate: Some(true),
                filter: Some("tree:0".to_string()),
            },
        )?
    }

    println!("Fetch remote formula success");

    Ok(())
}
