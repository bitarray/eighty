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
use handlebars::Handlebars;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct AssetStore {
    pub assets: HashMap<PathBuf, Vec<u8>>,
    pub handlebars: Handlebars<'static>,
}

impl AssetStore {
    pub fn new(root_path: &Path) -> Result<AssetStore, Error> {
        let mut assets = HashMap::new();

        let asset_path = root_path.join("_assets");

        let css_path = asset_path.join("css");
        let font_path = asset_path.join("font");
        let js_path = asset_path.join("js");
        for part_path in [css_path, font_path, js_path] {
            for entry in WalkDir::new(&part_path) {
                let entry = entry?;

                if entry.file_type().is_file() {
                    let content = fs::read(entry.path())?;
                    let rel_path = entry.path().strip_prefix(&asset_path)?;

                    assets.insert(rel_path.to_owned(), content);
                }
            }
        }

        let static_path = asset_path.join("static");
        for entry in WalkDir::new(&static_path) {
            let entry = entry?;

            if entry.file_type().is_file() {
                let content = fs::read(entry.path())?;
                let rel_path = entry.path().strip_prefix(&static_path)?;

                assets.insert(rel_path.to_owned(), content);
            }
        }

        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_templates_directory(asset_path.join("layouts"), Default::default())?;

        Ok(AssetStore { assets, handlebars })
    }
}
