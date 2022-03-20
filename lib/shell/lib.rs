// #![deny(warnings)]

use eyre::Report;

use std::path::Path;
use std::process::Command as ChildProcess;

pub fn run(cwd: &Path, command: &str) -> Result<(), Report> {
    let cmd: &str;
    let mut args = vec![""];
    if cfg!(unix) {
        cmd = "sh";
        args.push("-c");
    } else {
        cmd = "C:\\Windows\\System32\\cmd.exe";
        args.push("--%");
        args.push("/c");
    }

    args.push(command);

    args = args.iter().filter(|s| !s.is_empty()).cloned().collect();

    println!("{} {:?}", cmd, args);

    let mut child = match ChildProcess::new(cmd).current_dir(cwd).args(args).spawn() {
        Ok(child) => Ok(child),
        Err(e) => Err(eyre::format_err!("{}", e)),
    }?;

    match child.wait() {
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
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::run;

    #[test]
    fn test_shell_echo() {
        let cwd = env::current_dir().unwrap();

        run(&cwd, r#"echo "hello world""#).unwrap();
    }
}
