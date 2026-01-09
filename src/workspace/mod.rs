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

mod full;
mod metadatad;
mod post;
mod rendered;

pub use self::{
    full::{FullDocument, FullSite, FullWorkspace},
    metadatad::{WorkspacePath, MetadatadSite, MetadatadWorkspace},
    post::{SimplePostSite, SimplePostWorkspace},
    rendered::{RenderedSite, RenderedWorkspace},
};
