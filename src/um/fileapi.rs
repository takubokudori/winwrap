use crate::*;
use crate::handle::*;
use crate::raw::um::fileapi::*;
use crate::um::minwinbase::Overlapped;
use crate::string::*;
use crate::um::minwinbase::{Win32FindDataA, SecurityAttributes, Win32FindDataW};
use std::mem::{MaybeUninit, size_of};
use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror::NO_ERROR;
use winapi::um::fileapi::INVALID_FILE_SIZE;
use winwrap_derive::*;
use crate::OsError::Win32;

bitflags::bitflags! {
pub struct FileAccessRights: DWORD{
    const FILE_ADD_FILE=winapi::um::winnt::FILE_ADD_FILE;
    const FILE_ADD_SUBDIRECTORY=winapi::um::winnt::FILE_ADD_SUBDIRECTORY;
    const FILE_ALL_ACCESS=winapi::um::winnt::FILE_ALL_ACCESS;
    const FILE_APPEND_DATA=winapi::um::winnt::FILE_APPEND_DATA;
    const FILE_CREATE_PIPE_INSTANCE=winapi::um::winnt::FILE_CREATE_PIPE_INSTANCE;
    const FILE_DELETE_CHILD=winapi::um::winnt::FILE_DELETE_CHILD;
    const FILE_EXECUTE=winapi::um::winnt::FILE_EXECUTE;
    const FILE_LIST_DIRECTORY=winapi::um::winnt::FILE_LIST_DIRECTORY;
    const FILE_READ_ATTRIBUTES=winapi::um::winnt::FILE_READ_ATTRIBUTES;
    const FILE_READ_DATA=winapi::um::winnt::FILE_READ_DATA;
    const FILE_READ_EA=winapi::um::winnt::FILE_READ_EA;
    const FILE_TRAVERSE=winapi::um::winnt::FILE_TRAVERSE;
    const FILE_WRITE_ATTRIBUTES=winapi::um::winnt::FILE_WRITE_ATTRIBUTES;
    const FILE_WRITE_DATA=winapi::um::winnt::FILE_WRITE_DATA;
    const FILE_WRITE_EA=winapi::um::winnt::FILE_WRITE_EA;
    const STANDARD_RIGHTS_READ=winapi::um::winnt::STANDARD_RIGHTS_READ;
    const STANDARD_RIGHTS_WRITE=winapi::um::winnt::STANDARD_RIGHTS_WRITE;
    const FILE_GENERIC_EXECUTE=winapi::um::winnt::FILE_GENERIC_EXECUTE;
    const FILE_GENERIC_READ=winapi::um::winnt::FILE_GENERIC_READ;
    const FILE_GENERIC_WRITE=winapi::um::winnt::FILE_GENERIC_WRITE;
    const SYNCHRONIZE=winapi::um::winnt::SYNCHRONIZE;
    const GENERIC_ALL=winapi::um::winnt::GENERIC_ALL;
    const GENERIC_EXECUTE=winapi::um::winnt::GENERIC_EXECUTE;
    const GENERIC_READ=winapi::um::winnt::GENERIC_READ;
    const GENERIC_WRITE=winapi::um::winnt::GENERIC_WRITE;
}}

bitflags::bitflags! {
pub struct ShareModes: DWORD{
    /// FILE_SHARE_DELETE
    const DELETE=winapi::um::winnt::FILE_SHARE_DELETE;
    /// FILE_SHARE_READ
    const READ=winapi::um::winnt::FILE_SHARE_READ;
    /// FILE_SHARE_WRITE
    const WRITE=winapi::um::winnt::FILE_SHARE_WRITE;
    const NONE = 0x0;
}}

bitflags::bitflags! {
pub struct CreationDisposition: DWORD{
    const CREATE_ALWAYS=winapi::um::fileapi::CREATE_ALWAYS;
    const CREATE_NEW=winapi::um::fileapi::CREATE_NEW;
    const OPEN_ALWAYS=winapi::um::fileapi::OPEN_ALWAYS;
    const OPEN_EXISTING=winapi::um::fileapi::OPEN_EXISTING;
    const TRUNCATE_EXISTING=winapi::um::fileapi::TRUNCATE_EXISTING;
}}

