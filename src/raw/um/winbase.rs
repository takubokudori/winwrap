// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{
    ctypes::c_int,
    shared::{
        basetsd::{DWORD64, DWORD_PTR, PDWORD_PTR, PSIZE_T, SIZE_T, UINT_PTR},
        minwindef::{
            BOOL, DWORD, FARPROC, FILETIME, HGLOBAL, HLOCAL, LPBOOL, LPCVOID,
            LPDWORD, LPFILETIME, LPVOID, LPWORD, PDWORD, UINT, WORD,
        },
        ntdef::{
            HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR, NULL, PLUID, PULONG, PWSTR,
            ULONG, VOID,
        },
        windef::HWND,
    },
    um::{
        fileapi::STREAM_INFO_LEVELS,
        handleapi::INVALID_HANDLE_VALUE,
        minwinbase::{
            LMEM_INVALID_HANDLE, LPOVERLAPPED, LPOVERLAPPED_COMPLETION_ROUTINE,
            LPSECURITY_ATTRIBUTES,
        },
        winbase::{
            DEP_SYSTEM_POLICY_TYPE, GMEM_INVALID_HANDLE, LPCOMMCONFIG,
            LPCOMMTIMEOUTS, LPDCB, LPFIBER_START_ROUTINE, LPFILE_ID_DESCRIPTOR,
            LPMEMORYSTATUS,
        },
        winnt::{
            LPCH, PBOOLEAN, PCONTEXT, PCWSTR, PIO_COUNTERS, PPERFORMANCE_DATA,
            PSECURE_MEMORY_CACHE_CALLBACK, PSID, PSID_NAME_USE, PVOID,
        },
    },
    vc::vadefs::va_list,
};

const HANDLE_NULL: winapi::shared::ntdef::HANDLE =
    NULL as winapi::shared::ntdef::HANDLE;

