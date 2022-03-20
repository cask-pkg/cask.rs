#![deny(warnings)]

use core::result::Result;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use eyre::Report;
use libflate::gzip::Decoder as GzDecoder;
use tar::Archive;

fn extract_tar_gz(
    src_filepath: &Path,
    dest_dir: &Path,
    filename: &str,
    folder: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(filename);

    extract_archive(
        GzDecoder::new(File::open(&src_filepath)?)?,
        filename,
        folder,
        &output_file_path,
    )?;

    Ok(output_file_path)
}

fn extract_tar(
    src_filepath: &Path,
    dest_dir: &Path,
    filename: &str,
    folder: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(filename);

    extract_archive(
        File::open(&src_filepath)?,
        filename,
        folder,
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
        "can not found file '{}' in the '{}' of tarball",
        &filename,
        folder
    ))
}

fn extract_zip(
    src_filepath: &Path,
    dest_dir: &Path,
    filename: &str,
    folder: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(filename);

    let tar_file = File::open(&src_filepath)?;
    let mut archive = zip::ZipArchive::new(tar_file)?;

    let target_file_path = format!("{}/{}", folder, filename).replace("//", "/");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.is_dir() {
            continue;
        }

        let absolute_path = format!("/{}", file.name());

        if target_file_path == absolute_path {
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
        "can not found file '{}' in the '{}' of tarball",
        &filename,
        folder
    ))
}

pub fn extract(
    tarball: &Path,
    dest_dir: &Path,
    filename: &str,
    folder: &str,
) -> Result<PathBuf, Report> {
    let tar_file_name = tarball.file_name().unwrap().to_str().unwrap();

    let ensure_extract_file_exist = |s: &Path| {
        if s.exists() && s.is_file() {
            Ok(s.to_owned())
        } else {
            Err(eyre::format_err!(
                "can not found file '{}' in the '{}' of tarball",
                &filename,
                folder
            ))
        }
    };

    fs::create_dir_all(dest_dir)
        .map_err(|e| eyre::format_err!("can not create folder '{}': {}", dest_dir.display(), e))?;

    if tar_file_name.ends_with(".tar.gz") || tar_file_name.ends_with(".tgz") {
        match extract_tar_gz(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => Err(e),
        }
    } else if tar_file_name.ends_with(".tar") {
        match extract_tar(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => Err(e),
        }
    } else if tar_file_name.ends_with(".zip") {
        match extract_zip(tarball, dest_dir, filename, folder) {
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
    use std::{env, fs};

    use crate::extract;

    #[test]
    fn test_extract_tar_00() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("tar");

        let tar_file_path = extractor_dir.join("00.tar");

        let dest_dir = extractor_dir;

        let extracted_file_path = extract(&tar_file_path, &dest_dir, "00.txt", "/").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 2);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "00");

        fs::remove_file(extracted_file_path).ok();
    }

    #[test]
    fn test_extract_tar_01() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("tar");

        let tar_file_path = extractor_dir.join("01.tar");

        let dest_dir = extractor_dir;

        let extracted_file_path =
            extract(&tar_file_path, &dest_dir, "01.txt", "/sub-folder").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 2);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "01");

        fs::remove_file(extracted_file_path).ok();
    }

    #[test]
    fn test_extract_tar_02() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("tar");

        let tar_file_path = extractor_dir.join("02.tar");

        let dest_dir = extractor_dir;

        let r = extract(&tar_file_path, &dest_dir, "not_exist", "/");

        assert!(r.is_err());
    }

    #[test]
    fn test_extract_zip_00() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("zip");

        let tar_file_path = extractor_dir.join("00.zip");

        let dest_dir = extractor_dir;

        let extracted_file_path = extract(&tar_file_path, &dest_dir, "00.txt", "/").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 2);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "00");

        fs::remove_file(extracted_file_path).ok();
    }

    #[test]
    fn test_extract_zip_01() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("zip");

        let tar_file_path = extractor_dir.join("01.zip");

        let dest_dir = extractor_dir;

        let extracted_file_path =
            extract(&tar_file_path, &dest_dir, "01.txt", "/sub-folder").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 2);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "01");

        fs::remove_file(extracted_file_path).ok();
    }

    #[test]
    fn test_extract_zip_02() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("zip");

        let tar_file_path = extractor_dir.join("02.zip");

        let dest_dir = extractor_dir;

        let r = extract(&tar_file_path, &dest_dir, "not_exist", "/");

        assert!(r.is_err());
    }

    #[test]
    fn test_extract_tgz_00() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("tgz");

        let tar_file_path = extractor_dir.join("00.tgz");

        let dest_dir = extractor_dir;

        let extracted_file_path = extract(&tar_file_path, &dest_dir, "00.txt", "/").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 2);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "00");

        fs::remove_file(extracted_file_path).ok();
    }

    #[test]
    fn test_extract_tgz_01() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("tgz");

        let tar_file_path = extractor_dir.join("01.tgz");

        let dest_dir = extractor_dir;

        let extracted_file_path =
            extract(&tar_file_path, &dest_dir, "01.txt", "/sub-folder").unwrap();

        let meta = fs::metadata(&extracted_file_path).unwrap();

        assert_eq!(meta.len(), 2);

        let content = fs::read_to_string(&extracted_file_path).unwrap();

        assert_eq!(content, "01");

        fs::remove_file(extracted_file_path).ok();
    }

    #[test]
    fn test_extract_tgz_02() {
        let extractor_dir = env::current_dir().unwrap().join("fixtures").join("tgz");

        let tar_file_path = extractor_dir.join("02.tgz");

        let dest_dir = extractor_dir;

        let r = extract(&tar_file_path, &dest_dir, "not_exist", "/");

        assert!(r.is_err());
    }
}
