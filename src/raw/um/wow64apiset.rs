// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::shared::{
    minwindef::{PBOOL, UINT},
    ntdef::{HANDLE, LPSTR, LPWSTR, PUSHORT, PVOID},
};

make_func2! {winapi::um::wow64apiset,
pub fn Wow64DisableWow64FsRedirection(
    OldValue: *mut PVOID,
) -> BOOL;0}

make_func2! {winapi::um::wow64apiset,
pub fn Wow64RevertWow64FsRedirection(
    OlValue: PVOID,
) -> BOOL;0}

make_func2! {winapi::um::wow64apiset,
pub fn IsWow64Process(
    hProcess: HANDLE,
    Wow64Process: PBOOL,
) -> BOOL;0}

make_func! {winapi::um::wow64apiset,
pub fn GetSystemWow64DirectoryA(
    lpBuffer: LPSTR,
    uSize: UINT,
) -> UINT;0}

make_func! {winapi::um::wow64apiset,
pub fn GetSystemWow64DirectoryW(
    lpBuffer: LPWSTR,
    uSize: UINT,
) -> UINT;0}

make_func2! {winapi::um::wow64apiset,
pub fn IsWow64Process2(
    hProcess: HANDLE,
    pProcessMachine: PUSHORT,
    pNativeMachine: PUSHORT,
) -> BOOL;0}
