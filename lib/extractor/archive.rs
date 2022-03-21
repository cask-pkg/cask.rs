#![deny(warnings)]

use core::result::Result;
use std::{io::Read, path::Path};

use eyre::Report;
use tar::Archive;

pub(crate) fn extract<R: Read>(
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
