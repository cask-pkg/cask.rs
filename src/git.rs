#![deny(warnings)]

use core::result::Result;
use std::path::Path;
use std::process::{Command as ChildProcess, Stdio};

use eyre::Report;

pub fn clone(url: &str, dest: &Path, args: Vec<&str>) -> Result<(), Report> {
    match ChildProcess::new("git")
        .arg("clone")
        .args(args)
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg(dest.to_str().unwrap())
        .spawn()
    {
        Ok(mut child) => match child.wait() {
            Ok(state) => {
                if state.success() {
                    Ok(())
                } else {
                    Err(Report::msg("git clone process fail"))
                }
            }
            Err(e) => Err(eyre::format_err!("run git clone command exit: {}", e)),
        },
        Err(e) => Err(eyre::format_err!("run git clone command fail: {}", e)),
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
