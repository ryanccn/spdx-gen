use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// An SPDX license identifier (prompts with select if not provided)
    #[clap(short, long, env = "SPDX_GEN_LICENSE")]
    pub license: Option<String>,

    /// The output file path (defaults to LICENSE in the current working directory)
    #[clap(short, long, value_hint = ValueHint::FilePath, env = "SPDX_GEN_OUTPUT")]
    pub output: Option<PathBuf>,

    /// Overwrite if output file already exists
    #[clap(short, long, env = "SPDX_GEN_FORCE")]
    pub force: bool,

    /// Don't automatically update the cache
    #[clap(long, env = "SPDX_GEN_NO_UPDATE")]
    pub no_update: bool,

    /// Cache directory to use (default is dependent on platform)
    #[clap(long, env = "SPDX_GEN_CACHE_DIR")]
    pub cache_dir: Option<PathBuf>,

    /// Update the cache without doing anything else
    #[clap(long)]
    pub update: bool,

    /// Generate completions for a specific shell
    #[clap(long)]
    pub completions: Option<clap_complete::Shell>,
}
