// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::{handle::*, raw::um::winuser::*, string::*, *};
use std::ptr::null_mut;
use winapi::shared::minwindef::{BOOL, LPARAM, UINT, WPARAM};
use winwrap_derive::*;

#[ansi_fn]
pub fn find_window_a<'a, CN, WN>(
    class_name: CN,
    window_name: WN,
) -> OsResult<HWnd>
where
    CN: Into<Option<&'a AStr>>,
    WN: Into<Option<&'a AStr>>,
{
    unsafe {
        FindWindowA(
            class_name.into().map_or(null_mut(), |x| x.as_ptr()),
            window_name.into().map_or(null_mut(), |x| x.as_ptr()),
        )
        .map(HWnd::new)
    }
}

#[unicode_fn]
pub fn find_window_w<'a, CN, WN>(
    class_name: CN,
    window_name: WN,
) -> OsResult<HWnd>
where
    CN: Into<Option<&'a WStr>>,
    WN: Into<Option<&'a WStr>>,
{
    unsafe {
        FindWindowW(
            class_name.into().map_or(null_mut(), |x| x.as_ptr()),
            window_name.into().map_or(null_mut(), |x| x.as_ptr()),
        )
        .map(HWnd::new)
    }
}

#[ansi_fn]
pub fn find_window_ex_a<'a, HP, HC, WN>(
    hwnd_parent: HP,
    hwnd_child_after: HC,
    class: &AStr,
    window: WN,
) -> OsResult<HWnd>
where
    HP: Into<Option<&'a HWnd>>,
    HC: Into<Option<&'a HWnd>>,
    WN: Into<Option<&'a AStr>>,
{
    unsafe {
        FindWindowExA(
            hwnd_parent.into().map_or(null_mut(), |x| x.as_c_hwnd()),
            hwnd_child_after
                .into()
                .map_or(null_mut(), |x| x.as_c_hwnd()),
            class.as_ptr(),
            window.into().map_or(null_mut(), |x| x.as_ptr()),
        )
        .map(HWnd::new)
    }
}

#[unicode_fn]
pub fn find_window_ex_w<'a, HP, HC, WN>(
    hwnd_parent: HP,
    hwnd_child_after: HC,
    class: &WStr,
    window: WN,
) -> OsResult<HWnd>
where
    HP: Into<Option<&'a HWnd>>,
    HC: Into<Option<&'a HWnd>>,
    WN: Into<Option<&'a WStr>>,
{
    unsafe {
        FindWindowExW(
            hwnd_parent.into().map_or(null_mut(), |x| x.as_c_hwnd()),
            hwnd_child_after
                .into()
                .map_or(null_mut(), |x| x.as_c_hwnd()),
            class.as_ptr(),
            window.into().map_or(null_mut(), |x| x.as_ptr()),
        )
        .map(HWnd::new)
    }
}

#[ansi_fn]
pub fn post_message_a(
    hwnd: &HWnd,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> OsResult<()> {
    unsafe { PostMessageA(hwnd.as_c_hwnd(), msg, wparam, lparam) }
}

#[unicode_fn]
pub fn post_message_w(
    hwnd: &HWnd,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> OsResult<()> {
    unsafe { PostMessageW(hwnd.as_c_hwnd(), msg, wparam, lparam) }
}

bitflags::bitflags! {
pub struct MBType: DWORD{
    /// MB_ABORTRETRYIGNORE
    const ABORTRETRYIGNORE=winapi::um::winuser::MB_ABORTRETRYIGNORE;
    /// MB_CANCELTRYCONTINUE
    const CANCELTRYCONTINUE=winapi::um::winuser::MB_CANCELTRYCONTINUE;
    /// MB_HELP
    const HELP=winapi::um::winuser::MB_HELP;
    /// MB_OK
    const OK=winapi::um::winuser::MB_OK;
    /// MB_OKCANCEL
    const OKCANCEL=winapi::um::winuser::MB_OKCANCEL;
    /// MB_RETRYCANCEL
    const RETRYCANCEL=winapi::um::winuser::MB_RETRYCANCEL;
    /// MB_YESNO
    const YESNO=winapi::um::winuser::MB_YESNO;
    /// MB_YESNOCANCEL
    const YESNOCANCEL=winapi::um::winuser::MB_YESNOCANCEL;
    }
}

#[ansi_fn]
pub fn message_box_a<'a, HP>(
    hwnd: HP,
    text: &AStr,
    caption: &AStr,
    ty: MBType,
) -> OsResult<()>
where
    HP: Into<Option<&'a HWnd>>,
{
    unsafe {
        MessageBoxA(
            hwnd.into().map_or(null_mut(), |x| x.as_c_hwnd()),
            text.as_ptr(),
            caption.as_ptr(),
            ty.bits(),
        )
    }
}

#[unicode_fn]
pub fn message_box_w<'a, HP>(
    hwnd: HP,
    text: &WStr,
    caption: &WStr,
    ty: MBType,
) -> OsResult<()>
where
    HP: Into<Option<&'a HWnd>>,
{
    unsafe {
        MessageBoxW(
            hwnd.into().map_or(null_mut(), |x| x.as_c_hwnd()),
            text.as_ptr(),
            caption.as_ptr(),
            ty.bits(),
        )
    }
}

pub fn get_parent(hwnd: &HWnd) -> OsResult<HWnd> {
    unsafe { GetParent(hwnd.as_c_hwnd()).map(HWnd::new) }
}

bitflags::bitflags! {
pub struct GAFlags: DWORD{
    /// GA_PARENT
    const PARENT = winapi::um::winuser::GA_PARENT;
    /// GA_ROOT
    const ROOT = winapi::um::winuser::GA_ROOT;
    /// GA_ROOTOWNER
    const ROOTOWNER = winapi::um::winuser::GA_ROOTOWNER;
}}

pub fn get_ancestor(hwnd: &HWnd, flags: GAFlags) -> OsResult<HWnd> {
    unsafe { GetAncestor(hwnd.as_c_hwnd(), flags.bits()).map(HWnd::new) }
}

pub type EnumWindowsProc =
    extern "system" fn(hwnd: &mut HWnd, lparam: LPARAM) -> BOOL;
use crate::winapi::shared::windef::HWND;
pub type EnumWindowsProc2 =
    unsafe extern "system" fn(hwnd: HWND, lparam: LPARAM) -> BOOL;

pub fn enum_windows(
    callback: EnumWindowsProc2,
    lparam: LPARAM,
) -> OsResult<()> {
    unsafe { EnumWindows(Some(callback), lparam) }
}
