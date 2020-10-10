use crate::*;
use winapi::shared::minwindef::{PBOOL, BOOL};
use winapi::shared::ntdef::{LPCSTR, LPCWSTR, HANDLE};
use winapi::um::minwinbase::LPDEBUG_EVENT;

tp_func! {winapi::um::debugapi,
pub safe fn IsDebuggerPresent() -> BOOL;}

tp_func! {winapi::um::debugapi,
pub safe fn DebugBreak();}

tp_func! {winapi::um::debugapi,
pub fn OutputDebugStringA(
    lpOutputString: LPCSTR,
);}

tp_func! {winapi::um::debugapi,
pub fn OutputDebugStringW(
    lpOutputString: LPCWSTR,
);}

make_func2! {winapi::um::debugapi,
pub fn ContinueDebugEvent(
    dwProcessId: DWORD,
    dwThreadId: DWORD,
    dwContinueStatus: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::debugapi,
pub fn WaitForDebugEvent(
    lpDebugEvent: LPDEBUG_EVENT,
    dwMilliseconds: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::debugapi,
pub fn DebugActiveProcess(
    dwProcessId: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::debugapi,
pub fn DebugActiveProcessStop(
    dwProcessId: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::debugapi,
pub fn CheckRemoteDebuggerPresent(
    hProcess: HANDLE,
    pbDebuggerPresent: PBOOL,
) -> BOOL;0}

make_func2! {winapi::um::debugapi,
pub fn WaitForDebugEventEx(
    lpDebugEvent: LPDEBUG_EVENT,
    dwMilliseconds: DWORD,
) -> BOOL;0}
