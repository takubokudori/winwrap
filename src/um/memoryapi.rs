// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use crate::handle::*;
use crate::raw::um::memoryapi::*;
use crate::string::*;
use crate::um::minwinbase::SecurityAttributes;
use std::mem::size_of;
use std::ops;
use std::ptr::null_mut;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{DWORD, LPCVOID, LPVOID};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winwrap_derive::*;

bitflags::bitflags! {
    pub struct FileMappingAccessRights: u32 {
        /// FILE_MAP_ALL_ACCESS
        const ALL_ACCESS = winapi::um::memoryapi::FILE_MAP_ALL_ACCESS;
        /// FILE_MAP_EXECUTE
        const EXECUTE = winapi::um::memoryapi::FILE_MAP_EXECUTE;
        /// FILE_MAP_READ
        const READ = winapi::um::memoryapi::FILE_MAP_READ;
        /// FILE_MAP_WRITE
        const WRITE = winapi::um::memoryapi::FILE_MAP_WRITE;
        /// FILE_MAP_COPY
        const COPY = winapi::um::memoryapi::FILE_MAP_COPY;
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum FileMapProtectRight {
    EXECUTE_READ = 0x20,
    EXECUTE_READWRITE = 0x40,
    EXECUTE_WRITECOPY = 0x80,
    READONLY = 0x02,
    READWRITE = 0x04,
    WRITECOPY = 0x08,
}

impl From<u32> for FileMapProtectRight {
    fn from(x: u32) -> Self {
        match x {
            winapi::um::winnt::PAGE_EXECUTE_READ => Self::EXECUTE_READ,
            winapi::um::winnt::PAGE_EXECUTE_READWRITE => Self::EXECUTE_READWRITE,
            winapi::um::winnt::PAGE_EXECUTE_WRITECOPY => Self::EXECUTE_WRITECOPY,
            winapi::um::winnt::PAGE_READONLY => Self::READONLY,
            winapi::um::winnt::PAGE_READWRITE => Self::READWRITE,
            winapi::um::winnt::PAGE_WRITECOPY => Self::WRITECOPY,
            x => panic!("Unknown protect right: {}", x),
        }
    }
}

impl Into<u32> for FileMapProtectRight {
    fn into(self) -> u32 { self as u32 }
}

bitflags::bitflags! {
    pub struct FileMapProtectOptions: u32 {
        /// SEC_COMMIT
        const COMMIT=winapi::um::winnt::SEC_COMMIT;
        /// SEC_IMAGE
        const IMAGE=winapi::um::winnt::SEC_IMAGE;
        /// SEC_IMAGE_NO_EXECUTE
        const IMAGE_NO_EXECUTE=winapi::um::winnt::SEC_IMAGE_NO_EXECUTE;
        /// SEC_LARGE_PAGES
        const LARGE_PAGES=winapi::um::winnt::SEC_LARGE_PAGES;
        /// SEC_NOCACHE
        const NOCACHE=winapi::um::winnt::SEC_NOCACHE;
        /// SEC_RESERVE
        const RESERVE=winapi::um::winnt::SEC_RESERVE;
        /// SEC_WRITECOMBINE
        const WRITECOMBINE=winapi::um::winnt::SEC_WRITECOMBINE;
    }
}

#[unicode_fn]
pub fn create_file_mapping_w<'a, FH, SA>(
    file_handle: FH,
    sec_attrs: SA,
    protect_right: FileMapProtectRight,
    protect_options: FileMapProtectOptions,
    maximum_size: u64,
    name: &WStr,
) -> OsResult<FileMappingHandle>
    where
        FH: Into<Option<&'a mut FileHandle>>,
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
{
    unsafe {
        let (maximum_size_high, maximum_size_low) = SEP_QWORD(maximum_size);
        CreateFileMappingW(
            file_handle.into().map_or(INVALID_HANDLE_VALUE, |x| x.as_c_handle()),
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            protect_right as u32 | protect_options.bits,
            maximum_size_high,
            maximum_size_low,
            name.as_ptr(),
        ).and_then(|x| Ok(FileMappingHandle::new(x)))
    }
}


#[unicode_fn]
pub fn open_file_mapping_w(
    desired_access: FileMappingAccessRights,
    is_inherit_handle: bool,
    name: &WStr,
) -> OsResult<FileMappingHandle> {
    unsafe {
        OpenFileMappingW(
            desired_access.bits,
            is_inherit_handle.into(),
            name.as_ptr(),
        ).and_then(|x| Ok(FileMappingHandle::new(x)))
    }
}

bitflags::bitflags! {
    pub struct MapViewAccessRights: u32 {
        /// FILE_MAP_ALL_ACCESS
        const ALL_ACCESS = winapi::um::memoryapi::FILE_MAP_ALL_ACCESS;
        /// FILE_MAP_READ
        const READ = winapi::um::memoryapi::FILE_MAP_READ;
        /// FILE_MAP_WRITE
        const WRITE = winapi::um::memoryapi::FILE_MAP_WRITE;
        /// FILE_MAP_COPY
        const COPY = winapi::um::memoryapi::FILE_MAP_COPY;
        /// FILE_MAP_EXECUTE
        const EXECUTE = winapi::um::memoryapi::FILE_MAP_EXECUTE;
        /// FILE_MAP_LARGE_PAGES
        const LARGE_PAGES = winapi::um::memoryapi::FILE_MAP_LARGE_PAGES;
        /// FILE_MAP_TARGETS_INVALID
        const TARGETS_INVALID = winapi::um::memoryapi::FILE_MAP_TARGETS_INVALID;
    }
}

pub fn map_view_of_file(
    file_mapping_handle: &FileMappingHandle,
    desired_access: MapViewAccessRights,
    file_offset: u64,
    number_of_bytes_to_map: SIZE_T,
) -> OsResult<LPVOID> {
    unsafe {
        let (file_offset_high, file_offset_low) = SEP_QWORD(file_offset);
        MapViewOfFile(
            file_mapping_handle.as_c_handle(),
            desired_access.bits,
            file_offset_high,
            file_offset_low,
            number_of_bytes_to_map,
        )
    }
}

pub unsafe fn unmap_view_of_file(
    base_address: LPCVOID,
) -> OsResult<()> {
    UnmapViewOfFile(base_address)
}

pub unsafe fn flush_view_of_file(
    base_address: LPCVOID,
    number_of_bytes_to_flush: SIZE_T,
) -> OsResult<()> {
    FlushViewOfFile(
        base_address,
        number_of_bytes_to_flush,
    )
}

pub fn read_process_memory<T>(
    handle: &ProcessHandle,
    address: *const T,
    buffer: &mut [u8],
) -> OsResult<usize> {
    let mut number_of_bytes_read = 0;
    unsafe {
        ReadProcessMemory(
            handle.as_c_handle(),
            address as *const _,
            buffer.as_mut_ptr() as *mut _,
            buffer.len(),
            &mut number_of_bytes_read,
        )?;
    }
    Ok(number_of_bytes_read)
}

pub unsafe fn write_process_memory<T, U>(
    handle: &ProcessHandle,
    address: *mut T,
    buffer: &[U],
) -> OsResult<usize> {
    let mut number_of_bytes_write = 0;
    WriteProcessMemory(
        handle.as_c_handle(),
        address as *mut _,
        buffer.as_ptr() as *const _,
        buffer.len() * size_of::<U>(),
        &mut number_of_bytes_write,
    )?;
    Ok(number_of_bytes_write)
}

bitflags::bitflags! {
    pub struct VirtualFreeType: DWORD{
        /// MEM_DECOMMIT
        const DECOMMIT              = winapi::um::winnt::MEM_DECOMMIT;
        /// MEM_RELEASE
        const RELEASE               = winapi::um::winnt::MEM_RELEASE;
        /// MEM_COALESCE_PLACEHOLDERS
        const COALESCE_PLACEHOLDERS = 0x1;
        /// MEM_PRESERVE_PLACEHOLDERS
        const PRESERVE_PLACEHOLDERS = 0x2;
    }
}

/// Represents a region of pages in the virtual address space created by VirtualAlloc or VirtualAllocEx.
pub struct VirtualMemory<'a> {
    /// For VirtualAllocEx.
    proc_handle: Option<&'a ProcessHandle>,
    inner: &'a mut [u8],
    free_type: VirtualFreeType,
}

