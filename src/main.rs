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

use clap::{App, Arg, SubCommand};
use eighty::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    let matches = App::new("Eighty")
        .subcommand(
            SubCommand::with_name("build")
                .arg(Arg::with_name("root").index(1).required(true))
                .arg(Arg::with_name("target").index(2).required(true)),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .arg(Arg::with_name("root").index(1).required(true))
                .arg(Arg::with_name("site").index(2).required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));
        let target_path = Path::new(matches.value_of("target").expect("target is required"));

        command::build::build(&root_path, &target_path)?;
    } else if let Some(matches) = matches.subcommand_matches("serve") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));
        let site_name = matches.value_of("site").expect("site is required");

        command::serve::serve(&root_path, &site_name)?;
    } else {
        return Err(Error::UnknownCommand);
    }

    Ok(())
}
