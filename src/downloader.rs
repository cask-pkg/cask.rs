use core::result::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{cmp::min, fs};

use eyre::Report;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

pub async fn download(url: &str, filepath: &Path) -> Result<(), Report> {
    let client = &Client::new();

    let res = client.get(url).send().await?;

    assert_eq!(res.status(), 200);

    let total_size = res
        .content_length()
        .ok_or_else(|| eyre::format_err!("Failed to get content length from {}", &url))?;

    let progress_template = "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(progress_template)
            .progress_chars("#>-"),
    );
    pb.set_message(format!("Downloading {}", url));

    if filepath.exists() {
        fs::remove_file(filepath)?;
    }

    let mut dest = File::create(filepath)?;

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|_| eyre::format_err!("Error while downloading file"))?;

        dest.write_all(&chunk)
            .map_err(|_| eyre::format_err!("Error while write file"))?;

        downloaded = min(downloaded + (chunk.len() as u64), total_size);

        pb.set_position(downloaded);
    }

    pb.finish();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::downloader;
    use std::{env, fs};

    #[tokio::test]

    async fn test_symlink() {
        let url =
            "https://github.com/axetroy/prune.v/releases/download/v0.2.14/prune_darwin_amd64.tar.gz";

        let cwd = env::current_dir().unwrap();

        let dest = cwd.join("cask_darwin_amd64.tar.gz");

        downloader::download(url, &dest).await.unwrap();

        assert!(dest.exists());

        let meta = fs::metadata(&dest).unwrap();

        assert!(meta.is_file());
        assert_eq!(meta.len(), 62_310);

        fs::remove_file(&dest).unwrap();
    }
}