bitflags::bitflags! {
pub struct FileAttributeConstants: DWORD{
    const FILE_ATTRIBUTE_READONLY=winapi::um::winnt::FILE_ATTRIBUTE_READONLY;
    const FILE_ATTRIBUTE_HIDDEN=winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;
    const FILE_ATTRIBUTE_SYSTEM=winapi::um::winnt::FILE_ATTRIBUTE_SYSTEM;
    const FILE_ATTRIBUTE_DIRECTORY=winapi::um::winnt::FILE_ATTRIBUTE_DIRECTORY;
    const FILE_ATTRIBUTE_ARCHIVE=winapi::um::winnt::FILE_ATTRIBUTE_ARCHIVE;
    const FILE_ATTRIBUTE_DEVICE=winapi::um::winnt::FILE_ATTRIBUTE_DEVICE;
    const FILE_ATTRIBUTE_NORMAL=winapi::um::winnt::FILE_ATTRIBUTE_NORMAL;
    const FILE_ATTRIBUTE_TEMPORARY=winapi::um::winnt::FILE_ATTRIBUTE_TEMPORARY;
    const FILE_ATTRIBUTE_SPARSE_FILE=winapi::um::winnt::FILE_ATTRIBUTE_SPARSE_FILE;
    const FILE_ATTRIBUTE_REPARSE_POINT=winapi::um::winnt::FILE_ATTRIBUTE_REPARSE_POINT;
    const FILE_ATTRIBUTE_COMPRESSED=winapi::um::winnt::FILE_ATTRIBUTE_COMPRESSED;
    const FILE_ATTRIBUTE_OFFLINE=winapi::um::winnt::FILE_ATTRIBUTE_OFFLINE;
    const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED=winapi::um::winnt::FILE_ATTRIBUTE_NOT_CONTENT_INDEXED;
    const FILE_ATTRIBUTE_ENCRYPTED=winapi::um::winnt::FILE_ATTRIBUTE_ENCRYPTED;
    const FILE_ATTRIBUTE_INTEGRITY_STREAM=winapi::um::winnt::FILE_ATTRIBUTE_INTEGRITY_STREAM;
    const FILE_ATTRIBUTE_VIRTUAL=winapi::um::winnt::FILE_ATTRIBUTE_VIRTUAL;
    const FILE_ATTRIBUTE_NO_SCRUB_DATA=winapi::um::winnt::FILE_ATTRIBUTE_NO_SCRUB_DATA;
    const FILE_ATTRIBUTE_EA=winapi::um::winnt::FILE_ATTRIBUTE_EA;
    const FILE_ATTRIBUTE_PINNED=winapi::um::winnt::FILE_ATTRIBUTE_PINNED;
    const FILE_ATTRIBUTE_UNPINNED=winapi::um::winnt::FILE_ATTRIBUTE_UNPINNED;
    const FILE_ATTRIBUTE_RECALL_ON_OPEN=winapi::um::winnt::FILE_ATTRIBUTE_RECALL_ON_OPEN;
    const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS=winapi::um::winnt::FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS;
    const FILE_FLAG_WRITE_THROUGH = winapi::um::winbase::FILE_FLAG_OVERLAPPED;
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
}}

#[ansi_fn]
pub fn create_directory_a<'a, SA>(
    path_name: &AStr,
    sec_attrs: SA,
) -> OsResult<()>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
{
    unsafe {
        CreateDirectoryA(
            path_name.as_ptr(),
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )
    }
}

#[unicode_fn]
pub fn create_directory_w<'a, SA>(
    path_name: &WStr,
    sec_attrs: SA,
) -> OsResult<()>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
{
    unsafe {
        CreateDirectoryW(
            path_name.as_ptr(),
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )
    }
}

#[ansi_fn]
pub fn find_first_file_a(path: &AStr) -> OsResult<(FindFileHandle, Win32FindDataA)> {
    unsafe {
        let mut fd = MaybeUninit::<Win32FindDataA>::uninit();
        let handle = FindFirstFileA(path.as_ptr(), fd.as_mut_ptr() as *mut _)?;
        let fd = fd.assume_init();
        Ok((FindFileHandle::new(handle), fd))
    }
}

#[ansi_fn]
pub fn find_next_file_a(handle: &FindFileHandle) -> OsResult<Win32FindDataA> {
    unsafe {
        let mut fd = MaybeUninit::<Win32FindDataA>::uninit();
        FindNextFileA(handle.as_c_handle(), fd.as_mut_ptr() as *mut _)?;
        let fd = fd.assume_init();
        Ok(fd)
    }
}

