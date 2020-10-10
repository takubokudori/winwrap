use crate::*;
use winapi::shared::minwindef::{BOOL, UINT, LPDWORD, DWORD, LPVOID};
use winapi::shared::ntdef::{HANDLE, VOID};
use winapi::um::wincon::{PCONSOLE_READCONSOLE_CONTROL, PHANDLER_ROUTINE};
use winapi::um::wincontypes::{PINPUT_RECORD, COORD, HPCON};

make_func2! {winapi::um::consoleapi,
pub fn AllocConsole() -> BOOL;0}

make_func! {winapi::um::consoleapi,
pub fn GetConsoleCP() -> UINT;0}

make_func2! {winapi::um::consoleapi,
pub fn GetConsoleMode(
    hConsoleHandle: HANDLE,
    lpMode: LPDWORD,
) -> BOOL;0}

make_func! {winapi::um::consoleapi,
pub fn GetConsoleOutputCP() -> UINT;0}

make_func2! {winapi::um::consoleapi,
pub fn GetNumberOfConsoleInputEvents(
    hConsoleInput: HANDLE,
    lpNumberOfEvents: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn PeekConsoleInputA(
    hConsoleInput: HANDLE,
    lpBuffer: PINPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsRead: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn ReadConsoleA(
    hConsoleInput: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfCharsToRead: DWORD,
    lpNumberOfCharsRead: LPDWORD,
    pInputControl: PCONSOLE_READCONSOLE_CONTROL,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn ReadConsoleW(
    hConsoleInput: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfCharsToRead: DWORD,
    lpNumberOfCharsRead: LPDWORD,
    pInputControl: PCONSOLE_READCONSOLE_CONTROL,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn ReadConsoleInputA(
    hConsoleInput: HANDLE,
    lpBuffer: PINPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsRead: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn ReadConsoleInputW(
    hConsoleInput: HANDLE,
    lpBuffer: PINPUT_RECORD,
    nLength: DWORD,
    lpNumberOfEventsRead: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn SetConsoleCtrlHandler(
    HandlerRoutine: PHANDLER_ROUTINE,
    Add: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn SetConsoleMode(
    hConsoleHandle: HANDLE,
    dwMode: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn WriteConsoleA(
    hConsoleOutput: HANDLE,
    lpBuffer: *const VOID,
    nNumberOfCharsToWrite: DWORD,
    lpNumberOfCharsWritten: LPDWORD,
    lpReserved: LPVOID,
) -> BOOL;0}

make_func2! {winapi::um::consoleapi,
pub fn WriteConsoleW(
    hConsoleOutput: HANDLE,
    lpBuffer: *const VOID,
    nNumberOfCharsToWrite: DWORD,
    lpNumberOfCharsWritten: LPDWORD,
    lpReserved: LPVOID,
) -> BOOL;0}

make_func_hresult! {winapi::um::consoleapi,
pub fn CreatePseudoConsole(
    size: COORD,
    hInput: HANDLE,
    hOutput: HANDLE,
    dwFlags: DWORD,
    phPC: *mut HPCON,
) -> HRESULT;}

make_func_hresult! {winapi::um::consoleapi,
pub fn ResizePseudoConsole(
    hPC: HPCON,
    size: COORD,
) -> HRESULT;}

tp_func! {winapi::um::consoleapi,
pub fn ClosePseudoConsole(
    hPC: HPCON,
);}
