// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{
    shared::{
        basetsd::SIZE_T,
        minwindef::{BOOL, DWORD, LPLONG, LPVOID, PBOOL},
        ntdef::{HANDLE, LONG, LPCSTR, LPCWSTR, NULL, PVOID, ULONG, VOID},
    },
    um::{
        minwinbase::{
            LPCRITICAL_SECTION, LPSECURITY_ATTRIBUTES, PCRITICAL_SECTION,
            PREASON_CONTEXT,
        },
        synchapi::{
            LPINIT_ONCE, LPSYNCHRONIZATION_BARRIER, PCONDITION_VARIABLE,
            PINIT_ONCE, PINIT_ONCE_FN, PSRWLOCK, PTIMERAPCROUTINE,
        },
        winbase::WAIT_FAILED,
        winnt::LARGE_INTEGER,
    },
};

tp_func! {winapi::um::synchapi,
pub fn InitializeSRWLock(
    SRWLock: PSRWLOCK,
);}

tp_func! {winapi::um::synchapi,
pub fn ReleaseSRWLockExclusive(
    SRWLock: PSRWLOCK,
);}

tp_func! {winapi::um::synchapi,
pub fn ReleaseSRWLockShared(
    SRWLock: PSRWLOCK,
);}

tp_func! {winapi::um::synchapi,
pub fn AcquireSRWLockExclusive(
    SRWLock: PSRWLOCK,
);}

tp_func! {winapi::um::synchapi,
pub fn AcquireSRWLockShared(
    SRWLock: PSRWLOCK,
);}

make_func2! {winapi::um::synchapi,
pub fn TryAcquireSRWLockExclusive(
    SRWLock: PSRWLOCK,
) -> BOOLEAN;0}

make_func2! {winapi::um::synchapi,
pub fn TryAcquireSRWLockShared(
    SRWLock: PSRWLOCK,
) -> BOOLEAN;0}

tp_func! {winapi::um::synchapi,
pub fn InitializeCriticalSection(
    lpCriticalSection: LPCRITICAL_SECTION,
);}

tp_func! {winapi::um::synchapi,
pub fn EnterCriticalSection(
    lpCriticalSection: LPCRITICAL_SECTION,
);}

tp_func! {winapi::um::synchapi,
pub fn LeaveCriticalSection(
    lpCriticalSection: LPCRITICAL_SECTION,
);}

// This function always succeeds and returns a nonzero value.
tp_func! {winapi::um::synchapi,
pub fn InitializeCriticalSectionAndSpinCount(
    lpCriticalSection: LPCRITICAL_SECTION,
    dwSpinCount: DWORD,
) -> BOOL;}

make_func2! {winapi::um::synchapi,
pub fn InitializeCriticalSectionEx(
    lpCriticalSection: LPCRITICAL_SECTION,
    dwSpinCount: DWORD,
    Flags: DWORD,
) -> BOOL;0}

tp_func! {winapi::um::synchapi,
pub fn SetCriticalSectionSpinCount(
    lpCriticalSection: LPCRITICAL_SECTION,
    dwSpinCount: DWORD,
) -> DWORD;}

make_func2! {winapi::um::synchapi,
pub fn TryEnterCriticalSection(
    lpCriticalSection: LPCRITICAL_SECTION,
) -> BOOL;0}

tp_func! {winapi::um::synchapi,
pub fn DeleteCriticalSection(
    lpCriticalSection: LPCRITICAL_SECTION,
);}

tp_func! {winapi::um::synchapi,
pub fn InitOnceInitialize(
    InitOnce: PINIT_ONCE,
);}

