#![deny(warnings)]

use core::result::Result;
use std::path::Path;
use std::process::Command as ChildProcess;

use eyre::Report;

pub fn clone(url: &str, dest: &Path, args: Vec<&str>) -> Result<(), Report> {
    match ChildProcess::new("git")
        .arg("clone")
        .args(args)
        .arg(url)
        .arg(dest.to_str().unwrap())
        .spawn()
    {
        Ok(mut child) => match child.wait() {
            Ok(state) => {
                if state.success() {
                    Ok(())
                } else {
                    Err(eyre::format_err!(
                        "exit code: {}",
                        state.code().unwrap_or(1),
                    ))
                }
            }
            Err(e) => Err(eyre::format_err!("{}", e)),
        },
        Err(e) => Err(eyre::format_err!("{}", e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::git;
    use std::{fs, path::Path};

    #[test]
    fn test_clone() {
        let url1 = "https://github.com/axetroy/gpm.rs.git";

        let dest_dir = Path::new("./dist");

        let r1 = git::clone(url1, dest_dir, vec![]);

        assert!(r1.is_ok());
        assert!(dest_dir.exists());

        fs::remove_dir_all(dest_dir).unwrap();
    }
}
