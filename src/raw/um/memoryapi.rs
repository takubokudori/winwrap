// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{
    shared::{
        basetsd::{PSIZE_T, SIZE_T, ULONG_PTR},
        minwindef::{BOOL, DWORD, LPCVOID, LPDWORD, LPVOID, PDWORD, UINT},
        ntdef::{NULL, PVOID},
    },
    um::{
        minwinbase::LPSECURITY_ATTRIBUTES,
        winnt::{HANDLE, LPCWSTR, PMEMORY_BASIC_INFORMATION},
    },
};

make_func! {winapi::um::memoryapi,
pub fn VirtualAlloc(
    lpAddress: LPVOID,
    dwSize: SIZE_T,
    flAllocationType: DWORD,
    flProtect: DWORD,
) -> LPVOID;NULL}

make_func2! {winapi::um::memoryapi,
pub fn VirtualProtect(
    lpAddress: LPVOID,
    dwSize: SIZE_T,
    flNewProtect: DWORD,
    lpflOldProtect: PDWORD,
) -> BOOL;0}

make_func2! {winapi::um::memoryapi,
pub fn VirtualFree(
    lpAddress: LPVOID,
    dwSize: SIZE_T,
    dwFreeType: DWORD,
) -> BOOL;0}

make_func! {winapi::um::memoryapi,
pub fn VirtualQuery(
    lpAddress: LPCVOID,
    lpBuffer: PMEMORY_BASIC_INFORMATION,
    dwLength: SIZE_T,
) -> SIZE_T;0}

make_func! {winapi::um::memoryapi,
pub fn VirtualAllocEx(
    hProcess: HANDLE,
    lpAddress: LPVOID,
    dwSize: SIZE_T,
    flAllocationType: DWORD,
    flProtect: DWORD,
) -> LPVOID;NULL}

make_func2! {winapi::um::memoryapi,
pub fn VirtualFreeEx(
    hProcess: HANDLE,
    lpAddress: LPVOID,
    dwSize: SIZE_T,
    dwFreeType: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::memoryapi,
pub fn VirtualProtectEx(
    hProcess: HANDLE,
    lpAddress: LPVOID,
    dwSize: SIZE_T,
    flNewProtect: DWORD,
    lpflOldProtect: PDWORD,
) -> BOOL;0}

make_func! {winapi::um::memoryapi,
pub fn VirtualQueryEx(
    hProcess: HANDLE,
    lpAddress: LPCVOID,
    lpBuffer: PMEMORY_BASIC_INFORMATION,
    dwLength: SIZE_T,
) -> SIZE_T;0}

make_func2! {winapi::um::memoryapi,
pub fn ReadProcessMemory(
    hProcess: HANDLE,
    lpBaseAddress: LPCVOID,
    lpBuffer: LPVOID,
    nSize: SIZE_T,
    lpNumberOfBytesRead: *mut SIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::memoryapi,
pub fn WriteProcessMemory(
    hProcess: HANDLE,
    lpBaseAddress: LPVOID,
    lpBuffer: LPCVOID,
    nSize: SIZE_T,
    lpNumberOfBytesWritten: *mut SIZE_T,
) -> BOOL;0}

make_func! {winapi::um::memoryapi,
pub fn CreateFileMappingW(
    hFile: HANDLE,
    lpFileMappingAttributes: LPSECURITY_ATTRIBUTES,
    flProtect: DWORD,
    dwMaximumSizeHigh: DWORD,
    dwMaximumSizeLow: DWORD,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::memoryapi,
pub fn OpenFileMappingW(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::memoryapi,
pub fn MapViewOfFile(
    hFileMappingObject: HANDLE,
    dwDesiredAccess: DWORD,
    dwFileOffsetHigh: DWORD,
    dwFileOffsetLow: DWORD,
    dwNumberOfBytesToMap: SIZE_T,
) -> LPVOID;NULL}

make_func! {winapi::um::memoryapi,
pub fn MapViewOfFileEx(
    hFileMappingObject: HANDLE,
    dwDesiredAccess: DWORD,
    dwFileOffsetHigh: DWORD,
    dwFileOffsetLow: DWORD,
    dwNumberOfBytesToMap: SIZE_T,
    lpBaseAddress: LPVOID,
) -> LPVOID;NULL}

make_func2! {winapi::um::memoryapi,
pub fn FlushViewOfFile(
    lpBaseAddress: LPCVOID,
    dwNumberOfBytesToFlush: SIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::memoryapi,
pub fn UnmapViewOfFile(
    lpBaseAddress: LPCVOID,
) -> BOOL;0}

tp_func! {winapi::um::memoryapi,
pub safe fn GetLargePageMinimum() -> SIZE_T;}

make_func2! {winapi::um::memoryapi,
pub fn GetProcessWorkingSetSizeEx(
    hProcess: HANDLE,
    lpMinimumWorkingSetSize: PSIZE_T,
    lpMaximumWorkingSetSize: PSIZE_T,
    Flags: PDWORD,
) -> BOOL;0}

make_func2! {winapi::um::memoryapi,
pub fn SetProcessWorkingSetSizeEx(
    hProcess: HANDLE,
    dwMinimumWorkingSetSize: SIZE_T,
    dwMaximumWorkingSetSize: SIZE_T,
    Flags: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::memoryapi,
pub fn VirtualLock(
    lpAddress: LPVOID,
    dwSize: SIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::memoryapi,
pub fn VirtualUnlock(
    lpAddress: LPVOID,
    dwSize: SIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::memoryapi,
pub fn GetWriteWatch(
    dwFlags: DWORD,
    lpBaseAddress: PVOID,
    dwRegionSize: SIZE_T,
    lpAddresses: *mut PVOID,
    lpdwCount: *mut ULONG_PTR,
    lpdwGranularity: LPDWORD,
) -> UINT;UINT::MAX}

make_func2! {winapi::um::memoryapi,
pub fn ResetWriteWatch(
    lpBaseAddress: LPVOID,
    dwRegionSize: SIZE_T,
) -> UINT;UINT::MAX}