#[unicode_fn]
pub fn find_first_file_w(path: &WStr) -> OsResult<(FindFileHandle, Win32FindDataW)> {
    unsafe {
        let mut fd = MaybeUninit::<Win32FindDataW>::uninit();
        let handle = FindFirstFileW(path.as_ptr(), fd.as_mut_ptr() as *mut _)?;
        let fd = fd.assume_init();
        Ok((FindFileHandle::new(handle), fd))
    }
}

#[unicode_fn]
pub fn find_next_file_w(handle: &FindFileHandle) -> OsResult<Win32FindDataW> {
    unsafe {
        let mut fd = MaybeUninit::<Win32FindDataW>::uninit();
        FindNextFileW(handle.as_c_handle(), fd.as_mut_ptr() as *mut _)?;
        let fd = fd.assume_init();
        Ok(fd)
    }
}

#[ansi_fn]
pub fn create_file_a<'a, SA, TM>(
    file_name: &AStr,
    desired_access: FileAccessRights,
    share_mode: ShareModes,
    sec_attrs: SA,
    creation_disposition: CreationDisposition,
    template: TM,
) -> OsResult<FileHandle>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
        TM: Into<Option<&'a mut FileHandle>>,
{
    unsafe {
        CreateFileA(
            file_name.as_ptr(),
            desired_access.bits,
            share_mode.bits,
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            creation_disposition.bits,
            0,
            template.into().map_or(null_mut(), |x| x.0),
        ).and_then(|x| Ok(FileHandle::new(x)))
    }
}

#[unicode_fn]
pub fn create_file_w<'a, SA, TM>(
    file_name: &WStr,
    desired_access: FileAccessRights,
    share_mode: ShareModes,
    sec_attrs: SA,
    creation_disposition: CreationDisposition,
    template: TM,
) -> OsResult<FileHandle>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
        TM: Into<Option<&'a mut FileHandle>>,
{
    unsafe {
        CreateFileW(
            file_name.as_ptr(),
            desired_access.bits,
            share_mode.bits,
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            creation_disposition.bits,
            0,
            template.into().map_or(null_mut(), |x| x.0),
        ).and_then(|x| Ok(FileHandle::new(x)))
    }
}

pub fn read_file<'a, OL, T>(
    handle: &impl ReadableHandle,
    buf: &mut [T],
    overlapped: OL,
) -> OsResult<usize>
    where
        OL: Into<Option<&'a mut Overlapped>> {
    unsafe {
        let mut number_of_bytes_read = 0;
        ReadFile(
            handle.as_c_handle(),
            buf.as_mut_ptr() as *mut _,
            (buf.len() * size_of::<T>()) as u32,
            &mut number_of_bytes_read,
            overlapped.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )?;
        Ok(number_of_bytes_read as usize)
    }
}

pub fn write_file<'a, OL, T>(
    handle: &impl WritableHandle,
    buf: &[T],
    overlapped: OL,
) -> OsResult<usize>
    where
        OL: Into<Option<&'a mut Overlapped>> {
    unsafe {
        let mut number_of_bytes_write = 0;
        WriteFile(
            handle.as_c_handle(),
            buf.as_ptr() as *const _,
            (buf.len() * size_of::<T>()) as u32,
            &mut number_of_bytes_write,
            overlapped.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )?;
        Ok(number_of_bytes_write as usize)
    }
}

#[ansi_fn]
pub fn delete_file_a(
    file_name: &AStr,
) -> OsResult<()> {
    unsafe { DeleteFileA(file_name.as_ptr()) }
}

#[unicode_fn]
pub fn delete_file_w(
    file_name: &WStr,
) -> OsResult<()> {
    unsafe { DeleteFileW(file_name.as_ptr()) }
}

/// Returns `(file_size_high,file_size_low)` .
pub fn get_file_size(
    file_handle: &FileHandle,
) -> OsResult<(DWORD, DWORD)> {
    unsafe {
        let mut sz_high = 0;
        // if GetFileSize returns Err(NO_ERROR), the file size is 0xFFFFFFFF (INVALID_FILE_SIZE).
        let sz_low = match GetFileSize(file_handle.as_c_handle(), &mut sz_high) {
            Ok(x) => x,
            Err(Win32(NO_ERROR)) => INVALID_FILE_SIZE,
            Err(x) => return Err(x),
        };
        Ok((sz_high, sz_low))
    }
}

pub fn flush_file_buffers(
    handle: &impl WritableHandle,
) -> OsResult<()> {
    unsafe { FlushFileBuffers(handle.as_c_handle()) }
}
