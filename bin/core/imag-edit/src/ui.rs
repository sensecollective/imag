//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use clap::{Arg, ArgGroup, App};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .arg(Arg::with_name("entry")
             .index(1)
             .takes_value(true)
             .required(false)
             .multiple(true)
             .help("The entry/entries to edit")
             .value_name("ENTRY"))
        .arg(Arg::with_name("entries-from-stdin")
             .long("ids-from-stdin")
             .short("I")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("The entry/entries are piped in via stdin"))
        .group(ArgGroup::with_name("input-method")
               .args(&["entry", "entries-from-stdin"])
               .required(true))

        .arg(Arg::with_name("edit-header")
             .long("header")
             .short("H")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Also edit the header"))

        .arg(Arg::with_name("edit-header-only")
             .long("header-only")
             .takes_value(false)
             .required(false)
             .multiple(false)
             .help("Only edit the header"))
}

