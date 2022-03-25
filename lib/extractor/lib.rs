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
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Extension {
    TarGz,
    Tgz,
    TarBiz2,
    Tar,
    Zip,
}

impl Extension {
    pub fn as_str(&self) -> &'static str {
        match self {
            Extension::TarGz => ".tar.gz",
            Extension::Tgz => ".tgz",
            Extension::TarBiz2 => ".tar.bz2",
            Extension::Tar => ".tar",
            Extension::Zip => ".zip",
        }
    }
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

    if tar_file_name.ends_with(Extension::TarGz.as_str())
        || tar_file_name.ends_with(Extension::Tgz.as_str())
    {
        match tgz::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => handle_extract_error(e),
        }
    } else if tar_file_name.ends_with(Extension::TarBiz2.as_str()) {
        match tbz2::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => handle_extract_error(e),
        }
    } else if tar_file_name.ends_with(Extension::Tar.as_str()) {
        match tar::extract(tarball, dest_dir, filename, folder) {
            Ok(p) => ensure_extract_file_exist(&p),
            Err(e) => handle_extract_error(e),
        }
    } else if tar_file_name.ends_with(Extension::Zip.as_str()) {
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
