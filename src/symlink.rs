// #![deny(warnings)]

use eyre::Report;
use std::{fs, fs::File, io::Write, path::Path};

pub fn symlink(src: &Path, dest: &Path, package_name: &str) -> Result<(), Report> {
    if cfg!(unix) {
        // if file exists, then remove it
        if dest.exists() {
            fs::remove_file(dest)?;
        }

        // if symlink exists, then remove it
        if fs::read_link(dest).is_ok() {
            fs::remove_file(dest)?;
        }

        #[cfg(unix)]
        std::os::unix::fs::symlink(src, dest)?;
    } else {
        // instead of create a symlink in windows
        // we should generate a bat/shell file like this
        let filename = (*dest)
            .to_path_buf()
            .file_name()
            .ok_or_else(|| eyre::format_err!("can not get filename of '{}'", dest.display()))?
            .to_str()
            .ok_or_else(|| {
                eyre::format_err!("can not cover OsStr to str for '{}'", dest.display())
            })?
            .to_owned();

        let src_file_path = format!("{}", src.display());

        let dest_parent = dest
            .parent()
            .ok_or_else(|| eyre::format_err!("can not get parent of '{}'", dest.display()))?;

        // generate a bat
        {
            let bat_file_name = filename.clone() + ".bat";

            let bat_file_path = dest_parent.join(bat_file_name);

            let mut bat_file = File::create(bat_file_path)?;

            let bat_script = include_str!("./script/exe.bat")
                .replace("{filepath}", &src_file_path)
                .replace("{package}", package_name);

            bat_file.write_all(bat_script.as_bytes())?;
        }

        // generate a shell
        {
            let shell_file_name = &filename;

            let shell_file_path = dest_parent.join(shell_file_name);

            let mut shell_file = File::create(shell_file_path)?;

            let bat_script = include_str!("./script/exe.sh")
                .replace("{filepath}", &src_file_path)
                .replace("{package}", package_name)
                .replace("\r\n", "\n");

            shell_file.write_all(bat_script.as_bytes())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::symlink;
    use std::{env, fs};

    #[test]
    fn test_symlink() {
        let cwd = env::current_dir().unwrap();

        let src = cwd
            .join("fixtures")
            .join("symlink")
            .join("src")
            .join("test");

        let dest = cwd
            .join("fixtures")
            .join("symlink")
            .join("dest")
            .join("test");

        symlink::symlink(&src, &dest, "github.com/axetroy/test").unwrap();

        if cfg!(unix) {
            assert!(&dest.is_symlink());

            let d = fs::read_link(&dest).unwrap();

            assert_eq!(format!("{}", d.display()), format!("{}", src.display()));
        } else {
            assert!(dest.is_file());

            let shell_content = fs::read_to_string(&dest).unwrap();

            assert!(shell_content
                .contains(format!(r#"# package: {}"#, "github.com/axetroy/test").as_str()));
            assert!(shell_content.contains(format!(r#"# filepath: {}"#, src.display()).as_str()));
            assert!(shell_content.contains(format!(r#"xbin="{}""#, src.display()).as_str()));

            let bat = dest.parent().unwrap().join("test.bat");

            assert!(&bat.is_file());

            let bat_content = fs::read_to_string(&bat).unwrap();

            assert!(bat_content
                .contains(format!(r#":: package: {}"#, "github.com/axetroy/test").as_str()));
            assert!(bat_content.contains(format!(r#":: filepath: {}"#, src.display()).as_str()));
            assert!(bat_content.contains(format!(r#""{}" %*"#, src.display()).as_str()));
        }
    }
}
