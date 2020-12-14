// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use crate::handle::*;
use crate::raw::um::handleapi::*;

bitflags::bitflags! {
    pub struct HandleFlags: u32 {
        const HANDLE_FLAG_INHERIT = winapi::um::winbase::HANDLE_FLAG_INHERIT;
        const HANDLE_FLAG_PROTECT_FROM_CLOSE = winapi::um::winbase::HANDLE_FLAG_PROTECT_FROM_CLOSE;
    }
}

pub fn duplicate_handle<T: Handle>(
    src_proc_handle: &ProcessHandle,
    src_handle: &impl DuplicatableHandle,
    target_proc_handle: &ProcessHandle,
    desired_access: DWORD,
    is_inherit_handle: bool,
    options: DWORD,
) -> OsResult<T> {
    unsafe {
        let mut handle = 0 as *mut _;
        DuplicateHandle(
            src_proc_handle.as_c_handle(),
            src_handle.as_c_handle(),
            target_proc_handle.as_c_handle(),
            &mut handle,
            desired_access,
            is_inherit_handle.into(),
            options,
        )?;
        Ok(T::new(handle))
    }
}

pub fn compare_object_handles(
    handle1: &impl Handle,
    handle2: &impl Handle,
) -> bool {
    unsafe {
        CompareObjectHandles(
            handle1.as_c_handle(),
            handle2.as_c_handle(),
        ) != 0
    }
}

/// # Example
///
/// ```rust
///
/// fn main(){
///     use winwrap::um::handleapi::get_handle_information;
///     use winwrap::um::processthreadsapi::get_current_process;
///     let proc = get_current_process();
///     let info = get_handle_information(&proc);
///     println!("{:?}", info);
/// }
/// ```
pub fn get_handle_information(
    handle: &impl Handle,
) -> OsResult<HandleFlags> {
    unsafe {
        let mut flags = 0;
        GetHandleInformation(
            handle.as_c_handle(),
            &mut flags)?;
        Ok(HandleFlags::from_bits_truncate(flags))
    }
}

pub fn set_handle_information(
    handle: &impl Handle,
    mask: &HandleFlags,
    flags: &HandleFlags,
) -> OsResult<()> {
    unsafe {
        SetHandleInformation(
            handle.as_c_handle(),
            mask.bits(),
            flags.bits(),
        )
    }
}
