#![deny(warnings)]

use core::result::Result;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use eyre::Report;
use libflate::gzip::Decoder as GzDecoder;

use crate::archive;

pub(crate) fn extract(
    src_filepath: &Path,
    dest_dir: &Path,
    filename: &str,
    folder: &str,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(filename);

    archive::extract(
        GzDecoder::new(File::open(src_filepath)?)?,
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
