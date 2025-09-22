// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-only

use serde::Deserialize;
use std::path::Path;
use tokio::fs;

use eyre::Result;

use crate::update;

#[derive(Deserialize, Debug)]
struct LicenseManifest {
    licenses: Vec<License>,
}

#[derive(Deserialize, Debug)]
pub struct License {
    pub name: String,
    #[serde(rename = "licenseId")]
    pub id: String,
    #[serde(rename = "isDeprecatedLicenseId")]
    pub deprecated: bool,
}

pub async fn read_manifest(cache_dir: &Path, allow_deprecated: bool) -> Result<Vec<License>> {
    let data = fs::read(
        update::repo_path(cache_dir)
            .join("json")
            .join("licenses.json"),
    )
    .await?;

    let LicenseManifest { mut licenses } = serde_json::from_slice(&data)?;
    licenses.retain(|l| allow_deprecated || !l.deprecated);

    Ok(licenses)
}

pub async fn read_text(cache_dir: &Path, license: &License) -> Result<String> {
    Ok(
        fs::read_to_string(update::repo_path(cache_dir).join("text").join(format!(
            "{}{}.txt",
            if license.deprecated {
                "deprecated_"
            } else {
                ""
            },
            license.id
        )))
        .await?,
    )
}
