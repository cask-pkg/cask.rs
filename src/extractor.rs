#![deny(warnings)]

use core::result::Result;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command as ChildProcess;

use eyre::Report;
use libflate::gzip::Decoder as GzDecoder;
use tar::Archive;
use which::which;

fn extract_tar_gz(
    src_filepath: &Path,
    dest_dir: &Path,
    extract_file_name: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(extract_file_name);

    // use tar command
    // wait for fix: https://github.com/alexcrichton/tar-rs/issues/286
    if let Ok(tar_command_path) = which("tar") {
        fs::create_dir_all(dest_dir).map_err(|e| {
            eyre::format_err!("can not create folder '{}': {}", dest_dir.display(), e)
        })?;

        println!("open tar: {}", &*src_filepath.as_os_str().to_string_lossy());

        // bsdtar 3.5.1 - libarchive 3.5.1 zlib/1.2.11 liblzma/5.0.5 bz2lib/1.0.8
        #[cfg(target_os = "macos")]
        let args = vec![extract_file_name];
        #[cfg(windows)]
        let args = vec![extract_file_name];
        // tar (GNU tar) 1.29
        #[cfg(target_os = "linux")]
        let args = vec![extract_file_name];

        match ChildProcess::new(tar_command_path)
            .current_dir(dest_dir)
            .arg("-zvxf")
            .arg(&*src_filepath.as_os_str().to_string_lossy())
            .args(args)
            .spawn()
        {
            Ok(mut child) => match child.wait() {
                Ok(state) => {
                    if state.success() {
                        Ok(output_file_path)
                    } else {
                        Err(eyre::format_err!(
                            "exit code: {}",
                            state.code().unwrap_or(1),
                        ))
                    }
                }
                Err(e) => Err(eyre::format_err!("{}", e)),
            },
            Err(e) => Err(eyre::format_err!("{}", e)),
        }
    } else {
        let tar_file = File::open(&src_filepath)?;
        let input = GzDecoder::new(&tar_file)?;
        let mut archive = Archive::new(input);

        archive.set_unpack_xattrs(true);
        archive.set_overwrite(true);
        archive.set_preserve_permissions(true);
        archive.set_preserve_mtime(true);

        let files = archive.entries()?;

        for entry in files {
            let mut file = entry?;

            let file_path = file.path()?;

            if let Some(file_name) = file_path.file_name() {
                if file_name.to_str().unwrap() == extract_file_name {
                    file.unpack(&output_file_path)?;
                    return Ok(output_file_path);
                }
            }
        }

        Err(eyre::format_err!(
            "can not found the file '{}' in tar",
            extract_file_name
        ))
    }
}

fn extract_tar(
    src_filepath: &Path,
    dest_dir: &Path,
    extract_file_name: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(extract_file_name);

    let tar_file = File::open(&src_filepath)?;
    let mut archive = Archive::new(tar_file);

    archive.set_unpack_xattrs(true);
    archive.set_overwrite(true);
    archive.set_preserve_permissions(true);
    archive.set_preserve_mtime(true);

    let files = archive.entries()?;

    for entry in files {
        let mut file = entry?;

        let file_path = file.path()?;

        if let Some(file_name) = file_path.file_name() {
            if file_name.to_str().unwrap() == extract_file_name {
                file.unpack(&output_file_path)?;
                return Ok(output_file_path);
            }
        }
    }

    Err(eyre::format_err!(
        "can not found the file '{}' in tar",
        extract_file_name
    ))
}

fn extract_zip(
    src_filepath: &Path,
    dest_dir: &Path,
    extract_file_name: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(extract_file_name);

    let tar_file = File::open(&src_filepath)?;
    let mut archive = zip::ZipArchive::new(tar_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        if file.is_dir() {
            continue;
        }

        if file.name() == extract_file_name {
            let mut output_file = fs::File::create(&output_file_path)?;
            io::copy(&mut file, &mut output_file)?;

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::prelude::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&output_file_path, fs::Permissions::from_mode(mode))?;
                };
            };

            return Ok(output_file_path);
        }
    }

    Err(eyre::format_err!(
        "can not found the file '{}' in tar",
        extract_file_name
    ))
}

pub fn extract(
    tarball: &Path,
    dest_dir: &Path,
    extract_file_name: &str,
) -> Result<PathBuf, Report> {
    let tar_file_name = tarball.file_name().unwrap().to_str().unwrap();

    fs::create_dir_all(dest_dir)
        .map_err(|e| eyre::format_err!("can not create folder '{}': {}", dest_dir.display(), e))?;

    if tar_file_name.ends_with(".tar.gz") {
        extract_tar_gz(tarball, dest_dir, extract_file_name)
    } else if tar_file_name.ends_with(".tar") {
        extract_tar(tarball, dest_dir, extract_file_name)
    } else if tar_file_name.ends_with(".zip") {
        extract_zip(tarball, dest_dir, extract_file_name)
    } else {
        Err(eyre::format_err!(
            "not support extract file from '{}'",
            tar_file_name
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::extractor;
    use std::{env, fs};

    #[test]
    fn test_extract_tar_test() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = extractor_dir.join("test_tar.tar");

        let dest_dir = extractor_dir;

        let extracted_file_path =
            extractor::extract(&tar_file_path, &dest_dir, "test_tar").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 12);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "hello world\n");
    }

    #[test]
    fn test_extract_zip() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = extractor_dir.join("test_zip.zip");

        let dest_dir = extractor_dir;

        let extracted_file_path =
            extractor::extract(&tar_file_path, &dest_dir, "test_zip").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 12);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "hello world\n");
    }

    #[test]
    fn test_extract_tar_if_bin_not_exist() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = extractor_dir.join("test.tar");

        let dest_dir = extractor_dir;

        let r = extractor::extract(&tar_file_path, &dest_dir, "not_exist");

        assert!(r.is_err());
    }

    #[test]
    fn test_extract_tar_gz() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = extractor_dir.join("test.tar.gz");

        let dest_dir = extractor_dir;

        let extracted_file_path = extractor::extract(&tar_file_path, &dest_dir, "test").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 12);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "hello world\n");
    }

    #[test]
    fn test_extract_tar_gz_with_prune() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = extractor_dir.join("prune_darwin_amd64.tar.gz");

        let dest_dir = extractor_dir;

        let extracted_file_path = extractor::extract(&tar_file_path, &dest_dir, "prune").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 137_656);
    }

    #[test]
    fn test_extract_tar_gz_with_prune_win() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = extractor_dir.join("prune_window_386.tar.gz");

        let dest_dir = extractor_dir;

        let extracted_file_path =
            extractor::extract(&tar_file_path, &dest_dir, "prune.exe").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 657_408);
    }
}
