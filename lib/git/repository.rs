// #![deny(warnings)]

use core::result::Result;
use std::io;
use std::path::Path;
use std::process::{Command as ChildProcess, Stdio};
use std::time::Duration;

use git_url_parse::GitUrl;
use semver::Version;
use thiserror::Error;
use wait_timeout::ChildExt;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("io error {source:?}")]
    IO { source: io::Error },
    #[error("invalid git url: {url:?}")]
    GitUrlInvalid { url: String },
    #[error("repository already exist in {path:?}")]
    RepositoryExist { path: String },
    #[error("Running git command error: {source:?}")]
    CommandError { source: io::Error },
    #[error("Running git command exit {code:?}")]
    CommandExitError { code: i32 },
    #[error("Can not found remote repository {url:?})")]
    RemoteRepositoryNotExists { url: String },
    #[error("Can not get tag from output: {row:?})")]
    ParseTagError { row: String },
}

#[derive(Debug)]
pub struct GitTag {
    pub hash: String,
    pub tag: String,
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

pub struct Repository {
    remote: String,
}

pub fn new(url: &str) -> Result<Repository, GitError> {
    match GitUrl::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err(GitError::GitUrlInvalid {
            url: url.to_string(),
        }),
    }?;

    let r = Repository {
        remote: url.to_string(),
    };

    Ok(r)
}

pub struct CloneOption {
    pub depth: Option<i32>,
    pub quiet: Option<bool>,
    pub single_branch: Option<bool>,
    pub dissociate: Option<bool>,
    pub filter: Option<String>,
}

impl Repository {
    pub fn clone(&self, dest: &Path, options: CloneOption) -> Result<(), GitError> {
        if dest.exists() {
            return Err(GitError::RepositoryExist {
                path: format!("{}", dest.display()),
            });
        }

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

        let mut child = ChildProcess::new("git")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_SSH_COMMAND", "ssh -oBatchMode=yes")
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .arg("clone")
            .arg(self.remote.clone())
            .args(args)
            .arg(dest.to_str().unwrap())
            .spawn()
            .map_err(|e| GitError::CommandError { source: e })?;

        let timeout = Duration::from_secs(300); // 5min

        let state = match child
            .wait_timeout(timeout)
            .map_err(|e| GitError::IO { source: e })?
        {
            Some(status) => status.code(),
            None => {
                // child hasn't exited yet
                child.kill().map_err(|e| GitError::IO { source: e })?;
                child.wait().map_err(|e| GitError::IO { source: e })?.code()
            }
        };

        let exit_code = state.unwrap_or(1);

        if exit_code == 0 {
            return Ok(());
        }

        if exit_code == 128 {
            return Err(GitError::RemoteRepositoryNotExists {
                url: self.remote.to_string(),
            });
        }

        Err(GitError::CommandExitError { code: exit_code })
    }

    pub fn is_exist(&self) -> Result<bool, GitError> {
        let mut child = ChildProcess::new("git")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_SSH_COMMAND", "ssh -oBatchMode=yes")
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .arg("ls-remote")
            .arg("-h")
            .arg(self.remote.clone())
            .spawn()
            .map_err(|e| GitError::CommandError { source: e })?;

        let timeout = Duration::from_secs(30);

        let state = match child
            .wait_timeout(timeout)
            .map_err(|e| GitError::IO { source: e })?
        {
            Some(status) => status.code(),
            None => {
                // child hasn't exited yet
                child.kill().map_err(|e| GitError::IO { source: e })?;
                child.wait().map_err(|e| GitError::IO { source: e })?.code()
            }
        };

        let exit_code = state.unwrap_or(1);

        if exit_code == 0 {
            return Ok(true);
        }

        if exit_code == 128 {
            return Ok(false);
        }

        Err(GitError::CommandExitError { code: exit_code })
    }

    pub fn tags(&self) -> Result<Vec<GitTag>, GitError> {
        let mut tags: Vec<GitTag> = vec![];

        let child = ChildProcess::new("git")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_SSH_COMMAND", "ssh -oBatchMode=yes")
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .arg("ls-remote")
            .arg("-t")
            .arg(self.remote.clone())
            .spawn()
            .map_err(|e| GitError::CommandError { source: e })?;

        let output = child
            .wait_with_output()
            .map_err(|e| GitError::CommandError { source: e })?;

        if !output.status.success() {
            let exit_code = output.status.code().unwrap_or(1);

            if exit_code == 128 {
                return Err(GitError::RemoteRepositoryNotExists {
                    url: self.remote.to_string(),
                });
            }

            return Err(GitError::CommandExitError { code: exit_code });
        }

        let stdout = String::from_utf8(output.stdout).unwrap();

        for line in stdout.lines().into_iter().map(|f| f.to_string()) {
            let mut inter = line.split_whitespace();

            let hash = inter
                .next()
                .ok_or_else(|| GitError::ParseTagError { row: line.clone() })?;

            let refs = inter
                .next()
                .ok_or_else(|| GitError::ParseTagError { row: line.clone() })?;

            let tag = refs.trim_start_matches("refs/tags/");

            tags.push(GitTag {
                hash: hash.to_string(),
                tag: tag.to_string(),
            })
        }

        Ok(tags)
    }

