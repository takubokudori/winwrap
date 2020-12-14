// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::shared::basetsd::{ULONG_PTR, PULONG_PTR};
use winapi::shared::minwindef::{DWORD, LPDWORD, LPVOID, BOOL};
use winapi::shared::ntdef::{ULONG, PULONG, NULL};
use winapi::um::minwinbase::{LPOVERLAPPED, LPOVERLAPPED_ENTRY};
use winapi::um::winnt::HANDLE;

make_func! {winapi::um::ioapiset,
pub fn CreateIoCompletionPort(
    FileHandle: HANDLE,
    ExistingCompletionPort: HANDLE,
    CompletionKey: ULONG_PTR,
    NumberOfConcurrentThreads: DWORD,
) -> HANDLE;NULL}

make_func2! {winapi::um::ioapiset,
pub fn GetQueuedCompletionStatus(
    CompletionPort: HANDLE,
    lpNumberOfBytesTransferred: LPDWORD,
    lpCompletionKey: PULONG_PTR,
    lpOverlapped: *mut LPOVERLAPPED,
    dwMilliseconds: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::ioapiset,
pub fn GetQueuedCompletionStatusEx(
    CompletionPort: HANDLE,
    lpCompletionPortEntries: LPOVERLAPPED_ENTRY,
    ulCount: ULONG,
    ulNumEntriesRemoved: PULONG,
    dwMilliseconds: DWORD,
    fAlertable: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::ioapiset,
pub fn PostQueuedCompletionStatus(
    CompletionPort: HANDLE,
    dwNumberOfBytesTransferred: DWORD,
    dwCompletionKey: ULONG_PTR,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func2! {winapi::um::ioapiset,
pub fn DeviceIoControl(
    hDevice: HANDLE,
    dwIoControlCode: DWORD,
    lpInBuffer: LPVOID,
    nInBufferSize: DWORD,
    lpOutBuffer: LPVOID,
    nOutBufferSize: DWORD,
    lpBytesReturned: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func2! {winapi::um::ioapiset,
pub fn GetOverlappedResult(
    hFile: HANDLE,
    lpOverlapped: LPOVERLAPPED,
    lpNumberOfBytesTransferred: LPDWORD,
    bWait: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::ioapiset,
pub fn CancelIoEx(
    hFile: HANDLE,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func2! {winapi::um::ioapiset,
pub fn CancelIo(
    hFile: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::ioapiset,
pub fn GetOverlappedResultEx(
    hFile: HANDLE,
    lpOverlapped: LPOVERLAPPED,
    lpNumberOfBytesTransferred: LPDWORD,
    dwMilliseconds: DWORD,
    bAlertable: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::ioapiset,
pub fn CancelSynchronousIo(
    hThread: HANDLE,
) -> BOOL;0}
