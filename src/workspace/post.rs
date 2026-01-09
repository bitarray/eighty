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

use super::FullWorkspace;
use crate::{Error, site::SiteName};
use std::{collections::HashMap, ops::Deref, path::PathBuf};

pub struct SimplePostWorkspace(pub HashMap<SiteName, SimplePostSite>);

impl SimplePostWorkspace {
    pub fn new(full: &FullWorkspace) -> Result<Self, Error> {
        let mut sites = HashMap::new();

        for (site_name, full_site) in &full.sites {
            let mut post_site = HashMap::new();

            for (asset_path, asset) in &full_site.assets.assets {
                post_site.insert(asset_path.clone(), asset.clone());
            }

            for (file_path, file) in full_site.files.as_ref() {
                post_site.insert(file_path.clone(), file.content.clone());
            }

            for document in &full_site.documents {
                post_site.insert(
                    document.rendered.name.path(),
                    document.content.as_bytes().to_owned(),
                );
            }

            sites.insert(
                site_name.clone(),
                SimplePostSite {
                    base_url: full_site.site.config.base_url.clone(),
                    files: post_site,
                },
            );
        }

        Ok(Self(sites))
    }
}

impl Deref for SimplePostWorkspace {
    type Target = HashMap<SiteName, SimplePostSite>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SimplePostSite {
    pub base_url: String,
    pub files: HashMap<PathBuf, Vec<u8>>,
}
