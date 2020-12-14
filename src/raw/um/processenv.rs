// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::shared::minwindef::{DWORD, BOOL};
use winapi::shared::ntdef::{NULL, LPCH, LPWCH, LPCSTR, LPSTR, HANDLE, PHANDLE, LPWSTR, LPCWSTR};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;

const LPCH_NULL: winapi::shared::ntdef::LPCH = NULL as winapi::shared::ntdef::LPCH;
const LPWCH_NULL: winapi::shared::ntdef::LPWCH = NULL as winapi::shared::ntdef::LPWCH;

make_func! {winapi::um::processenv,
pub fn GetEnvironmentStrings() -> LPCH;LPCH_NULL}

make_func! {winapi::um::processenv,
pub fn GetEnvironmentStringsW() -> LPWCH;LPWCH_NULL}

make_func2! {winapi::um::processenv,
pub fn SetEnvironmentStringsW(
    NewEnvironment: LPWCH,
) -> BOOL;0}

make_func2! {winapi::um::processenv,
pub fn FreeEnvironmentStringsA(
    penv: LPCH,
) -> BOOL;0}

make_func2! {winapi::um::processenv,
pub fn FreeEnvironmentStringsW(
    penv: LPWCH,
) -> BOOL;0}

make_func! {winapi::um::processenv,
pub fn GetStdHandle(
    nStdHandle: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::processenv,
pub fn SetStdHandle(
    nStdHandle: DWORD,
    hHandle: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::processenv,
pub fn SetStdHandleEx(
    nStdHandle: DWORD,
    hHandle: HANDLE,
    phPrevValue: PHANDLE,
) -> BOOL;0}

tp_func! {winapi::um::processenv,
pub safe fn GetCommandLineA() -> LPSTR;}

tp_func! {winapi::um::processenv,
pub safe fn GetCommandLineW() -> LPWSTR;}

make_func! {winapi::um::processenv,
pub fn GetEnvironmentVariableA(
    lpName: LPCSTR,
    lpBuffer: LPSTR,
    nSize: DWORD,
) -> DWORD;0}

make_func! {winapi::um::processenv,
pub fn GetEnvironmentVariableW(
    lpName: LPCWSTR,
    lpBuffer: LPWSTR,
    nSize: DWORD,
) -> DWORD;0}

make_func2! {winapi::um::processenv,
pub fn SetEnvironmentVariableA(
    lpName: LPCSTR,
    lpValue: LPCSTR,
) -> BOOL;0}

make_func2! {winapi::um::processenv,
pub fn SetEnvironmentVariableW(
    lpName: LPCWSTR,
    lpValue: LPCWSTR,
) -> BOOL;0}

make_func! {winapi::um::processenv,
pub fn ExpandEnvironmentStringsA(
    lpSrc: LPCSTR,
    lpDst: LPSTR,
    nSize: DWORD,
) -> DWORD;0}

make_func! {winapi::um::processenv,
pub fn ExpandEnvironmentStringsW(
    lpSrc: LPCWSTR,
    lpDst: LPWSTR,
    nSize: DWORD,
) -> DWORD;0}

make_func2! {winapi::um::processenv,
pub fn SetCurrentDirectoryA(
    lpPathName: LPCSTR,
) -> BOOL;0}

make_func2! {winapi::um::processenv,
pub fn SetCurrentDirectoryW(
    lpPathName: LPCWSTR,
) -> BOOL;0}

make_func! {winapi::um::processenv,
pub fn GetCurrentDirectoryA(
    nBufferLength: DWORD,
    lpBuffer: LPSTR,
) -> DWORD;0}

make_func! {winapi::um::processenv,
pub fn GetCurrentDirectoryW(
    nBufferLength: DWORD,
    lpBuffer: LPWSTR,
) -> DWORD;0}

make_func! {winapi::um::processenv,
pub fn SearchPathW(
    lpPath: LPCWSTR,
    lpFileName: LPCWSTR,
    lpExtension: LPCWSTR,
    nBufferLength: DWORD,
    lpBuffer: LPWSTR,
    lpFilePart: *mut LPWSTR,
) -> DWORD;0}

make_func! {winapi::um::processenv,
pub fn SearchPathA(
    lpPath: LPCSTR,
    lpFileName: LPCSTR,
    lpExtension: LPCSTR,
    nBufferLength: DWORD,
    lpBuffer: LPSTR,
    lpFilePart: *mut LPSTR,
) -> DWORD;0}

tp_func! {winapi::um::processenv,
pub fn NeedCurrentDirectoryForExePathA(
    ExeName: LPCSTR,
) -> BOOL;}
tp_func! {winapi::um::processenv,
pub fn NeedCurrentDirectoryForExePathW(
    ExeName: LPCWSTR,
) -> BOOL;}
