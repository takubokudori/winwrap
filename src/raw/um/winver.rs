use winapi::um::winnt::{LPCSTR, LPCWSTR, LPSTR, LPWSTR};
use crate::*;
use winapi::shared::minwindef::{DWORD, LPCVOID, LPVOID, PUINT};
use winapi::ctypes::c_void;
use winapi::STRUCT;

STRUCT!{#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_snake_case)]
struct LANGANDCODEPAGE{
wLanguage:WORD,
wCodePage:WORD,
}}

make_func! {winapi::um::winver,
pub fn GetFileVersionInfoSizeA(
    lptstrFilename: LPCSTR,
    lpdwHandle: *mut DWORD,
) -> DWORD;0}

make_func! {winapi::um::winver,
pub fn GetFileVersionInfoSizeW(
    lptstrFilename: LPCWSTR,
    lpdwHandle: *mut DWORD,
) -> DWORD;0}

make_func2! {winapi::um::winver,
pub fn GetFileVersionInfoA(
    lptstrFilename: LPCSTR,
    dwHandle: DWORD,
    dwLen: DWORD,
    lpData: *mut c_void,
) -> BOOL;0}

make_func2! {winapi::um::winver,
pub fn GetFileVersionInfoW(
    lptstrFilename: LPCWSTR,
    dwHandle: DWORD,
    dwLen: DWORD,
    lpData: *mut c_void,
) -> BOOL;0}

make_func2! {winapi::um::winver,
pub fn VerQueryValueA(
    pBlock: LPCVOID,
    lpSubBlock: LPCSTR,
    lplpBuffer: &mut LPVOID,
    puLen: PUINT,
) -> BOOL;0}

make_func2! {winapi::um::winver,
pub fn VerQueryValueW(
    pBlock: LPCVOID,
    lpSubBlock: LPCWSTR,
    lplpBuffer: &mut LPVOID,
    puLen: PUINT,
) -> BOOL;0}

make_func! {winapi::um::winver,
pub fn VerLanguageNameA(
    wLang: DWORD,
    szLang: LPSTR,
    cchLang: DWORD,
) -> DWORD;0}

make_func! {winapi::um::winver,
pub fn VerLanguageNameW(
    wLang: DWORD,
    szLang: LPWSTR,
    cchLang: DWORD,
) -> DWORD;0}
