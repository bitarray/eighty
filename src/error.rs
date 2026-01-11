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

use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    PathContainNonUnicode,
    InvalidPathComponent,
    RunCommandFailed,
    UnknownCommand,
    UnexpectedSiteName,
    Io(#[from] std::io::Error),
    Json(#[from] serde_json::Error),
    StripPrefix(#[from] std::path::StripPrefixError),
    WalkDir(#[from] walkdir::Error),
    ReservedSiteName,

    TokioJoin(#[from] tokio::task::JoinError),
    SiteNotExist,
    DocumentNotFound,
    HyperHttp(#[from] hyper::http::Error),
    HandlebarsTemplate(#[from] handlebars::TemplateError),
    HandlebarsRender(#[from] handlebars::RenderError),

    Poisoned,
    Notify(#[from] notify::Error),
    Regex(#[from] regex::Error),
    UnprocessedRegexMatch,
    UnsupportedVariable,
    UnresolvedXreflink,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Error {
        Error::Poisoned
    }
}
