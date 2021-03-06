// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{
    ctypes::c_int,
    shared::minwindef::{BOOL, DWORD, LPARAM, LPBOOL, LPVOID, LPWORD, UINT},
    um::{
        winnls::LPNLSVERSIONINFO,
        winnt::{LCID, LPCSTR, LPCWCH, LPCWSTR, LPSTR, LPWSTR, PCNZWCH},
    },
};

make_func! {winapi::um::stringapiset,
pub fn CompareStringEx(
    lpLocaleName: LPCWSTR,
    dwCmpFlags: DWORD,
    lpString1: LPCWCH,
    cchCount1: c_int,
    lpString2: LPCWCH,
    cchCount2: c_int,
    lpVersionInformation: LPNLSVERSIONINFO,
    lpReserved: LPVOID,
    lParam: LPARAM,
) -> c_int;0}

make_func! {winapi::um::stringapiset,
pub fn CompareStringOrdinal(
    lpString1: LPCWCH,
    cchCount1: c_int,
    lpString2: LPCWCH,
    cchCount2: c_int,
    bIgnoreCase: BOOL,
) -> c_int;0}

make_func! {winapi::um::stringapiset,
pub fn CompareStringW(
    Locale: LCID,
    dwCmpFlags: DWORD,
    lpString1: PCNZWCH,
    cchCount1: c_int,
    lpString2: PCNZWCH,
    cchCount2: c_int,
) -> c_int;0}

make_func! {winapi::um::stringapiset,
pub fn FoldStringW(
    dwMapFlags: DWORD,
    lpSrcStr: LPCWCH,
    cchSrc: c_int,
    lpDestStr: LPWSTR,
    cchDest: c_int,
) -> c_int;0}

make_func2! {winapi::um::stringapiset,
pub fn GetStringTypeExW(
    Locale: LCID,
    dwInfoType: DWORD,
    lpSrcStr: LPCWCH,
    cchSrc: c_int,
    lpCharType: LPWORD,
) -> BOOL;0}

make_func2! {winapi::um::stringapiset,
pub fn GetStringTypeW(
    dwInfoType: DWORD,
    lpSrcStr: LPCWCH,
    cchSrc: c_int,
    lpCharType: LPWORD,
) -> BOOL;0}

make_func! {winapi::um::stringapiset,
pub fn MultiByteToWideChar(
    CodePage: UINT,
    dwFlags: DWORD,
    lpMultiByteStr: LPCSTR,
    cbMultiByte: c_int,
    lpWideCharStr: LPWSTR,
    cchWideChar: c_int,
) -> c_int;0}

make_func! {winapi::um::stringapiset,
pub fn WideCharToMultiByte(
    CodePage: UINT,
    dwFlags: DWORD,
    lpWideCharStr: LPCWSTR,
    cchWideChar: c_int,
    lpMultiByteStr: LPSTR,
    cbMultiByte: c_int,
    lpDefaultChar: LPCSTR,
    lpUsedDefaultChar: LPBOOL,
) -> c_int;0}
