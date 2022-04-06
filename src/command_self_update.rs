#![deny(warnings)]

use std::env;
use std::fs;

use crate::cask;

use eyre::Report;
use semver::Version;

/// run the following command to show all build target
/// rustc --print target-list

fn get_arch() -> String {
    #[cfg(target_arch = "arm")]
    {
        "arm".to_string()
    }
    #[cfg(target_arch = "armebv7r")]
    {
        "armebv7r".to_string()
    }
    #[cfg(target_arch = "armv4t")]
    {
        "armv4t".to_string()
    }
    #[cfg(target_arch = "armv5te")]
    {
        "armv5te".to_string()
    }
    #[cfg(target_arch = "armv6")]
    {
        "armv6".to_string()
    }
    #[cfg(target_arch = "armv6k")]
    {
        "armv6k".to_string()
    }
    #[cfg(target_arch = "armv7")]
    {
        "armv7".to_string()
    }
    #[cfg(target_arch = "armv7a")]
    {
        "armv7a".to_string()
    }
    #[cfg(target_arch = "armv7r")]
    {
        "armv7r".to_string()
    }
    #[cfg(target_arch = "armv7s")]
    {
        "armv7s".to_string()
    }
    #[cfg(target_arch = "mips")]
    {
        "mips".to_string()
    }
    #[cfg(target_arch = "mipsel")]
    {
        "mipsel".to_string()
    }
    #[cfg(target_arch = "mipsisa32r6")]
    {
        "mipsisa32r6".to_string()
    }
    #[cfg(target_arch = "mipsisa32r6el")]
    {
        "mipsisa32r6el".to_string()
    }
    #[cfg(target_arch = "mipsisa64r6")]
    {
        "mipsisa64r6".to_string()
    }
    #[cfg(target_arch = "mipsisa64r6el")]
    {
        "mipsisa64r6el".to_string()
    }
    #[cfg(target_arch = "mips64")]
    {
        "mips64".to_string()
    }
    #[cfg(target_arch = "mips64el")]
    {
        "mips64el".to_string()
    }
    #[cfg(target_arch = "riscv64")]
    {
        "riscv64gc".to_string()
    }
    #[cfg(target_arch = "i686")]
    {
        "i686".to_string()
    }
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
    #[cfg(target_vendor = "uwp")]
    {
        "uwp".to_string()
    }
    #[cfg(target_vendor = "wrs")]
    {
        "wrs".to_string()
    }
    #[cfg(target_vendor = "sony")]
    {
        "sony".to_string()
    }
    #[cfg(target_vendor = "sun")]
    {
        "sun".to_string()
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
    #[cfg(target_os = "freebsd")]
    {
        "freebsd".to_string()
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
        "https://github.com/cask-pkg/cask.rs/releases/download/v{}/{}",
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

    let current_bin_path = {
        let p = env::current_exe()?;

        if p.is_symlink() {
            fs::read_link(p)?
        } else {
            p
        }
    };

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
