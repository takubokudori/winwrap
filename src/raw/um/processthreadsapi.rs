use crate::*;
use std::os::raw::c_int;
use winapi::shared::basetsd::{ULONG_PTR, SIZE_T, PSIZE_T, DWORD_PTR, PULONG_PTR};
use winapi::shared::guiddef::LPCGUID;
use winapi::shared::minwindef::{LPVOID, DWORD, BOOL, LPFILETIME, UINT, LPDWORD, PBOOL, LPCVOID, PDWORD};
use winapi::shared::ntdef::{LPCSTR, LPSTR, HANDLE, NULL, LPCWSTR, LPWSTR, PHANDLE, PULONG, PVOID};
use winapi::um::minwinbase::{LPSECURITY_ATTRIBUTES, LPTHREAD_START_ROUTINE, LPCONTEXT};
use winapi::um::processthreadsapi::{LPPROCESS_INFORMATION, LPSTARTUPINFOA, TLS_OUT_OF_INDEXES, LPSTARTUPINFOW, LPPROC_THREAD_ATTRIBUTE_LIST, THREAD_INFORMATION_CLASS, PROCESS_INFORMATION_CLASS};
use winapi::um::winbase::THREAD_PRIORITY_ERROR_RETURN;
use winapi::um::winnt::{PAPCFUNC, CONTEXT, PROCESS_MITIGATION_POLICY, PPROCESSOR_NUMBER};

make_func2! {winapi::um::processthreadsapi,
pub fn QueueUserAPC(
    pfnAPC: PAPCFUNC,
    hThread: HANDLE,
    dwData: ULONG_PTR,
) -> DWORD;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetProcessTimes(
    hProcess: HANDLE,
    lpCreationTime: LPFILETIME,
    lpExitTime: LPFILETIME,
    lpKernelTime: LPFILETIME,
    lpUserTime: LPFILETIME,
) -> BOOL;0}

tp_func! {winapi::um::processthreadsapi,
pub safe fn GetCurrentProcess() -> HANDLE;}

tp_func! {winapi::um::processthreadsapi,
pub safe fn GetCurrentProcessId() -> DWORD;}

tp_func! {winapi::um::processthreadsapi,
pub safe fn ExitProcess(
    uExitCode: UINT,
);}

