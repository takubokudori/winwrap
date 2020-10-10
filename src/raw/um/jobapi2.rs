use crate::*;
use winapi::shared::minwindef::{DWORD, BOOL, UINT, LPVOID, LPDWORD};
use winapi::shared::ntdef::NULL;
use winapi::shared::ntdef::{HANDLE, LPCWSTR, VOID, PCWSTR, ULONG};
use winapi::um::jobapi2::JOBOBJECT_IO_RATE_CONTROL_INFORMATION;
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::winnt::JOBOBJECTINFOCLASS;

make_func! {winapi::um::jobapi2,
pub fn CreateJobObjectW(
    lpJobAttributes: LPSECURITY_ATTRIBUTES,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

tp_func! {winapi::um::jobapi2,
pub fn FreeMemoryJobObject(
    Buffer: *mut VOID,
) -> (); }

make_func! {winapi::um::jobapi2,
pub fn OpenJobObjectW(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func2! {winapi::um::jobapi2,
pub fn AssignProcessToJobObject(
    hJob: HANDLE,
    hProcess: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::jobapi2,
pub fn TerminateJobObject(
    hJob: HANDLE,
    uExitCode: UINT,
) -> BOOL;0}

make_func2! {winapi::um::jobapi2,
pub fn SetInformationJobObject(
    hJob: HANDLE,
    JobObjectInformationClass: JOBOBJECTINFOCLASS,
    lpJobObjectInformation: LPVOID,
    cbJovObjectInformationLength: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::jobapi2,
pub fn SetIoRateControlInformationJobObject(
    hJob: HANDLE,
    IoRateControlInfo: *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION,
) -> DWORD;0}

make_func2! {winapi::um::jobapi2,
pub fn QueryInformationJobObject(
    hJob: HANDLE,
    JobObjectInformationClass: JOBOBJECTINFOCLASS,
    lpJobObjectInformation: LPVOID,
    cbJovObjectInformationLength: DWORD,
    lpReturnLength: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::jobapi2,
pub fn QueryIoRateControlInformationJobObject(
    hJob: HANDLE,
    VolumeName: PCWSTR,
    InfoBlocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION,
    InfoBlockCount: *mut ULONG,
) -> DWORD;0}
