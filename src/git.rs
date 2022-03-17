use core::result::Result;
use std::path::Path;
use std::process::{Command as ChildProcess, Stdio};
use std::time::Duration;

use eyre::Report;
use semver::Version;
use wait_timeout::ChildExt;

pub struct CloneOption {
    pub depth: Option<i32>,
    pub quiet: Option<bool>,
    pub single_branch: Option<bool>,
    pub dissociate: Option<bool>,
    pub filter: Option<String>,
}

#[derive(Debug)]
pub struct GitTag {
    hash: String,
    tag: String,
}

impl PartialEq for GitTag {
    fn eq(&self, other: &Self) -> bool {
        if self.hash != other.hash {
            return false;
        }
        if self.tag != other.tag {
            return false;
        }
        true
    }
}

fn fetch_tags(git_url: &str) -> Result<Vec<GitTag>, Report> {
    let mut tags: Vec<GitTag> = vec![];

    let child = match ChildProcess::new("git")
        .stdout(Stdio::piped()) // Can do the same for stderr
        .arg("ls-remote")
        .arg("-t")
        .arg(git_url)
        .spawn()
    {
        Ok(child) => Ok(child),
        Err(e) => Err(eyre::format_err!("{}", e)),
    }?;

    let output = child.wait_with_output()?;

    let output = String::from_utf8(output.stdout).unwrap();

    for line in output.lines().into_iter().map(|f| f.to_string()) {
        let mut inter = line.split_whitespace();

        let hash = inter
            .next()
            .ok_or_else(|| eyre::format_err!("can not get hash of tag"))?;

        let refs = inter
            .next()
            .ok_or_else(|| eyre::format_err!("can not get refs of tag"))?;

        let tag = refs.trim_start_matches("refs/tags/");

        tags.push(GitTag {
            hash: hash.to_string(),
            tag: tag.to_string(),
        })
    }

    Ok(tags)
}

// get versions of remote repository
// the newest version at the head of vector
pub fn get_versions(git_url: &str) -> Result<Vec<String>, Report> {
    let mut versions: Vec<semver::Version> = vec![];
    let tags = fetch_tags(git_url)?;

    for tag in tags {
        // remove v prefix
        let version = tag.tag.trim_start_matches('v');

        if let Ok(v) = Version::parse(version) {
            versions.push(v);
        };
    }

    versions.sort_by(|a, b| b.cmp(a));

    let versions_str: Vec<String> = versions.into_iter().map(|v| v.to_string()).collect();

    Ok(versions_str)
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
        .stderr(Stdio::null())
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

    Err(eyre::format_err!(
        "clone repository fail and exit code: {}",
        exit_code,
    ))
}

#[cfg(test)]
mod tests {
    use crate::git::{self, GitTag};
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
    fn test_fetch_tags() {
        let url1 = "https://github.com/axetroy/prune.v.git";

        let tags = git::fetch_tags(url1).unwrap();

        let expect: Vec<GitTag> = vec![
            GitTag {
                hash: "30510408d1aa8d60ba1652e496b98d2739f12ef0".to_string(),
                tag: "v0.1.0".to_string(),
            },
            GitTag {
                hash: "bced83687a38f0a1f38b62f46b684373dc432109".to_string(),
                tag: "v0.1.1".to_string(),
            },
            GitTag {
                hash: "4f0f6aa2fe46549af49acb539ed041cd2b5fc192".to_string(),
                tag: "v0.2.0".to_string(),
            },
            GitTag {
                hash: "a80ba36c8b526281fa2d61e3bd0e105cdb9361d4".to_string(),
                tag: "v0.2.1".to_string(),
            },
            GitTag {
                hash: "689061589b0e0d728ea7f8f2d0923499957c8381".to_string(),
                tag: "v0.2.10".to_string(),
            },
            GitTag {
                hash: "1d023ac817e8168f2ce0e21a7d8ec5c269f99af4".to_string(),
                tag: "v0.2.11".to_string(),
            },
            GitTag {
                hash: "e2e87e7e9ddbcdfcb14bf0a2652eb5e05717e914".to_string(),
                tag: "v0.2.12".to_string(),
            },
            GitTag {
                hash: "96e1a60529f9dd5601818f1abcf017b39ec303b8".to_string(),
                tag: "v0.2.13".to_string(),
            },
            GitTag {
                hash: "462ed6570a0724ccb7b071016775e38c13b8f0cc".to_string(),
                tag: "v0.2.14".to_string(),
            },
            GitTag {
                hash: "7dede228a7d9521daae58ce4b5bd418e35474285".to_string(),
                tag: "v0.2.2".to_string(),
            },
            GitTag {
                hash: "24772db9ce73faa05cd9be6c4ee1fa3e18e1e634".to_string(),
                tag: "v0.2.3".to_string(),
            },
            GitTag {
                hash: "b5860a2a9aff28c4fe66e359f09bcdb293fbb8b1".to_string(),
                tag: "v0.2.4".to_string(),
            },
            GitTag {
                hash: "2172887b3387094e7ddb161827cdd1dbd12d2f30".to_string(),
                tag: "v0.2.5".to_string(),
            },
            GitTag {
                hash: "cba31715f0383b1158e9a0702f757fed00624187".to_string(),
                tag: "v0.2.6".to_string(),
            },
            GitTag {
                hash: "c46dcc5111d4b5906887b674c406f259b3f33f1b".to_string(),
                tag: "v0.2.7".to_string(),
            },
            GitTag {
                hash: "faf36650ea10a2f688dfdb7a2183efb309387361".to_string(),
                tag: "v0.2.8".to_string(),
            },
            GitTag {
                hash: "685610fae7cfb2152ce6a50cd43c5f751850300f".to_string(),
                tag: "v0.2.9".to_string(),
            },
        ];

        assert_eq!(tags, expect);
    }

    #[test]
    fn test_get_versions() {
        let url1 = "https://github.com/axetroy/prune.v.git";

        let tags = git::get_versions(url1).unwrap();

        let expect: Vec<String> = vec![
            "0.2.14", "0.2.13", "0.2.12", "0.2.11", "0.2.10", "0.2.9", "0.2.8", "0.2.7", "0.2.6",
            "0.2.5", "0.2.4", "0.2.3", "0.2.2", "0.2.1", "0.2.0", "0.1.1", "0.1.0",
        ]
        .into_iter()
        .map(|f| f.to_string())
        .collect();

        assert_eq!(tags, expect);
    }
}
