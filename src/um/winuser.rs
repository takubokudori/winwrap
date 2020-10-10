use crate::*;
use crate::handle::*;
use crate::raw::um::winuser::*;
use crate::string::*;
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM};
use winwrap_derive::*;
use std::ptr::null_mut;

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
        ).and_then(|x| Ok(HWnd::new(x)))
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
        ).and_then(|x| Ok(HWnd::new(x)))
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
            hwnd_child_after.into().map_or(null_mut(), |x| x.as_c_hwnd()),
            class.as_ptr(),
            window.into().map_or(null_mut(), |x| x.as_ptr()),
        ).and_then(|x| Ok(HWnd::new(x)))
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
            hwnd_child_after.into().map_or(null_mut(), |x| x.as_c_hwnd()),
            class.as_ptr(),
            window.into().map_or(null_mut(), |x| x.as_ptr()),
        ).and_then(|x| Ok(HWnd::new(x)))
    }
}

#[ansi_fn]
pub fn post_message_a(
    hwnd: HWnd,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> OsResult<()> {
    unsafe {
        PostMessageA(
            hwnd.as_c_hwnd(),
            msg,
            wparam,
            lparam,
        )
    }
}

#[unicode_fn]
pub fn post_message_w(
    hwnd: HWnd,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> OsResult<()> {
    unsafe {
        PostMessageW(
            hwnd.as_c_hwnd(),
            msg,
            wparam,
            lparam,
        )
    }
}
