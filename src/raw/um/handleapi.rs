// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{
    shared::minwindef::{BOOL, DWORD, LPDWORD, LPHANDLE},
    um::winnt::HANDLE,
};

make_func2! {winapi::um::handleapi,
pub fn CloseHandle(
    hObject: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::handleapi,
pub unsafe fn DuplicateHandle(
    hSourceProcessHandle: HANDLE,
    hSourceHandle: HANDLE,
    hTargetProcessHandle: HANDLE,
    lpTargetHandle: LPHANDLE,
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    dwOptions: DWORD,
) -> BOOL;0}

tp_func! {winapi::um::handleapi,
pub fn CompareObjectHandles(
    hFirstObjectHandle: HANDLE,
    hSecondObjectHandle: HANDLE,
) -> BOOL;}

make_func2! {winapi::um::handleapi,
pub fn GetHandleInformation(
    hObject: HANDLE,
    lpdwFlags: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::handleapi,
pub fn SetHandleInformation(
    hObject: HANDLE,
    dwMask: DWORD,
    dwFlags: DWORD,
) -> BOOL;0}
