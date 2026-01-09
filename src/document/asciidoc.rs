// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Eighty.
//
// Copyright (c) 2021 Wei Tang.
//
// Eighty is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Eighty is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Eighty. If not, see <http://www.gnu.org/licenses/>.

use crate::Error;
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Command};
use tracing::error;

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsciiDocOutput {
    pub document: AsciiDocOutputDocument,
    pub specs: Vec<AsciiDocOutputSpec>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsciiDocOutputDocument {
    pub title: String,
    pub license: Option<String>,
    pub license_code: Option<String>,
    pub author: Option<String>,
    pub email: Option<String>,
    pub order: Option<usize>,
    pub toc: Option<String>,
    pub created: Option<String>,
    pub description: String,
    pub content: String,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsciiDocOutputSpec {
    pub id: String,
    pub description: String,
    pub discuss: String,
    pub source_path: String,
    pub anchor: String,
}

pub fn process_asciidoc(site_path: &Path, rel_path: &Path) -> Result<AsciiDocOutput, Error> {
    let output = Command::new("eighty-asciidoc")
        .arg(rel_path)
        .current_dir(site_path)
        .output()?;

    if !output.status.success() {
        match str::from_utf8(&output.stderr) {
            Ok(stderr) => error!("Asciidoc command failed with stderr: {}", stderr),
            Err(_) => error!("Asciidoc command failed and cannot decode stderr."),
        }
        return Err(Error::RunCommandFailed);
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}
