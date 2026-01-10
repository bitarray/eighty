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
    sitemap::{LocalSitemap, Sitemap, SitemapItem},
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentContext {
    pub site_title: String,
    pub site_title_only: bool,
    pub site_base_url: String,

    pub page_title: String,
    pub page_description: Option<String>,
    pub page_description_content: Option<String>,
    pub page_url: String,

    pub has_site_links: bool,
    pub site_links: Vec<DocumentContextSiteLink>,

    pub toc: Option<String>,
    pub page_content: String,

    pub page_author_url: String,
    pub page_author: String,
    pub page_copyright_years: String,

    pub page_license: Option<String>,
    pub page_license_code: Option<String>,

    pub sitemap: Option<Vec<DocumentContextSitemapItem>>,
    pub local_sitemap: DocumentContextLocalSitemap,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentContextSiteLink {
    pub url: String,
    pub name: String,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentContextSitemapItem {
    pub title: String,
    pub url: String,
    pub children: Vec<DocumentContextSitemapItem>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentContextLocalSitemap {
    pub breadcrumb: Option<Vec<DocumentContextBreadcrumbItem>>,
    pub children: Option<Vec<DocumentContextBreadcrumbItem>>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentContextBreadcrumbItem {
    pub title: String,
    pub url: String,
    pub description: String,
}

impl DocumentContextSitemapItem {
    pub fn from_sitemap_item(
        item: SitemapItem,
        max_depth: Option<usize>,
        site_base_url: &str,
    ) -> Self {
        let show_children = max_depth.map(|d| d > 0).unwrap_or(true);

        Self {
            title: item.item.title,
            url: format!(
                "{}{}/",
                site_base_url,
                item.item.document_name.folder_path().display()
            ),
            children: if show_children {
                item.children
                    .iter()
                    .map(|child| {
                        Self::from_sitemap_item(
                            child.clone(),
                            max_depth.map(|d| d.saturating_sub(1)),
                            site_base_url,
                        )
                    })
                    .collect()
            } else {
                Vec::new()
            },
        }
    }
}

pub fn layout(
    rendered: &RenderedDocument,
    sitemap: &Sitemap,
    local_sitemap: &LocalSitemap,
    handlebars: &Handlebars,
) -> Result<String, Error> {
    let site_config = &rendered.site_metadata.config;

    let context = DocumentContext {
        site_title: site_config.title.clone(),
        site_title_only: site_config.title == rendered.data.title,
        site_base_url: site_config.base_url.clone(),

        page_title: rendered.data.title.clone(),
        page_description: Some(rendered.data.description.clone()),
        page_description_content: rendered.data.description_content.clone(),
        page_url: format!(
            "{}{}",
            rendered.site_metadata.config.url,
            rendered.data.name.folder_path().display()
        ),

        has_site_links: false,
        site_links: site_config
            .links
            .iter()
            .map(|link| DocumentContextSiteLink {
                url: link.url.clone(),
                name: link.name.clone(),
            })
            .collect(),

        toc: rendered.data.toc.clone(),
        page_content: rendered.data.content.clone(),

        page_author_url: "https://social.pacna.org/@wei".to_string(),
        page_author: "Wei Tang".to_string(),
        page_copyright_years: "2019-2026".to_string(),

        page_license: rendered.data.license.clone(),
        page_license_code: rendered.data.license_code.clone(),

        sitemap: if site_config.sitemap.enable {
            Some(
                sitemap
                    .iter()
                    .map(|child| {
                        DocumentContextSitemapItem::from_sitemap_item(
                            child.clone(),
                            site_config.sitemap.depth.map(|d| d.saturating_sub(1)),
                            &site_config.base_url,
                        )
                    })
                    .chain(site_config.sitemap.extra_links.iter().map(|link| {
                        DocumentContextSitemapItem {
                            title: link.title.clone(),
                            url: link.url.clone(),
                            children: Vec::new(),
                        }
                    }))
                    .collect(),
            )
        } else {
            None
        },
        local_sitemap: DocumentContextLocalSitemap {
            breadcrumb: if local_sitemap.breadcrumb.len() > 0 {
                Some(
                    local_sitemap
                        .breadcrumb
                        .iter()
                        .map(|item| DocumentContextBreadcrumbItem {
                            title: item.title.clone(),
                            url: format!(
                                "{}{}/",
                                site_config.base_url,
                                item.document_name.folder_path().display()
                            ),
                            description: item.description.clone(),
                        })
                        .collect(),
                )
            } else {
                None
            },
            children: if local_sitemap.children.len() > 0 {
                Some(
                    local_sitemap
                        .children
                        .iter()
                        .map(|item| DocumentContextBreadcrumbItem {
                            title: item.title.clone(),
                            url: format!(
                                "{}{}/",
                                site_config.base_url,
                                item.document_name.folder_path().display()
                            ),
                            description: item.description.clone(),
                        })
                        .collect(),
                )
            } else {
                None
            },
        },
    };

    let layouted = handlebars.render("document/main", &context)?;

    Ok(layouted)
}
