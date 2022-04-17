#![deny(warnings)]

use std::{collections::HashMap, path::Path};

use eyre::Report;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HookDefinition {
    pub preinstall: Option<String>, // The script will run before install package
    pub postinstall: Option<String>, // The script will run after install package
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Terminal {
    pub cmd: Option<HookDefinition>,
    pub powershell: Option<HookDefinition>,
    pub sh: Option<HookDefinition>,
    pub bash: Option<HookDefinition>,
}

pub struct TerminalHook {
    pub terminal: shell::Terminal,
    pub hook: HookDefinition,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hook {
    pub windows: Option<Terminal>,
    pub unix: Option<Terminal>,
    pub linux: Option<Terminal>,
    pub macos: Option<Terminal>,
    pub freebsd: Option<Terminal>,
}

impl Hook {
    pub fn resolve(&self) -> Option<TerminalHook> {
        let terminal = {
            if cfg!(unix) {
                if cfg!(target_os = "linux") {
                    if self.linux.is_some() {
                        self.linux.as_ref()
                    } else {
                        self.unix.as_ref()
                    }
                } else if cfg!(target_os = "macos") {
                    if self.macos.is_some() {
                        self.macos.as_ref()
                    } else {
                        self.unix.as_ref()
                    }
                } else if cfg!(target_os = "freebsd") {
                    if self.freebsd.is_some() {
                        self.freebsd.as_ref()
                    } else {
                        self.unix.as_ref()
                    }
                } else {
                    self.unix.as_ref()
                }
            } else {
                self.windows.as_ref()
            }
        };

        if let Some(t) = terminal {
            if cfg!(target_os = "windows") {
                t.clone()
                    .cmd
                    .map(|hook| TerminalHook {
                        terminal: shell::Terminal::Cmd,
                        hook,
                    })
                    .or_else(|| {
                        t.clone().powershell.map(|hook| TerminalHook {
                            terminal: shell::Terminal::PowerShell,
                            hook,
                        })
                    })
            } else {
                t.clone()
                    .sh
                    .map(|hook| TerminalHook {
                        terminal: shell::Terminal::Sh,
                        hook,
                    })
                    .or_else(|| {
                        t.clone().bash.map(|hook| TerminalHook {
                            terminal: shell::Terminal::Bash,
                            hook,
                        })
                    })
            }
        } else {
            None
        }
    }

    pub fn run(&self, hook_name: &str, cwd: &Path) -> Result<(), Report> {
        let hook_op = self.resolve();

        if let Some(terminal_hook) = hook_op {
            let hook = terminal_hook.hook;

            let script_op = match hook_name {
                "preinstall" => Ok(&hook.preinstall),
                "postinstall" => Ok(&hook.postinstall),
                _ => Err(eyre::format_err!(
                    "trying to run a unknown hook, names {}",
                    hook_name
                )),
            }?;

            if let Some(script) = script_op {
                eprintln!("Running '{}' hook", hook_name);

                shell::run_with(
                    terminal_hook.terminal,
                    cwd,
                    script,
                    &mut shell::Output::Inherit,
                    HashMap::from([]),
                )?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::hooker::{self, HookDefinition, Terminal};

    #[test]
    fn test_run_hooker() {
        let preinstall_script = r#"echo "preinstall""#.to_string();
        let postinstall_script = r#"echo "postinstall""#.to_string();

        let hook = hooker::Hook {
            windows: Some(Terminal {
                cmd: Some(HookDefinition {
                    preinstall: Some(preinstall_script.clone()),
                    postinstall: Some(postinstall_script.clone()),
                }),
                powershell: None,
                sh: None,
                bash: None,
            }),
            unix: Some(Terminal {
                cmd: None,
                powershell: None,
                sh: Some(HookDefinition {
                    preinstall: Some(preinstall_script),
                    postinstall: Some(postinstall_script),
                }),
                bash: None,
            }),
            linux: None,
            macos: None,
            freebsd: None,
        };

        let r1 = hook.run("preinstall", &env::current_dir().unwrap());

        assert!(r1.is_ok());

        let r2 = hook.run("postinstall", &env::current_dir().unwrap());

        assert!(r2.is_ok());

        let r3 = hook.run("unknown", &env::current_dir().unwrap());

        assert!(r3.is_err());
    }
}
