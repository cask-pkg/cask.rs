#![deny(warnings)]

mod archive;
mod tar;
mod tbz2;
mod tgz;
mod zip;

use core::result::Result;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use eyre::Report;

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
        match tgz::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => Err(e),
        }
    } else if tar_file_name.ends_with(".tar.bz2") {
        match tbz2::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => Err(e),
        }
    } else if tar_file_name.ends_with(".tar") {
        match tar::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => Err(e),
        }
    } else if tar_file_name.ends_with(".zip") {
        match zip::extract(tarball, dest_dir, filename, folder) {
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
