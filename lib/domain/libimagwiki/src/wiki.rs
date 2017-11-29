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

pub struct Wiki<'a, N: AsRef<str>>(&'a Store, name: N);

impl<'a, N: AsRef<str>> Wiki<'a, N> {

    pub(crate) new(store: &'a Store, name: N) -> Wiki {
        Wiki(store, name)
    }

}

pub(crate) create_wiki_entry<'a>(store: &'a Store, wiki_name: &str, entry_name: &str)
    -> Result<FileLockEntry<'a>>
{
    let sid = StoreId::new_baseless(PathBuf::from(format!("{}/{}", wiki_name, entry_name)))?;
    let entry = store.create(sid)?;

    unimplemented!();

    Ok(entry)
}


pub(crate) retrieve_wiki_entry<'a>(store: &'a Store, wiki_name: &str, entry_name: &str)
    -> Result<FileLockEntry<'a>>
{
    let sid = StoreId::new_baseless(PathBuf::from(format!("{}/{}", wiki_name, entry_name)))?;
    let entry = store.retrieve(sid)?;

    unimplemented!();

    Ok(entry)
}
