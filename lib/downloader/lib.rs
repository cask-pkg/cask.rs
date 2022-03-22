// #![deny(warnings)]

mod rustls;

use core::result::Result;
use std::path::Path;

use eyre::Report;

pub async fn download(url: &str, filepath: &Path) -> Result<(), Report> {
    rustls::download(url, filepath).await
}
