use crate::*;
use crate::handle::*;
use crate::raw::um::libloaderapi::*;
use crate::string::*;
use std::ptr::null_mut;
use winapi::shared::minwindef::FARPROC;
use winapi::um::winuser::MAKEINTRESOURCEA;
use winwrap_derive::*;

#[ansi_fn]
pub fn load_library_a(
    file_name: &AStr,
) -> OsResult<HModule> {
    unsafe {
        LoadLibraryA(file_name.as_ptr())
            .and_then(|x| Ok(HModule::new(x)))
    }
}

#[unicode_fn]
pub fn load_library_w(
    file_name: &WStr,
) -> OsResult<HModule> {
    unsafe {
        LoadLibraryW(file_name.as_ptr())
            .and_then(|x| Ok(HModule::new(x)))
    }
}

pub unsafe fn free_library(
    module: HModule
) -> OsResult<()> {
    FreeLibrary(module.0)
}

/// get_proc_address can specify the function by name or ordinal.
pub enum ProcAddress<'a> {
    ByName(&'a AStr),
    ByOrdinal(WORD),
}

pub fn get_proc_address(
    module: &HModule,
    proc_addr: ProcAddress,
) -> OsResult<FARPROC> {
    unsafe {
        GetProcAddress(
            module.as_c_hmodule(),
            match proc_addr {
                ProcAddress::ByName(name) => name.as_ptr(),
                ProcAddress::ByOrdinal(ordinal) => MAKEINTRESOURCEA(ordinal),
            },
        )
    }
}

bitflags::bitflags! {
    pub struct LoadFlags: DWORD{
    /// DONT_RESOLVE_DLL_REFERENCES
    const DONT_RESOLVE_DLL_REFERENCES = winapi::um::libloaderapi::DONT_RESOLVE_DLL_REFERENCES;
    /// LOAD_IGNORE_CODE_AUTHZ_LEVEL
    const IGNORE_CODE_AUTHZ_LEVEL = winapi::um::libloaderapi::LOAD_IGNORE_CODE_AUTHZ_LEVEL;
    /// LOAD_LIBRARY_AS_DATAFILE
    const LIBRARY_AS_DATAFILE = winapi::um::libloaderapi::LOAD_LIBRARY_AS_DATAFILE;
    /// LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE
    const LIBRARY_AS_DATAFILE_EXCLUSIVE = winapi::um::libloaderapi::LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE;
    /// LOAD_LIBRARY_AS_IMAGE_RESOURCE
    const LIBRARY_AS_IMAGE_RESOURCE = winapi::um::libloaderapi::LOAD_LIBRARY_AS_IMAGE_RESOURCE;
    /// LOAD_LIBRARY_SEARCH_APPLICATION_DIR
    const LIBRARY_SEARCH_APPLICATION_DIR = winapi::um::libloaderapi::LOAD_LIBRARY_SEARCH_APPLICATION_DIR;
    /// LOAD_LIBRARY_SEARCH_DEFAULT_DIRS
    const LIBRARY_SEARCH_DEFAULT_DIRS = winapi::um::libloaderapi::LOAD_LIBRARY_SEARCH_DEFAULT_DIRS;
    /// LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR
    const LIBRARY_SEARCH_DLL_LOAD_DIR = winapi::um::libloaderapi::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR;
    /// LOAD_LIBRARY_SEARCH_SYSTEM32
    const LIBRARY_SEARCH_SYSTEM32 = winapi::um::libloaderapi::LOAD_LIBRARY_SEARCH_SYSTEM32;
    /// LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER
    const LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER = winapi::um::libloaderapi::LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER;
    /// LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY
    const LIBRARY_OS_INTEGRITY_CONTINUITY = winapi::um::libloaderapi::LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY;
    /// LOAD_LIBRARY_SEARCH_USER_DIRS
    const LIBRARY_SEARCH_USER_DIRS = winapi::um::libloaderapi::LOAD_LIBRARY_SEARCH_USER_DIRS;
    /// LOAD_WITH_ALTERED_SEARCH_PATH
    const WITH_ALTERED_SEARCH_PATH = winapi::um::libloaderapi::LOAD_WITH_ALTERED_SEARCH_PATH;
    /// LOAD_LIBRARY_REQUIRE_SIGNED_TARGET
    const LIBRARY_REQUIRE_SIGNED_TARGET = winapi::um::libloaderapi::LOAD_LIBRARY_REQUIRE_SIGNED_TARGET;
    /// LOAD_LIBRARY_SAFE_CURRENT_DIRS
    const LIBRARY_SAFE_CURRENT_DIRS = winapi::um::libloaderapi::LOAD_LIBRARY_SAFE_CURRENT_DIRS;
    }
}

