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

use chrono::NaiveDateTime;

use libimagentryref::reference::Ref;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;
use libimagstore::store::Entry;

use error::*;

provide_kindflag_path!(pub IsEvent, "calendar.is_event");

/// A Event is a Entry in the store which refers to a calendar file that contains said Event.
pub trait Event : Ref {
    fn is_event(&self) -> Result<bool>;

    // Accessing the actual icalendar file

    fn get_start(&self)       -> Result<NaiveDateTime>;
    fn get_end(&self)         -> Result<NaiveDateTime>;
    fn get_location(&self)    -> Result<String>;
    fn get_categories(&self)  -> Result<Vec<String>>;
    fn get_description(&self) -> Result<String>;
}

impl Event for Entry {
    fn is_event(&self) -> Result<bool> {
        self.is::<IsEvent>().map_err(From::from)
    }

    // Accessing the actual icalendar file

    fn get_start(&self) -> Result<NaiveDateTime> {
        unimplemented!()
    }

    fn get_end(&self) -> Result<NaiveDateTime> {
        unimplemented!()
    }

    fn get_location(&self) -> Result<String> {
        unimplemented!()
    }

    fn get_categories(&self) -> Result<Vec<String>> {
        unimplemented!()
    }

    fn get_description(&self) -> Result<String> {
        unimplemented!()
    }

}