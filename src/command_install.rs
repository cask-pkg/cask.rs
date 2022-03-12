#![deny(warnings)]

extern crate flate2;
extern crate tar;

use crate::formula;
use crate::git;
use crate::util;
use crate::util::iso8601;

use eyre::Report;
use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::fs::{set_permissions, File};
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use tar::Archive;

pub async fn install(package_name: &str, _version: Option<&str>) -> Result<(), Report> {
    let url = format!("https://{}-cask.git", package_name);

    let unix_time = {
        let start = SystemTime::now();

        let t = start.duration_since(UNIX_EPOCH)?;

        t.as_secs()
    };

    let formula_cloned_dir = env::temp_dir().join(format!("cask_{}", unix_time));
    let cask_file_path = formula_cloned_dir.join("Cask.toml");

    let package_formula = match git::clone(&url, &formula_cloned_dir, vec![]) {
        Ok(()) => {
            if !cask_file_path.exists() {
                // remove cloned repo
                fs::remove_dir_all(formula_cloned_dir)?;
                return Err(eyre::format_err!(
                    "{} is not a valid formula!",
                    package_name
                ));
            }

            let f = formula::new(&cask_file_path)?;

            Ok(f)
        }
        Err(e) => Err(e),
    }?;

    let option_target = if cfg!(target_os = "macos") {
        package_formula.darwin
    } else if cfg!(target_os = "windows") {
        package_formula.windows
    } else if cfg!(target_os = "linux") {
        package_formula.linux
    } else {
        fs::remove_dir_all(formula_cloned_dir)?;
        return Err(eyre::format_err!(
            "{} not support your system",
            package_name
        ));
    };

    let target = match option_target {
        Some(p) => Ok(p),
        None => Err(eyre::format_err!(
            "{} not support your system",
            package_name
        )),
    }?;

    let hash_of_package = {
        let mut hasher = Sha256::new();

        hasher.update(package_name);
        format!("{:X}", hasher.finalize())
    };

    let package_dir = {
        let mut d = match dirs::home_dir() {
            Some(d) => Ok(d),
            None => Err(eyre::format_err!("can not found home dir")),
        }?;

        d = d.join(".cask").join("formula").join(hash_of_package);

        d
    };

    // init formula folder
    {
        if !&package_dir.exists() {
            fs::create_dir_all(&package_dir)?;
            fs::create_dir_all(&package_dir.join("bin"))?;
            fs::create_dir_all(&package_dir.join("version"))?;
        }

        let cask_file_content = {
            let cask_file = File::open(&cask_file_path)?;
            let mut buf_reader = BufReader::new(&cask_file);
            let mut file_content = String::new();
            buf_reader.read_to_string(&mut file_content)?;

            file_content
        };

        let file_path = &package_dir.join("Cask.toml");

        let mut formula_file = File::create(&file_path)?;

        formula_file.write_all(
            format!(
                r#"# The file is generated by Cask. DO NOT MODIFY IT.
[cask]
package_name = "{}"
created_at = "{}"

"#,
                package_name,
                iso8601(&SystemTime::now())
            )
            .as_str()
            .as_bytes(),
        )?;
        formula_file.write_all(cask_file_content.as_bytes())?;
    }

    // remove cloned repo
    fs::remove_dir_all(formula_cloned_dir)?;

    let option_arch = if cfg!(target_arch = "x86") {
        target.x86
    } else if cfg!(target_arch = "x86_64") {
        target.x86_64
    } else if cfg!(target_arch = "arm") {
        target.arm
    } else if cfg!(target_arch = "aarch64") {
        target.aarch64
    } else if cfg!(target_arch = "mips") {
        target.mips
    } else if cfg!(target_arch = "mips64") {
        target.mips64
    } else if cfg!(target_arch = "mips64el") {
        target.mips64el
    } else {
        None
    };

    let arch = match option_arch {
        Some(a) => Ok(a),
        None => Err(eyre::format_err!("{} not support your arch", package_name)),
    }?;

    let tar_file_path = &package_dir
        .join("version")
        .join(format!("{}.tar.gz", package_formula.package.version));
    let tar_file_name = tar_file_path.file_name().unwrap().to_str().unwrap();

    util::download(&arch.url, tar_file_path).await?;

    let tar_file = File::open(tar_file_path)?;

    let bin_name = if cfg!(target_os = "windows") {
        format!("{}.exe", package_formula.package.bin)
    } else {
        package_formula.package.bin
    };

    let mut bin_found = false;

    let output_file_path = package_dir.join("bin").join(&bin_name);

    // .tar.gz
    if tar_file_name.ends_with(".tar.gz") {
        let tar = GzDecoder::new(&tar_file);
        let mut archive = Archive::new(tar);

        let files = archive.entries()?;

        for e in files {
            let mut entry = e?;

            let entry_file = entry.path()?;

            if let Some(file_name) = entry_file.file_name() {
                if file_name.to_str().unwrap() == bin_name {
                    entry.unpack(&output_file_path)?;
                    bin_found = true;
                    break;
                }
            }
        }
    } else if tar_file_name.ends_with(".zip") {
        let mut archive = zip::ZipArchive::new(&tar_file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            if file.is_file() && file.name() == bin_name {
                let mut output_file = File::create(&output_file_path)?;

                io::copy(&mut file, &mut output_file)?;

                bin_found = true;

                // Get and Set permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;

                    if let Some(mode) = file.unix_mode() {
                        set_permissions(&output_file_path, fs::Permissions::from_mode(mode))?;
                    }
                }
                break;
            }
        }
    }

    if !bin_found {
        return Err(eyre::format_err!(
            "can not found binary file '{}' in tar",
            bin_name
        ));
    }

    Ok(())
}