make_func2! {winapi::um::winbase,
pub fn GetBinaryTypeA(
    lpApplicationName: LPCSTR,
    lpBinaryType: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetBinaryTypeW(
    lpApplicationName: LPCWSTR,
    lpBinaryType: LPDWORD,
) -> BOOL;0}

make_func! {winapi::um::winbase,
pub fn GetShortPathNameA(
    lpszLongPath: LPCSTR,
    lpszShortPath: LPSTR,
    cchBuffer: DWORD,
) -> DWORD;0}

make_func! {winapi::um::winbase,
pub fn GetLongPathNameTransactedA(
    lpszShortPath: LPCSTR,
    lpszLongPath: LPSTR,
    cchBuffer: DWORD,
    hTransaction: HANDLE,
) -> DWORD;0}

make_func! {winapi::um::winbase,
pub fn GetLongPathNameTransactedW(
    lpszShortPath: LPCWSTR,
    lpszLongPath: LPWSTR,
    cchBuffer: DWORD,
    hTransaction: HANDLE,
) -> DWORD;0}

make_func2! {winapi::um::winbase,
pub fn GetProcessAffinityMask(
    hProcess: HANDLE,
    lpProcessAffinityMask: PDWORD_PTR,
    lpSystemAffinityMask: PDWORD_PTR,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn SetProcessAffinityMask(
    hProcess: HANDLE,
    dwProcessAffinityMask: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetProcessIoCounters(
    hProcess: HANDLE,
    lpIoCounters: PIO_COUNTERS,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetProcessWorkingSetSize(
    hProcess: HANDLE,
    lpMinimumWorkingSetSize: PSIZE_T,
    lpMaximumWorkingSetSize: PSIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn SetProcessWorkingSetSize(
    hProcess: HANDLE,
    dwMinimumWorkingSetSize: SIZE_T,
    dwMaximumWorkingSetSize: SIZE_T,
) -> BOOL;0}

tp_func! {winapi::um::winbase,
pub safe fn FatalExit(
    ExitCode: c_int,
);}

make_func2! {winapi::um::winbase,
pub fn SetEnvironmentStringsA(
    NewEnvironment: LPCH,
) -> BOOL;0}

tp_func! {winapi::um::winbase,
pub fn SwitchToFiber(
    lpFiber: LPVOID,
);}

tp_func! {winapi::um::winbase,
pub fn DeleteFiber(
    lpFiber: LPVOID,
);}

make_func2! {winapi::um::winbase,
pub safe fn ConvertFiberToThread() -> BOOL;0}

make_func! {winapi::um::winbase,
pub fn CreateFiberEx(
    dwStackCommitSize: SIZE_T,
    dwStackReserveSize: SIZE_T,
    dwFlags: DWORD,
    lpStartAddress: LPFIBER_START_ROUTINE,
    lpParameter: LPVOID,
) -> LPVOID;NULL}

make_func! {winapi::um::winbase,
pub fn ConvertThreadToFiberEx(
    lpParameter: LPVOID,
    dwFlags: DWORD,
) -> LPVOID;NULL}

make_func! {winapi::um::winbase,
pub fn CreateFiber(
    dwStackSize: SIZE_T,
    lpStartAddress: LPFIBER_START_ROUTINE,
    lpParameter: LPVOID,
) -> LPVOID;NULL}

make_func! {winapi::um::winbase,
pub fn SetThreadAffinityMask(
    hThread: HANDLE,
    dwThreadAffinityMask: DWORD_PTR,
) -> DWORD_PTR;0}

make_func! {winapi::um::winbase,
pub fn ConvertThreadToFiber(
    lpParameter: LPVOID,
) -> LPVOID;NULL}

tp_func! {winapi::um::winbase,
pub safe fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE; }

make_func2! {winapi::um::winbase,
pub fn GetSystemRegistryQuota(
    pdwQuotaAllowed: PDWORD,
    pdwQuotaUsed: PDWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn FileTimeToDosDateTime(
    lpFileTime: *const FILETIME,
    lpFatDate: LPWORD,
    lpFatTime: LPWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn DosDateTimeToFileTime(
    wFatDate: WORD,
    wFatTime: WORD,
    lpFileTime: LPFILETIME,
) -> BOOL;0}

make_func! {winapi::um::winbase,
pub fn FormatMessageA(
    dwFlags: DWORD,
    lpSource: LPCVOID,
    dwMessageId: DWORD,
    dwLanguageId: DWORD,
    lpBuffer: LPSTR,
    nSize: DWORD,
    Arguments: *mut va_list,
) -> DWORD;0}

make_func! {winapi::um::winbase,
pub fn FormatMessageW(
    dwFlags: DWORD,
    lpSource: LPCVOID,
    dwMessageId: DWORD,
    dwLanguageId: DWORD,
    lpBuffer: LPWSTR,
    nSize: DWORD,
    Arguments: *mut va_list,
) -> DWORD;0}

make_func! {winapi::um::winbase,
pub fn CreateFileMappingA(
    hFile: HANDLE,
    lpFileMappingAttributes: LPSECURITY_ATTRIBUTES,
    flProtect: DWORD,
    dwMaximumSizeHigh: DWORD,
    dwMaximumSizeLow: DWORD,
    lpName: LPCSTR,
) -> HANDLE;HANDLE_NULL}

make_func! {winapi::um::winbase,
pub fn OpenFileMappingA(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    lpName: LPCSTR,
) -> HANDLE;HANDLE_NULL}

make_func2! {winapi::um::winbase,
pub fn CreateHardLinkA(
    lpFileName: LPCSTR,
    lpExistingFileName: LPCSTR,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn CreateHardLinkW(
    lpFileName: LPCWSTR,
    lpExistingFileName: LPCWSTR,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
) -> BOOL;0}

make_func! {winapi::um::winbase,
pub fn FindFirstStreamTransactedW(
    lpFileName: LPCWSTR,
    InfoLevel: STREAM_INFO_LEVELS,
    lpFindStreamData: LPVOID,
    dwFlags: DWORD,
    hTransaction: HANDLE,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func! {winapi::um::winbase,
pub fn FindFirstFileNameTransactedW(
    lpFileName: LPCWSTR,
    dwFlags: DWORD,
    StringLength: LPDWORD,
    LinkName: PWSTR,
    hTransaction: HANDLE,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func! {winapi::um::winbase,
pub fn CreateNamedPipeA(
    lpName: LPCSTR,
    dwOpenMode: DWORD,
    dwPipeMode: DWORD,
    nMaxInstances: DWORD,
    nOutBufferSize: DWORD,
    nInBufferSize: DWORD,
    nDefaultTimeOut: DWORD,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::winbase,
pub fn GetNamedPipeHandleStateA(
    hNamedPipe: HANDLE,
    lpState: LPDWORD,
    lpCurInstances: LPDWORD,
    lpMaxCollectionCount: LPDWORD,
    lpCollectDataTimeout: LPDWORD,
    lpUserName: LPSTR,
    nMaxUserNameSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn CallNamedPipeA(
    lpNamedPipeName: LPCSTR,
    lpInBuffer: LPVOID,
    nInBufferSize: DWORD,
    lpOutBuffer: LPVOID,
    nOutBufferSize: DWORD,
    lpBytesRead: LPDWORD,
    nTimeOut: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn WaitNamedPipeA(
    lpNamedPipeName: LPCSTR,
    nTimeOut: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetNamedPipeClientComputerNameA(
    Pipe: HANDLE,
    ClientComputerName: LPSTR,
    ClientComputerNameLength: ULONG,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetNamedPipeClientProcessId(
    Pipe: HANDLE,
    ClientProcessId: PULONG,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetNamedPipeClientSessionId(
    Pipe: HANDLE,
    ClientSessionId: PULONG,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetNamedPipeServerProcessId(
    Pipe: HANDLE,
    ServerProcessId: PULONG,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetNamedPipeServerSessionId(
    Pipe: HANDLE,
    ServerSessionId: PULONG,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn SetVolumeLabelA(
    lpRootPathName: LPCSTR,
    lpVolumeName: LPCSTR,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn SetVolumeLabelW(
    lpRootPathName: LPCWSTR,
    lpVolumeName: LPCWSTR,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn SetFileBandwidthReservation(
    hFile: HANDLE,
    nPeriodMilliseconds: DWORD,
    nBytesPerPeriod: DWORD,
    bDiscardable: BOOL,
    lpTransferSize: LPDWORD,
    lpNumOutstandingRequests: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn GetFileBandwidthReservation(
    hFile: HANDLE,
    lpPeriodMilliseconds: LPDWORD,
    lpBytesPerPeriod: LPDWORD,
    pDiscardable: LPBOOL,
    lpTransferSize: LPDWORD,
    lpNumOutstandingRequests: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn DeregisterEventSource(
    hEventLog: HANDLE,
) -> BOOL;0}

make_func! {winapi::um::winbase,
pub fn RegisterEventSourceA(
    lpUNCServerName: LPCSTR,
    lpSourceName: LPCSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::winbase,
pub fn RegisterEventSourceW(
    lpUNCServerName: LPCWSTR,
    lpSourceName: LPCWSTR,
) -> HANDLE;NULL}

make_func2! {winapi::um::winbase,
pub fn ReportEventA(
    hEventLog: HANDLE,
    wType: WORD,
    wCategory: WORD,
    dwEventID: DWORD,
    lpUserSid: PSID,
    wNumStrings: WORD,
    dwDataSize: DWORD,
    lpStrings: *mut LPCSTR,
    lpRawData: LPVOID,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn ReportEventW(
    hEventLog: HANDLE,
    wType: WORD,
    wCategory: WORD,
    dwEventID: DWORD,
    lpUserSid: PSID,
    wNumStrings: WORD,
    dwDataSize: DWORD,
    lpStrings: *mut LPCWSTR,
    lpRawData: LPVOID,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn ReadDirectoryChangesW(
    hDirectory: HANDLE,
    lpBuffer: LPVOID,
    nBufferLength: DWORD,
    bWatchSubtree: BOOL,
    dwNotifyFilter: DWORD,
    lpBytesReturned: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
    lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
) -> BOOL;0}

make_func! {winapi::um::winbase,
pub fn MapViewOfFileExNuma(
    hFileMappingObject: HANDLE,
    dwDesiredAccess: DWORD,
    dwFileOffsetHigh: DWORD,
    dwFileOffsetLow: DWORD,
    dwNumberOfBytesToMap: SIZE_T,
    lpBaseAddress: LPVOID,
    nndPreferred: DWORD,
) -> LPVOID;NULL}

tp_func! {winapi::um::winbase,
pub fn IsBadReadPtr(
    lp: *const VOID,
    ucb: UINT_PTR,
) -> BOOL;}

tp_func! {winapi::um::winbase,
pub fn IsBadWritePtr(
    lp: LPVOID,
    ucb: UINT_PTR,
) -> BOOL;}

tp_func! {winapi::um::winbase,
pub fn IsBadHugeReadPtr(
    lp: *const VOID,
    ucb: UINT_PTR,
) -> BOOL;}

tp_func! {winapi::um::winbase,
pub fn IsBadHugeWritePtr(
    lp: LPVOID,
    ucb: UINT_PTR,
) -> BOOL;}

make_func2! {winapi::um::winbase,
pub fn IsBadCodePtr(
    lpfn: FARPROC,
) -> BOOL;0}

tp_func! {winapi::um::winbase,
pub fn IsBadStringPtrA(
    lpsz: LPCSTR,
    ucchMax: UINT_PTR,
) -> BOOL;}

tp_func! {winapi::um::winbase,
pub fn IsBadStringPtrW(
    lpsz: LPCWSTR,
    ucchMax: UINT_PTR,
) -> BOOL;}

make_func2! {winapi::um::winbase,
pub fn LookupAccountSidA(
    lpSystemName: LPCSTR,
    Sid: PSID,
    Name: LPSTR,
    cchName: LPDWORD,
    ReferencedDomainName: LPSTR,
    cchReferencedDomainName: LPDWORD,
    peUse: PSID_NAME_USE,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn LookupAccountSidW(
    lpSystemName: LPCWSTR,
    Sid: PSID,
    Name: LPWSTR,
    cchName: LPDWORD,
    ReferencedDomainName: LPWSTR,
    cchReferencedDomainName: LPDWORD,
    peUse: PSID_NAME_USE,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn LookupAccountNameA(
    lpSystemName: LPCSTR,
    lpAccountName: LPCSTR,
    Sid: PSID,
    cbSid: LPDWORD,
    ReferencedDomainName: LPCSTR,
    cchReferencedDomainName: LPDWORD,
    peUse: PSID_NAME_USE,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn LookupAccountNameW(
    lpSystemName: LPCWSTR,
    lpAccountName: LPCWSTR,
    Sid: PSID,
    cbSid: LPDWORD,
    ReferencedDomainName: LPCWSTR,
    cchReferencedDomainName: LPDWORD,
    peUse: PSID_NAME_USE,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn LookupPrivilegeValueA(
    lpSystemName: LPCSTR,
    lpName: LPCSTR,
    lpLuid: PLUID,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn LookupPrivilegeValueW(
    lpSystemName: LPCWSTR,
    lpName: LPCWSTR,
    lpLuid: PLUID,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn LookupPrivilegeNameA(
    lpSystemName: LPCSTR,
    lpLuid: PLUID,
    lpName: LPSTR,
    cchName: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn LookupPrivilegeNameW(
    lpSystemName: LPCWSTR,
    lpLuid: PLUID,
    lpName: LPWSTR,
    cchName: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn BuildCommDCBA(
    lpDef: LPCSTR,
    lpDCB: LPDCB,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn BuildCommDCBW(
    lpDef: LPCWSTR,
    lpDCB: LPDCB,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn BuildCommDCBAndTimeoutsA(
    lpDef: LPCSTR,
    lpDCB: LPDCB,
    lpCommTimeouts: LPCOMMTIMEOUTS,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn BuildCommDCBAndTimeoutsW(
lpDef: LPCWSTR,
lpDCB: LPDCB,
lpCommTimeouts: LPCOMMTIMEOUTS,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn CommConfigDialogA(
lpszName: LPCSTR,
hWnd: HWND,
lpCC: LPCOMMCONFIG,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn CommConfigDialogW(
lpszName: LPCWSTR,
hWnd: HWND,
lpCC: LPCOMMCONFIG,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn GetDefaultCommConfigA(
lpszName: LPCSTR,
lpCC: LPCOMMCONFIG,
lpdwSize: LPDWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn GetDefaultCommConfigW(
lpszName: LPCWSTR,
lpCC: LPCOMMCONFIG,
lpdwSize: LPDWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn SetDefaultCommConfigA(
lpszName: LPCSTR,
lpCC: LPCOMMCONFIG,
dwSize: DWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn SetDefaultCommConfigW(
lpszName: LPCWSTR,
lpCC: LPCOMMCONFIG,
dwSize: DWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn GetComputerNameA(
lpBuffer: LPSTR,
nSize: LPDWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn GetComputerNameW(
lpBuffer: LPWSTR,
nSize: LPDWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn DnsHostnameToComputerNameA(
Hostname: LPCSTR,
ComputerName: LPCSTR,
nSize: LPDWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn DnsHostnameToComputerNameW(
Hostname: LPCWSTR,
ComputerName: LPWSTR,
nSize: LPDWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn GetUserNameA(
lpBuffer: LPSTR,
pcbBuffer: LPDWORD,
) -> BOOL; 0}

make_func2! {winapi::um::winbase,
pub fn GetUserNameW(
lpBuffer: LPWSTR,
pcbBuffer: LPDWORD,
) -> BOOL; 0}

make_func! {winapi::um::winbase,
pub fn OpenFileById(
    hVolumeHint: HANDLE,
    lpFileId: LPFILE_ID_DESCRIPTOR,
    dwDesiredAccess: DWORD,
    dwShareMode: DWORD,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    dwFlagsAndAttributes: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::winbase,
pub fn CreateSymbolicLinkA(
    lpSymlinkFileName: LPCSTR,
    lpTargetFileName: LPCSTR,
    dwFlags: DWORD,
) -> BOOLEAN;0}

make_func2! {winapi::um::winbase,
pub fn CreateSymbolicLinkW(
    lpSymlinkFileName: LPCWSTR,
    lpTargetFileName: LPCWSTR,
    dwFlags: DWORD,
) -> BOOLEAN;0}

make_func2! {winapi::um::winbase,
pub fn QueryActCtxSettingsW(
    dwFlags: DWORD,
    hActCtx: HANDLE,
    settingsNameSpace: PCWSTR,
    settingName: PCWSTR,
    pvBuffer: PWSTR,
    dwBuffer: SIZE_T,
    pdwWrittenOrRequired: *mut SIZE_T,
) -> BOOL;0}
make_func2! {winapi::um::winbase,
pub fn CreateSymbolicLinkTransactedA(
    lpSymlinkFileName: LPCSTR,
    lpTargetFileName: LPCSTR,
    dwFlags: DWORD,
    hTransaction: HANDLE,
) -> BOOLEAN;0}

make_func2! {winapi::um::winbase,
pub fn CreateSymbolicLinkTransactedW(
    lpSymlinkFileName: LPCWSTR,
    lpTargetFileName: LPCWSTR,
    dwFlags: DWORD,
    hTransaction: HANDLE,
) -> BOOLEAN;0}

make_func2! {winapi::um::winbase,
pub fn ReplacePartitionUnit(
    TargetPartition: PWSTR,
    SparePartition: PWSTR,
    Flags: ULONG,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn AddSecureMemoryCacheCallback(
    pfnCallBack: PSECURE_MEMORY_CACHE_CALLBACK,
) -> BOOL;0}

// TODO: Returns NT_STATUS code
e_make_func2! {winapi::um::winbase,
pub fn RemoveSecureMemoryCacheCallback(
    pfnCallBack: PSECURE_MEMORY_CACHE_CALLBACK,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn CopyContext(
    Destination: PCONTEXT,
    ContextFlags: DWORD,
    Source: PCONTEXT,
) -> BOOL;0}

make_func2! {winapi::um::winbase,
pub fn InitializeContext(
    Buffer: PVOID,
    ContextFlags: DWORD,
    Context: *mut PCONTEXT,
    ContextLength: PDWORD,
) -> BOOL;0}

tp_func! {winapi::um::winbase,
pub safe fn GetEnabledXStateFeatures() -> DWORD64;}

// TODO: Returns NT_STATUS code
e_make_func2! {winapi::um::winbase,
pub fn EnableThreadProfiling(
    ThreadHandle: HANDLE,
    Flags: DWORD,
    HardwareCounters: DWORD64,
    PerformanceDataHandle: *mut HANDLE,
) -> BOOL;0}

// TODO: Returns NT_STATUS code
e_make_func2! {winapi::um::winbase,
pub fn DisableThreadProfiling(
    PerformanceDataHandle: HANDLE,
) -> DWORD;0}

// TODO: Returns NT_STATUS code
e_make_func2! {winapi::um::winbase,
pub fn QueryThreadProfiling(
    ThreadHandle: HANDLE,
    Enabled: PBOOLEAN,
) -> DWORD;0}

// TODO: Returns ERROR_SUCCESS NT_STATUS code
tp_func! {winapi::um::winbase,
pub fn ReadThreadProfilingData(
    PerformanceDataHandle: HANDLE,
    Flags: DWORD,
    PerformanceData: PPERFORMANCE_DATA,
) -> DWORD;}

make_func! {winapi::um::winbase,
pub fn GlobalAlloc(
    uFlags: UINT,
    dwBytes: SIZE_T,
) -> HGLOBAL;NULL}

make_func! {winapi::um::winbase,
pub fn GlobalReAlloc(
    hMem: HGLOBAL,
    dwBytes: SIZE_T,
    uFlags: UINT,
) -> HGLOBAL;NULL}

make_func! {winapi::um::winbase,
pub fn GlobalSize(
    hMem: HGLOBAL,
) -> SIZE_T;0}

make_func! {winapi::um::winbase,
pub fn GlobalFlags(
    hMem: HGLOBAL,
) -> UINT;GMEM_INVALID_HANDLE}

make_func! {winapi::um::winbase,
pub fn GlobalLock(
    hMem: HGLOBAL,
) -> LPVOID;NULL}

make_func! {winapi::um::winbase,
pub fn GlobalHandle(
    pMem: LPCVOID,
) -> HGLOBAL;NULL}

make_func2! {winapi::um::winbase,
pub fn GlobalUnlock(
    hMem: HGLOBAL,
) -> BOOL;0}

#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn GlobalFree(hMem: HGLOBAL) -> OsResult<()> {
    let ret = winapi::um::winbase::GlobalFree(hMem);
    match ret {
        NULL => Ok(()),
        _ => Err(OsError::last_os_error()),
    }
}

/*
/// Undocumented
pub fn GlobalCompact(
    dwMinFree: DWORD,
) -> SIZE_T;

pub fn GlobalFix(
    hMem: HGLOBAL,
);
pub fn GlobalUnfix(
    hMem: HGLOBAL,
);
pub fn GlobalWire(
    hMem: HGLOBAL,
) -> LPVOID;
pub fn GlobalUnWire(
    hMem: HGLOBAL,
) -> BOOL;
 */
tp_func! {winapi::um::winbase,
pub fn GlobalMemoryStatus(
    lpBuffer: LPMEMORYSTATUS,
);}

make_func! {winapi::um::winbase,
pub fn LocalAlloc(
    uFlags: UINT,
    uBytes: SIZE_T,
) -> HLOCAL;NULL}

make_func! {winapi::um::winbase,
pub fn LocalReAlloc(
    hMem: HLOCAL,
    uBytes: SIZE_T,
    uFlags: UINT,
) -> HLOCAL;NULL}

make_func! {winapi::um::winbase,
pub fn LocalLock(
    hMem: HLOCAL,
) -> LPVOID;NULL}

make_func! {winapi::um::winbase,
pub fn LocalHandle(
    pMem: LPCVOID,
) -> HLOCAL;NULL}

make_func2! {winapi::um::winbase,
pub fn LocalUnlock(
    hMem: HLOCAL,
) -> BOOL;0}

make_func! {winapi::um::winbase,
pub fn LocalSize(
    hMem: HLOCAL,
) -> SIZE_T;0}

make_func! {winapi::um::winbase,
pub fn LocalFlags(
    hMem: HLOCAL,
) -> UINT;LMEM_INVALID_HANDLE}

make_func! {winapi::um::winbase,
pub fn LocalFree(
    hMem: HLOCAL,
) -> HLOCAL;NULL}

/*
pub fn LocalShrink(
    hMem: HLOCAL,
    cbNewSize: UINT,
) -> SIZE_T;
pub fn LocalCompact(
    uMinFree: UINT,
) -> SIZE_T;
 */
