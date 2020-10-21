use crate::*;
use crate::handle::*;
use crate::raw::um::winbase::*;
use crate::string::*;
use crate::um::minwinbase::SecurityAttributes;
use crate::um::namedpipeapi::{OpenModes, PipeModes};
use std::ptr::null_mut;
use winapi::shared::basetsd::{DWORD_PTR, ULONG_PTR};
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::shared::winerror::{ERROR_INSUFFICIENT_BUFFER, ERROR_MORE_DATA};
use winapi::um::winnt::HANDLE;
use winwrap_derive::*;
use crate::OsError::Win32;

bitflags::bitflags! {
pub struct FormatFlags: DWORD{
    /// FORMAT_MESSAGE_IGNORE_INSERTS
    const IGNORE_INSERTS  = winapi::um::winbase::FORMAT_MESSAGE_IGNORE_INSERTS;
    /// FORMAT_MESSAGE_FROM_STRING
    const FROM_STRING     = winapi::um::winbase::FORMAT_MESSAGE_FROM_STRING;
    /// FORMAT_MESSAGE_FROM_HMODULE
    const FROM_HMODULE    = winapi::um::winbase::FORMAT_MESSAGE_FROM_HMODULE;
    /// FORMAT_MESSAGE_FROM_SYSTEM
    const FROM_SYSTEM     = winapi::um::winbase::FORMAT_MESSAGE_FROM_SYSTEM;
    /// FORMAT_MESSAGE_ARGUMENT_ARRAY
    const ARGUMENT_ARRAY  = winapi::um::winbase::FORMAT_MESSAGE_ARGUMENT_ARRAY;
    /// FORMAT_MESSAGE_MAX_WIDTH_MASK
    const MAX_WIDTH_MASK  = winapi::um::winbase::FORMAT_MESSAGE_MAX_WIDTH_MASK;
}}

// unsafe:
// FORMAT_MESSAGE_ALLOCATE_BUFFER
// const ALLOCATE_BUFFER = winapi::um::winbase::FORMAT_MESSAGE_ALLOCATE_BUFFER;

#[repr(u32)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum DEPSystemPolicyType {
    AlwaysOff = 0,
    AlwaysOn = 1,
    OptIn = 2,
    OptOut = 3,
}

impl From<u32> for DEPSystemPolicyType {
    fn from(x: u32) -> Self {
        match x {
            0 => Self::AlwaysOff,
            1 => Self::AlwaysOn,
            2 => Self::OptIn,
            3 => Self::OptOut,
            _ => panic!("Unknown DEPSystemPolicyType"),
        }
    }
}

pub fn get_system_dep_policy() -> DEPSystemPolicyType {
    DEPSystemPolicyType::from(GetSystemDEPPolicy())
}

#[ansi_fn]
pub fn format_message_a<'a, SO, LI, AG>(
    flags: FormatFlags,
    source: SO,
    message_id: DWORD,
    language_id: LI,
    arguments: AG,
) -> OsResult<AString>
    where
        SO: Into<Option<HANDLE>>,
        LI: Into<DWORD>,
        AG: Into<Option<LPVOID>>,
{
    let mut ret: Vec<u8> = Vec::with_capacity(128);
    let source = source.into().map_or(null_mut(), |x| x);
    let language_id = language_id.into();
    let arguments = arguments.into().map_or(null_mut(), |x| x);
    unsafe {
        loop {
            match FormatMessageA(
                flags.bits,
                source,
                message_id,
                language_id,
                ret.as_mut_ptr() as *mut _,
                ret.capacity() as DWORD,
                arguments as *mut _) {
                Ok(nb) => {
                    assert!(ret.capacity() >= nb as usize);
                    ret.set_len(nb as usize);
                    return Ok(AString::from(ret));
                }
                Err(Win32(ERROR_INSUFFICIENT_BUFFER)) => ret = Vec::with_capacity(ret.capacity() * 2),
                Err(x) => return Err(x),
            }
        }
    }
}

#[unicode_fn]
pub fn format_message_w<'a, SO, LI, AG>(
    flags: FormatFlags,
    source: SO,
    message_id: DWORD,
    language_id: LI,
    arguments: AG,
) -> OsResult<WString>
    where
        SO: Into<Option<HANDLE>>,
        LI: Into<DWORD>,
        AG: Into<Option<LPVOID>>,
{
    let mut ret: Vec<u16> = Vec::with_capacity(128);
    let source = source.into().map_or(null_mut(), |x| x);
    let language_id = language_id.into();
    let arguments = arguments.into().map_or(null_mut(), |x| x);
    unsafe {
        loop {
            match FormatMessageW(
                flags.bits,
                source,
                message_id,
                language_id,
                ret.as_mut_ptr() as *mut _,
                ret.capacity() as DWORD,
                arguments as *mut _) {
                Ok(nb) => {
                    assert!(ret.capacity() >= nb as usize);
                    ret.set_len(nb as usize);
                    return Ok(WString::new(ret));
                }
                Err(Win32(ERROR_INSUFFICIENT_BUFFER)) => ret = Vec::with_capacity(ret.capacity() * 2),
                Err(x) => return Err(x),
            }
        }
    }
}

#[unicode_fn]
pub fn get_user_name_w() -> OsResult<WString> {
    unsafe {
        let mut v: Vec<u16> = Vec::with_capacity(128);
        let mut a = 128;
        match GetUserNameW(v.as_mut_ptr(), &mut a) {
            Ok(()) => {
                v.set_len((a - 1) as usize);
                Ok(WString::new(v))
            }
            Err(Win32(ERROR_INSUFFICIENT_BUFFER)) => {
                let mut v: Vec<u16> = Vec::with_capacity(a as usize);
                GetUserNameW(v.as_mut_ptr(), &mut a)?;
                assert_eq!(v.capacity(), a as usize);
                v.set_len((a - 1) as usize);
                Ok(WString::new(v))
            }
            Err(x) => Err(x),
        }
    }
}