impl<'a> VirtualMemory<'a> {
    pub unsafe fn from_raw<T>(proc_handle: Option<&'a ProcessHandle>, x: *mut T, size: usize) -> Self {
        Self {
            proc_handle,
            inner: std::slice::from_raw_parts_mut(x as *mut u8, size),
            free_type: VirtualFreeType::DECOMMIT,
        }
    }

    pub fn set_free_type(&mut self, free_type: VirtualFreeType) {
        self.free_type = free_type;
    }
}

impl<'a> Drop for VirtualMemory<'a> {
    fn drop(&mut self) {
        unsafe {
            debug_assert_eq!(
                match self.proc_handle {
                    Some(handle) => virtual_free_ex(handle, self.inner, self.free_type),
                    None => virtual_free(self.inner, self.free_type)
                }
                , Ok(()));
        }
    }
}

impl<'a> ops::Deref for VirtualMemory<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target { self.inner }
}

impl<'a> ops::DerefMut for VirtualMemory<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.inner }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MemAllocationFlags {
    /// MEM_COMMIT
    COMMIT = 0x1000,
    /// MEM_RESERVE
    RESERVE = 0x2000,
    /// MEM_RESET
    RESET = 0x80000,
    /// MEM_RESET_UNDO
    RESET_UNDO = 0x1000000,
}

