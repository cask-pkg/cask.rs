use core::result::Result;
use std::path::Path;
use std::process::{Command as ChildProcess, Stdio};
use std::time::Duration;

use eyre::Report;
use wait_timeout::ChildExt;

pub struct CloneOption {
    pub depth: Option<i32>,
    pub quiet: Option<bool>,
    pub single_branch: Option<bool>,
    pub dissociate: Option<bool>,
    pub filter: Option<String>,
}

// check remote repository exist or not
pub fn check_exist(url: &str) -> Result<bool, Report> {
    let mut child = match ChildProcess::new("git")
        .arg("ls-remote")
        .arg("-h")
        .arg(url)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .env("GIT_TERMINAL_PROMPT", "0")
        .spawn()
    {
        Ok(child) => Ok(child),
        Err(e) => Err(eyre::format_err!("{}", e)),
    }?;

    let timeout = Duration::from_secs(30);

    let state = match child.wait_timeout(timeout)? {
        Some(status) => status.code(),
        None => {
            // child hasn't exited yet
            child.kill()?;
            child.wait()?.code()
        }
    };

    let exit_code = state.unwrap_or(1);

    if exit_code == 0 {
        return Ok(true);
    }

    if exit_code == 128 {
        return Ok(false);
    }

    Ok(false)
}

// clone repository into dest dir
pub fn clone(url: &str, dest: &Path, options: CloneOption) -> Result<(), Report> {
    let mut args: Vec<String> = vec![];

    if let Some(depth) = options.depth {
        args.push(format!("--depth={}", depth))
    }

    if let Some(quiet) = options.quiet {
        if quiet {
            args.push("--quiet".to_string())
        }
    }

    if let Some(dissociate) = options.dissociate {
        if dissociate {
            args.push("--dissociate".to_string())
        }
    }

    if let Some(single_branch) = options.single_branch {
        if single_branch {
            args.push("--single-branch".to_string())
        }
    }

    if let Some(filter) = options.filter {
        args.push(format!("--filter={}", filter))
    }

    let mut child = match ChildProcess::new("git")
        .arg("clone")
        .arg(url)
        .args(args)
        .arg(dest.to_str().unwrap())
        .spawn()
    {
        Ok(child) => Ok(child),
        Err(e) => Err(eyre::format_err!("{}", e)),
    }?;

    let timeout = Duration::from_secs(300); // 5min

    let state = match child.wait_timeout(timeout)? {
        Some(status) => status.code(),
        None => {
            // child hasn't exited yet
            child.kill()?;
            child.wait()?.code()
        }
    };

    let exit_code = state.unwrap_or(1);

    if exit_code == 0 {
        return Ok(());
    }

    if exit_code == 128 {
        eprintln!("It looks like the package does not support Cask");
        eprintln!(
            "If you are the package owner, try to create a new repository '{}' and add a Cask.toml file",
            url
        )
    }

    Err(eyre::format_err!(
        "clone repository fail and exit code: {}",
        exit_code,
    ))
}

#[cfg(test)]
mod tests {
    use crate::git;
    use std::{fs, path::Path};

    #[test]
    fn test_clone() {
        let url1 = "https://github.com/axetroy/gpm.rs.git";

        let dest_dir = Path::new("./dist");

        let r1 = git::clone(
            url1,
            dest_dir,
            git::CloneOption {
                depth: Some(1),
                quiet: Some(true),
                single_branch: Some(true),
                dissociate: Some(true),
                filter: Some("tree:0".to_string()),
            },
        );

        assert!(r1.is_ok());
        assert!(dest_dir.exists());

        fs::remove_dir_all(dest_dir).unwrap();
    }

    #[test]
    fn test_check_exist_if_exist() {
        let url1 = "https://github.com/axetroy/gpm.rs.git";

        let r1 = git::check_exist(url1);

        assert!(r1.is_ok());
        assert!(r1.unwrap());
    }

    #[test]
    fn test_check_exist_if_not_exist() {
        let url1 = "https://github.com/axetroy/not_exist.git";

        let r1 = git::check_exist(url1);

        assert!(r1.is_ok());
        assert!(!r1.unwrap())
    }
}
