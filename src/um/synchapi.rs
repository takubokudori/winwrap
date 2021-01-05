// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::{
    handle::*, raw::um::synchapi::*, string::*,
    um::minwinbase::SecurityAttributes, *,
};
use std::{ptr::null_mut, time::Duration};
use winapi::um::winbase::INFINITE;
use winwrap_derive::*;

#[repr(u32)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum WaitStatus {
    WaitObject0 = 0x00000000,
    WaitAbandoned = 0x00000080,
    WaitTimeout = 0x00000102,
    WaitIoCompletion = 0x00000C0,
}

impl From<u32> for WaitStatus {
    fn from(x: u32) -> Self {
        match x {
            0x00000000 => Self::WaitObject0,
            0x00000080 => Self::WaitAbandoned,
            0x00000102 => Self::WaitTimeout,
            0x000000C0 => Self::WaitIoCompletion,
            x => panic!("Unknown WaitStatus returned: {}", x),
        }
    }
}

/// if duration is None, specified `INFINITE`.
pub fn wait_for_single_object<DU>(
    handle: &impl WaitableHandle,
    duration: DU,
) -> OsResult<WaitStatus>
where
    DU: Into<Option<Duration>>,
{
    unsafe {
        WaitForSingleObject(
            handle.as_c_handle(),
            duration.into().map_or(INFINITE, |x| x.as_millis() as DWORD),
        )
        .map(WaitStatus::from)
    }
}

pub fn wait_for_single_object_ex<DU>(
    handle: &impl WaitableHandle,
    duration: DU,
    is_alertable: bool,
) -> OsResult<WaitStatus>
where
    DU: Into<Option<Duration>>,
{
    unsafe {
        WaitForSingleObjectEx(
            handle.as_c_handle(),
            duration.into().map_or(INFINITE, |x| x.as_millis() as DWORD),
            i32::from(is_alertable),
        )
        .map(WaitStatus::from)
    }
}

#[ansi_fn]
pub fn create_event_a<'a, SA, NA>(
    sec_attrs: SA,
    is_manual_reset: bool,
    is_initial_state: bool,
    name: NA,
) -> OsResult<EventHandle>
where
    SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
    NA: Into<Option<&'a AStr>>,
{
    unsafe {
        CreateEventA(
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            is_manual_reset.into(),
            is_initial_state.into(),
            name.into().map_or(null_mut(), |x| x.as_ptr()),
        )
        .map(EventHandle::new)
    }
}

#[unicode_fn]
pub fn create_event_w<'a, SA, NA>(
    sec_attrs: SA,
    is_manual_reset: bool,
    is_initial_state: bool,
    name: NA,
) -> OsResult<EventHandle>
where
    SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
    NA: Into<Option<&'a WStr>>,
{
    unsafe {
        CreateEventW(
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            is_manual_reset.into(),
            is_initial_state.into(),
            name.into().map_or(null_mut(), |x| x.as_ptr()),
        )
        .map(EventHandle::new)
    }
}

#[ansi_fn]
pub fn create_mutex_a<'a, SA>(
    sec_attrs: SA,
    initial_owner: bool,
    name: &AStr,
) -> OsResult<MutexHandle>
where
    SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
{
    unsafe {
        CreateMutexA(
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            initial_owner.into(),
            name.as_ptr(),
        )
        .map(MutexHandle::new)
    }
}

#[unicode_fn]
pub fn create_mutex_w<'a, SA>(
    sec_attrs: SA,
    initial_owner: bool,
    name: &WStr,
) -> OsResult<MutexHandle>
where
    SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
{
    unsafe {
        CreateMutexW(
            sec_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            initial_owner.into(),
            name.as_ptr(),
        )
        .map(MutexHandle::new)
    }
}

#[inline]
pub fn sleep(msec: DWORD) { Sleep(msec) }
