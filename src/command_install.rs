#![deny(warnings)]

use crate::{cask, formula, symlink, util::iso8601};

use std::{
    fs,
    fs::File,
    io::Write,
    io::{self, Read},
    time::SystemTime,
};

use atty::{is, Stream};
use eyre::Report;
use semver::{Version, VersionReq};
use sha2::{Digest, Sha256};

pub async fn install(
    cask: &cask::Cask,
    package_name: &str,
    version: Option<&str>,
    is_verbose: bool,
) -> Result<(), Report> {
    let package_formula = if !is(Stream::Stdin) {
        // Read Cask.toml from stdin
        // cat Cask.toml | cask install
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;

        let content = std::str::from_utf8(&buffer).unwrap();

        let mut f: formula::Formula = toml::from_str(content.trim())?;

        let cask_file_path = cask.formula_dir().join("Cask.toml");
        fs::write(&cask_file_path, content)?;

        f.filepath = cask_file_path;
        f.repository = "".to_string();
        f.file_content = content.to_string();

        f
    } else {
        if package_name.is_empty() {
            return Err(eyre::format_err!("<PACKAGE> required"));
        }

        formula::fetch(cask, package_name, false, is_verbose)?
    };

    // detect binary name conflict
    for f in cask.list_formula()? {
        if f.package.bin == package_formula.package.bin {
            let exist_package_name = f
                .cask
                .map(|f| f.name)
                .unwrap_or_else(|| f.package.name.clone());
            if exist_package_name == f.package.name.clone() {
                continue;
            }

            return Err(eyre::format_err!(
                r#"The package '{}' binary file name conflict with '{}'. Try uninstall '{}' and try again."#,
                &package_formula.package.name,
                &exist_package_name,
                &exist_package_name
            ));
        }
    }

    if let Some(hook) = &package_formula.hook {
        hook.run(
            "preinstall",
            &cask
                .package_dir(&package_formula.package.name)
                .join("repository"),
        )?;
    }

    let remote_versions = package_formula.get_versions()?;

    if remote_versions.is_empty() {
        return Err(eyre::format_err!(
            "can not found any version of '{}'",
            package_name
        ));
    }

    let download_version = {
        let v = version
            .or_else(|| remote_versions.first().map(|v| v.as_str()))
            .expect("can not found remote version");

        let version_req = VersionReq::parse(v)
            .map_err(|e| eyre::format_err!("invalid semver version {}: {}", v, e))?;

        let mut target_version: String = "".to_string();

        for remote_v_str in &remote_versions {
            if let Ok(remote_v) = Version::parse(remote_v_str) {
                if version_req.matches(&remote_v) {
                    target_version = remote_v_str.to_string();
                    break;
                }
            }
        }

        if target_version.is_empty() {
            Err(eyre::format_err!(
                "can not found version '{}' of formula",
                v
            ))
        } else {
            Ok(target_version)
        }
    }?;

    // init formula folder
    cask.init_package(&package_formula.package.name)?;

    let package_dir = cask.package_dir(&package_formula.package.name);

    let download_target = package_formula.get_current_download_url(&download_version)?;

    let tar_file_path = cask
        .package_version_dir(&package_formula.package.name)
        .join(format!("{}{}", &download_version, download_target.ext));

    downloader::download(&download_target.url, &tar_file_path).await?;

    if let Some(checksum) = download_target.checksum {
        let mut file = File::open(&tar_file_path)?;
        let mut hasher = Sha256::new();
        io::copy(&mut file, &mut hasher)?;
        drop(file);
        let hash = format!("{:x}", hasher.finalize());
        if hash != checksum {
            fs::remove_file(tar_file_path)?;
            return Err(eyre::format_err!(
                "The file SHA256 is '{}' but expect '{}'",
                hash,
                checksum
            ));
        }
    }

    #[cfg(target_family = "unix")]
    let executable_name = package_formula.package.bin.clone();
    #[cfg(target_family = "windows")]
    let executable_name = format!("{}.exe", &package_formula.package.bin);

    let output_file_path = {
        if download_target.executable {
            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::prelude::PermissionsExt;

                fs::set_permissions(&tar_file_path, fs::Permissions::from_mode(0o755))?;
            };
            tar_file_path
        } else {
            extractor::extract(
                &tar_file_path,
                &package_dir.join("bin"),
                &executable_name,
                download_target.path.as_str(),
            )?
        }
    };

    // create symlink to $CASK_ROOT/bin
    {
        let symlink_file = cask.bin_dir().join(&package_formula.package.bin);

        symlink::symlink(
            &output_file_path,
            &symlink_file,
            &package_formula.package.name,
        )?;
    }

    // init Cask information in Cask.toml
    {
        let file_path = &package_dir.join("Cask.toml");

        let mut formula_file = File::create(&file_path)?;

        formula_file.write_all(
            format!(
                r#"# The file is generated by Cask. DO NOT MODIFY IT.
                [cask]
                name = "{}"
                created_at = "{}"
                version = "{}"
                repository = "{}"

                "#,
                package_formula.package.name,
                iso8601(&SystemTime::now()),
                download_version,
                package_formula.repository
            )
            .lines()
            .map(|s| s.trim_start().to_owned())
            .collect::<Vec<String>>()
            .join("\n")
            .as_str()
            .as_bytes(),
        )?;
        formula_file.write_all(package_formula.get_file_content().as_bytes())?;
    }

    if let Some(hook) = &package_formula.hook {
        hook.run(
            "postinstall",
            &cask
                .package_dir(&package_formula.package.name)
                .join("repository"),
        )?;
    }

    eprintln!(
        "The package '{} {}' has been installed!",
        &package_formula.package.name, download_version
    );

    eprintln!(
        "Try run the command '{} --help' to make sure it works!",
        &package_formula.package.bin,
    );

    Ok(())
}
