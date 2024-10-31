// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-only

use crossterm::{cursor, terminal, ExecutableCommand as _};
use std::{
    io::{self, Cursor},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::fs;

use eyre::{eyre, Result};
use owo_colors::{OwoColorize as _, Stream};

use flate2::read::GzDecoder;
use tar::Archive;

pub fn default_cache_dir() -> Result<PathBuf> {
    Ok(dirs::cache_dir()
        .ok_or_else(|| eyre!("could not obtain cache directory"))?
        .join("spdx-gen"))
}

pub fn repo_dir(cache_dir: &Path) -> PathBuf {
    cache_dir.join("license-list-data-main")
}

pub fn updated_file(cache_dir: &Path) -> PathBuf {
    cache_dir.join("updated")
}

pub async fn update(cache_dir: &Path) -> Result<()> {
    let repo_dir = repo_dir(cache_dir);
    let updated_file = updated_file(cache_dir);

    eprint!(
        "{} Updating SPDX license data... ",
        "↓".if_supports_color(Stream::Stderr, |t| t.cyan())
    );

    io::stderr().execute(cursor::SavePosition)?;

    let _ = fs::remove_dir_all(&repo_dir).await;

    let client = reqwest::Client::builder().https_only(true).build()?;

    let mut response = client
        .get("https://github.com/spdx/license-list-data/archive/refs/heads/main.tar.gz")
        .send()
        .await?
        .error_for_status()?;

    let mut data = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        data.extend(&chunk);

        io::stderr()
            .execute(cursor::RestorePosition)?
            .execute(terminal::Clear(terminal::ClearType::UntilNewLine))?;

        eprint!(
            "{}",
            humansize::format_size(
                data.len(),
                humansize::FormatSizeOptions::default()
                    .space_after_value(true)
                    .decimal_places(1)
            )
            .if_supports_color(Stream::Stderr, |t| t.dimmed()),
        );
    }

    eprintln!();

    let decoder = GzDecoder::new(Cursor::new(data));
    let mut archive = Archive::new(decoder);
    archive.unpack(cache_dir)?;

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    fs::write(&updated_file, now.to_string()).await?;

    Ok(())
}

pub async fn auto_update(cache_dir: &Path) -> Result<()> {
    let repo_dir = repo_dir(cache_dir);
    let updated_file = updated_file(cache_dir);

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    if !fs::read_to_string(&updated_file)
        .await
        .is_ok_and(|s| s.parse::<u128>().is_ok_and(|st| now - st <= 1_209_600_000))
        || !repo_dir.exists()
    {
        update(cache_dir).await?;
    }

    Ok(())
}
