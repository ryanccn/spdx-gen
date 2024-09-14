use std::io;
use std::{fs, path::PathBuf};

use clap::{CommandFactory as _, Parser, ValueHint};

use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
use eyre::{eyre, Result};
use owo_colors::{OwoColorize as _, Stream};

mod licenses;
mod update;

use crate::licenses::{read_license_text, read_licenses};
use crate::update::update_spdx;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// An SPDX license identifier (prompts with select if not provided)
    #[clap(short, long)]
    license: Option<String>,

    /// The output file path (defaults to LICENSE in the current working directory)
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,

    /// Cache directory to use (defaults to "$XDG_CACHE_HOME/spdx-gen")
    #[clap(long)]
    cache_dir: Option<PathBuf>,

    /// Generate completions for a specific shell
    #[clap(long)]
    completions: Option<clap_complete::Shell>,
}

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

    let cache_dir = update_spdx().await?;

    let licenses = read_licenses(&cache_dir).await?;

    eprintln!(
        "{} {} SPDX licenses supported",
        "i".if_supports_color(Stream::Stderr, |t| t.blue()),
        licenses
            .len()
            .if_supports_color(Stream::Stderr, |t| t.bold())
    );

    let license_idx = match &cli.license {
        Some(license) => licenses
            .iter()
            .position(|l| &l.id == license)
            .ok_or_else(|| eyre!("Invalid SPDX license identifier provided"))?,

        None => FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an SPDX license")
            .max_length(5)
            .items(&licenses.iter().map(|l| l.name.clone()).collect::<Vec<_>>())
            .interact()?,
    };

    let license = &licenses[license_idx];

    if cli.license.is_some() {
        eprintln!(
            "{} Selected {} via CLI",
            "✔".if_supports_color(Stream::Stderr, |s| s.green()),
            license.id.if_supports_color(Stream::Stderr, |s| s.bold()),
        );
    }

    let output = cli.output.unwrap_or_else(|| PathBuf::from("LICENSE"));

    if !output.exists()
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{} already exists. Overwrite?", output.display()))
            .interact()?
    {
        let text = read_license_text(&cache_dir, license).await?;
        fs::write(output, &text)?;

        eprintln!(
            "{} Wrote {} ({}).",
            "✔".if_supports_color(Stream::Stderr, |s| s.green()),
            license.name.bold(),
            license.id
        );

        eprintln!(
            "{} {}",
            "!".if_supports_color(Stream::Stderr, |s| s.yellow())
                .if_supports_color(Stream::Stderr, |s| s.dimmed()),
            "Check the license for placeholders to replace with your information!"
                .if_supports_color(Stream::Stderr, |s| s.dimmed())
        );
    }

    Ok(())
}
