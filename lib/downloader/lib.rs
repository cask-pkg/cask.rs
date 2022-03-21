// #![deny(warnings)]

use core::result::Result;
use std::{cmp::min, fs, fs::File, io::Write, path::Path};

use eyre::Report;
#[cfg(feature = "lib")]
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn download(url: &str, filepath: &Path) -> Result<(), Report> {
    if cfg!(feature = "lib") {
        let client = &reqwest::Client::new();

        let res = client.get(url).send().await?;

        if res.status() != 200 {
            return Err(eyre::format_err!(
                "Download {} fail with http code {}",
                &url,
                res.status()
            ));
        }

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
    } else if cfg!(feature = "cmd") {
        println!("download {} to {}", url, filepath.display());
        Ok(())
    } else {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use crate::download;

    #[tokio::test]

    async fn test_download() {
        let url =
            "https://github.com/axetroy/prune.v/releases/download/v0.2.14/prune_darwin_amd64.tar.gz";

        let cwd = env::current_dir().unwrap();

        let dest = cwd.join("cask_darwin_amd64.tar.gz");

        download(url, &dest).await.unwrap();

        assert!(dest.exists());

        let meta = fs::metadata(&dest).unwrap();

        assert!(meta.is_file());
        assert_eq!(meta.len(), 62_310);

        fs::remove_file(&dest).unwrap();
    }

    #[tokio::test]

    async fn test_download_invalid_url() {
        let url = "https://github.com/axetroy/prune.v/releases/download/v0.2.14/not_exist.tar.gz";

        let cwd = env::current_dir().unwrap();

        let dest = cwd.join("cask_darwin_amd64.tar.gz");

        let r = download(url, &dest).await;

        assert!(r.is_err())
    }
}
