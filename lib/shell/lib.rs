#![deny(warnings)]

use eyre::Report;

use std::{
    io,
    path::Path,
    process::{Command as ChildProcess, Stdio},
};

pub enum Output<'a> {
    Writer(&'a mut dyn io::Write), // write command output to writer
    Inherit,                       // inherit stdout/stderr from parent process
    None,                          // do none output anything
}

pub enum Terminal {
    Cmd,
    PowerShell,
    Sh,
    Bash,
}

pub fn run_with(
    terminal: Terminal,
    cwd: &Path,
    command: &str,
    output: &mut Output,
) -> Result<(), Report> {
    let commands: Vec<&str> = {
        match terminal {
            Terminal::Cmd => vec!["cmd.exe", "--%", "/c"],
            Terminal::PowerShell => vec![
                "powershell.exe",
                "-NoLogo",
                "-NoProfile",
                "-NonInteractive",
                "-Command",
            ],
            Terminal::Sh => vec!["sh", "-c"],
            Terminal::Bash => vec!["bash", "-c"],
        }
    };

    let cmd = commands.first().unwrap();
    let mut args = commands.clone().split_off(1);

    args.push(command);

    args = args.iter().filter(|s| !s.is_empty()).cloned().collect();

    let mut cmd = ChildProcess::new(cmd);

    let mut ps = cmd.current_dir(cwd).args(args);

    match &output {
        Output::Writer(_) => {
            ps = ps.stdout(Stdio::piped()).stderr(Stdio::piped());
        }
        Output::Inherit => ps = ps.stdout(Stdio::inherit()).stderr(Stdio::inherit()),
        Output::None => ps = ps.stdout(Stdio::null()).stderr(Stdio::null()),
    }

    let mut child = match ps.spawn() {
        Ok(child) => Ok(child),
        Err(e) => Err(eyre::format_err!("{}", e)),
    }?;

    if let Output::Writer(r) = output {
        io::copy(&mut child.stdout.take().unwrap(), r)?;
        io::copy(&mut child.stderr.take().unwrap(), r)?;
    };

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

pub fn run(cwd: &Path, command: &str, output: &mut Output) -> Result<(), Report> {
    let terminal = if cfg!(unix) {
        Terminal::Sh
    } else {
        Terminal::Cmd
    };

    run_with(terminal, cwd, command, output)
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{run, run_with, Output, Terminal};

    #[test]
    fn test_echo() {
        let cwd = env::current_dir().unwrap();

        let mut buf = Vec::new();

        run(&cwd, r#"echo 'hello world'"#, &mut Output::Writer(&mut buf)).unwrap();

        let result = std::str::from_utf8(&buf)
            .unwrap()
            .trim()
            .trim_matches(&['\''] as &[_]);

        assert_eq!(result, "hello world")
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_with_cmd() {
        let cwd = env::current_dir().unwrap();

        let mut buf = Vec::new();

        run_with(
            Terminal::Cmd,
            &cwd,
            r#"echo 'hello cmd'"#,
            &mut Output::Writer(&mut buf),
        )
        .unwrap();

        let result = std::str::from_utf8(&buf)
            .unwrap()
            .trim()
            .trim_matches(&['\''] as &[_]);

        assert_eq!(result, "hello cmd")
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_with_powershell() {
        let cwd = env::current_dir().unwrap();

        let mut buf = Vec::new();

        run_with(
            Terminal::Cmd,
            &cwd,
            r#"echo 'hello powershell'"#,
            &mut Output::Writer(&mut buf),
        )
        .unwrap();

        let result = std::str::from_utf8(&buf)
            .unwrap()
            .trim()
            .trim_matches(&['\''] as &[_]);

        assert_eq!(result, "hello powershell")
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn test_with_sh() {
        let cwd = env::current_dir().unwrap();

        let mut buf = Vec::new();

        run_with(
            Terminal::Sh,
            &cwd,
            r#"echo 'hello sh'"#,
            &mut Output::Writer(&mut buf),
        )
        .unwrap();

        let result = std::str::from_utf8(&buf).unwrap().trim();

        assert_eq!(result, "hello sh")
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn test_with_bash() {
        let cwd = env::current_dir().unwrap();

        let mut buf = Vec::new();

        run_with(
            Terminal::Bash,
            &cwd,
            r#"echo 'hello bash'"#,
            &mut Output::Writer(&mut buf),
        )
        .unwrap();

        let result = std::str::from_utf8(&buf).unwrap().trim();

        assert_eq!(result, "hello bash")
    }
}
