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
    asset::AssetStore,
    document::{DocumentMetadata, DocumentName, RenderedData},
    file::FileMetadata,
    layout,
    site::{SiteMetadata, SiteName},
    sitemap::{BreadcrumbItem, LocalSitemap, Sitemap},
    utils,
    variable::{self, Variable},
    workspace::{RenderedSite, RenderedWorkspace, WorkspacePath},
};
use lol_html::{RewriteStrSettings, element, rewrite_str};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub struct FullWorkspace {
    pub path: WorkspacePath,
    pub sites: HashMap<SiteName, FullSite>,
}

impl FullWorkspace {
    pub fn new(rendered: &RenderedWorkspace) -> Result<Self, Error> {
        let sites = rendered
            .sites
            .iter()
            .map(|(name, site)| Ok((name.clone(), FullSite::new(&site)?)))
            .collect::<Result<HashMap<SiteName, FullSite>, Error>>()?;

        Ok(Self {
            path: rendered.path.clone(),
            sites,
        })
    }
}

pub struct FullSite {
    pub site: Arc<SiteMetadata>,
    pub documents: Vec<FullDocument>,
    pub files: Arc<HashMap<PathBuf, FileMetadata>>,
    pub xrefs: HashMap<PathBuf, DocumentName>,
    pub sitemap: Sitemap,
    pub assets: AssetStore,
}

impl FullSite {
    pub fn new(rendered: &RenderedSite) -> Result<Self, Error> {
        let assets = AssetStore::new(&rendered.site.source_path)?;

        let name_titles = rendered
            .documents
            .iter()
            .map(|(_, v)| BreadcrumbItem {
                document_name: v.data.name.clone(),
                title: v.data.sitemap_title.clone().unwrap_or(v.data.title.clone()),
                description: v.data.description.clone(),
                order: v.data.order.clone(),
            })
            .collect::<Vec<_>>();

        let sitemap = Sitemap::from(name_titles.clone());

        let mut xrefs = HashMap::new();
        for (_, document) in &rendered.documents {
            let rel_path = document
                .metadata
                .source_path
                .strip_prefix(&rendered.site.source_path)?;
            xrefs.insert(rel_path.to_owned(), document.data.name.clone());
        }

        let full_documents = rendered
            .documents
            .iter()
            .map(|(_, v)| {
                let local_sitemap = sitemap.local(&v.data.name).ok_or(Error::DocumentNotFound)?;

                let content = layout::document(&v, &sitemap, &local_sitemap, &assets.handlebars)?;
                let variables = variable::search(&content)?;

                let rewrote_content = rewrite_str(
                    &content,
                    RewriteStrSettings {
                        element_content_handlers: vec![element!("a[href]", |el| {
                            if let Some(href) = el.get_attribute("href")
                                && !href.starts_with("/")
                                && !href.starts_with("http:")
                                && !href.starts_with("https:")
                                && !href.starts_with("mailto:")
                                && !href.starts_with("@@")
                                && !href.starts_with("#")
                            {
                                let (href_target, section_target) = if href.contains("::") {
                                    let mut s = href.splitn(2, "::");
                                    let href_target = s.next().ok_or(Error::MalformedLink)?;
                                    let mut section_target =
                                        s.next().ok_or(Error::MalformedLink)?;
                                    while let Some(s) = section_target.strip_prefix("*") {
                                        section_target = s;
                                    }
                                    let section_target =
                                        section_target.trim().to_lowercase().replace(" ", "-");
                                    (href_target.to_string(), Some(section_target.to_string()))
                                } else if href.contains("#") {
                                    let mut s = href.splitn(2, "::");
                                    let href_target = s.next().ok_or(Error::MalformedLink)?;
                                    let section_target = s.next().ok_or(Error::MalformedLink)?;
                                    (href_target.to_string(), Some(section_target.to_string()))
                                } else {
                                    (href, None)
                                };

                                let source_target = utils::normalize_path(
                                    &v.metadata.rel_source_path.join("..").join(href_target),
                                );

                                if let Some(document_name) = xrefs.get(&source_target) {
                                    let render_target = document_name.folder_path();
                                    let rel_render_target = pathdiff::diff_paths(
                                        &render_target,
                                        &v.data.name.folder_path(),
                                    )
                                    .ok_or(Error::PathDiffFailed)?;
                                    let target_with_section = match section_target {
                                        Some(section_target) => format!(
                                            "{}#{}",
                                            rel_render_target.to_string_lossy(),
                                            section_target
                                        ),
                                        None => format!("{}", rel_render_target.to_string_lossy()),
                                    };

                                    el.set_attribute("href", &target_with_section)?;
                                }
                            }

                            Ok(())
                        })],
                        ..RewriteStrSettings::new()
                    },
                )?;

                Ok(FullDocument {
                    site_metadata: v.site_metadata.clone(),
                    metadata: v.metadata.clone(),
                    rendered: v.data.clone(),
                    content: rewrote_content,
                    variables,
                    local_sitemap,
                })
            })
            .collect::<Result<Vec<FullDocument>, Error>>()?;

        Ok(Self {
            site: rendered.site.clone(),
            documents: full_documents,
            files: rendered.files.clone(),
            xrefs,
            sitemap,
            assets,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FullDocument {
    pub site_metadata: Arc<SiteMetadata>,
    pub metadata: Arc<DocumentMetadata>,
    pub rendered: Arc<RenderedData>,
    pub content: String,
    pub local_sitemap: LocalSitemap,
    pub variables: Vec<Variable>,
}
