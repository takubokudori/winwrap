// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{
    shared::{
        basetsd::{SIZE_T, ULONG_PTR},
        minwindef::{DWORD, LPCVOID, LPVOID},
    },
    um::{
        handleapi::INVALID_HANDLE_VALUE,
        tlhelp32::{
            LPHEAPENTRY32, LPHEAPLIST32, LPMODULEENTRY32, LPMODULEENTRY32W,
            LPPROCESSENTRY32, LPPROCESSENTRY32W, LPTHREADENTRY32,
        },
        winnt::HANDLE,
    },
};

make_func! {winapi::um::tlhelp32,
pub fn CreateToolhelp32Snapshot(
    dwFlags: DWORD,
    th32ProcessID: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::tlhelp32,
pub fn Heap32ListFirst(
    hSnapshot: HANDLE,
    lphl: LPHEAPLIST32,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Heap32ListNext(
    hSnapshot: HANDLE,
    lphl: LPHEAPLIST32,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Heap32First(
    lphe: LPHEAPENTRY32,
    th32ProcessID: DWORD,
    th32HeapID: ULONG_PTR,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Heap32Next(
    lphe: LPHEAPENTRY32,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Toolhelp32ReadProcessMemory(
    th32ProcessID: DWORD,
    lpBaseAddress: LPCVOID,
    lpBuffer: LPVOID,
    cbRead: SIZE_T,
    lpNumberOfBytesRead: *mut SIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Process32FirstW(
    hSnapshot: HANDLE,
    lppe: LPPROCESSENTRY32W,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Process32NextW(
    hSnapshot: HANDLE,
    lppe: LPPROCESSENTRY32W,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Process32First(
    hSnapshot: HANDLE,
    lppe: LPPROCESSENTRY32,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Process32Next(
    hSnapshot: HANDLE,
    lppe: LPPROCESSENTRY32,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Thread32First(
    hSnapshot: HANDLE,
    lpte: LPTHREADENTRY32,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Thread32Next(
    hSnapshot: HANDLE,
    lpte: LPTHREADENTRY32,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Module32FirstW(
    hSnapshot: HANDLE,
    lpme: LPMODULEENTRY32W,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Module32NextW(
    hSnapshot: HANDLE,
    lpme: LPMODULEENTRY32W,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Module32First(
    hSnapshot: HANDLE,
    lpme: LPMODULEENTRY32,
) -> BOOL;0}

make_func2! {winapi::um::tlhelp32,
pub fn Module32Next(
    hSnapshot: HANDLE,
    lpme: LPMODULEENTRY32,
) -> BOOL;0}
