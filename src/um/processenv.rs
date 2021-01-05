// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::{raw::um::processenv::*, string::*, *};
use winwrap_derive::*;

/// Gets an environment variable string.
///
/// # Example
///
/// ```rust
/// use winwrap::um::processenv::get_environment_variable_a;
/// use winwrap::string::AString;
///
/// let name = AString::from_str_lossy("path");
/// let s = get_environment_variable_a(name.as_c_str()).unwrap();
/// println!("{:?}",s);
/// ```
#[ansi_fn]
pub fn get_environment_variable_a(name: &AStr) -> OsResult<AString> {
    unsafe {
        let mut ret: Vec<u8> = Vec::with_capacity(128);
        let nb = GetEnvironmentVariableA(
            name.as_ptr(),
            ret.as_mut_ptr() as *mut _,
            128,
        )?;
        let nb = if ret.capacity() < nb as usize {
            ret = Vec::with_capacity(nb as usize);
            let nb = GetEnvironmentVariableA(
                name.as_ptr(),
                ret.as_mut_ptr() as *mut _,
                nb,
            )?;
            assert_eq!(nb as usize + 1, ret.capacity());
            nb
        } else {
            nb
        };
        ret.set_len(nb as usize);
        Ok(AString::new_unchecked(ret))
    }
}
