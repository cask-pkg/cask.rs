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

pub fn run(cwd: &Path, command: &str, output: &mut Output) -> Result<(), Report> {
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

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{run, Output};

    #[test]
    fn test_shell_echo() {
        let cwd = env::current_dir().unwrap();

        let mut buf = Vec::new();

        // let mut buffer = io::BufWriter::new(io::stdout());

        run(&cwd, r#"echo 'hello world'"#, &mut Output::Writer(&mut buf)).unwrap();

        let result = std::str::from_utf8(&buf).unwrap();

        println!("{}", result);
    }
}
