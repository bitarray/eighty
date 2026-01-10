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

use crate::document::DocumentName;
use std::{fmt, ops::Deref};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct LocalSitemap {
    pub breadcrumb: Breadcrumb,
    pub children: Vec<BreadcrumbItem>,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Breadcrumb(pub Vec<BreadcrumbItem>);

impl Deref for Breadcrumb {
    type Target = Vec<BreadcrumbItem>;

    fn deref(&self) -> &Vec<BreadcrumbItem> {
        &self.0
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct BreadcrumbItem {
    pub title: String,
    pub document_name: DocumentName,
    pub description: String,
    pub order: Option<usize>,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SitemapItem {
    pub item: BreadcrumbItem,
    pub children: Vec<SitemapItem>,
}

impl SitemapItem {
    pub fn maybe_insert(&mut self, item: BreadcrumbItem) -> bool {
        if self.item.document_name.is_ancestor_of(&item.document_name) {
            let mut inserted = false;

            for child in &mut self.children {
                if child.maybe_insert(item.clone()) {
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                self.children.push(SitemapItem {
                    item,
                    children: Vec::new(),
                });
            }

            true
        } else {
            false
        }
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Sitemap(pub Vec<SitemapItem>);

impl Sitemap {
    pub fn insert(&mut self, item: BreadcrumbItem) {
        let mut inserted = false;

        for child in &mut self.0 {
            if child.maybe_insert(item.clone()) {
                inserted = true;
                break;
            }
        }

        if !inserted {
            self.0.push(SitemapItem {
                item,
                children: Vec::new(),
            });
        }
    }

    pub fn local(&self, document_name: &DocumentName) -> Option<LocalSitemap> {
        if document_name.is_root() {
            return Some(LocalSitemap {
                breadcrumb: Breadcrumb(Vec::new()),
                children: self.0.iter().map(|item| item.item.clone()).collect(),
            });
        }

        let mut breadcrumb = Vec::new();
        let mut current = &self.0;

        loop {
            let target = current.iter().find(|item| {
                item.item.document_name.is_ancestor_of(&document_name)
                    || item.item.document_name == *document_name
            });

            if let Some(target) = target {
                if target.item.document_name == *document_name {
                    return Some(LocalSitemap {
                        breadcrumb: Breadcrumb(breadcrumb),
                        children: target
                            .children
                            .iter()
                            .map(|item| item.item.clone())
                            .collect(),
                    });
                } else {
                    breadcrumb.push(target.item.clone());

                    current = &target.children;
                }
            } else {
                return None;
            }
        }
    }

    pub fn sort(&mut self) {
        sort_sitemap_item_vec(&mut self.0)
    }
}

impl From<Vec<BreadcrumbItem>> for Sitemap {
    fn from(mut name_titles: Vec<BreadcrumbItem>) -> Sitemap {
        name_titles.sort_by_key(|k| (k.document_name.labels.clone(), k.document_name.post.clone()));
        let ordered_name_titles = name_titles;

        let mut sitemap = Sitemap(Vec::new());
        for item in &ordered_name_titles {
            if !item.document_name.is_root() {
                sitemap.insert(item.clone());
            }
        }

        sitemap
    }
}

impl Deref for Sitemap {
    type Target = Vec<SitemapItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Sitemap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in &self.0 {
            fmt_sitemap_item(f, child, "")?;
        }

        Ok(())
    }
}

fn fmt_sitemap_item(f: &mut fmt::Formatter<'_>, item: &SitemapItem, prefix: &str) -> fmt::Result {
    write!(
        f,
        "{}- {}: {}\n",
        prefix, item.item.document_name, item.item.title
    )?;
    for child in &item.children {
        fmt_sitemap_item(f, child, &(prefix.to_owned() + "\t"))?;
    }

    Ok(())
}

fn sort_sitemap_item_vec(items: &mut Vec<SitemapItem>) {
    items.sort_by_key(|k| (k.item.order.unwrap_or(usize::MAX), k.item.document_name.labels.clone(), k.item.document_name.post.clone()));
    for item in items {
        sort_sitemap_item_vec(&mut item.children);
    }
}
