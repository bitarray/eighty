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

use eighty::{
    Error,
    workspace::{FullWorkspace, MetadatadWorkspace, RenderedWorkspace, SimplePostWorkspace},
};
use std::{fs, path::Path};

pub fn build(site_path: &Path, target_path: &Path) -> Result<(), Error> {
    let metadatad = MetadatadWorkspace::new_single(&site_path)?.0;
    let rendered = RenderedWorkspace::new(&metadatad)?;
    let full = FullWorkspace::new(&rendered)?;
    let post = SimplePostWorkspace::new(&full)?;

    for (site_name, site) in &post.0 {
        let site_target_path = target_path.join(&site_name.0);

        for (file_path, file) in &site.files {
            let file_target_path = site_target_path.join(file_path);

            fs::create_dir_all(
                file_target_path
                    .parent()
                    .ok_or(Error::InvalidPathComponent)?,
            )?;
            fs::write(file_target_path, &file[..])?;
        }
    }

    Ok(())
}

pub fn build_all(root_path: &Path, target_path: &Path) -> Result<(), Error> {
    let metadatad = MetadatadWorkspace::new_multi(&root_path)?;
    let rendered = RenderedWorkspace::new(&metadatad)?;
    let full = FullWorkspace::new(&rendered)?;
    let post = SimplePostWorkspace::new(&full)?;

    for (site_name, site) in &post.0 {
        let site_target_path = target_path.join(&site_name.0);

        for (file_path, file) in &site.files {
            let file_target_path = site_target_path.join(file_path);

            fs::create_dir_all(
                file_target_path
                    .parent()
                    .ok_or(Error::InvalidPathComponent)?,
            )?;
            fs::write(file_target_path, &file[..])?;
        }
    }

    Ok(())
}
