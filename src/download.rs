use core::result::Result;
use eyre::Report;
use std::fs::File;
use std::path::Path;

pub fn download(url: &str, filepath: &Path) -> Result<(), Report> {
    let mut resp = reqwest::blocking::get(url)?;

    assert_eq!(resp.status(), 200);

    let mut dest = File::create(filepath)?;

    resp.copy_to(&mut dest)?;

    Ok(())
}
