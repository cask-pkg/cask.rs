use eyre::Report;
use std::path::Path;

pub fn symlink(src: &Path, dest: &Path) -> Result<(), Report> {
    #[cfg(target_family = "unix")]
    std::os::unix::fs::symlink(&src, &dest)?;
    #[cfg(target_family = "windows")]
    {
        use std::process::Command as ChildProcess;

        match std::os::windows::fs::symlink_file(&src, &desk) {
            Ok() => Ok(()),
            Err() => {
                // https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/mklink
                match ChildProcess::new("mklink")
                    .arg(&src.as_os_str().to_str().unwrap())
                    .arg(&dest.as_os_str().to_str().unwrap())
                    .spawn()
                {
                    Ok(mut child) => match child.wait() {
                        Ok(state) => {
                            if state.success() {
                                Ok(())
                            } else {
                                Err(Report::msg("create symlink fail"))
                            }
                        }
                        Err(e) => Err(eyre::format_err!("create symlink fail: {}", e)),
                    },
                    Err(e) => Err(eyre::format_err!("run mklink command fail: {}", e)),
                }
            }
        };
    }

    Ok(())
}
