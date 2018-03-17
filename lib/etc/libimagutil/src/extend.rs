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

/// Macro for creating a trait and implementing it on a type in one go.
///
/// The purpose of this macro is that implementing extensions for `libimagstore::store::Store` and
/// `libimagstore::store::Entry` gets easy.
///
#[macro_export]
macro_rules! extend {
    {
        trait $traitname:ident extending $toextend:ty {
            $(
                fn $fnname:ident($this:ident, $($argname:ident : $argtype:ty),* ) -> $rettype:ty $block:block
            )+
        }
    } => {
        pub trait $traitname {
            $(
                fn $fnname($this, $($argname : $argtype ),* ) -> $rettype ;
            )*
        }

        impl $traitname for $toextend {
            $(
                fn $fnname($this, $($argname : $argtype ),* ) -> $rettype { $block }
            )*
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn compile_test_1() {
        extend! {
            trait Test extending ::std::vec::Vec<i32> {
                fn foo() -> Result<i32, i32> {
                    return Ok(1)
                }
            }
        }

        assert_eq!(Vec::<i32>::foo(), Ok(1));
    }

    #[test]
    fn compile_test_2() {
        extend! {
            trait Test extending ::std::collections::HashMap<i32, i32> {
                fn foo() -> Result<i32, i32> {
                    return Ok(2)
                }
            }
        }

        assert_eq!(::std::collections::HashMap::<i32, i32>::foo(), Ok(2));
    }

}


