// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    env, fs,
    io::{Error, ErrorKind},
};

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let completions_out_dir = env::var("COMPLETIONS_OUT_DIR")
        .ok()
        .or_else(|| env::var("OUT_DIR").ok());

    if let Some(completions_out_dir) = &completions_out_dir {
        match fs::create_dir_all(completions_out_dir) {
            Ok(()) => (),
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => (),
                _ => return Err(err),
            },
        }

        let mut command = Cli::command();

        for &shell in Shell::value_variants() {
            generate_to(
                shell,
                &mut command,
                env!("CARGO_PKG_NAME"),
                completions_out_dir,
            )?;
        }
    }

    Ok(())
}
