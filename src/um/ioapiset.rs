// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::{handle::*, raw::um::ioapiset::*, um::minwinbase::Overlapped, *};
use std::ptr::null_mut;

pub fn device_io_control<'a, OL>(
    handle: &FileHandle,
    io_ctrl_code: DWORD,
    input_buffer: &mut [u8],
    output_buffer: &mut [u8],
    overlapped: OL,
) -> OsResult<DWORD>
where
    OL: Into<Option<&'a mut Overlapped>>,
{
    unsafe {
        let mut ret = 0;
        DeviceIoControl(
            handle.as_c_handle(),
            io_ctrl_code,
            input_buffer.as_mut_ptr() as *mut _,
            input_buffer.len() as u32,
            output_buffer.as_mut_ptr() as *mut _,
            output_buffer.len() as u32,
            &mut ret,
            overlapped.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )?;
        Ok(ret)
    }
}

pub fn get_overlapped_result(
    handle: &impl ReadableHandle,
    overlapped: &mut Overlapped,
    is_wait: bool,
) -> OsResult<DWORD> {
    unsafe {
        let mut number_of_bytes_transferred = 0;
        GetOverlappedResult(
            handle.as_c_handle(),
            overlapped.as_mut_c_ptr(),
            &mut number_of_bytes_transferred,
            is_wait.into(),
        )?;
        Ok(number_of_bytes_transferred)
    }
}

pub fn cancel_io(handle: &impl CancelableIoHandle) -> OsResult<()> {
    unsafe { CancelIo(handle.as_c_handle()) }
}

pub fn cancel_io_ex<'a, OL>(
    handle: &impl CancelableIoHandle,
    overlapped: OL,
) -> OsResult<()>
where
    OL: Into<Option<&'a mut Overlapped>>,
{
    unsafe {
        CancelIoEx(
            handle.as_c_handle(),
            overlapped.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        )
    }
}

pub fn cancel_synchronous_io(handle: &ThreadHandle) -> OsResult<()> {
    unsafe { CancelSynchronousIo(handle.as_c_handle()) }
}
