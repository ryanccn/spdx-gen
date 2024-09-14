use std::{
    io::Cursor,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::fs;

use eyre::Result;
use owo_colors::{OwoColorize, Stream};

use flate2::read::GzDecoder;
use tar::Archive;

use xdg::BaseDirectories;

pub async fn update_spdx() -> Result<PathBuf> {
    let cache_dir = BaseDirectories::new()?.create_cache_directory("spdx-gen")?;
    let repo_dir = cache_dir.join("license-list-data-main");
    let updated_file = cache_dir.join("updated");

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    if !fs::read_to_string(&updated_file)
        .await
        .is_ok_and(|s| s.parse::<u128>().is_ok_and(|st| now - st <= 1_209_600_000))
        || !repo_dir.exists()
    {
        eprintln!(
            "{} Updating SPDX license data...",
            "↓".if_supports_color(Stream::Stderr, |t| t.cyan())
        );

        let _ = fs::remove_dir_all(&repo_dir).await;

        let mut response = reqwest::get(
            "https://github.com/spdx/license-list-data/archive/refs/heads/main.tar.gz",
        )
        .await?
        .error_for_status()?;

        let mut data = Vec::new();

        while let Some(chunk) = response.chunk().await? {
            data.extend(&chunk);
        }

        let decoder = GzDecoder::new(Cursor::new(data));
        let mut archive = Archive::new(decoder);
        archive.unpack(&cache_dir)?;

        fs::write(&updated_file, now.to_string()).await?;
    }

    Ok(repo_dir)
}
