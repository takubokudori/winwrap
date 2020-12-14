// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::shared::basetsd::{PDWORD_PTR, ULONG_PTR};
use winapi::shared::guiddef::LPCGUID;
use winapi::shared::minwindef::{UINT, BOOL, LPARAM, WPARAM, LRESULT, LPVOID};
use winapi::shared::ntdef::NULL;
use winapi::shared::windef::HWND;
use winapi::um::winnt::{LONG, HANDLE, LPCSTR, LPCWSTR};
use winapi::um::winuser::{LPMSG, SENDASYNCPROC, HPOWERNOTIFY, MSG, HDEVNOTIFY, WNDENUMPROC};

const HWND_NULL: winapi::shared::windef::HWND = NULL as winapi::shared::windef::HWND;

make_func2! {winapi::um::winuser,
pub fn GetMessageA(
    lpMsg: LPMSG,
    hWnd: HWND,
    wMsgFilterMin: UINT,
    wMsgFilterMax: UINT,
) -> BOOL;-1}

make_func2! {winapi::um::winuser,
pub fn GetMessageW(
    lpMsg: LPMSG,
    hWnd: HWND,
    wMsgFilterMin: UINT,
    wMsgFilterMax: UINT,
) -> BOOL;-1}

tp_func! {winapi::um::winuser,
pub fn TranslateMessage(
    lpmsg: *const MSG,
) -> BOOL;}

tp_func! {winapi::um::winuser,
pub fn DispatchMessageA(
    lpmsg: *const MSG,
) -> LRESULT;}

tp_func! {winapi::um::winuser,
pub fn DispatchMessageW(
    lpmsg: *const MSG,
) -> LRESULT;}

/*
use winapi::ctypes::c_int;
make_func2! {winapi::um::winuser,
/// Undocumented function. Does nothing.
/// Always returns 1.
pub fn SetMessageQueue(
    cMessagesMax: c_int,
) -> BOOL;0}
 */

tp_func! {winapi::um::winuser,
pub fn PeekMessageA(
    lpMsg: LPMSG,
    hWnd: HWND,
    wMsgFilterMin: UINT,
    wMsgFilterMax: UINT,
    wRemoveMsg: UINT,
) -> BOOL;}

tp_func! {winapi::um::winuser,
pub fn PeekMessageW(
    lpMsg: LPMSG,
    hWnd: HWND,
    wMsgFilterMin: UINT,
    wMsgFilterMax: UINT,
    wRemoveMsg: UINT,
) -> BOOL;}

make_func2! {winapi::um::winuser,
pub fn ExitWindowsEx(
    uFlags: UINT,
    dwReason: DWORD,
) -> BOOL;0}

tp_func! {winapi::um::winuser,
pub fn SwapMouseButton(
    fSwap: BOOL,
) -> BOOL;}

tp_func! {winapi::um::winuser,
pub safe fn GetMessagePos() -> DWORD;}

tp_func! {winapi::um::winuser,
pub safe fn GetMessageTime() -> LONG;}

tp_func! {winapi::um::winuser,
pub safe fn GetMessageExtraInfo() -> LPARAM;}

tp_func! {winapi::um::winuser,
pub safe fn GetUnpredictedMessagePos() -> DWORD;}

tp_func! {winapi::um::winuser,
pub fn IsWow64Message() -> BOOL;}

