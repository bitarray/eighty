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

mod asciidoc;
mod markdown;
mod org;

use crate::{Error, site::SiteMetadata};
use chrono::NaiveDate;
use std::{
    collections::BTreeMap,
    fmt,
    path::{Component, Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use tracing::info;

#[derive(Hash, Eq, Clone, PartialEq, Debug)]
pub struct DocumentName {
    pub id: Option<String>,
    pub labels: Vec<String>,
    pub post: Option<DocumentPostLabel>,
}

impl fmt::Display for DocumentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.labels.is_empty() && self.post.is_none() {
            write!(f, "[index]")?;
            return Ok(());
        }

        write!(f, "{}", self.labels.join("/"))?;
        if let Some(post) = self.post.as_ref() {
            write!(f, "[post:{},{}]", post.date, post.label)?;
        }
        Ok(())
    }
}

impl DocumentName {
    pub fn path(&self) -> PathBuf {
        let mut path = self.folder_path();

        path.push("index.html");

        path
    }

    pub fn folder_path(&self) -> PathBuf {
        let mut path = PathBuf::new();

        if let Some(id) = &self.id {
            path.push(id);
        } else {
            for label in &self.labels {
                path.push(label);
            }

            if let Some(post) = self.post.as_ref() {
                path.push(post.date.split('-').collect::<Vec<_>>().join("/"));
                path.push(&post.label);
            }
        }

        path
    }

    pub fn is_matched(&self, other: &Path) -> bool {
        other == self.path() || other == self.folder_path()
    }

    pub fn is_ancestor_of(&self, child: &Self) -> bool {
        if self.post.is_some() {
            return false;
        }

        if self.labels.len() > child.labels.len() {
            return false;
        }

        if self.labels[0..self.labels.len()] != child.labels[0..self.labels.len()] {
            return false;
        }

        if self.labels.len() == child.labels.len() {
            return self.post.is_none() && child.post.is_some();
        }

        return true;
    }

    pub fn is_root(&self) -> bool {
        self.post.is_none() && self.labels.is_empty()
    }
}

#[derive(Hash, Eq, Clone, PartialEq, Debug, PartialOrd, Ord)]
pub struct DocumentPostLabel {
    pub date: String,
    pub label: String,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub enum DocumentType {
    AsciiDoc,
    Markdown,
    Org,
}

impl<'a> TryFrom<&'a str> for DocumentType {
    type Error = ();

