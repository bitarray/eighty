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

use snafu::Snafu;
use std::backtrace::Backtrace;

#[derive(Debug, Snafu)]
pub enum Error {
    PathContainNonUnicode,
    InvalidPathComponent,
    RunCommandFailed,
    UnknownCommand,
    UnexpectedSiteName,
    #[snafu(context(false))]
    Io {
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[snafu(context(false))]
    Json {
        source: serde_json::Error,
    },
    #[snafu(context(false))]
    StripPrefix {
        source: std::path::StripPrefixError,
    },
    #[snafu(context(false))]
    WalkDir {
        source: walkdir::Error,
    },
    ReservedSiteName,

    #[snafu(context(false))]
    TokioJoin {
        source: tokio::task::JoinError,
    },
    SiteNotExist,
    DocumentNotFound,
    #[snafu(context(false))]
    HyperHttp {
        source: hyper::http::Error,
    },
    #[snafu(context(false))]
    HandlebarsTemplate {
        source: handlebars::TemplateError,
    },
    #[snafu(context(false))]
    HandlebarsRender {
        source: handlebars::RenderError,
    },

    Poisoned,
    #[snafu(context(false))]
    Notify {
        source: notify::Error,
    },
    #[snafu(context(false))]
    Regex {
        source: regex::Error,
    },
    UnprocessedRegexMatch,
    UnsupportedVariable,
    UnresolvedXreflink,

    #[snafu(context(false))]
    ChronoParse {
        source: chrono::ParseError,
    },
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Error {
        Error::Poisoned
    }
}
