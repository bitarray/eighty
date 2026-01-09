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
    document::{DocumentMetadata, DocumentName, RenderedData, Spec},
    file::FileMetadata,
    layout,
    site::{SiteMetadata, SiteName},
    sitemap::{BreadcrumbItem, LocalSitemap, Sitemap},
    variable,
    workspace::{RenderedSite, RenderedWorkspace, WorkspacePath},
};
use handlebars::Handlebars;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct FullWorkspace {
    pub path: WorkspacePath,
    pub assets: AssetStore,
    pub sites: HashMap<SiteName, FullSite>,
    pub spec_site: FullSpecSite,
}

impl FullWorkspace {
    pub fn new(rendered: &RenderedWorkspace) -> Result<Self, Error> {
        let assets = AssetStore::new(rendered.path.path())?;

        let sites = rendered
            .sites
            .iter()
            .map(|(name, site)| Ok((name.clone(), FullSite::new(&site, &assets.handlebars)?)))
            .collect::<Result<HashMap<SiteName, FullSite>, Error>>()?;

        let mut specs = HashMap::new();
        for (_, site) in &sites {
            for document in &site.documents {
                for spec in &document.rendered.specs {
                    let site_url = site.site.config.url.clone();
                    let redirect_url = format!(
                        "{}{}/",
                        site_url,
                        document.rendered.name.folder_path().display()
                    );
                    let redirect_content =
                        layout::spec_redirect(&spec, &redirect_url, &assets.handlebars)?;

                    specs.insert(
                        spec.id.clone(),
                        FullSpec {
                            redirect_content,
                            redirect_url,
                            data: spec.clone(),
                        },
                    );
                }
            }
        }

        let spec_site = FullSpecSite::new(specs, &assets.handlebars)?;

        Ok(Self {
            path: rendered.path.clone(),
            assets,
            sites,
            spec_site,
        })
    }
}

pub struct FullSite {
    pub site: Arc<SiteMetadata>,
    pub documents: Vec<FullDocument>,
    pub files: Arc<HashMap<PathBuf, FileMetadata>>,
    pub xrefs: HashMap<PathBuf, DocumentName>,
    pub sitemap: Sitemap,
}

impl FullSite {
    pub fn new(rendered: &RenderedSite, handlebars: &Handlebars) -> Result<Self, Error> {
        let name_titles = rendered
            .documents
            .iter()
            .map(|(_, v)| BreadcrumbItem {
                document_name: v.data.name.clone(),
                title: v.data.sitemap_title.clone().unwrap_or(v.data.title.clone()),
                description: v.data.description.clone(),
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

                let mut content = layout::document(&v, &sitemap, &local_sitemap, handlebars)?;
                let variables = variable::search(&content)?;

                for variable in variables {
                    if &variable.name == "XREFLINK" {
                        if let Some(xreflink) = variable.arguments {
                            let resolved = format!(
                                "{}{}/",
                                rendered.site.config.base_url,
                                xrefs
                                    .get(Path::new(&xreflink))
                                    .ok_or(Error::UnresolvedXreflink)?
                                    .folder_path()
                                    .display()
                            );

                            content = content.replace(&variable.full, &resolved);
                        } else {
                            return Err(Error::UnsupportedVariable);
                        }
                    } else {
                        return Err(Error::UnsupportedVariable);
                    }
                }

                Ok(FullDocument {
                    site_metadata: v.site_metadata.clone(),
                    metadata: v.metadata.clone(),
                    rendered: v.data.clone(),
                    content,
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
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FullSpecSite {
    pub specs: HashMap<String, FullSpec>,
    pub index_content: String,
}

impl FullSpecSite {
    pub fn new(specs: HashMap<String, FullSpec>, handlebars: &Handlebars) -> Result<Self, Error> {
        let sorted_specs = {
            let mut specs = specs
                .values()
                .map(|v| (v.data.clone(), v.redirect_url.clone()))
                .collect::<Vec<_>>();
            specs.sort_by_key(|v| v.0.id.clone());
            specs
        };

        let index_content = layout::spec_index(&sorted_specs, handlebars)?;

        Ok(FullSpecSite {
            specs,
            index_content,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FullSpec {
    pub data: Spec,
    pub redirect_content: String,
    pub redirect_url: String,
}

impl FullSpec {
    pub fn folder_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(self.data.id.clone());
        path
    }

    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(self.data.id.clone());
        path.push("index.html");
        path
    }
}
