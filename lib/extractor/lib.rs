#![deny(warnings)]

mod archive;
mod tar;
mod tbz2;
mod tgz;
mod zip;

use core::result::Result;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use eyre::Report;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("can not create folder '{folder:?}': {source:?}")]
    CreateFolderFail { folder: String, source: io::Error },
    #[error("can not found file '{filename:?}' in path '{path:?}' for tarball")]
    FindNotFoundInTarball { filename: String, path: String },
    #[error("not support extract file from '{filename:?}'")]
    NotSupportExtension { filename: String },
    #[error("extract file '{filename:?}' in '{path:?}' error: {msg:?}")]
    ExtractFail {
        filename: String,
        path: String,
        msg: String,
    },
}

pub fn extract(
    tarball: &Path,
    dest_dir: &Path,
    filename: &str,
    folder: &str,
) -> Result<PathBuf, ExtractorError> {
    let tar_file_name = tarball.file_name().unwrap().to_str().unwrap();

    let ensure_extract_file_exist = |s: &Path| {
        if s.exists() && s.is_file() {
            Ok(s.to_owned())
        } else {
            Err(ExtractorError::FindNotFoundInTarball {
                filename: filename.to_string(),
                path: folder.to_string(),
            })
        }
    };

    let handle_extract_error = |e: Report| {
        Err(ExtractorError::ExtractFail {
            filename: filename.to_string(),
            path: folder.to_string(),
            msg: format!("{}", e),
        })
    };

    fs::create_dir_all(dest_dir).map_err(|e| ExtractorError::CreateFolderFail {
        folder: format!("{}", dest_dir.display()),
        source: e,
    })?;

    if tar_file_name.ends_with(".tar.gz") || tar_file_name.ends_with(".tgz") {
        match tgz::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => handle_extract_error(e),
        }
    } else if tar_file_name.ends_with(".tar.bz2") {
        match tbz2::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => handle_extract_error(e),
        }
    } else if tar_file_name.ends_with(".tar") {
        match tar::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => handle_extract_error(e),
        }
    } else if tar_file_name.ends_with(".zip") {
        match zip::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => handle_extract_error(e),
        }
    } else {
        Err(ExtractorError::NotSupportExtension {
            filename: tar_file_name.to_string(),
        })
    }
}
