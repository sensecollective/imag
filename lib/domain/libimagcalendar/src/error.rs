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

use std::path::PathBuf;

error_chain! {
    types {
        CalendarError, CalendarErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
        LinkError(::libimagentrylink::error::LinkError, ::libimagentrylink::error::LinkErrorKind);
        RefError(::libimagentryref::error::RefError, ::libimagentryref::error::RefErrorKind);
        IcalendarError(::vobject::error::VObjectError, ::vobject::error::VObjectErrorKind);
        EntryUtilError(::libimagentryutil::error::EntryUtilError, ::libimagentryutil::error::EntryUtilErrorKind);
        TomlQueryError(::toml_query::error::Error, ::toml_query::error::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Utf8Error(::std::string::FromUtf8Error);
    }

    errors {
        NotAnEvent(filepath: PathBuf) {
            description("Object is not an event")
                display("Object in {:?} is not an event", filepath)
        }

        EventWithoutUid(filepath: PathBuf) {
            description("Event has no UID field")
                display("Event in {:?} has no UID field", filepath)
        }
    }
}

