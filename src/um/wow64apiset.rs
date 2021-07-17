// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::{handle::*, raw::um::wow64apiset::*, string::*, *};
use winapi::ctypes::wchar_t;
use winwrap_derive::*;

pub fn is_wow64_process(proc_handle: &ProcessHandle) -> OsResult<bool> {
    unsafe {
        let mut b = 0;
        IsWow64Process(proc_handle.as_c_handle(), &mut b)?;
        Ok(b != 0)
    }
}

#[ansi_fn]
pub fn get_system_wow64_directory_a() -> OsResult<AString> {
    unsafe {
        let mut ret: Vec<u8> = Vec::with_capacity(128);
        let nb = GetSystemWow64DirectoryA(ret.as_mut_ptr() as *mut _, 128)?;
        let nb = if ret.capacity() < nb as usize {
            ret = Vec::with_capacity(nb as usize);
            let nb = GetSystemWow64DirectoryA(ret.as_mut_ptr() as *mut _, nb)?;
            assert_eq!(nb as usize + 1, ret.capacity());
            nb
        } else {
            nb
        };
        ret.set_len(nb as usize);
        Ok(AString::new_unchecked(ret))
    }
}

#[unicode_fn]
pub fn get_system_wow64_directory_w() -> OsResult<WString> {
    unsafe {
        let mut ret: Vec<wchar_t> = Vec::with_capacity(128);
        let nb = GetSystemWow64DirectoryW(ret.as_mut_ptr() as *mut _, 128)?;
        let nb = if ret.capacity() < nb as usize {
            ret = Vec::with_capacity(nb as usize);
            let nb = GetSystemWow64DirectoryW(ret.as_mut_ptr() as *mut _, nb)?;
            assert_eq!(nb as usize + 1, ret.capacity());
            nb
        } else {
            nb
        };
        ret.set_len(nb as usize);
        Ok(WString::new_unchecked(ret))
    }
}

pub fn is_wow64_process2(proc_handle: &ProcessHandle) -> OsResult<(u16, u16)> {
    unsafe {
        let mut process_machine = 0;
        let mut native_machine = 0;
        IsWow64Process2(
            proc_handle.as_c_handle(),
            &mut process_machine,
            &mut native_machine,
        )?;
        Ok((process_machine, native_machine))
    }
}
