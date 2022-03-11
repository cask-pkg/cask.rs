use core::result::Result;
use eyre::Report;
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub fn download(url: &str, dest: &Path) -> Result<(), Report> {
    let resp = reqwest::blocking::get(url)?;
    let content = resp.text()?;
    let mut dest = File::create(dest)?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}