bitflags::bitflags! {
pub struct MemOptionFlags: u32 {
    /// MEM_LARGE_PAGES
    const LARGE_PAGES = winapi::um::winnt::MEM_LARGE_PAGES;
    /// MEM_PHYSICAL
    const PHYSICAL    = winapi::um::winnt::MEM_PHYSICAL;
    /// MEM_TOP_DOWN
    const TOP_DOWN    = winapi::um::winnt::MEM_TOP_DOWN;
}}

bitflags::bitflags! {
pub struct MemProtectionFlags: u32{
    /// PAGE_EXECUTE
    const EXECUTE = winapi::um::winnt::PAGE_EXECUTE;
    /// PAGE_EXECUTE_READ
    const EXECUTE_READ = winapi::um::winnt::PAGE_EXECUTE_READ;
    /// PAGE_EXECUTE_READWRITE
    const EXECUTE_READWRITE = winapi::um::winnt::PAGE_EXECUTE_READWRITE;
    /// PAGE_EXECUTE_WRITECOPY
    const EXECUTE_WRITECOPY = winapi::um::winnt::PAGE_EXECUTE_WRITECOPY;
    /// PAGE_NOACCESS
    const NOACCESS = winapi::um::winnt::PAGE_NOACCESS;
    /// PAGE_READONLY
    const READONLY = winapi::um::winnt::PAGE_READONLY;
    /// PAGE_READWRITE
    const READWRITE = winapi::um::winnt::PAGE_READWRITE;
    /// PAGE_WRITECOPY
    const WRITECOPY = winapi::um::winnt::PAGE_WRITECOPY;
    /// PAGE_TARGETS_INVALID
    const TARGETS_INVALID = winapi::um::winnt::PAGE_TARGETS_INVALID;
    /// PAGE_TARGETS_NO_UPDATE
    const TARGETS_NO_UPDATE = winapi::um::winnt::PAGE_TARGETS_NO_UPDATE;
    /// PAGE_GUARD
    const GUARD = winapi::um::winnt::PAGE_GUARD;
    /// PAGE_NOCACHE
    const NOCACHE = winapi::um::winnt::PAGE_NOCACHE;
    /// PAGE_WRITECOMBINE
    const WRITECOMBINE = winapi::um::winnt::PAGE_WRITECOMBINE;
    /// PAGE_ENCLAVE_THREAD_CONTROL
    const ENCLAVE_THREAD_CONTROL = winapi::um::winnt::PAGE_ENCLAVE_THREAD_CONTROL;
    /// PAGE_ENCLAVE_UNVALIDATED
    const ENCLAVE_UNVALIDATED = winapi::um::winnt::PAGE_ENCLAVE_UNVALIDATED;
}}

pub unsafe fn virtual_alloc<'a, AD, MO>(
    address: AD,
    size: SIZE_T,
    allocate_type: MemAllocationFlags,
    option_type: MO,
    protect: MemProtectionFlags,
) -> OsResult<VirtualMemory<'a>>
    where
        AD: Into<Option<usize>>,
        MO: Into<Option<MemProtectionFlags>>
{
    let p = VirtualAlloc(
        address.into().map_or(null_mut(), |x| x as LPVOID),
        size,
        allocate_type as u32 | option_type.into().map_or(0, |x| x.bits),
        protect.bits,
    )?;
    Ok(VirtualMemory::from_raw(None, p, size))
}

pub unsafe fn virtual_alloc_ex<AD, MO>(
    proc_handle: &ProcessHandle,
    address: AD,
    size: SIZE_T,
    allocate_type: MemAllocationFlags,
    option_type: MO,
    protect: MemProtectionFlags,
) -> OsResult<VirtualMemory>
    where
        AD: Into<Option<usize>>,
        MO: Into<Option<MemOptionFlags>>
{
    let p = VirtualAllocEx(
        proc_handle.as_c_handle(),
        address.into().map_or(null_mut(), |x| x as LPVOID),
        size,
        allocate_type as u32 | option_type.into().map_or(0, |x| x.bits),
        protect.bits,
    )?;
    Ok(VirtualMemory::from_raw(Some(proc_handle), p, size))
}

pub unsafe fn virtual_free(
    vp: &[u8],
    free_type: VirtualFreeType,
) -> OsResult<()> {
    VirtualFree(
        vp.as_ptr() as *mut _,
        if free_type.contains(VirtualFreeType::RELEASE) { vp.len() } else { 0 },
        free_type.bits,
    )
}

pub unsafe fn virtual_free_ex<'a>(
    proc_handle: &'a ProcessHandle,
    vp: &'a [u8],
    free_type: VirtualFreeType,
) -> OsResult<()> {
    VirtualFreeEx(
        proc_handle.as_c_handle(),
        vp.as_ptr() as *mut _,
        if free_type.contains(VirtualFreeType::RELEASE) { vp.len() } else { 0 },
        free_type.bits,
    )
}
