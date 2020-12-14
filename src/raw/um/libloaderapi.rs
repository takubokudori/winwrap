// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::ctypes::c_int;
use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::minwindef::{HMODULE, WORD, DWORD, BOOL, HGLOBAL, FARPROC, HRSRC, HINSTANCE, UINT, LPVOID};
use winapi::shared::ntdef::NULL;
use winapi::um::libloaderapi::{DLL_DIRECTORY_COOKIE, ENUMRESLANGPROCA, ENUMRESLANGPROCW, ENUMRESNAMEPROCA, ENUMRESTYPEPROCW, ENUMRESTYPEPROCA, ENUMRESNAMEPROCW};
use winapi::um::winnt::{LPCWSTR, LPSTR, LPWSTR, LPCSTR, HANDLE, PCWSTR, LANGID};

const HRSRC_NULL: winapi::shared::minwindef::HRSRC = NULL as winapi::shared::minwindef::HRSRC;
const HMODULE_NULL: winapi::shared::minwindef::HINSTANCE = NULL as winapi::shared::minwindef::HINSTANCE;
const FARPROC_NULL: *mut winapi::shared::minwindef::__some_function = NULL as *mut winapi::shared::minwindef::__some_function;

make_func2! {winapi::um::libloaderapi,
pub fn DisableThreadLibraryCalls(
    hLibModule: HMODULE,
) -> BOOL;0}

make_func! {winapi::um::libloaderapi,
pub fn FindResourceExW(
    hModule: HMODULE,
    lpName: LPCWSTR,
    lpType: LPCWSTR,
    wLanguage: WORD,
) -> HRSRC;HRSRC_NULL}

make_func! {winapi::um::libloaderapi,
pub fn FindStringOrdinal(
    dwFindStringOrdinalFlags: DWORD,
    lpStringSource: LPCWSTR,
    cchSource: c_int,
    lpStringValue: LPCWSTR,
    cchValue: c_int,
    bIgnoreCase: BOOL,
) -> c_int;-1}

make_func2! {winapi::um::libloaderapi,
pub fn FreeLibrary(
    hLibModule: HMODULE,
) -> BOOL;0}

tp_func! {winapi::um::libloaderapi,
pub fn FreeLibraryAndExitThread(
    hLibModule: HMODULE,
    dwExitCode: DWORD,
);}

// TODO: use GetLastError?
tp_func! {winapi::um::libloaderapi,
pub fn FreeResource(
    hResData: HGLOBAL,
) -> BOOL;}

make_func! {winapi::um::libloaderapi,
pub fn GetModuleFileNameA(
    hModule: HMODULE,
    lpFilename: LPSTR,
    nSize: DWORD,
) -> DWORD;0}

make_func! {winapi::um::libloaderapi,
pub fn GetModuleFileNameW(
    hModule: HMODULE,
    lpFilename: LPWSTR,
    nSize: DWORD,
) -> DWORD;0}

make_func! {winapi::um::libloaderapi,
pub fn GetModuleHandleA(
    lpModuleName: LPCSTR,
) -> HMODULE;HMODULE_NULL}

make_func! {winapi::um::libloaderapi,
pub fn GetModuleHandleW(
    lpModuleName: LPCWSTR,
) -> HMODULE;HMODULE_NULL}

make_func2! {winapi::um::libloaderapi,
pub fn GetModuleHandleExA(
    dwFlags: DWORD,
    lpModuleName: LPCSTR,
    phModule: *mut HMODULE,
) -> BOOL;0}

make_func2! {winapi::um::libloaderapi,
pub fn GetModuleHandleExW(
    dwFlags: DWORD,
    lpModuleName: LPCWSTR,
    phModule: *mut HMODULE,
) -> BOOL;0}

make_func! {winapi::um::libloaderapi,
pub fn GetProcAddress(
    hModule: HMODULE,
    lpProcName: LPCSTR,
) -> FARPROC;FARPROC_NULL}

make_func! {winapi::um::libloaderapi,
pub fn LoadLibraryExA(
    lpLibFileName: LPCSTR,
    hFile: HANDLE,
    dwFlags: DWORD,
) -> HMODULE;HMODULE_NULL}

make_func! {winapi::um::libloaderapi,
pub fn LoadLibraryExW(
    lpLibFileName: LPCWSTR,
    hFile: HANDLE,
    dwFlags: DWORD,
) -> HMODULE;HMODULE_NULL}

