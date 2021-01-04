// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use crate::handle::*;
use crate::raw::um::namedpipeapi::*;
use crate::um::minwinbase::Overlapped;
use crate::string::*;
use crate::um::minwinbase::SecurityAttributes;
use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winwrap_derive::*;

/// Returns `(read_pipe, write_pipe)`.
pub fn create_pipe<'a, SA>(
    sa: SA,
    size: DWORD,
) -> OsResult<(PipeHandle, PipeHandle)>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
{
    unsafe {
        let mut read_pipe = null_mut();
        let mut write_pipe = null_mut();
        CreatePipe(
            &mut read_pipe,
            &mut write_pipe,
            sa.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            size)?;
        Ok((PipeHandle::new(read_pipe), PipeHandle::new(write_pipe)))
    }
}

bitflags::bitflags! {
    pub struct OpenModes : u32 {
        const PIPE_ACCESS_DUPLEX   = winapi::um::winbase::PIPE_ACCESS_DUPLEX;
        const PIPE_ACCESS_INBOUND  = winapi::um::winbase::PIPE_ACCESS_INBOUND;
        const PIPE_ACCESS_OUTBOUND = winapi::um::winbase::PIPE_ACCESS_OUTBOUND;
        const FILE_FLAG_WRITE_THROUGH = winapi::um::winbase::FILE_FLAG_WRITE_THROUGH;
        const FILE_FLAG_OVERLAPPED = winapi::um::winbase::FILE_FLAG_OVERLAPPED;
        const FILE_FLAG_NO_BUFFERING = winapi::um::winbase::FILE_FLAG_NO_BUFFERING;
        const FILE_FLAG_RANDOM_ACCESS = winapi::um::winbase::FILE_FLAG_RANDOM_ACCESS;
        const FILE_FLAG_SEQUENTIAL_SCAN = winapi::um::winbase::FILE_FLAG_SEQUENTIAL_SCAN;
        const FILE_FLAG_DELETE_ON_CLOSE = winapi::um::winbase::FILE_FLAG_DELETE_ON_CLOSE;
        const FILE_FLAG_BACKUP_SEMANTICS = winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;
        const FILE_FLAG_POSIX_SEMANTICS = winapi::um::winbase::FILE_FLAG_POSIX_SEMANTICS;
        const FILE_FLAG_SESSION_AWARE = winapi::um::winbase::FILE_FLAG_SESSION_AWARE;
        const FILE_FLAG_OPEN_REPARSE_POINT = winapi::um::winbase::FILE_FLAG_OPEN_REPARSE_POINT;
        const FILE_FLAG_OPEN_NO_RECALL = winapi::um::winbase::FILE_FLAG_OPEN_NO_RECALL;
        const FILE_FLAG_FIRST_PIPE_INSTANCE = winapi::um::winbase::FILE_FLAG_FIRST_PIPE_INSTANCE;
        const FILE_FLAG_OPEN_REQUIRING_OPLOCK = winapi::um::winbase::FILE_FLAG_OPEN_REQUIRING_OPLOCK;
        const WRITE_DAC = winapi::um::winnt::WRITE_DAC;
        const WRITE_OWNER = winapi::um::winnt::WRITE_OWNER;
        const ACCESS_SYSTEM_SECURITY = winapi::um::winnt::ACCESS_SYSTEM_SECURITY;
    }
}

bitflags::bitflags! {
    pub struct PipeModes: u32{
        const TYPE_BYTE             = winapi::um::winbase::PIPE_TYPE_BYTE;
        const TYPE_MESSAGE          = winapi::um::winbase::PIPE_TYPE_MESSAGE;
        const READMODE_BYTE         = winapi::um::winbase::PIPE_READMODE_BYTE;
        const READMODE_MESSAGE      = winapi::um::winbase::PIPE_READMODE_MESSAGE;
        const WAIT                  = winapi::um::winbase::PIPE_WAIT;
        const NOWAIT                = winapi::um::winbase::PIPE_NOWAIT;
        const ACCEPT_REMOTE_CLIENTS = winapi::um::winbase::PIPE_ACCEPT_REMOTE_CLIENTS;
        const REJECT_REMOTE_CLIENTS = winapi::um::winbase::PIPE_REJECT_REMOTE_CLIENTS;
    }
}

#[unicode_fn]
pub fn create_named_pipe_w<'a, SA>(
    name: &WStr,
    open_mode: OpenModes,
    pipe_mode: PipeModes,
    max_instances: u8, // 1 -- PIPE_UNLIMITED_INSTANCES(255)
    out_buffer_size: DWORD,
    in_buffer_size: DWORD,
    default_timeout: DWORD,
    sec_attrs: SA,
) -> OsResult<PipeHandle>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
{
    unsafe {
        CreateNamedPipeW(
            name.as_ptr(),
            open_mode.bits,
            pipe_mode.bits,
            max_instances as DWORD,
            out_buffer_size,
            in_buffer_size,
            default_timeout,
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        ).map(PipeHandle::new)
    }
}

pub fn connect_named_pipe<'a, OL>(
    handle: &PipeHandle,
    overlapped: OL,
) -> OsResult<()>
    where
        OL: Into<Option<&'a mut Overlapped>>,
{
    unsafe {
        ConnectNamedPipe(
            handle.as_c_handle(),
            overlapped.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )
    }
}

pub fn disconnect_named_pipe(
    handle: &PipeHandle,
) -> OsResult<()>
{
    unsafe { DisconnectNamedPipe(handle.as_c_handle()) }
}

pub fn get_named_pipe_info<'a, FL, OB, IB, MI>(
    handle: &PipeHandle,
    flags: FL,
    out_buffer_size: OB,
    in_buffer_size: IB,
    max_instances: MI,
) -> OsResult<()>
    where
        FL: Into<Option<&'a mut DWORD>>,
        OB: Into<Option<&'a mut DWORD>>,
        IB: Into<Option<&'a mut DWORD>>,
        MI: Into<Option<&'a mut DWORD>>,
{
    unsafe {
        GetNamedPipeInfo(
            handle.as_c_handle(),
            flags.into().map_or(null_mut(), |x| x),
            out_buffer_size.into().map_or(null_mut(), |x| x),
            in_buffer_size.into().map_or(null_mut(), |x| x),
            max_instances.into().map_or(null_mut(), |x| x),
        )
    }
}