#[ansi_fn]
pub fn load_library_ex_a(
    file_name: &AStr,
    load_flag: LoadFlags,
) -> OsResult<HModule> {
    unsafe {
        LoadLibraryExA(
            file_name.as_ptr(),
            null_mut(),
            load_flag.bits,
        ).and_then(|x| Ok(HModule::new(x)))
    }
}

#[unicode_fn]
pub fn load_library_ex_w(
    file_name: &WStr,
    load_flag: LoadFlags,
) -> OsResult<HModule> {
    unsafe {
        LoadLibraryExW(
            file_name.as_ptr(),
            null_mut(),
            load_flag.bits,
        ).and_then(|x| Ok(HModule::new(x)))
    }
}

#[ansi_fn]
pub fn get_module_file_name_a<'a, MH>(
    h_mod: MH
) -> OsResult<AString>
    where
        MH: Into<Option<&'a HModule>>
{
    unsafe {
        let mut v = Vec::with_capacity(128);
        GetModuleFileNameA(
            h_mod.into().map_or(null_mut(), |x| x.as_c_hmodule()),
            v.as_mut_ptr() as *mut _,
            128,
        )?;
        Ok(AString::new_unchecked(v))
    }
}

#[unicode_fn]
pub fn get_module_file_name_w<'a, MH>(
    h_mod: MH
) -> OsResult<WString>
    where
        MH: Into<Option<&'a HModule>>
{
    unsafe {
        let mut v = Vec::with_capacity(128);
        GetModuleFileNameW(
            h_mod.into().map_or(null_mut(), |x| x.as_c_hmodule()),
            v.as_mut_ptr() as *mut _,
            128,
        )?;
        Ok(WString::new_unchecked(v))
    }
}

#[ansi_fn]
pub fn get_module_handle_a(
    module_name: &AStr,
) -> OsResult<HModule> {
    unsafe {
        GetModuleHandleA(module_name.as_ptr())
            .and_then(|x| Ok(HModule::new(x)))
    }
}

#[unicode_fn]
pub fn get_module_handle_w(
    module_name: &WStr,
) -> OsResult<HModule> {
    unsafe {
        GetModuleHandleW(module_name.as_ptr())
            .and_then(|x| Ok(HModule::new(x)))
    }
}

bitflags::bitflags! {
pub struct GetModuleHandleFlags: u32{
    /// GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
    const FROM_ADDRESS = winapi::um::libloaderapi::GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS;
    /// GET_MODULE_HANDLE_EX_FLAG_PIN
    const PIN = winapi::um::libloaderapi::GET_MODULE_HANDLE_EX_FLAG_PIN;
    /// GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT
    const UNCHANGED_REFCOUNT = winapi::um::libloaderapi::GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;
}}

#[ansi_fn]
pub fn get_module_handle_ex_a(
    flags: GetModuleHandleFlags,
    module_name: &AStr,
) -> OsResult<HModule> {
    unsafe {
        let mut handle = null_mut();
        GetModuleHandleExA(
            flags.bits,
            module_name.as_ptr(),
            &mut handle,
        )?;
        Ok(HModule::new(handle))
    }
}

#[unicode_fn]
pub fn get_module_handle_ex_w(
    flags: GetModuleHandleFlags,
    module_name: &WStr,
) -> OsResult<HModule> {
    unsafe {
        let mut handle = null_mut();
        GetModuleHandleExW(
            flags.bits,
            module_name.as_ptr(),
            &mut handle,
        )?;
        Ok(HModule::new(handle))
    }
}
