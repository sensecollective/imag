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

use libimagstore::store::Store;

use error::WikiError as WE;
use error::WikiErrorKind as WEK;

pub trait WikiStore {

    fn get_wiki<'a, N: AsRef<str>>(&'a self, name: N) -> Result<Option<Wiki<'a>>>;

    fn create_wiki<'a, N: AsRef<str>>(&'a self, name: N, mainpagename: Option<String>)
        -> Result<FileLockEntry<'a>>;

    fn delete_wiki<N: AsRef<str>>(&self, name: N) -> Result<()>;

}

impl WikiStore for Store {

    /// get a wiki by its name
    fn get_wiki<'a, N: AsRef<str>>(&'a self, name: N) -> Result<Option<Wiki<'a>>> {
        if !wiki_path(self, name)?.exists()? {
            Ok(None)
        } else {
            Ok(Some(Wiki::new(self, name)))
        }
    }

    /// Create a wiki.
    ///
    /// # Returns
    ///
    /// Returns the Wiki object.
    ///
    /// Ob success, an empty Wiki entry with the name `mainpagename` (or "main" if none is passed)
    /// is created inside the wiki.
    ///
    fn create_wiki<'a, N: AsRef<str>>(&'a self, name: N, mainpagename: Option<&str>)
        -> Result<Wiki<'a, N>>
    {
        let sid = wiki_path(self, name)?;
        if sid.exists()? {
            return Err(WEK::WikiExists(String::from(name.as_ref())).into())
        }

        create_wiki_entry(self, name.as_ref(), mainpagename.unwrap_or("main"))
            .and_then(|_| self.get_wiki(name))
    }

    /// Delete a wiki and all entries inside
    fn delete_wiki<N: AsRef<str>>(&self, name: N) -> Result<()> {
        unimplemented!()
    }

}

fn wiki_path<N: AsRef<str>>(store: &Store, name: N) -> Result<StoreId> {
    PathBuf::from(store.path())
        .join(name.as_ref())
        .into_storeid()
        .map_err(WE::from)
}

