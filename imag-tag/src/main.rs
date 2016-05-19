extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagerror;

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagentrytag::tagable::Tagable;
use libimagentrytag::tag::Tag;
use libimagstore::storeid::build_entry_path;
use libimagerror::trace::trace_error;
use libimagentrytag::ui::{get_add_tags, get_remove_tags};

mod ui;

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-store",
                                    &version!()[..],
                                    "Direct interface to the store. Use with great care!",
                                    build_ui);

    let id = rt.cli().value_of("id").unwrap(); // enforced by clap
    rt.cli()
        .subcommand_name()
        .map_or_else(
            || {
                let add = get_add_tags(rt.cli());
                let rem = get_remove_tags(rt.cli());
                alter(&rt, id, add, rem);
            },
            |name| {
                debug!("Call: {}", name);
                match name {
                    "list" => list(id, &rt),
                    _ => {
                        warn!("Unknown command");
                        // More error handling
                    },
                };
            });
}

fn alter(rt: &Runtime, id: &str, add: Option<Vec<Tag>>, rem: Option<Vec<Tag>>) {
    let path = {
        match build_entry_path(rt.store(), id) {
            Err(e) => {
                trace_error(&e);
                exit(1);
            },
            Ok(s) => s,
        }
    };
    debug!("path = {:?}", path);

    rt.store()
        // "id" must be present, enforced via clap spec
        .retrieve(path)
        .map(|mut e| {
            add.map(|tags| {
                for tag in tags {
                    debug!("Adding tag '{:?}'", tag);
                    if let Err(e) = e.add_tag(tag) {
                        trace_error(&e);
                    }
                }
            }); // it is okay to ignore a None here

            rem.map(|tags| {
                for tag in tags {
                    debug!("Removing tag '{:?}'", tag);
                    if let Err(e) = e.remove_tag(tag) {
                        trace_error(&e);
                    }
                }
            }); // it is okay to ignore a None here
        })
        .map_err(|e| {
            info!("No entry.");
            trace_error(&e);
        })
        .ok();
}

fn list(id: &str, rt: &Runtime) {
    let path = {
        match build_entry_path(rt.store(), id) {
            Err(e) => {
                trace_error(&e);
                exit(1);
            },
            Ok(s) => s,
        }
    };
    debug!("path = {:?}", path);

    let entry = rt.store().retrieve(path.clone());
    if entry.is_err() {
        debug!("Could not retrieve '{:?}' => {:?}", id, path);
        warn!("Could not retrieve entry '{}'", id);
        trace_error(&entry.unwrap_err());
        exit(1);
    }
    let entry = entry.unwrap();

    let scmd = rt.cli().subcommand_matches("list").unwrap(); // safe, we checked in main()

    let json_out = scmd.is_present("json");
    let line_out = scmd.is_present("linewise");
    let sepp_out = scmd.is_present("sep");
    let mut comm_out = scmd.is_present("commasep");

    if !vec![json_out, line_out, comm_out, sepp_out].iter().any(|v| *v) {
        // None of the flags passed, go to default
        comm_out = true;
    }

    let tags = entry.get_tags();
    if tags.is_err() {
        trace_error(&tags.unwrap_err());
        exit(1);
    }
    let tags = tags.unwrap();

    if json_out {
        unimplemented!()
    }

    if line_out {
        for tag in &tags {
            println!("{}", tag);
        }
    }

    if sepp_out {
        let sepp = scmd.value_of("sep").unwrap(); // we checked before
        println!("{}", tags.join(sepp));
    }

    if comm_out {
        println!("{}", tags.join(", "));
    }
}

