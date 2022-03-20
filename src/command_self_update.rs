#![deny(warnings)]

use std::env;
use std::fs;

use crate::cask;

use eyre::Report;
use semver::Version;

fn get_arch() -> String {
    #[cfg(target_arch = "x86_64")]
    {
        "x86_64".to_string()
    }
    #[cfg(target_arch = "aarch64")]
    {
        "aarch64".to_string()
    }
}

fn get_vendor() -> String {
    #[cfg(target_vendor = "apple")]
    {
        "apple".to_string()
    }
    #[cfg(target_vendor = "fortanix")]
    {
        "fortanix".to_string()
    }
    #[cfg(target_vendor = "pc")]
    {
        "pc".to_string()
    }
    #[cfg(target_vendor = "unknown")]
    {
        "unknown".to_string()
    }
}

fn get_os() -> String {
    #[cfg(target_os = "windows")]
    {
        "windows".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        "darwin".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        "linux".to_string()
    }
}

fn get_abi() -> Option<String> {
    #[cfg(target_env = "")]
    {
        None
    }
    #[cfg(target_env = "gnu")]
    {
        Some("gnu".to_string())
    }
    #[cfg(target_env = "msvc")]
    {
        Some("msvc".to_string())
    }
    #[cfg(target_env = "musl")]
    {
        Some("musl".to_string())
    }
    #[cfg(target_env = "sgx")]
    {
        Some("sgx".to_string())
    }
}

// get the latest version without 'v' prefix
fn get_latest_release() -> Result<String, Report> {
    let versions = git::new(env!("CARGO_PKG_REPOSITORY"))?.versions()?;

    let err_can_not_found_release = eyre::format_err!("There is no one release of Cask");

    if versions.is_empty() {
        return Err(err_can_not_found_release);
    }

    let latest_version = versions.first().ok_or(err_can_not_found_release)?;

    Ok(latest_version.to_string())
}

pub async fn self_update(_cask: &cask::Cask) -> Result<(), Report> {
    let latest_release = get_latest_release()?;

    let latest_remote_version = Version::parse(&latest_release)
        .map_err(|e| eyre::format_err!("parse latest version '{}' fail: {}", &latest_release, e))?;

    let current_version = Version::parse(env!("CARGO_PKG_VERSION")).map_err(|e| {
        eyre::format_err!(
            "parse current version '{}' fail: {}",
            env!("CARGO_PKG_VERSION"),
            e
        )
    })?;

    if latest_remote_version <= current_version {
        eprintln!("You are using the latest version of Cask");
        return Ok(());
    }

    let arch = get_arch();
    let vendor = get_vendor();
    let os = get_os();

    let mut filename = format!("{}-{}-{}-{}", env!("CARGO_BIN_NAME"), arch, vendor, os);

    if let Some(abi) = get_abi() {
        filename += format!("-{}", abi).as_str();
    }

    filename += ".tar.gz";

    let resource_url = format!(
        "https://github.com/axetroy/cask.rs/releases/download/v{}/{}",
        &latest_release, filename,
    );

    let resource_file_path = env::temp_dir().join(format!("{}-{}", &latest_release, filename));

    downloader::download(&resource_url, &resource_file_path).await?;

    #[cfg(unix)]
    let exe_name = env!("CARGO_BIN_NAME").to_string();
    #[cfg(windows)]
    let exe_name = format!("{}.exe", env!("CARGO_BIN_NAME"));

    let binary_file_path =
        extractor::extract(&resource_file_path, &env::temp_dir(), &exe_name, "/")?;

    // remove tarball file
    fs::remove_file(&resource_file_path).ok();

    let current_bin_path = env::current_exe()?;

    let temp_file = env::temp_dir().join(format!("old_{}", exe_name));

    fs::rename(&current_bin_path, &temp_file)?;
    fs::rename(binary_file_path, &current_bin_path)?;

    // remove temp file
    fs::remove_file(temp_file).ok();

    eprintln!(
        "Update from '{}' to '{}' success!",
        env!("CARGO_PKG_VERSION"),
        &latest_release
    );

    Ok(())
}
