#![deny(warnings)]

use core::result::Result;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use eyre::Report;

pub(crate) fn extract(
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

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use crate::extract;

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
}