    pub fn versions(&self) -> Result<Vec<String>, GitError> {
        let mut versions: Vec<semver::Version> = vec![];
        let tags = self.tags()?;

        for tag in tags {
            // remove v prefix
            let version = tag.tag.trim_start_matches('v');

            if let Ok(v) = Version::parse(version) {
                // ignore unstable version
                // eg. 2.5.2-test
                if v.pre.is_empty() {
                    versions.push(v);
                }
            };
        }

        versions.sort_by(|a, b| b.cmp(a));

        let versions_str: Vec<String> = versions.into_iter().map(|v| v.to_string()).collect();

        Ok(versions_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_url() {
        assert!(new("invalid_url").is_err());
    }
}

#[cfg(test)]
mod tests_clone {
    use std::{env, fs, path::Path};

    use super::*;

    #[test]
    fn test_clone() {
        let repo = new("https://github.com/axetroy/gpm.rs.git").unwrap();

        let dest_dir = Path::new("./dist");

        fs::remove_dir_all(dest_dir).ok();

        repo.clone(
            dest_dir,
            CloneOption {
                depth: Some(1),
                quiet: Some(true),
                single_branch: Some(true),
                dissociate: Some(true),
                filter: Some("tree:0".to_string()),
            },
        )
        .unwrap();

        fs::remove_dir_all(dest_dir).ok();
    }

    #[test]
    fn test_clone_if_remote_not_exist() {
        let repo = new("https://github.com/axetroy/not_exist.git").unwrap();

        let dest_dir = env::temp_dir().join("cask_test_1");

        let r1 = repo.clone(
            &dest_dir,
            CloneOption {
                depth: Some(1),
                quiet: Some(true),
                single_branch: Some(true),
                dissociate: Some(true),
                filter: Some("tree:0".to_string()),
            },
        );

        fs::remove_dir_all(dest_dir).ok();

        assert!(r1.is_err());

        if let Err(e) = r1 {
            assert!(match e {
                GitError::RemoteRepositoryNotExists { url } => {
                    assert_eq!(url, repo.remote);
                    true
                }
                _ => false,
            })
        }
    }
}

#[cfg(test)]
mod tests_is_exist {
    use super::*;

    #[test]
    fn test_is_exist_if_exist() {
        let repo = new("https://github.com/axetroy/cask.rs.git").unwrap();

        let exist = repo.is_exist().unwrap();

        assert!(exist);
    }

    #[test]
    fn test_check_exist_if_not_exist() {
        let repo = new("https://github.com/axetroy/not_exist.git").unwrap();

        let exist = repo.is_exist().unwrap();

        assert!(!exist);
    }
}

#[cfg(test)]
mod tests_tags {
    use super::*;

    #[test]
    fn test_tags() {
        let repo = new("https://github.com/axetroy/prune.v.git").unwrap();

        let tags = repo.tags().unwrap();

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
    fn test_tags_if_remote_not_exist() {
        let repo = new("https://github.com/axetroy/not_eexist.git").unwrap();

        let r = repo.tags();

        assert!(r.is_err());

        if let Err(e) = r {
            assert!(match e {
                GitError::RemoteRepositoryNotExists { url } => {
                    assert_eq!(url, repo.remote);
                    true
                }
                _ => false,
            })
        }
    }

    #[test]
    fn test_fetch_tags_if_remote_does_not_exist_tags() {
        let repo = new("https://github.com/axetroy/axetroy.git").unwrap();

        let tags = repo.tags().unwrap();

        assert!(tags.is_empty());
    }
}

#[cfg(test)]
mod tests_versions {
    use super::*;

    #[test]
    fn test_versions() {
        let repo = new("https://github.com/axetroy/prune.v.git").unwrap();

        let versions = repo.versions().unwrap();

        let expect: Vec<String> = vec![
            "0.2.14", "0.2.13", "0.2.12", "0.2.11", "0.2.10", "0.2.9", "0.2.8", "0.2.7", "0.2.6",
            "0.2.5", "0.2.4", "0.2.3", "0.2.2", "0.2.1", "0.2.0", "0.1.1", "0.1.0",
        ]
        .into_iter()
        .map(|f| f.to_string())
        .collect();

        assert_eq!(versions, expect);
    }

    #[test]
    fn test_get_versions_from_a_not_exist_repo() {
        let repo = new("https://github.com/axetroy/not_exist.git").unwrap();

        let r1 = repo.versions();

        assert!(r1.is_err());

        if let Err(e) = r1 {
            assert!(match e {
                GitError::RemoteRepositoryNotExists { url } => {
                    assert_eq!(url, repo.remote);
                    true
                }
                _ => false,
            })
        }
    }
}
