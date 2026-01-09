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

use crate::{Error, document::Spec};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecIndexContext {
    pub specs: Vec<SpecItem>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecRedirectContext {
    pub spec: SpecItem,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecItem {
    pub id: String,
    pub url: String,
    pub description: String,
    pub discuss: String,
}

pub fn index_layout(specs: &[(Spec, String)], handlebars: &Handlebars) -> Result<String, Error> {
    let context = SpecIndexContext {
        specs: specs
            .iter()
            .map(|(spec, redirect_url)| SpecItem {
                id: spec.id.clone(),
                url: redirect_url.clone(),
                description: spec.description.clone(),
                discuss: spec.discuss.clone(),
            })
            .collect(),
    };

    let layouted = handlebars.render("spec/index", &context)?;

    Ok(layouted)
}

pub fn redirect_layout(spec: &Spec, url: &str, handlebars: &Handlebars) -> Result<String, Error> {
    let context = SpecRedirectContext {
        spec: SpecItem {
            id: spec.id.clone(),
            url: url.to_owned(),
            description: spec.description.clone(),
            discuss: spec.discuss.clone(),
        },
    };

    let layouted = handlebars.render("spec/redirect", &context)?;

    Ok(layouted)
}
