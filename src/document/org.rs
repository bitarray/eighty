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
use std::{path::Path, process::Command, collections::BTreeMap};

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgOutput {
    pub title: String,
    pub description: String,
    pub description_content: String,
    pub order: Option<usize>,
    pub content: String,
    pub toc: String,
    pub id: Option<String>,
    pub sitemap_title: Option<String>,
    pub revisions: BTreeMap<String, String>,
}

pub fn process_org(site_path: &Path, rel_path: &Path) -> Result<OrgOutput, Error> {
    let output = Command::new("eighty-pandoc")
        .arg(rel_path)
        .current_dir(site_path)
        .output()?;

    if !output.status.success() {
        return Err(Error::RunCommandFailed);
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}
