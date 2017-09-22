//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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

#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate version;

extern crate libimagentrygps;
extern crate libimagrt;
extern crate libimagutil;
extern crate libimagerror;
extern crate libimagstore;

use std::process::exit;
use std::path::PathBuf;
use std::str::FromStr;

use libimagentrygps::types::*;
use libimagentrygps::entry::*;
use libimagrt::setup::generate_runtime_setup;
use libimagrt::runtime::Runtime;
use libimagutil::warn_exit::warn_exit;
use libimagerror::trace::MapErrTrace;
use libimagstore::storeid::IntoStoreId;

mod ui;

fn main() {
    let rt = generate_runtime_setup("imag-gps",
                                    &version!()[..],
                                    "Add GPS coordinates to entries",
                                    ui::build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            match name {
                "add"    => add(&rt),
                "remove" => remove(&rt),
                "get"    => get(&rt),
                _        => warn_exit("No commandline call", 1)
            }
        });
}

fn add(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("add").unwrap(); // safed by main()

    let entry_name  = scmd.value_of("entry").unwrap(); // safed by clap
    let sid         = PathBuf::from(entry_name)
        .into_storeid()
        .map_err_trace_exit(1)
        .unwrap(); // safed by above call

    let c = {
        let parse = |value: &str| -> Vec<i8> {
            value.split(".")
                .map(FromStr::from_str)
                .map(|elem| elem.map_err_trace_exit(1).unwrap())
                .collect::<Vec<i8>>()
        };

        let long = parse(scmd.value_of("longitude").unwrap()); // unwrap safed by clap
        let lati = parse(scmd.value_of("latitude").unwrap()); // unwrap safed by clap

        let long = GPSValue::new(long[0], long[1], long[2]);
        let lati = GPSValue::new(lati[0], lati[1], lati[2]);

        Coordinates::new(long, lati)
    };

    rt.store()
        .get(sid)
        .map_err_trace_exit(1)
        .unwrap() // safed by above call
        .map(|mut entry| {
            let _ = entry.set_coordinates(c)
                .map_err_trace_exit(1);
        })
        .unwrap_or_else(|| {
            error!("No such entry: {}", entry_name);
            exit(1)
        });
}

fn remove(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("remove").unwrap(); // safed by main()

    let entry_name  = scmd.value_of("entry").unwrap(); // safed by clap
    let sid         = PathBuf::from(entry_name)
        .into_storeid()
        .map_err_trace_exit(1)
        .unwrap(); // safed by above call

    let removed_value = rt
        .store()
        .get(sid)
        .map_err_trace_exit(1)
        .unwrap() // safed by above call
        .unwrap_or_else(|| { // if we have Ok(None)
            error!("No such entry: {}", entry_name);
            exit(1)
        })
        .remove_coordinates()
        .map_err_trace_exit(1) // The delete action failed
        .unwrap() // safed by above call
        .unwrap_or_else(|| { // if we have Ok(None)
            error!("Entry had no coordinates: {}", entry_name);
            exit(1)
        })
        .map_err_trace_exit(1) // The parsing of the deleted values failed
        .unwrap(); // safed by above call

    if scmd.is_present("print-removed") {
        println!("{}", removed_value);
    }

    info!("Ok");
}

fn get(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("get").unwrap(); // safed by main()

    let entry_name  = scmd.value_of("entry").unwrap(); // safed by clap
    let sid         = PathBuf::from(entry_name)
        .into_storeid()
        .map_err_trace_exit(1)
        .unwrap(); // safed by above call

    let value = rt
        .store()
        .get(sid)
        .map_err_trace_exit(1)
        .unwrap() // safed by above call
        .unwrap_or_else(|| { // if we have Ok(None)
            error!("No such entry: {}", entry_name);
            exit(1)
        })
        .get_coordinates()
        .map_err_trace_exit(1) // The get action failed
        .unwrap() // safed by above call
        .unwrap_or_else(|| { // if we have Ok(None)
            error!("Entry has no coordinates: {}", entry_name);
            exit(1)
        });

    println!("{}", value);
    info!("Ok");
}