make_func2! {winapi::um::synchapi,
pub fn InitOnceExecuteOnce(
    InitOnce: PINIT_ONCE,
    InitFn: PINIT_ONCE_FN,
    Parameter: PVOID,
    Context: *mut LPVOID,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn InitOnceBeginInitialize(
    lpInitOnce: LPINIT_ONCE,
    dwFlags: DWORD,
    fPending: PBOOL,
    lpContext: *mut LPVOID,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn InitOnceComplete(
    lpInitOnce: LPINIT_ONCE,
    dwFlags: DWORD,
    lpContext: LPVOID,
) -> BOOL;0}

tp_func! {winapi::um::synchapi,
pub fn InitializeConditionVariable(
    ConditionVariable: PCONDITION_VARIABLE,
);}

tp_func! {winapi::um::synchapi,
pub fn WakeConditionVariable(
    ConditionVariable: PCONDITION_VARIABLE,
);}

tp_func! {winapi::um::synchapi,
pub fn WakeAllConditionVariable(
    ConditionVariable: PCONDITION_VARIABLE,
);}

make_func2! {winapi::um::synchapi,
pub fn SleepConditionVariableCS(
    ConditionVariable: PCONDITION_VARIABLE,
    CriticalSection: PCRITICAL_SECTION,
    dwMilliseconds: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn SleepConditionVariableSRW(
    ConditionVariable: PCONDITION_VARIABLE,
    SRWLock: PSRWLOCK,
    dwMilliseconds: DWORD,
    Flags: ULONG,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn SetEvent(
    hEvent: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn ResetEvent(
    hEvent: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn ReleaseSemaphore(
    hSemaphore: HANDLE,
    lReleaseCount: LONG,
    lpPreviousCount: LPLONG,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn ReleaseMutex(
    hMutex: HANDLE,
) -> BOOL;0}

make_func! {winapi::um::synchapi,
pub fn WaitForSingleObject(
    hHandle: HANDLE,
    dwMilliseconds: DWORD,
) -> DWORD;WAIT_FAILED}

tp_func! {winapi::um::synchapi,
pub fn SleepEx(
    dwMilliseconds: DWORD,
    bAlertable: BOOL,
) -> DWORD;}

make_func! {winapi::um::synchapi,
pub fn WaitForSingleObjectEx(
    hHandle: HANDLE,
    dwMilliseconds: DWORD,
    bAlertable: BOOL,
) -> DWORD;WAIT_FAILED}

make_func! {winapi::um::synchapi,
pub fn WaitForMultipleObjectsEx(
    nCount: DWORD,
    lpHandles: *const HANDLE,
    bWaitAll: BOOL,
    dwMilliseconds: DWORD,
    bAlertable: BOOL,
) -> DWORD;WAIT_FAILED}

make_func! {winapi::um::synchapi,
pub fn CreateMutexA(
    lpMutexAttributes: LPSECURITY_ATTRIBUTES,
    bInitialOwner: BOOL,
    lpName: LPCSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateMutexW(
    lpMutexAttributes: LPSECURITY_ATTRIBUTES,
    bInitialOwner: BOOL,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn OpenMutexW(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateEventA(
    lpEventAttributes: LPSECURITY_ATTRIBUTES,
    bManualReset: BOOL,
    bInitialState: BOOL,
    lpName: LPCSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateEventW(
    lpEventAttributes: LPSECURITY_ATTRIBUTES,
    bManualReset: BOOL,
    bInitialState: BOOL,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn OpenEventA(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    lpName: LPCSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn OpenEventW(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn OpenSemaphoreW(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn OpenWaitableTimerW(
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    lpTimerName: LPCWSTR,
) -> HANDLE;NULL}

make_func2! {winapi::um::synchapi,
pub fn SetWaitableTimerEx(
    hTimer: HANDLE,
    lpDueTime: *const LARGE_INTEGER,
    lPeriod: LONG,
    pfnCompletionRoutine: PTIMERAPCROUTINE,
    lpArgToCompletionRoutine: LPVOID,
    WakeContext: PREASON_CONTEXT,
    TolerableDelay: ULONG,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn SetWaitableTimer(
    hTimer: HANDLE,
    lpDueTime: *const LARGE_INTEGER,
    lPeriod: LONG,
    pfnCompletionRoutine: PTIMERAPCROUTINE,
    lpArgToCompletionRoutine: LPVOID,
    fResume: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::synchapi,
pub fn CancelWaitableTimer(
    hTimer: HANDLE,
) -> BOOL;0}

make_func! {winapi::um::synchapi,
pub fn CreateMutexExA(
    lpMutexAttributes: LPSECURITY_ATTRIBUTES,
    lpName: LPCSTR,
    dwFlags: DWORD,
    dwDesiredAccess: DWORD,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateMutexExW(
    lpMutexAttributes: LPSECURITY_ATTRIBUTES,
    lpName: LPCWSTR,
    dwFlags: DWORD,
    dwDesiredAccess: DWORD,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateEventExA(
    lpEventAttributes: LPSECURITY_ATTRIBUTES,
    lpName: LPCSTR,
    dwFlags: DWORD,
    dwDesiredAccess: DWORD,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateEventExW(
    lpEventAttributes: LPSECURITY_ATTRIBUTES,
    lpName: LPCWSTR,
    dwFlags: DWORD,
    dwDesiredAccess: DWORD,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateSemaphoreExW(
    lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES,
    lInitialCount: LONG,
    lMaximumCount: LONG,
    lpName: LPCWSTR,
    dwFlags: DWORD,
    dwDesiredAccess: DWORD,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateWaitableTimerExW(
    lpTimerAttributes: LPSECURITY_ATTRIBUTES,
    lpTimerName: LPCWSTR,
    dwFlags: DWORD,
    dwDesiredAccess: DWORD,
) -> HANDLE;NULL}

#[allow(non_snake_case)]
pub unsafe fn EnterSynchronizationBarrier(
    lpBarrier: LPSYNCHRONIZATION_BARRIER,
    dwFlags: DWORD,
) -> Result<(), ()> {
    match winapi::um::synchapi::EnterSynchronizationBarrier(lpBarrier, dwFlags)
    {
        0 => Err(()),
        _ => Ok(()),
    }
}

make_func2! {winapi::um::synchapi,
pub fn InitializeSynchronizationBarrier(
    lpBarrier: LPSYNCHRONIZATION_BARRIER,
    lTotalThreads: LONG,
    lSpinCount: LONG,
) -> BOOL;0}

tp_func! {winapi::um::synchapi,
pub fn DeleteSynchronizationBarrier(
    lpBarrier: LPSYNCHRONIZATION_BARRIER,
) -> BOOL;}

tp_func! {winapi::um::synchapi,
pub safe fn Sleep(
    dwMilliseconds: DWORD,
);}

make_func2! {winapi::um::synchapi,
pub fn WaitOnAddress(
    Address: *mut VOID,
    CompareAddress: PVOID,
    AddressSize: SIZE_T,
    dwMilliseconds: DWORD,
) -> BOOL;0}

tp_func! {winapi::um::synchapi,
pub fn WakeByAddressSingle(
    Address: PVOID,
);}

tp_func! {winapi::um::synchapi,
pub fn WakeByAddressAll(
    Address: PVOID,
);}

// If the function fails, returns 0xFFFFFFFF.
make_func! {winapi::um::synchapi,
pub fn SignalObjectAndWait(
    hObjectToSignal: HANDLE,
    hObjectToWaitOn: HANDLE,
    dwMilliseconds: DWORD,
    bAlertable: BOOL,
) -> DWORD;DWORD::MAX}

make_func! {winapi::um::synchapi,
pub fn WaitForMultipleObjects(
    nCount: DWORD,
    lpHandles: *const HANDLE,
    bWaitAll: BOOL,
    dwMilliseconds: DWORD,
) -> DWORD;WAIT_FAILED}

make_func! {winapi::um::synchapi,
pub fn CreateSemaphoreW(
    lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES,
    lInitialCount: LONG,
    lMaximumCount: LONG,
    lpName: LPCWSTR,
) -> HANDLE;NULL}

make_func! {winapi::um::synchapi,
pub fn CreateWaitableTimerW(
    lpTimerAttributes: LPSECURITY_ATTRIBUTES,
    bManualReset: BOOL,
    lpTimerName: LPCWSTR,
) -> HANDLE;NULL}
