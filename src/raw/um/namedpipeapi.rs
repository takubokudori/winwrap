use crate::*;
use winapi::shared::minwindef::{DWORD, LPDWORD, LPVOID};
use winapi::shared::ntdef::{HANDLE, PHANDLE, LPCWSTR, LPWSTR, ULONG};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES};

make_func2! {winapi::um::namedpipeapi,
pub fn CreatePipe(
    hReadPipe: PHANDLE,
    hWritePipe: PHANDLE,
    lpPipeAttributes: LPSECURITY_ATTRIBUTES,
    nSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn ConnectNamedPipe(
    hNamedPipe: HANDLE,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn DisconnectNamedPipe(
    hNamedPipe: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn SetNamedPipeHandleState(
    hNamedPipe: HANDLE,
    lpMode: LPDWORD,
    lpMaxCollectionCount: LPDWORD,
    lpCollectDataTimeout: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn PeekNamedPipe(
    hNamedPipe: HANDLE,
    lpBuffer: LPVOID,
    nBufferSize: DWORD,
    lpBytesRead: LPDWORD,
    lpTotalBytesAvail: LPDWORD,
    lpBytesLeftThisMessage: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn TransactNamedPipe(
    hNamedPipe: HANDLE,
    lpInBuffer: LPVOID,
    nInBufferSize: DWORD,
    lpOutBuffer: LPVOID,
    nOutBufferSize: DWORD,
    lpBytesRead: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func! {winapi::um::namedpipeapi,
pub fn CreateNamedPipeW(
    lpName: LPCWSTR,
    dwOpenMode: DWORD,
    dwPipeMode: DWORD,
    nMaxInstances: DWORD,
    nOutBufferSize: DWORD,
    nInBufferSize: DWORD,
    nDefaultTimeOut: DWORD,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::namedpipeapi,
pub fn WaitNamedPipeW(
    lpNamedPipeName: LPCWSTR,
    nTimeOut: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn GetNamedPipeClientComputerNameW(
    Pipe: HANDLE,
    ClientComputerName: LPWSTR,
    ClientComputerNameLength: ULONG,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn ImpersonateNamedPipeClient(
    hNamedPipe: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn GetNamedPipeInfo(
    hNamedPipe: HANDLE,
    lpFlags: LPDWORD,
    lpOutBufferSize: LPDWORD,
    lpInBufferSize: LPDWORD,
    lpMaxInstances: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn GetNamedPipeHandleStateW(
    hNamedPipe: HANDLE,
    lpState: LPDWORD,
    lpCurInstances: LPDWORD,
    lpMaxCollectionCount: LPDWORD,
    lpCollectDataTimeout: LPDWORD,
    lpUserName: LPWSTR,
    nMaxUserNameSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::namedpipeapi,
pub fn CallNamedPipeW(
    lpNamedPipeName: LPCWSTR,
    lpInBuffer: LPVOID,
    nInBufferSize: DWORD,
    lpOutBuffer: LPVOID,
    nOutBufferSize: DWORD,
    lpBytesRead: LPDWORD,
    nTimeOut: DWORD,
) -> BOOL;0}
