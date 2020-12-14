// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::minwindef::{DWORD, UINT, LPDWORD};
use winapi::shared::ntdef::{LONG, ULONG, PVOID, NULL, LPCSTR, LPCWSTR};
use winapi::um::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER;
use winapi::um::winnt::{EXCEPTION_POINTERS, PVECTORED_EXCEPTION_HANDLER, PEXCEPTION_RECORD, PCONTEXT};

tp_func! {winapi::um::errhandlingapi,
pub unsafe fn RaiseException(
    dwExceptionCode: DWORD,
    dwExceptionFlags: DWORD,
    nNumberOfArguments: DWORD,
    lpArguments: *const ULONG_PTR,
);}

tp_func! {winapi::um::errhandlingapi,
pub unsafe fn UnhandledExceptionFilter(
    ExceptionInfo: *mut EXCEPTION_POINTERS,
) -> LONG;}

tp_func! {winapi::um::errhandlingapi,
pub unsafe fn SetUnhandledExceptionFilter(
    lpTopLevelExceptionFilter: LPTOP_LEVEL_EXCEPTION_FILTER,
) -> LPTOP_LEVEL_EXCEPTION_FILTER;}

tp_func! {winapi::um::errhandlingapi,
pub safe fn GetLastError() -> DWORD;}

tp_func! {winapi::um::errhandlingapi,
pub safe fn SetLastError(
    dwErrCode: DWORD,
);}

tp_func! {winapi::um::errhandlingapi,
pub safe fn GetErrorMode() -> UINT;}

tp_func! {winapi::um::errhandlingapi,
pub safe fn SetErrorMode(
    uMode: UINT,
) -> UINT;}

e_make_func! {winapi::um::errhandlingapi,
pub fn AddVectoredExceptionHandler(
    First: ULONG,
    Handler: PVECTORED_EXCEPTION_HANDLER,
) -> PVOID;NULL}

e_make_func2! {winapi::um::errhandlingapi,
pub fn RemoveVectoredExceptionHandler(
    Handle: PVOID,
) -> ULONG;0}

e_make_func! {winapi::um::errhandlingapi,
pub fn AddVectoredContinueHandler(
    First: ULONG,
    Handler: PVECTORED_EXCEPTION_HANDLER,
) -> PVOID;NULL}

e_make_func2! {winapi::um::errhandlingapi,
pub fn RemoveVectoredContinueHandler(
    Handle: PVOID,
) -> ULONG;0}

tp_func! {winapi::um::errhandlingapi,
pub unsafe fn RaiseFailFastException(
pExceptionRecord: PEXCEPTION_RECORD,
pContextRecord: PCONTEXT,
dwFlags: DWORD,
);}

tp_func! {winapi::um::errhandlingapi,
pub unsafe fn FatalAppExitA(
uAction: UINT,
lpMessageText: LPCSTR,
);}

tp_func! {winapi::um::errhandlingapi,
pub unsafe fn FatalAppExitW(
uAction: UINT,
lpMessageText: LPCWSTR,
); }

tp_func! {winapi::um::errhandlingapi,
pub safe fn GetThreadErrorMode() -> DWORD;}

make_func2! {winapi::um::errhandlingapi,
pub unsafe fn SetThreadErrorMode(
dwNewMode: DWORD,
lpOldMode: LPDWORD,
) -> BOOL;0
}
