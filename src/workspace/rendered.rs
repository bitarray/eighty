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

use crate::{
    Error,
    document::RenderedDocument,
    file::FileMetadata,
    site::{SiteMetadata, SiteName},
    workspace::{MetadatadSite, MetadatadWorkspace, WorkspacePath},
};
use rayon::prelude::*;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedWorkspace {
    pub path: WorkspacePath,
    pub sites: HashMap<SiteName, RenderedSite>,
}

impl RenderedWorkspace {
    pub fn new(metadata: &MetadatadWorkspace) -> Result<RenderedWorkspace, Error> {
        let sites = metadata
            .sites
            .par_iter()
            .map(|(name, site)| Ok((name.clone(), RenderedSite::new(&site)?)))
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            sites,
            path: metadata.path.clone(),
        })
    }

    pub fn new_with_old(
        metadata: &MetadatadWorkspace,
        old: &RenderedWorkspace,
    ) -> Result<RenderedWorkspace, Error> {
        let sites = metadata
            .sites
            .par_iter()
            .map(|(name, site)| {
                if let Some(old_site) = old.sites.get(&name) {
                    Ok((name.clone(), RenderedSite::new_with_old(&site, old_site)?))
                } else {
                    Ok((name.clone(), RenderedSite::new(&site)?))
                }
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            sites,
            path: metadata.path.clone(),
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedSite {
    pub site: Arc<SiteMetadata>,
    pub documents: HashMap<PathBuf, RenderedDocument>,
    pub files: Arc<HashMap<PathBuf, FileMetadata>>,
}

impl RenderedSite {
    pub fn new(metadata: &MetadatadSite) -> Result<RenderedSite, Error> {
        let documents = metadata
            .documents
            .par_iter()
            .map(|document| {
                Ok((
                    document.rel_source_path.clone(),
                    RenderedDocument::new(metadata.site.clone(), document.clone())?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            site: metadata.site.clone(),
            files: metadata.files.clone(),
            documents,
        })
    }

    pub fn new_with_old(
        metadata: &MetadatadSite,
        old: &RenderedSite,
    ) -> Result<RenderedSite, Error> {
        let documents = metadata
            .documents
            .par_iter()
            .map(|document| {
                if let Some(old_document) = old.documents.get(&document.rel_source_path) {
                    if old_document.site_metadata == metadata.site
                        && old_document.metadata == *document
                    {
                        return Ok((document.rel_source_path.clone(), old_document.clone()));
                    }
                }

                Ok((
                    document.rel_source_path.clone(),
                    RenderedDocument::new(metadata.site.clone(), document.clone())?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            site: metadata.site.clone(),
            files: metadata.files.clone(),
            documents,
        })
    }
}
