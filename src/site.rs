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
use serde::{de::Deserializer, Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

#[derive(Hash, Eq, Clone, PartialEq, Debug, PartialOrd, Ord)]
pub struct SiteName(pub String);

impl fmt::Display for SiteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfig {
    pub name: String,
    pub title: String,
    #[serde(deserialize_with = "deserialize_site_url")]
    pub url: String,
    #[serde(deserialize_with = "deserialize_site_url")]
    pub base_url: String,
    pub author: String,
    pub email: String,
    pub sitemap: SiteConfigSitemap,
    #[serde(default)]
    pub links: Vec<SiteConfigLink>,
}

fn deserialize_site_url<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    if !value.ends_with("/") {
        return Err(<D::Error as serde::de::Error>::custom(
            "site url must end with /",
        ));
    }

    Ok(value)
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfigLink {
    pub name: String,
    pub url: String,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfigSitemap {
    pub enable: bool,
    pub depth: Option<usize>,
    #[serde(default)]
    pub extra_links: Vec<SiteConfigSitemapExtraLink>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfigSitemapExtraLink {
    pub title: String,
    pub url: String,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SiteMetadata {
    pub name: SiteName,
    pub source_path: PathBuf,
    pub config: SiteConfig,
}

impl SiteMetadata {
    pub fn new(expected_sitename: Option<SiteName>, path: &Path) -> Result<SiteMetadata, Error> {
        let site_config_path = path.join("_site.json");
        let site_config: SiteConfig = serde_json::from_reader(BufReader::new(File::open(site_config_path)?))?;

        let name = SiteName(site_config.name.clone());
        if name.0 == "specs" {
            return Err(Error::ReservedSiteName);
        }
        if let Some(expected_sitename) = expected_sitename && expected_sitename != name {
            return Err(Error::UnexpectedSiteName);
        }

        let site = SiteMetadata {
            name,
            source_path: path.to_owned(),
            config: site_config,
        };

        Ok(site)
    }
}