    fn try_from(s: &'a str) -> Result<DocumentType, ()> {
        match s {
            "adoc" => Ok(DocumentType::AsciiDoc),
            "md" => Ok(DocumentType::Markdown),
            "org" => Ok(DocumentType::Org),
            _ => Err(()),
        }
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct DocumentMetadata {
    pub typ: DocumentType,
    pub modified: SystemTime,
    pub source_path: PathBuf,
    pub rel_source_path: PathBuf,
}

impl DocumentMetadata {
    pub fn new(
        site: &SiteMetadata,
        file_path: &Path,
        typ: DocumentType,
        modified: SystemTime,
    ) -> Result<DocumentMetadata, Error> {
        Ok(DocumentMetadata {
            source_path: file_path.to_owned(),
            rel_source_path: file_path.strip_prefix(&site.source_path)?.to_owned(),
            modified,
            typ,
        })
    }
}

fn derive_name(rel_file_path: &Path, id: Option<String>) -> Result<DocumentName, Error> {
    let mut labels = Vec::new();
    let mut is_post = false;
    let mut components = rel_file_path.components().peekable();

    while let Some(component) = components.next() {
        if let Component::Normal(component_name) = component {
            let component_name = component_name
                .to_str()
                .ok_or(Error::PathContainNonUnicode)?;

            if component_name == "_posts" {
                is_post = true;
                break;
            }

            if components.peek().is_none() {
                break;
            } else {
                labels.push(component_name.to_owned());
            }
        } else {
            return Err(Error::InvalidPathComponent);
        }
    }

    let file_stem = rel_file_path
        .file_stem()
        .ok_or(Error::InvalidPathComponent)?
        .to_str()
        .ok_or(Error::PathContainNonUnicode)?;
    let post = if is_post {
        let file_parts = file_stem.split('-').collect::<Vec<_>>();
        let date_part = file_parts[0..3].join("-");
        let label_part = file_parts[3..].join("-");
        Some(DocumentPostLabel {
            date: date_part,
            label: label_part,
        })
    } else {
        if file_stem != "index" {
            labels.push(file_stem.to_owned());
        }
        None
    };

    Ok(DocumentName { id, labels, post })
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub enum RevisionData {
    Created,
    Custom(String),
}

impl RevisionData {
    pub fn new(s: String) -> Self {
        if &s == "created" {
            Self::Created
        } else {
            Self::Custom(s)
        }
    }
}

impl fmt::Display for RevisionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => write!(f, "Document created."),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedData {
    pub name: DocumentName,
    pub title: String,
    pub sitemap_title: Option<String>,
    pub content: String,
    pub toc: Option<String>,
    pub description: String,
    pub description_content: Option<String>,
    pub license: Option<String>,
    pub license_code: Option<String>,
    pub specs: Vec<Spec>,
    pub order: Option<usize>,
    pub revisions: BTreeMap<NaiveDate, RevisionData>,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedDocument {
    pub site_metadata: Arc<SiteMetadata>,
    pub metadata: Arc<DocumentMetadata>,
    pub data: Arc<RenderedData>,
}

impl RenderedDocument {
    pub fn new(
        site: Arc<SiteMetadata>,
        document: Arc<DocumentMetadata>,
    ) -> Result<RenderedDocument, Error> {
        let rel_file_path = &document.rel_source_path;

        info!("[{}] Rendering document {:?} ...", site.name, rel_file_path);

        Ok(match document.typ {
            DocumentType::AsciiDoc => {
                let output = self::asciidoc::process_asciidoc(&site.source_path, &rel_file_path)?;
                let id = output.document.id.clone();
                let name = derive_name(rel_file_path, id)?;

                RenderedDocument {
                    site_metadata: site,
                    metadata: document,
                    data: Arc::new(RenderedData {
                        name,
                        title: output.document.title,
                        sitemap_title: output.document.sitemap_title,
                        content: output.document.content,
                        toc: output.document.toc,
                        description: output.document.description.clone(),
                        description_content: None,
                        license: output.document.license,
                        license_code: output.document.license_code,
                        order: output.document.order,
                        specs: output
                            .specs
                            .into_iter()
                            .map(|spec| Spec {
                                id: spec.id,
                                description: spec.description,
                                discuss: spec.discuss,
                                source_path: Path::new(&spec.source_path).to_owned(),
                                anchor: spec.anchor,
                            })
                            .collect(),
                        revisions: BTreeMap::new(),
                    }),
                }
            }
            DocumentType::Markdown => {
                let output = self::markdown::process_markdown(&site.source_path, &rel_file_path)?;
                let id = output.id;
                let name = derive_name(rel_file_path, id)?;

                RenderedDocument {
                    site_metadata: site,
                    metadata: document,
                    data: Arc::new(RenderedData {
                        name,
                        title: output.title,
                        sitemap_title: output.sitemap_title,
                        content: output.content,
                        toc: Some(output.toc),
                        description: output.description,
                        description_content: Some(output.description_content),
                        license: None,
                        license_code: None,
                        specs: Vec::new(),
                        order: output.order,
                        revisions: BTreeMap::new(),
                    }),
                }
            }
            DocumentType::Org => {
                let output = self::org::process_org(&site.source_path, &rel_file_path)?;
                let id = output.id.clone();
                let name = derive_name(rel_file_path, id)?;
                let revisions = output
                    .revisions
                    .into_iter()
                    .map(|(k, v)| {
                        Ok((
                            NaiveDate::parse_from_str(&k, "%Y%m%d")?,
                            RevisionData::new(v),
                        ))
                    })
                    .collect::<Result<_, Error>>()?;

                RenderedDocument {
                    site_metadata: site,
                    metadata: document,
                    data: Arc::new(RenderedData {
                        name,
                        title: output.title,
                        sitemap_title: output.sitemap_title,
                        content: output.content,
                        toc: Some(output.toc),
                        description: output.description,
                        description_content: Some(output.description_content),
                        license: None,
                        license_code: None,
                        specs: Vec::new(),
                        order: output.order,
                        revisions,
                    }),
                }
            }
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Spec {
    pub id: String,
    pub description: String,
    pub discuss: String,
    pub source_path: PathBuf,
    pub anchor: String,
}