tp_func! {winapi::um::winuser,
pub fn SetMessageExtraInfo(
    lParam: LPARAM,
) -> LPARAM;}
make_func2! {winapi::um::winuser,
pub fn SendMessageA(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT;0}

make_func2! {winapi::um::winuser,
pub fn SendMessageW(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT;0}

make_func2! {winapi::um::winuser,
pub fn SendMessageTimeoutA(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    fuFlags: UINT,
    uTimeout: UINT,
    lpdwResult: PDWORD_PTR,
) -> LRESULT;0}

make_func2! {winapi::um::winuser,
pub fn SendMessageTimeoutW(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    fuFlags: UINT,
    uTimeout: UINT,
    lpdwResult: PDWORD_PTR,
) -> LRESULT;0}

make_func2! {winapi::um::winuser,
pub fn SendNotifyMessageA(
    hWnd: HWND,
    msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn SendNotifyMessageW(
    hWnd: HWND,
    msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn SendMessageCallbackA(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    lpResultCallBack: SENDASYNCPROC,
    dwData: ULONG_PTR,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn SendMessageCallbackW(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    lpResultCallBack: SENDASYNCPROC,
    dwData: ULONG_PTR,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn RegisterDeviceNotificationA(
    hRecipient: HANDLE,
    notificationFilter: LPVOID,
    flags: DWORD,
) -> HDEVNOTIFY;NULL}

make_func2! {winapi::um::winuser,
pub fn RegisterDeviceNotificationW(
    hRecipient: HANDLE,
    notificationFilter: LPVOID,
    flags: DWORD,
) -> HDEVNOTIFY;NULL}

make_func2! {winapi::um::winuser,
pub fn UnregisterDeviceNotification(
    Handle: HDEVNOTIFY,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn RegisterPowerSettingNotification(
    hRecipient: HANDLE,
    PowerSettingGuid: LPCGUID,
    Flags: DWORD,
) -> HPOWERNOTIFY;NULL}

make_func2! {winapi::um::winuser,
pub fn UnregisterPowerSettingNotification(
    Handle: HPOWERNOTIFY,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn RegisterSuspendResumeNotification(
    hRecipient: HANDLE,
    Flags: DWORD,
) -> HPOWERNOTIFY;NULL}

make_func2! {winapi::um::winuser,
pub fn UnregisterSuspendResumeNotification(
    Handle: HPOWERNOTIFY,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn PostMessageA(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn PostMessageW(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn PostThreadMessageA(
    idThread: DWORD,
    msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn PostThreadMessageW(
    idThread: DWORD,
    msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn MessageBoxA(
    hWnd: HWND,
    lpText: LPCSTR,
    lpCaption: LPCSTR,
    uType: UINT,
) -> c_int;0}

make_func2! {winapi::um::winuser,
pub fn MessageBoxW(
    hWnd: HWND,
    lpText: LPCWSTR,
    lpCaption: LPCWSTR,
    uType: UINT,
) -> c_int;0}

make_func2! {winapi::um::winuser,
pub fn MessageBoxExA(
    hWnd: HWND,
    lpText: LPCSTR,
    lpCaption: LPCSTR,
    uType: UINT,
    wLanguageId: WORD,
) -> c_int;0}

make_func2! {winapi::um::winuser,
pub fn MessageBoxExW(
    hWnd: HWND,
    lpText: LPCWSTR,
    lpCaption: LPCWSTR,
    uType: UINT,
    wLanguageId: WORD,
) -> c_int;0}

make_func2! {winapi::um::winuser,
pub fn GetProcessDefaultLayout(
    pdwDefaultLayout: *mut DWORD,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn SetProcessDefaultLayout(
    dwDefaultLayout: DWORD,
) -> BOOL;0}

tp_func! {winapi::um::winuser,
pub fn GetDesktopWindow() -> HWND;}

make_func! {winapi::um::winuser,
pub fn GetParent(
    hWnd: HWND,
) -> HWND;HWND_NULL}

make_func! {winapi::um::winuser,
pub fn SetParent(
    hWndChild: HWND,
    hWndNewParent: HWND,
) -> HWND;HWND_NULL}

tp_func! {winapi::um::winuser,
/// The return value is not used.
pub fn EnumChildWindows(
    hWndParent: HWND,
    lpEnumFunc: WNDENUMPROC,
    lParam: LPARAM,
) -> BOOL;}

make_func! {winapi::um::winuser,
pub fn FindWindowA(
    lpClassName: LPCSTR,
    lpWindowName: LPCSTR,
) -> HWND;HWND_NULL}

make_func! {winapi::um::winuser,
pub fn FindWindowW(
    lpClassName: LPCWSTR,
    lpWindowName: LPCWSTR,
) -> HWND;HWND_NULL}

make_func! {winapi::um::winuser,
pub fn FindWindowExA(
    hWndParent: HWND,
    hWndChildAfter: HWND,
    lpszClass: LPCSTR,
    lpszWindow: LPCSTR,
) -> HWND;HWND_NULL}

make_func! {winapi::um::winuser,
pub fn FindWindowExW(
    hWndParent: HWND,
    hWndChildAfter: HWND,
    lpszClass: LPCWSTR,
    lpszWindow: LPCWSTR,
) -> HWND;HWND_NULL}

e_make_func! {winapi::um::winuser,
pub safe fn GetShellWindow() -> HWND;HWND_NULL}

e_make_func2! {winapi::um::winuser,
pub fn RegisterShellHookWindow(
    hwnd: HWND,
) -> BOOL;0}

e_make_func2! {winapi::um::winuser,
pub fn DeregisterShellHookWindow(
    hwnd: HWND,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn EnumWindows(
    lpEnumFunc: WNDENUMPROC,
    lParam: LPARAM,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn EnumThreadWindows(
    dwThreadId: DWORD,
    lpfn: WNDENUMPROC,
    lParam: LPARAM,
) -> BOOL;0}

make_func2! {winapi::um::winuser,
pub fn SetForegroundWindow(
    hWnd: HWND,
) -> BOOL;0}
