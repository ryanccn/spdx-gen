// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-only

use eyre::{Result, eyre};
use std::{fs, io, path::PathBuf};

use clap::{CommandFactory as _, Parser as _};

use anstream::eprintln;
use dialoguer::{Confirm, FuzzySelect, theme::ColorfulTheme};
use owo_colors::OwoColorize as _;

mod cli;
mod licenses;
mod update;

use crate::cli::Cli;
use crate::update::{auto_update, default_cache_dir_path, update};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if let Some(completion) = &cli.completions {
        let cmd = &mut Cli::command();
        clap_complete::generate(
            *completion,
            cmd,
            cmd.get_name().to_string(),
            &mut io::stdout(),
        );

        return Ok(());
    }

    let cache_dir = match cli.cache_dir {
        Some(cache_dir) => cache_dir,
        None => default_cache_dir_path()?,
    };

    if cli.update {
        update(&cache_dir).await?;
        return Ok(());
    }

    if !cli.no_update {
        auto_update(&cache_dir).await?;
    }

    let licenses = licenses::read_manifest(&cache_dir, cli.allow_deprecated).await?;

    eprintln!(
        "{} {} SPDX licenses supported{}",
        "i".blue(),
        licenses.len().bold(),
        if cli.allow_deprecated {
            " (including deprecated)".yellow().to_string()
        } else {
            String::new()
        }
    );

    let license_idx = match &cli.license {
        Some(license) => licenses
            .iter()
            .position(|l| l.id.eq_ignore_ascii_case(license))
            .ok_or_else(|| eyre!("Invalid SPDX license identifier provided"))?,

        None => FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an SPDX license")
            .max_length(5)
            .items(&licenses.iter().map(|l| l.name.clone()).collect::<Vec<_>>())
            .interact()?,
    };

    let license = &licenses[license_idx];

    if cli.license.is_some() {
        eprintln!("{} Selected {} via CLI", "✔".green(), license.id.bold());
    }

    let output = cli.output.unwrap_or_else(|| PathBuf::from("LICENSE"));

    if !output.exists()
        || cli.force
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{} already exists. Overwrite?", output.display()))
            .interact()?
    {
        let text = licenses::read_text(&cache_dir, license).await?;
        fs::write(output, &text)?;

        eprintln!(
            "{} Wrote {} ({}).",
            "✔".green(),
            license.name.bold(),
            license.id
        );

        eprintln!(
            "{} {}",
            "!".yellow().dimmed(),
            "Check the license for placeholders to replace with your information!".dimmed()
        );
    }

    Ok(())
}
