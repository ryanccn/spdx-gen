// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::path::Path;
use tokio::fs;

use eyre::Result;

use crate::update::repo_dir;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LicenseManifest {
    licenses: Vec<License>,
}

#[derive(serde::Deserialize, Debug)]
pub struct License {
    pub name: String,
    #[serde(rename = "licenseId")]
    pub id: String,
    #[serde(rename = "isDeprecatedLicenseId")]
    pub deprecated: bool,
}

pub async fn read_licenses(cache_dir: &Path) -> Result<Vec<License>> {
    let data = fs::read(repo_dir(cache_dir).join("json").join("licenses.json")).await?;
    let manifest: LicenseManifest = serde_json::from_slice(&data)?;

    Ok(manifest
        .licenses
        .into_iter()
        .filter(|l| !l.deprecated)
        .collect())
}

pub async fn read_license_text(cache_dir: &Path, license: &License) -> Result<String> {
    let path = repo_dir(cache_dir).join("text").join(format!(
        "{}.txt",
        if license.deprecated {
            "deprecated_".to_owned() + &license.id
        } else {
            license.id.clone()
        }
    ));

    let text = fs::read_to_string(path).await?;

    Ok(text)
}
