#![deny(warnings)]

use core::result::Result;
use std::fs::{self, File};
use std::io;
use std::io::Read;
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
    extract_file_in_tarball_file_path: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(extract_file_name);

    // use tar command
    // wait for fix: https://github.com/alexcrichton/tar-rs/issues/286
    if let Ok(tar_command_path) = which("tar") {
        let unpack_exe_file_path = {
            let mut unpack = dest_dir.to_path_buf();
            for p in extract_file_in_tarball_file_path.split('/') {
                if p.is_empty() {
                    continue;
                }
                unpack = dest_dir.join(p).to_path_buf();
            }

            unpack = unpack.join(extract_file_name);

            unpack
        };
        fs::create_dir_all(dest_dir).map_err(|e| {
            eyre::format_err!("can not create folder '{}': {}", dest_dir.display(), e)
        })?;

        match ChildProcess::new(tar_command_path)
            .current_dir(dest_dir)
            .arg("-f")
            .arg(format!("{}", src_filepath.display()))
            .arg("-zx")
            .spawn()
        {
            Ok(mut child) => match child.wait() {
                Ok(state) => {
                    if state.success() {
                        // Rename it to the target folder if the executable file is not locate in root tarball
                        if extract_file_in_tarball_file_path != "/" {
                            fs::rename(&unpack_exe_file_path, &output_file_path)?;
                        }
                        Ok(output_file_path)
                    } else {
                        Err(eyre::format_err!(
                            "unpack file and exit code: {}",
                            state.code().unwrap_or(1),
                        ))
                    }
                }
                Err(e) => Err(eyre::format_err!("{}", e)),
            },
            Err(e) => Err(eyre::format_err!("{}", e)),
        }
    } else {
        extract_archive(
            GzDecoder::new(File::open(&src_filepath)?)?,
            extract_file_name,
            extract_file_in_tarball_file_path,
            &output_file_path,
        )?;

        Ok(output_file_path)
    }
}

fn extract_tar(
    src_filepath: &Path,
    dest_dir: &Path,
    extract_file_name: &str,
    extract_file_in_tarball_file_path: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(extract_file_name);

    extract_archive(
        File::open(&src_filepath)?,
        extract_file_name,
        extract_file_in_tarball_file_path,
        &output_file_path,
    )?;

    Ok(output_file_path)
}

fn extract_archive<R: Read>(
    reader: R,
    filename: &str,
    folder: &str,
    dest: &Path,
) -> Result<(), Report> {
    let mut archive = Archive::new(reader);
    archive.set_unpack_xattrs(true);
    archive.set_overwrite(true);
    archive.set_preserve_permissions(true);
    archive.set_preserve_mtime(true);

    let files = archive.entries()?.flatten();

    let target_file_path = format!("{}/{}", folder, filename).replace("//", "/");

    for mut entry in files {
        let file_path = entry.path()?;

        let relative_path = format!("{}", file_path.display());

        let absolute_path = format!(
            "/{}",
            relative_path
                .trim_start_matches("./")
                .trim_start_matches('/')
        );

        if target_file_path == absolute_path {
            entry.unpack(&dest)?;
            return Ok(());
        }
    }

    Err(eyre::format_err!(
        "can not found the file '{}' in tar",
        filename
    ))
}

fn extract_zip(
    src_filepath: &Path,
    dest_dir: &Path,
    extract_file_name: &str,
    _extract_file_in_tarball_file_path: &str,
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
    extract_file_in_tarball_file_path: &str,
) -> Result<PathBuf, Report> {
    let tar_file_name = tarball.file_name().unwrap().to_str().unwrap();

    let ensure_extract_file_exist = |s: &Path| {
        if s.exists() && s.is_file() {
            Ok(s.to_owned())
        } else {
            Err(eyre::format_err!(
                "can not found file '{}' in the root of tarball",
                &extract_file_name
            ))
        }
    };

    fs::create_dir_all(dest_dir)
        .map_err(|e| eyre::format_err!("can not create folder '{}': {}", dest_dir.display(), e))?;

    if tar_file_name.ends_with(".tar.gz") || tar_file_name.ends_with(".tgz") {
        match extract_tar_gz(
            tarball,
            dest_dir,
            extract_file_name,
            extract_file_in_tarball_file_path,
        ) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => Err(e),
        }
    } else if tar_file_name.ends_with(".tar") {
        match extract_tar(
            tarball,
            dest_dir,
            extract_file_name,
            extract_file_in_tarball_file_path,
        ) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => Err(e),
        }
    } else if tar_file_name.ends_with(".zip") {
        match extract_zip(
            tarball,
            dest_dir,
            extract_file_name,
            extract_file_in_tarball_file_path,
        ) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => Err(e),
        }
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
            extractor::extract(&tar_file_path, &dest_dir, "test_tar", "/").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 12);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        println!("{}", extracted_file_path.display());

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
            extractor::extract(&tar_file_path, &dest_dir, "test_zip", "/").unwrap();

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

        let r = extractor::extract(&tar_file_path, &dest_dir, "not_exist", "/");

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

        let extracted_file_path =
            extractor::extract(&tar_file_path, &dest_dir, "test", "/").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 12);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "hello world\n");
    }

    #[test]
    fn test_extract_tgz() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = extractor_dir.join("cross-env_darwin_amd64.tgz");

        let dest_dir = extractor_dir;

        let extracted_file_path =
            extractor::extract(&tar_file_path, &dest_dir, "cross-env", "/").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 153_464);

        fs::remove_file(extracted_file_path).ok();
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
            extractor::extract(&tar_file_path, &dest_dir, "prune.exe", "/").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 657_408);
    }

    #[test]
    fn test_extract_tar_gz_without_binary_file() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = extractor_dir.join("test_without_binary.tar.gz");

        let dest_dir = extractor_dir;

        let r = extractor::extract(&tar_file_path, &dest_dir, "without_test", "/");

        assert!(r.is_err())
    }

    #[test]
    fn test_extract_tar_gz_file_in_sub_folder() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = &extractor_dir.join("golangci-lint-in-sub-folder.tar.gz");

        let dest_dir = extractor_dir.clone();

        let dest_file = &extractor_dir.join("golangci-lint");

        fs::remove_file(&dest_file).ok();

        let r = extractor::extract(
            tar_file_path,
            &dest_dir,
            "golangci-lint",
            "/golangci-lint-1.45.0-darwin-amd64",
        );

        assert!(r.is_ok());

        assert!(dest_file.exists());
        assert!(dest_file.is_file());
        assert_eq!(
            format!("{}", dest_file.display()),
            format!("{}", r.unwrap().display())
        );
    }

    #[test]
    fn test_extract_tar_file_in_sub_folder() {
        let extractor_dir = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("extractor");

        let tar_file_path = &extractor_dir.join("sub-folder.tar");

        let dest_dir = extractor_dir.clone();

        let dest_file = &extractor_dir.join("sub-folder");

        fs::remove_file(&dest_file).ok();

        let r = extractor::extract(tar_file_path, &dest_dir, "sub-folder", "/sub-folder");

        assert!(r.is_ok());

        assert!(dest_file.exists());
        assert!(dest_file.is_file());
        assert_eq!(
            format!("{}", dest_file.display()),
            format!("{}", r.unwrap().display())
        );

        fs::remove_file(&dest_file).ok();
    }
}
