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

mod command;

use clap::{Parser, Subcommand};
use eighty::Error;
use std::path::Path;
use snafu::ErrorCompat;

#[derive(Debug, Parser)]
#[command(name = "eighty")]
#[command(about = "Static website generator, mainly for the Core Paper project.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Build a project for production.
    Build {
        /// Site root.
        site: String,
        /// Build target.
        target: String,
    },
    /// Build multiple projects.
    BuildAll {
        /// Project root.
        root: String,
        /// Build target.
        target: String,
    },
    /// Serve a project in localhost for development.
    Serve {
        /// Site root.
        site: String,
    },
}

fn run() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    match args.command {
        Command::Build { site, target } => {
            let site_path = Path::new(&site);
            let target_path = Path::new(&target);
            command::build::build(&site_path, &target_path)?;
        }
        Command::BuildAll { root, target } => {
            let root_path = Path::new(&root);
            let target_path = Path::new(&target);
            command::build::build_all(&root_path, &target_path)?;
        }
        Command::Serve { site } => {
            let site_path = Path::new(&site);
            command::serve::serve(&site_path)?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {}", e);
        if let Some(bt) = ErrorCompat::backtrace(&e) {
            eprintln!("{}", bt);
        }
    }
}
