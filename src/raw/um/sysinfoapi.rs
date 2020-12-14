// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::shared::minwindef::{LPFILETIME, PDWORD, PBOOL, UINT, LPDWORD, BYTE, BOOL};
use winapi::shared::ntdef::USHORT;
use winapi::um::minwinbase::{LPSYSTEMTIME, SYSTEMTIME};
use winapi::um::sysinfoapi::{LPMEMORYSTATUSEX, LPSYSTEM_INFO, COMPUTER_NAME_FORMAT};
use winapi::um::winnt::{ULONGLONG, LPSTR, LPWSTR, LPCWSTR, LPOSVERSIONINFOA, LPOSVERSIONINFOW, PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, LOGICAL_PROCESSOR_RELATIONSHIP, PVOID, PULONGLONG, HANDLE, PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, LPCSTR};

make_func2! {winapi::um::sysinfoapi,
pub fn GlobalMemoryStatusEx(
    lpBuffer: LPMEMORYSTATUSEX,
) -> BOOL;0}

tp_func! {winapi::um::sysinfoapi,
pub fn GetSystemInfo(
    lpSystemInfo: LPSYSTEM_INFO,
);}

tp_func! {winapi::um::sysinfoapi,
pub fn GetSystemTime(
    lpSystemTime: LPSYSTEMTIME,
);}

tp_func! {winapi::um::sysinfoapi,
pub fn GetSystemTimeAsFileTime(
    lpSystemTimeAsFileTime: LPFILETIME,
);}

tp_func! {winapi::um::sysinfoapi,
pub fn GetLocalTime(
    lpSystemTime: LPSYSTEMTIME,
);}

tp_func! {winapi::um::sysinfoapi,
pub safe fn GetVersion() -> DWORD;}

make_func2! {winapi::um::sysinfoapi,
pub fn SetLocalTime(
    lpSystemTime: *const SYSTEMTIME,
) -> BOOL;0}

tp_func! {winapi::um::sysinfoapi,
pub safe fn GetTickCount() -> DWORD;}

tp_func! {winapi::um::sysinfoapi,
pub safe fn GetTickCount64() -> ULONGLONG;}

make_func2! {winapi::um::sysinfoapi,
pub fn GetSystemTimeAdjustment(
    lpTimeAdjustment: PDWORD,
    lpTimeIncrement: PDWORD,
    lpTimeAdjustmentDisabled: PBOOL,
) -> BOOL;0}

make_func! {winapi::um::sysinfoapi,
pub fn GetSystemDirectoryA(
    lpBuffer: LPSTR,
    uSize: UINT,
) -> UINT;0}

make_func! {winapi::um::sysinfoapi,
pub fn GetSystemDirectoryW(
    lpBuffer: LPWSTR,
    uSize: UINT,
) -> UINT;0}

make_func! {winapi::um::sysinfoapi,
pub fn GetWindowsDirectoryA(
    lpBuffer: LPSTR,
    uSize: UINT,
) -> UINT;0}

make_func! {winapi::um::sysinfoapi,
pub fn GetWindowsDirectoryW(
    lpBuffer: LPWSTR,
    uSize: UINT,
) -> UINT;0}

make_func! {winapi::um::sysinfoapi,
pub fn GetSystemWindowsDirectoryA(
    lpBuffer: LPSTR,
    uSize: UINT,
) -> UINT;0}

make_func! {winapi::um::sysinfoapi,
pub fn GetSystemWindowsDirectoryW(
    lpBuffer: LPWSTR,
    uSize: UINT,
) -> UINT;0}

make_func2! {winapi::um::sysinfoapi,
pub fn GetComputerNameExA(
    NameType: COMPUTER_NAME_FORMAT,
    lpBuffer: LPSTR,
    nSize: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn GetComputerNameExW(
    NameType: COMPUTER_NAME_FORMAT,
    lpBuffer: LPWSTR,
    nSize: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn SetComputerNameExW(
    NameType: COMPUTER_NAME_FORMAT,
    lpBuffer: LPCWSTR,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn SetSystemTime(
    lpSystemTime: *const SYSTEMTIME,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn GetVersionExA(
    lpVersionInformation: LPOSVERSIONINFOA,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn GetVersionExW(
    lpVersionInformation: LPOSVERSIONINFOW,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn GetLogicalProcessorInformation(
    Buffer: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION,
    ReturnedLength: PDWORD,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn GetLogicalProcessorInformationEx(
    RelationshipType: LOGICAL_PROCESSOR_RELATIONSHIP,
    Buffer: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION,
    ReturnedLength: PDWORD,
) -> BOOL;0}

tp_func! {winapi::um::sysinfoapi,
pub fn GetNativeSystemInfo(
    lpSystemInfo: LPSYSTEM_INFO,
);}

tp_func! {winapi::um::sysinfoapi,
pub fn GetSystemTimePreciseAsFileTime(
    lpSystemTimeAsFileTime: LPFILETIME,
);}

make_func2! {winapi::um::sysinfoapi,
pub fn GetProductInfo(
    dwOSMajorVersion: DWORD,
    dwOSMinorVersion: DWORD,
    dwSpMajorVersion: DWORD,
    dwSpMinorVersion: DWORD,
    pdwReturnedProductType: PDWORD,
) -> BOOL;0}

tp_func! {winapi::um::sysinfoapi,
pub fn VerSetConditionMask(
    ConditionMask: ULONGLONG,
    TypeMask: DWORD,
    Condition: BYTE,
) -> ULONGLONG;}

make_func! {winapi::um::sysinfoapi,
pub fn EnumSystemFirmwareTables(
    FirmwareTableProviderSignature: DWORD,
    pFirmwareTableEnumBuffer: PVOID,
    BufferSize: DWORD,
) -> UINT;0}

make_func! {winapi::um::sysinfoapi,
pub fn GetSystemFirmwareTable(
    FirmwareTableProviderSignature: DWORD,
    FirmwareTableID: DWORD,
    pFirmwareTableBuffer: PVOID,
    BufferSize: DWORD,
) -> UINT;0}

make_func2! {winapi::um::sysinfoapi,
/// Undocumented
pub fn DnsHostnameToComputerNameExW(
    Hostname: LPCWSTR,
    ComputerName: LPWSTR,
    nSize: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn GetPhysicallyInstalledSystemMemory(
    TotalMemoryInKilobytes: PULONGLONG,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
/// Undocumented
pub fn SetComputerNameEx2W(
    NameType: COMPUTER_NAME_FORMAT,
    Flags: DWORD,
    lpBuffer: LPCWSTR,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn SetSystemTimeAdjustment(
    dwTimeAdjustment: DWORD,
    bTimeAdjustmentDisabled: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn InstallELAMCertificateInfo(
    ELAMFile: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn GetProcessorSystemCycleTime(
    Group: USHORT,
    Buffer: PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION,
    ReturnedLength: PDWORD,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn SetComputerNameA(
    lpComputerName: LPCSTR,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn SetComputerNameW(
    lpComputerName: LPCWSTR,
) -> BOOL;0}

make_func2! {winapi::um::sysinfoapi,
pub fn SetComputerNameExA(
    NameType: COMPUTER_NAME_FORMAT,
    lpBuffer: LPCSTR,
) -> BOOL;0}