make_func2! {winapi::um::processthreadsapi,
pub fn TerminateProcess(
    hProcess: HANDLE,
    uExitCode: UINT,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetExitCodeProcess(
    hProcess: HANDLE,
    lpExitCode: LPDWORD,
) -> BOOL;0}

e_make_func2! {winapi::um::processthreadsapi,
pub fn SwitchToThread() -> BOOL;0}

make_func! {winapi::um::processthreadsapi,
pub fn CreateThread(
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    dwStackSize: SIZE_T,
    lpStartAddress: LPTHREAD_START_ROUTINE,
    lpParameter: LPVOID,
    dwCreationFlags: DWORD,
    lpThreadId: LPDWORD,
) -> HANDLE;NULL}

make_func! {winapi::um::processthreadsapi,
pub fn CreateRemoteThread(
    hProcess: HANDLE,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    dwStackSize: SIZE_T,
    lpStartAddress: LPTHREAD_START_ROUTINE,
    lpParameter: LPVOID,
    dwCreationFlags: DWORD,
    lpThreadId: LPDWORD,
) -> HANDLE;NULL}

tp_func! {winapi::um::processthreadsapi,
pub safe fn GetCurrentThread() -> HANDLE;}

tp_func! {winapi::um::processthreadsapi,
pub safe fn GetCurrentThreadId() -> DWORD;}

make_func! {winapi::um::processthreadsapi,
pub fn OpenThread(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    dwThreadId: DWORD,
) -> HANDLE;NULL}

make_func2! {winapi::um::processthreadsapi,
pub fn SetThreadPriority(
    hThread: HANDLE,
    nPriority: c_int,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetThreadPriorityBoost(
    hThread: HANDLE,
    bDisablePriorityBoost: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetThreadPriorityBoost(
    hThread: HANDLE,
    pDisablePriorityBoost: PBOOL,
) -> BOOL;0}

const I32_THREAD_PRIORITY_ERROR_RETURN: i32 = THREAD_PRIORITY_ERROR_RETURN as i32;

make_func! {winapi::um::processthreadsapi,
pub fn GetThreadPriority(
    hThread: HANDLE,
) -> c_int;I32_THREAD_PRIORITY_ERROR_RETURN}

tp_func! {winapi::um::processthreadsapi,
pub safe fn ExitThread(
    dwExitCode: DWORD,
);}

make_func2! {winapi::um::processthreadsapi,
pub fn TerminateThread(
    hThread: HANDLE,
    dwExitCode: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetExitCodeThread(
    hThread: HANDLE,
    lpExitCode: LPDWORD,
) -> BOOL;0}

make_func! {winapi::um::processthreadsapi,
pub fn SuspendThread(
    hThread: HANDLE,
) -> DWORD;DWORD::MAX}

make_func! {winapi::um::processthreadsapi,
pub fn ResumeThread(
    hThread: HANDLE,
) -> DWORD;DWORD::MAX}

make_func! {winapi::um::processthreadsapi,
pub fn TlsAlloc() -> DWORD;TLS_OUT_OF_INDEXES}

make_func! {winapi::um::processthreadsapi,
pub fn TlsGetValue(
    dwTlsIndex: DWORD,
) -> LPVOID;NULL}

make_func2! {winapi::um::processthreadsapi,
pub fn TlsSetValue(
    dwTlsIndex: DWORD,
    lpTlsValue: LPVOID,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn TlsFree(
    dwTlsIndex: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn CreateProcessA(
    lpApplicationName: LPCSTR,
    lpCommandLine: LPSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCSTR,
    lpStartupInfo: LPSTARTUPINFOA,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn CreateProcessW(
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetProcessShutdownParameters(
    dwLevel: DWORD,
    dwFlags: DWORD,
) -> BOOL;0}

make_func! {winapi::um::processthreadsapi,
pub fn GetProcessVersion(
    ProcessId: DWORD,
) -> DWORD;0}

tp_func! {winapi::um::processthreadsapi,
pub fn GetStartupInfoW(
    lpStartupInfo: LPSTARTUPINFOW,
);}
make_func2! {winapi::um::processthreadsapi,
pub fn CreateProcessAsUserW(
    hToken: HANDLE,
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetThreadToken(
    Thread: PHANDLE,
    Token: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn OpenProcessToken(
    ProcessHandle: HANDLE,
    DesiredAccess: DWORD,
    TokenHandle: PHANDLE,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn OpenThreadToken(
    ThreadHandle: HANDLE,
    DesiredAccess: DWORD,
    OpenAsSelf: BOOL,
    TokenHandle: PHANDLE,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetPriorityClass(
    hProcess: HANDLE,
    dwPriorityClass: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetThreadStackGuarantee(
    StackSizeInBytes: PULONG,
) -> BOOL;0}

make_func! {winapi::um::processthreadsapi,
pub fn GetPriorityClass(
    hProcess: HANDLE,
) -> DWORD;0}

make_func2! {winapi::um::processthreadsapi,
pub fn ProcessIdToSessionId(
    dwProcessId: DWORD,
    pSessionId: *mut DWORD,
) -> BOOL;0}

make_func! {winapi::um::processthreadsapi,
pub fn GetProcessId(
    Process: HANDLE,
) -> DWORD;0}

make_func! {winapi::um::processthreadsapi,
pub fn GetThreadId(
    Thread: HANDLE,
) -> DWORD;0}

tp_func! {winapi::um::processthreadsapi,
pub safe fn FlushProcessWriteBuffers();}

make_func! {winapi::um::processthreadsapi,
pub fn GetProcessIdOfThread(
    Thread: HANDLE,
) -> DWORD;0}

make_func2! {winapi::um::processthreadsapi,
pub fn InitializeProcThreadAttributeList(
    lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
    dwAttributeCount: DWORD,
    dwFlags: DWORD,
    lpSize: PSIZE_T,
) -> BOOL;0}

tp_func! {winapi::um::processthreadsapi,
pub fn DeleteProcThreadAttributeList(
    lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
);}

make_func2! {winapi::um::processthreadsapi,
pub fn SetProcessAffinityUpdateMode(
    hProcess: HANDLE,
    dwFlags: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn QueryProcessAffinityUpdateMode(
    hProcess: HANDLE,
    lpdwFlags: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn UpdateProcThreadAttribute(
    lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
    dwFlags: DWORD,
    Attribute: DWORD_PTR,
    lpValue: PVOID,
    cbSize: SIZE_T,
    lpPreviousValue: PVOID,
    lpReturnSize: PSIZE_T,
) -> BOOL;0}

make_func! {winapi::um::processthreadsapi,
pub fn CreateRemoteThreadEx(
    hProcess: HANDLE,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    dwStackSize: SIZE_T,
    lpStartAddress: LPTHREAD_START_ROUTINE,
    lpParameter: LPVOID,
    dwCreationFlags: DWORD,
    lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
    lpThreadId: LPDWORD,
) -> HANDLE;NULL}

tp_func! {winapi::um::processthreadsapi,
pub fn GetCurrentThreadStackLimits(
    LowLimit: PULONG_PTR,
    HighLimit: PULONG_PTR,
);}

make_func2! {winapi::um::processthreadsapi,
pub fn GetThreadContext(
    hThread: HANDLE,
    lpContext: LPCONTEXT,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetThreadContext(
    hThread: HANDLE,
    lpContext: *const CONTEXT,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetProcessMitigationPolicy(
    MitigationPolicy: PROCESS_MITIGATION_POLICY,
    lpBuffer: PVOID,
    dwLength: SIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetProcessMitigationPolicy(
    hProcess: HANDLE,
    MitigationPolicy: PROCESS_MITIGATION_POLICY,
    lpBuffer: PVOID,
    dwLength: SIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn FlushInstructionCache(
    hProcess: HANDLE,
    lpBaseAddress: LPCVOID,
    dwSize: SIZE_T,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetThreadTimes(
    hThread: HANDLE,
    lpCreationTime: LPFILETIME,
    lpExitTime: LPFILETIME,
    lpKernelTime: LPFILETIME,
    lpUserTime: LPFILETIME,
) -> BOOL;0}

make_func! {winapi::um::processthreadsapi,
pub safe fn OpenProcess(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    dwProcessId: DWORD,
) -> HANDLE;NULL}

tp_func! {winapi::um::processthreadsapi,
pub safe fn IsProcessorFeaturePresent(
    ProcessorFeature: DWORD,
) -> BOOL;}

make_func2! {winapi::um::processthreadsapi,
pub fn GetProcessHandleCount(
    hProcess: HANDLE,
    pdwHandleCount: PDWORD,
) -> BOOL;0}

tp_func! {winapi::um::processthreadsapi,
pub safe fn GetCurrentProcessorNumber() -> DWORD;}

make_func2! {winapi::um::processthreadsapi,
pub fn SetThreadIdealProcessorEx(
    hThread: HANDLE,
    lpIdealProcessor: PPROCESSOR_NUMBER,
    lpPreviousIdealProcessor: PPROCESSOR_NUMBER,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetThreadIdealProcessorEx(
    hThread: HANDLE,
    lpIdealProcessor: PPROCESSOR_NUMBER,
) -> BOOL;0}

tp_func! {winapi::um::processthreadsapi,
pub fn GetCurrentProcessorNumberEx(
    ProcNumber: PPROCESSOR_NUMBER,
);}

make_func2! {winapi::um::processthreadsapi,
pub fn GetProcessPriorityBoost(
    hProcess: HANDLE,
    pDisablePriorityBoost: PBOOL,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetProcessPriorityBoost(
    hProcess: HANDLE,
    bDisablePriorityBoost: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetThreadIOPendingFlag(
    hThread: HANDLE,
    lpIOIsPending: PBOOL,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetSystemTimes(
    lpIdleTime: LPFILETIME,
    lpKernelTime: LPFILETIME,
    lpUserTime: LPFILETIME,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetThreadInformation(
    hThread: HANDLE,
    ThreadInformationClass: THREAD_INFORMATION_CLASS,
    ThreadInformation: LPVOID,
    ThreadInformationSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetThreadInformation(
    hThread: HANDLE,
    ThreadInformationClass: THREAD_INFORMATION_CLASS,
    ThreadInformation: LPVOID,
    ThreadInformationSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn IsProcessCritical(
    hProcess: HANDLE,
    Critical: PBOOL,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn SetProtectedPolicy(
    PolicyGuid: LPCGUID,
    PolicyValue: ULONG_PTR,
    OldPolicyValue: PULONG_PTR,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn QueryProtectedPolicy(
    PolicyGuid: LPCGUID,
    PolicyValue: PULONG_PTR,
) -> BOOL;0}

make_func! {winapi::um::processthreadsapi,
pub fn SetThreadIdealProcessor(
    hThread: HANDLE,
    dwIdealProcessor: DWORD,
) -> DWORD;DWORD::MAX}

make_func2! {winapi::um::processthreadsapi,
pub fn SetProcessInformation(
    hProcess: HANDLE,
    ProcessInformationClass: PROCESS_INFORMATION_CLASS,
    ProcessInformation: LPVOID,
    ProcessInformationSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetProcessInformation(
    hProcess: HANDLE,
    ProcessInformationClass: PROCESS_INFORMATION_CLASS,
    ProcessInformation: LPVOID,
    ProcessInformationSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::processthreadsapi,
pub fn GetProcessShutdownParameters(
    lpdwLevel: LPDWORD,
    lpdwFlags: LPDWORD,
) -> BOOL;0}
