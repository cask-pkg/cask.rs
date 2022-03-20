#![deny(warnings)]

use core::result::Result;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use eyre::Report;

use crate::archive;

pub(crate) fn extract(
    src_filepath: &Path,
    dest_dir: &Path,
    filename: &str,
    folder: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(filename);

    archive::extract_archive(
        File::open(&src_filepath)?,
        filename,
        folder,
        &output_file_path,
    )?;

    Ok(output_file_path)
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
}
