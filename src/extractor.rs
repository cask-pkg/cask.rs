#![deny(warnings)]

use core::result::Result;
use std::fs::File;
use std::path::{Path, PathBuf};

use eyre::Report;
use libflate::gzip::Decoder as GzDecoder;
use tar::Archive;

pub fn extract(
    tar_file_path: &Path,
    binary_name: &str,
    dest_dir: &Path,
) -> Result<PathBuf, Report> {
    let tar_file_name = tar_file_path.file_name().unwrap().to_str().unwrap();
    let output_file_path = dest_dir.join(binary_name);
    let mut binary_found = false;

    if tar_file_name.ends_with(".tar.gz") {
        let tar_file = File::open(&tar_file_path)?;
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
                if file_name.to_str().unwrap() == binary_name {
                    binary_found = true;
                    file.unpack(&output_file_path)?;
                    break;
                }
            }
        }

        if !binary_found {
            Err(eyre::format_err!(
                "can not found binary file '{}' in tar",
                binary_name
            ))
        } else {
            Ok(output_file_path)
        }
    } else if tar_file_name.ends_with(".tar") {
        let tar_file = File::open(&tar_file_path)?;
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
                if file_name.to_str().unwrap() == binary_name {
                    binary_found = true;
                    file.unpack(&output_file_path)?;
                    break;
                }
            }
        }

        if !binary_found {
            Err(eyre::format_err!(
                "can not found binary file '{}' in tar",
                binary_name
            ))
        } else {
            Ok(output_file_path)
        }
    } else {
        Err(eyre::format_err!(
            "Can not extract file from file '{}'",
            tar_file_name
        ))
    }
}
