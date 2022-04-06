// #![deny(warnings)]

use std::path::Path;

use eyre::Report;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Hook {
    pub preinstall: Option<String>, // The script will run before install package
    pub postinstall: Option<String>, // The script will run after install package
}

impl Hook {
    pub fn run(&self, hook_name: &str, cwd: &Path) -> Result<(), Report> {
        let script_op = match hook_name {
            "preinstall" => Ok(&self.preinstall),
            "postinstall" => Ok(&self.postinstall),
            _ => Err(eyre::format_err!(
                "trying to run a unknown hook, names {}",
                hook_name
            )),
        }?;

        if let Some(scripts) = script_op {
            eprintln!("Running '{}' hook", hook_name);

            for script in scripts.split('\n').map(|s| s.trim_start()) {
                if script.is_empty() {
                    continue;
                }

                if script.starts_with('#') {
                    continue;
                }

                shell::run(cwd, script, &mut shell::Output::Inherit)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::hooker;

    #[test]
    fn test_run_hooker() {
        let hook = hooker::Hook {
            preinstall: Some(r#"echo "hello world""#.to_string()),
            postinstall: Some(r#"echo "hello world""#.to_string()),
        };

        let r1 = hook.run("preinstall", &env::current_dir().unwrap());

        assert!(r1.is_ok());

        let r2 = hook.run("postinstall", &env::current_dir().unwrap());

        assert!(r2.is_ok());

        let r3 = hook.run("unknown", &env::current_dir().unwrap());

        assert!(r3.is_err());
    }
}
