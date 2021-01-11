// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::{
    handle::*, raw::um::winreg::*, string::*,
    um::minwinbase::SecurityAttributes, *,
};
use std::{fmt::Debug, ptr::null_mut};
use winapi::{
    shared::{
        basetsd::DWORD64,
        minwindef::{BYTE, HKEY},
    },
    um::winnt::{
        REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_DWORD_LITTLE_ENDIAN,
        REG_EXPAND_SZ, REG_FULL_RESOURCE_DESCRIPTOR, REG_LINK, REG_NONE,
        REG_OPTION_OPEN_LINK, REG_QWORD, REG_QWORD_LITTLE_ENDIAN,
        REG_RESOURCE_LIST, REG_RESOURCE_REQUIREMENTS_LIST, REG_SZ,
    },
};
use winwrap_derive::*;

#[repr(usize)]
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum HKeyFlags {
    /// HKEY_CLASSES_ROOT
    CLASSES_ROOT = 0x80000000,
    /// HKEY_CURRENT_USER
    CURRENT_USER = 0x80000001,
    /// HKEY_LOCAL_MACHINE
    LOCAL_MACHINE = 0x80000002,
    /// HKEY_USERS
    USERS = 0x80000003,
    /// HKEY_PERFORMANCE_DATA
    PERFORMANCE_DATA = 0x80000004,
    /// HKEY_PERFORMANCE_TEXT
    PERFORMANCE_TEXT = 0x80000050,
    /// HKEY_PERFORMANCE_NLSTEXT
    PERFORMANCE_NLSTEXT = 0x80000060,
    /// HKEY_CURRENT_CONFIG
    CURRENT_CONFIG = 0x80000005,
    /// HKEY_DYN_DATA
    DYN_DATA = 0x80000006,
    /// HKEY_CURRENT_USER_LOCAL_SETTINGS
    CURRENT_USER_LOCAL_SETTINGS = 0x80000007,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum RegValueA<'a> {
    /// REG_NONE
    NONE(&'a [u8]),
    /// REG_SZ
    SZ(&'a AStr),
    /// REG_EXPAND_SZ
    EXPAND_SZ(&'a AStr),
    /// REG_BINARY
    BINARY(&'a [u8]),
    /// REG_DWORD
    DWORD(DWORD),
    /// REG_DWORD_LITTLE_ENDIAN
    DWORD_LITTLE_ENDIAN(DWORD),
    /// REG_DWORD_BIG_ENDIAN
    DWORD_BIG_ENDIAN(DWORD),
    /// REG_LINK
    LINK(&'a WStr),
    /*
    // TODO:
    /// REG_MULTI_SZ
    MULTI_SZ(&'a [&'a AStr]),
     */
    /// REG_RESOURCE_LIST
    RESOURCE_LIST(&'a [u8]),
    /// REG_FULL_RESOURCE_DESCRIPTOR
    FULL_RESOURCE_DESCRIPTOR(&'a [u8]),
    /// REG_RESOURCE_REQUIREMENTS_LIST
    RESOURCE_REQUIREMENTS_LIST(&'a [u8]),
    ///  REG_QWORD
    QWORD(DWORD64),
    /// REG_QWORD_LITTLE_ENDIAN
    QWORD_LITTLE_ENDIAN(DWORD64),
}

impl<'a> RegValueA<'a> {
    pub fn get_data(&self) -> (DWORD, *const BYTE, usize) {
        macro_rules! to_ptr {
            ($x:ident) => {
                $x as *const _ as *const _
            };
        }
        match self {
            Self::NONE(x) => (REG_NONE, x.as_ptr() as *const _, x.len()),
            Self::SZ(x) => {
                (REG_SZ, x.as_ptr() as *const _, x.to_bytes_with_nul().len())
            }
            Self::EXPAND_SZ(x) => (
                REG_EXPAND_SZ,
                x.as_ptr() as *const _,
                x.to_bytes_with_nul().len(),
            ),
            Self::BINARY(x) => (REG_BINARY, x.as_ptr() as *const _, x.len()),
            Self::DWORD(x) => {
                (REG_DWORD, to_ptr!(x), std::mem::size_of::<DWORD>())
            }
            Self::DWORD_LITTLE_ENDIAN(x) => (
                REG_DWORD_LITTLE_ENDIAN,
                to_ptr!(x),
                std::mem::size_of::<DWORD>(),
            ),
            Self::DWORD_BIG_ENDIAN(x) => (
                REG_DWORD_BIG_ENDIAN,
                to_ptr!(x),
                std::mem::size_of::<DWORD>(),
            ),
            Self::LINK(x) => (
                REG_LINK,
                x.as_ptr() as *const _,
                x.to_bytes_with_nul().len(),
            ),
            // Self::MULTI_SZ(x) => (REG_MULTI_SZ, x.as_ptr() as *const _, x.to_bytes_with_nul().len()),
            Self::RESOURCE_LIST(x) => {
                (REG_RESOURCE_LIST, x.as_ptr() as *const _, x.len())
            }
            Self::FULL_RESOURCE_DESCRIPTOR(x) => (
                REG_FULL_RESOURCE_DESCRIPTOR,
                x.as_ptr() as *const _,
                x.len(),
            ),
            Self::RESOURCE_REQUIREMENTS_LIST(x) => (
                REG_RESOURCE_REQUIREMENTS_LIST,
                x.as_ptr() as *const _,
                x.len(),
            ),
            Self::QWORD(x) => {
                (REG_QWORD, to_ptr!(x), std::mem::size_of::<DWORD>())
            }
            Self::QWORD_LITTLE_ENDIAN(x) => (
                REG_QWORD_LITTLE_ENDIAN,
                to_ptr!(x),
                std::mem::size_of::<DWORD>(),
            ),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum RegValueW<'a> {
    /// REG_NONE
    NONE(&'a [u8]),
    /// REG_SZ
    SZ(&'a WStr),
    /// REG_EXPAND_SZ
    EXPAND_SZ(&'a WStr),
    /// REG_BINARY
    BINARY(&'a [u8]),
    /// REG_DWORD
    DWORD(DWORD),
    /// REG_DWORD_LITTLE_ENDIAN
    DWORD_LITTLE_ENDIAN(DWORD),
    /// REG_DWORD_BIG_ENDIAN
    DWORD_BIG_ENDIAN(DWORD),
    /// REG_LINK
    LINK(&'a WStr),
    /*
    // TODO:
    /// REG_MULTI_SZ
    MULTI_SZ(&'a [&'a AStr]),
     */
    /// REG_RESOURCE_LIST
    RESOURCE_LIST(&'a [u8]),
    /// REG_FULL_RESOURCE_DESCRIPTOR
    FULL_RESOURCE_DESCRIPTOR(&'a [u8]),
    /// REG_RESOURCE_REQUIREMENTS_LIST
    RESOURCE_REQUIREMENTS_LIST(&'a [u8]),
    ///  REG_QWORD
    QWORD(DWORD64),
    /// REG_QWORD_LITTLE_ENDIAN
    QWORD_LITTLE_ENDIAN(DWORD64),
}

impl<'a> RegValueW<'a> {
    pub fn get_data(&self) -> (DWORD, *const BYTE, usize) {
        macro_rules! to_ptr {
            ($x:ident) => {
                $x as *const _ as *const _
            };
        }
        match self {
            Self::NONE(x) => (REG_NONE, x.as_ptr() as *const _, x.len()),
            Self::SZ(x) => (
                REG_SZ,
                x.as_ptr() as *const _,
                x.to_u8_bytes_with_nul().len(),
            ),
            Self::EXPAND_SZ(x) => (
                REG_EXPAND_SZ,
                x.as_ptr() as *const _,
                x.to_u8_bytes_with_nul().len(),
            ),
            Self::BINARY(x) => (REG_BINARY, x.as_ptr() as *const _, x.len()),
            Self::DWORD(x) => {
                (REG_DWORD, to_ptr!(x), std::mem::size_of::<DWORD>())
            }
            Self::DWORD_LITTLE_ENDIAN(x) => (
                REG_DWORD_LITTLE_ENDIAN,
                to_ptr!(x),
                std::mem::size_of::<DWORD>(),
            ),
            Self::DWORD_BIG_ENDIAN(x) => (
                REG_DWORD_BIG_ENDIAN,
                to_ptr!(x),
                std::mem::size_of::<DWORD>(),
            ),
            Self::LINK(x) => (
                REG_LINK,
                x.as_ptr() as *const _,
                x.to_u8_bytes_with_nul().len(),
            ),
            // Self::MULTI_SZ(x) => (REG_MULTI_SZ, x.as_ptr() as *const _, x.to_u8_bytes_with_nul().len()),
            Self::RESOURCE_LIST(x) => {
                (REG_RESOURCE_LIST, x.as_ptr() as *const _, x.len())
            }
            Self::FULL_RESOURCE_DESCRIPTOR(x) => (
                REG_FULL_RESOURCE_DESCRIPTOR,
                x.as_ptr() as *const _,
                x.len(),
            ),
            Self::RESOURCE_REQUIREMENTS_LIST(x) => (
                REG_RESOURCE_REQUIREMENTS_LIST,
                x.as_ptr() as *const _,
                x.len(),
            ),
            Self::QWORD(x) => {
                (REG_QWORD, to_ptr!(x), std::mem::size_of::<DWORD>())
            }
            Self::QWORD_LITTLE_ENDIAN(x) => (
                REG_QWORD_LITTLE_ENDIAN,
                to_ptr!(x),
                std::mem::size_of::<DWORD>(),
            ),
        }
    }
}

bitflags::bitflags! {
    pub struct RegKeyRights: u32{
        /// KEY_QUERY_VALUE
        const QUERY_VALUE        = winapi::um::winnt::KEY_QUERY_VALUE;
        /// KEY_CREATE_SUB_KEY
        const CREATE_SUB_KEY     = winapi::um::winnt::KEY_CREATE_SUB_KEY;
        /// KEY_ENUMERATE_SUB_KEYS
        const ENUMERATE_SUB_KEYS = winapi::um::winnt::KEY_ENUMERATE_SUB_KEYS;
        /// KEY_NOTIFY
        const NOTIFY             = winapi::um::winnt::KEY_NOTIFY;
        /// KEY_CREATE_LINK
        const CREATE_LINK        = winapi::um::winnt::KEY_CREATE_LINK;
        /// KEY_WOW64_32KEY
        const WOW64_32KEY        = winapi::um::winnt::KEY_WOW64_32KEY;
        /// KEY_WOW64_64KEY
        const WOW64_64KEY        = winapi::um::winnt::KEY_WOW64_64KEY;
        /// KEY_WOW64_RES
        const WOW64_RES          = winapi::um::winnt::KEY_WOW64_RES;
        /// KEY_READ
        const READ               = winapi::um::winnt::KEY_READ;
        /// KEY_WRITE
        const WRITE              = winapi::um::winnt::KEY_WRITE;
        /// KEY_EXECUTE
        const EXECUTE            = winapi::um::winnt::KEY_EXECUTE;
        /// KEY_ALL_ACCESS
        const ALL_ACCESS         = winapi::um::winnt::KEY_ALL_ACCESS;
    }
}

bfi! {RegKeyRights,u32}

impl HKeyType for HKeyFlags {
    fn as_c_hkey(&self) -> HKEY { *self as usize as HKEY }
}

/// Safe RegCreateKeyA.
///
/// # Example
///
/// ```rust
/// use winwrap::{
///     string::AString,
///     um::winreg::{reg_create_key_a, HKeyFlags},
/// };
///
/// let sub_key = AString::from_str_lossy("Environment");
/// reg_create_key_a(&HKeyFlags::CURRENT_USER, sub_key.as_c_str())
///     .expect("Failed to reg_create_key_a");
/// ```
#[ansi_fn]
pub fn reg_create_key_a(
    hkey: &impl HKeyType,
    sub_key: &AStr,
) -> OsResult<HKey> {
    unsafe {
        let mut ret = std::mem::zeroed();
        RegCreateKeyA(hkey.as_c_hkey(), sub_key.as_ptr(), &mut ret)?;
        Ok(HKey::new(ret))
    }
}

#[unicode_fn]
pub fn reg_create_key_w(
    hkey: &impl HKeyType,
    sub_key: &WStr,
) -> OsResult<HKey> {
    unsafe {
        let mut ret = std::mem::zeroed();
        RegCreateKeyW(hkey.as_c_hkey(), sub_key.as_ptr(), &mut ret)?;
        Ok(HKey::new(ret))
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum RegDisposition {
    /// REG_CREATED_NEW_KEY
    CREATED_NEW_KEY = 0x1,
    /// REG_OPENED_EXISTING_KEY
    OPENED_EXISTING_KEY = 0x2,
}

impl From<u32> for RegDisposition {
    fn from(x: u32) -> Self {
        match x {
            1 => Self::CREATED_NEW_KEY,
            2 => Self::OPENED_EXISTING_KEY,
            _ => panic!("Unknown RegDisposition value"),
        }
    }
}

pub fn reg_create_key_ex_w<'a, CL, SA>(
    hkey: &impl HKeyType,
    sub_key: &WStr,
    class: CL,
    options: DWORD,
    sam: RegKeyRights,
    security_attributes: SA,
) -> OsResult<(HKey, RegDisposition)>
where
    CL: Into<Option<&'a mut WStr>>,
    SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
{
    unsafe {
        let mut ret = std::mem::zeroed();
        let mut disposition = 0;
        RegCreateKeyExW(
            hkey.as_c_hkey(),
            sub_key.as_ptr(),
            0,
            class.into().map_or(null_mut(), |x| x.as_mut_ptr()),
            options,
            sam.bits,
            security_attributes
                .into()
                .map_or(null_mut(), |x| x.as_mut_c_ptr()),
            &mut ret,
            &mut disposition,
        )?;
        Ok((HKey::new(ret), RegDisposition::from(disposition)))
    }
}

#[ansi_fn]
pub fn reg_open_key_ex_a<SA>(
    hkey: &impl HKeyType,
    sub_key: &AStr,
    is_option_open_link: bool,
    sam: SA,
) -> OsResult<HKey>
where
    SA: Into<DWORD>,
{
    unsafe {
        let mut ret = std::mem::zeroed();
        RegOpenKeyExA(
            hkey.as_c_hkey(),
            sub_key.as_ptr(),
            if is_option_open_link {
                REG_OPTION_OPEN_LINK
            } else {
                0
            },
            sam.into(),
            &mut ret,
        )?;
        Ok(HKey::new(ret))
    }
}

#[unicode_fn]
pub fn reg_open_key_ex_w(
    hkey: &impl HKeyType,
    sub_key: &WStr,
    is_option_open_link: bool,
    sam: RegKeyRights,
) -> OsResult<HKey> {
    unsafe {
        let mut ret = std::mem::zeroed();
        RegOpenKeyExW(
            hkey.as_c_hkey(),
            sub_key.as_ptr(),
            if is_option_open_link {
                REG_OPTION_OPEN_LINK
            } else {
                0
            },
            sam.bits,
            &mut ret,
        )?;
        Ok(HKey::new(ret))
    }
}

#[ansi_fn]
pub fn reg_set_value_ex_a(
    hkey: &HKey,
    name: &AStr,
    value: RegValueA,
) -> OsResult<()> {
    let (ty, data, data_len) = value.get_data();
    unsafe {
        RegSetValueExA(
            hkey.as_c_hkey(),
            name.as_ptr(),
            0,
            ty,
            data,
            data_len as u32,
        )?;
    }
    Ok(())
}

#[unicode_fn]
pub fn reg_set_value_ex_w(
    hkey: &HKey,
    name: &WStr,
    value: RegValueW,
) -> OsResult<()> {
    let (ty, data, data_len) = value.get_data();
    unsafe {
        RegSetValueExW(
            hkey.as_c_hkey(),
            name.as_ptr(),
            0,
            ty,
            data,
            data_len as u32,
        )?;
    }
    Ok(())
}

#[ansi_fn]
pub fn reg_delete_key_a(hkey: &impl HKeyType, sub_key: &AStr) -> OsResult<()> {
    unsafe { RegDeleteKeyA(hkey.as_c_hkey(), sub_key.as_ptr()) }
}

#[unicode_fn]
pub fn reg_delete_key_w(hkey: &impl HKeyType, sub_key: &WStr) -> OsResult<()> {
    unsafe { RegDeleteKeyW(hkey.as_c_hkey(), sub_key.as_ptr()) }
}

#[ansi_fn]
pub fn reg_delete_value_a(
    hkey: &impl HKeyType,
    value_name: &AStr,
) -> OsResult<()> {
    unsafe { RegDeleteValueA(hkey.as_c_hkey(), value_name.as_ptr()) }
}

#[unicode_fn]
pub fn reg_delete_value_w(
    hkey: &impl HKeyType,
    value_name: &WStr,
) -> OsResult<()> {
    unsafe { RegDeleteValueW(hkey.as_c_hkey(), value_name.as_ptr()) }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum RegQuery {
    BINARY(Vec<u8>),
    DWORD(DWORD),
    DWORD_LITTLE_ENDIAN(DWORD),
    DWORD_BIG_ENDIAN(DWORD),
    EXPAND_SZ(AString),
    // AString or WString
    LINK(WString),
    // WString only
    MULTI_SZ(AString),
    NONE,
    QWORD(u64),
    QWORD_LITTLE_ENDIAN(u64),
    SZ(AString),
}

pub fn reg_query_value_ex_a(
    hkey: &impl HKeyType,
    value_name: &AStr,
) -> OsResult<RegQuery> {
    let mut ty = 0;
    let mut data = Vec::with_capacity(128);
    unsafe {
        RegQueryValueExA(
            hkey.as_c_hkey(),
            value_name.as_ptr(),
            null_mut(),
            &mut ty,
            data.as_mut_ptr(),
            data.len() as _,
        )?;
        Ok(match ty {
            REG_SZ => RegQuery::SZ(AString::new_unchecked(data)),
            REG_DWORD => RegQuery::DWORD(*(data.as_ptr() as *const DWORD)),
            _ => RegQuery::NONE,
        })
    }
}