#[ansi_fn]
pub fn create_named_pipe_a<'a, SA>(
    name: &AStr,
    open_mode: OpenModes,
    pipe_mode: PipeModes,
    max_instances: u8,
    out_buffer_size: DWORD,
    in_buffer_size: DWORD,
    default_timeout: DWORD,
    sec_attrs: SA,
) -> OsResult<PipeHandle>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>
{
    unsafe {
        CreateNamedPipeA(
            name.as_ptr(),
            open_mode.bits(),
            pipe_mode.bits(),
            max_instances as DWORD,
            out_buffer_size,
            in_buffer_size,
            default_timeout,
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        ).and_then(|x| Ok(PipeHandle::new(x)))
    }
}

#[ansi_fn]
pub fn dns_hostname_to_computer_name_a(
    host_name: &AStr,
) -> OsResult<AString> {
    unsafe {
        let mut v: Vec<u8> = Vec::with_capacity(128);
        let mut nb = 128;
        match DnsHostnameToComputerNameA(
            host_name.as_ptr(),
            v.as_ptr() as *const _,
            &mut nb,
        ) {
            Ok(_) => {
                v.set_len(nb as usize - 1);
                Ok(AString::from(v))
            }
            Err(Win32(ERROR_MORE_DATA)) => {
                let mut v: Vec<u8> = Vec::with_capacity(nb as usize);
                DnsHostnameToComputerNameA(host_name.as_ptr(),
                                           v.as_ptr() as *const _,
                                           &mut nb)?;
                assert_eq!(v.capacity(), nb as usize);
                v.set_len(nb as usize - 1);
                Ok(AString::from(v))
            }
            Err(x) => Err(x),
        }
    }
}

#[ansi_fn]
pub fn get_user_name_a() -> OsResult<AString> {
    unsafe {
        let mut v: Vec<u8> = Vec::with_capacity(128);
        let mut a = 128;
        match GetUserNameA(v.as_mut_ptr() as *mut i8, &mut a) {
            Ok(()) => {
                v.set_len(a as usize - 1);
                Ok(AString::from(v))
            }
            Err(Win32(ERROR_INSUFFICIENT_BUFFER)) => {
                let mut v: Vec<u8> = Vec::with_capacity(a as usize);
                GetUserNameA(v.as_mut_ptr() as *mut i8, &mut a)?;
                assert_eq!(v.capacity(), a as usize);
                v.set_len(a as usize - 1);
                Ok(AString::from(v))
            }
            Err(x) => Err(x),
        }
    }
}

pub fn get_process_affinity_mask(
    proc_handle: &ProcessHandle,
) -> OsResult<(ULONG_PTR, ULONG_PTR)> {
    let mut proc_aff_mask = 0;
    let mut sys_aff_mask = 0;
    unsafe {
        GetProcessAffinityMask(
            proc_handle.as_c_handle(),
            &mut proc_aff_mask,
            &mut sys_aff_mask,
        )?;
        Ok((proc_aff_mask, sys_aff_mask))
    }
}

pub fn set_process_affinity_mask(
    proc_handle: &ProcessHandle,
    new_aff_mask: DWORD_PTR,
) -> OsResult<()> {
    unsafe {
        SetProcessAffinityMask(
            proc_handle.as_c_handle(),
            new_aff_mask as DWORD,
        )
    }
}

pub fn set_thread_affinity_mask(
    proc_handle: &ProcessHandle,
    new_aff_mask: DWORD_PTR,
) -> OsResult<ULONG_PTR> {
    unsafe {
        SetThreadAffinityMask(
            proc_handle.as_c_handle(),
            new_aff_mask,
        )
    }
}

#[ansi_fn]
pub fn create_hard_link_a<'a, SA>(
    file_name: &AStr,
    existing_file_name: &AStr,
    sec_attrs: SA,
) -> OsResult<()>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>
{
    unsafe {
        CreateHardLinkA(
            file_name.as_ptr(),
            existing_file_name.as_ptr(),
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )
    }
}

#[unicode_fn]
pub fn create_hard_link_w<'a, SA>(
    file_name: &WStr,
    existing_file_name: &WStr,
    sec_attrs: SA,
) -> OsResult<()>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>
{
    unsafe {
        CreateHardLinkW(
            file_name.as_ptr(),
            existing_file_name.as_ptr(),
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )
    }
}

bitflags::bitflags! {
pub struct SymlinkFlags: DWORD {
    const NONE = 0;
    /// SYMBOLIC_LINK_FLAG_DIRECTORY
    const DIRECTORY = 1;
    /// SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE
    const ALLOW_UNPRIVILEGED_CREAT = 2;
}}

#[ansi_fn]
pub fn create_symbolic_link_a(
    symlink_file_name: &AStr,
    target_file_name: &AStr,
    sym_flag: SymlinkFlags,
) -> OsResult<()> {
    unsafe {
        CreateSymbolicLinkA(
            symlink_file_name.as_ptr(),
            target_file_name.as_ptr(),
            sym_flag.bits,
        )
    }
}

#[unicode_fn]
pub fn create_symbolic_link_w(
    symlink_file_name: &WStr,
    target_file_name: &WStr,
    sym_flag: SymlinkFlags,
) -> OsResult<()> {
    unsafe {
        CreateSymbolicLinkW(
            symlink_file_name.as_ptr(),
            target_file_name.as_ptr(),
            sym_flag.bits,
        )
    }
}