make_func! {winapi::um::libloaderapi,
pub fn LoadResource(
    hModule: HMODULE,
    hResInfo: HRSRC,
) -> HGLOBAL;NULL}

make_func! {winapi::um::libloaderapi,
pub fn LoadStringA(
    hInstance: HINSTANCE,
    uID: UINT,
    lpBuffer: LPSTR,
    cchBufferMax: c_int,
) -> c_int;0}

make_func! {winapi::um::libloaderapi,
pub fn LoadStringW(
    hInstance: HINSTANCE,
    uID: UINT,
    lpBuffer: LPWSTR,
    cchBufferMax: c_int,
) -> c_int;0}

make_func! {winapi::um::libloaderapi,
pub fn LockResource(
    hResData: HGLOBAL,
) -> LPVOID;NULL}

make_func! {winapi::um::libloaderapi,
pub fn SizeofResource(
    hModule: HMODULE,
    hResInfo: HRSRC,
) -> DWORD;0}

make_func! {winapi::um::libloaderapi,
pub fn AddDllDirectory(
    NewDirectory: PCWSTR,
) -> DLL_DIRECTORY_COOKIE;NULL}

make_func2! {winapi::um::libloaderapi,
pub fn RemoveDllDirectory(
    Cookie: DLL_DIRECTORY_COOKIE,
) -> BOOL;0}

make_func2! {winapi::um::libloaderapi,
pub fn SetDefaultDllDirectories(
    DirectoryFlags: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::libloaderapi,
pub fn EnumResourceLanguagesExA(
    hModule: HMODULE,
    lpType: LPCSTR,
    lpName: LPCSTR,
    lpEnumFunc: ENUMRESLANGPROCA,
    lParam: LONG_PTR,
    dwFlags: DWORD,
    LangId: LANGID,
) -> BOOL;0}

make_func2! {winapi::um::libloaderapi,
pub fn EnumResourceLanguagesExW(
    hModule: HMODULE,
    lpType: LPCWSTR,
    lpName: LPCWSTR,
    lpEnumFunc: ENUMRESLANGPROCW,
    lParam: LONG_PTR,
    dwFlags: DWORD,
    LangId: LANGID,
) -> BOOL;0}

make_func2! {winapi::um::libloaderapi,
pub fn EnumResourceNamesExA(
    hModule: HMODULE,
    lpType: LPCSTR,
    lpEnumFunc: ENUMRESNAMEPROCA,
    lParam: LONG_PTR,
    dwFlags: DWORD,
    LangId: LANGID,
) -> BOOL;0}

make_func2! {winapi::um::libloaderapi,
pub fn EnumResourceNamesExW(
    hModule: HMODULE,
    lpType: LPCWSTR,
    lpEnumFunc: ENUMRESNAMEPROCW,
    lParam: LONG_PTR,
    dwFlags: DWORD,
    LangId: LANGID,
) -> BOOL;0}

make_func2! {winapi::um::libloaderapi,
pub fn EnumResourceTypesExA(
    hModule: HMODULE,
    lpEnumFunc: ENUMRESTYPEPROCA,
    lParam: LONG_PTR,
    dwFlags: DWORD,
    LangId: LANGID,
) -> BOOL;0}

make_func2! {winapi::um::libloaderapi,
pub fn EnumResourceTypesExW(
    hModule: HMODULE,
    lpEnumFunc: ENUMRESTYPEPROCW,
    lParam: LONG_PTR,
    dwFlags: DWORD,
    LangId: LANGID,
) -> BOOL;0}

make_func! {winapi::um::libloaderapi,
pub fn FindResourceW(
    hModule: HMODULE,
    lpName: LPCWSTR,
    lpType: LPCWSTR,
) -> HRSRC;HRSRC_NULL}

make_func! {winapi::um::libloaderapi,
pub fn LoadLibraryA(
    lpFileName: LPCSTR,
) -> HMODULE;HMODULE_NULL}

make_func! {winapi::um::libloaderapi,
pub fn LoadLibraryW(
    lpFileName: LPCWSTR,
) -> HMODULE;HMODULE_NULL}

make_func2! {winapi::um::libloaderapi,
pub fn EnumResourceNamesW(
    hModule: HMODULE,
    lpType: LPCWSTR,
    lpEnumFunc: ENUMRESNAMEPROCW,
    lParam: LONG_PTR,
) -> BOOL;0}
