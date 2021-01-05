// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::{
    handle::*, raw::um::errhandlingapi::*, string::*,
    um::winnt::ExceptionPointers, vc::excpt::ExceptionHandler, *,
};
use winapi::{
    shared::minwindef::{DWORD, UINT},
    um::winnt::PVECTORED_EXCEPTION_HANDLER,
};
use winwrap_derive::*;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ErrorMode {
    FailCriticalErrors = 0x0001,
    NoAlignmentFaultExcept = 0x0004,
    NoGPFaultErrorBox = 0x0002,
    NoOpenFileErrorBox = 0x8000,
}

impl From<UINT> for ErrorMode {
    fn from(x: u32) -> Self {
        match x {
            0x0001 => Self::FailCriticalErrors,
            0x0004 => Self::NoAlignmentFaultExcept,
            0x0002 => Self::NoGPFaultErrorBox,
            0x8000 => Self::NoOpenFileErrorBox,
            x => panic!("Unknown ErrorMode: {}", x),
        }
    }
}

pub fn get_last_error() -> DWORD { GetLastError() }

pub fn set_last_error(err_code: DWORD) { SetLastError(err_code) }

pub fn get_error_mode() -> ErrorMode { ErrorMode::from(GetErrorMode()) }

pub fn set_error_mode(err_mode: ErrorMode) -> ErrorMode {
    ErrorMode::from(SetErrorMode(err_mode as u32))
}

#[ansi_fn]
pub fn fatal_app_exit_a(action: UINT, message_text: &AStr) {
    unsafe { FatalAppExitA(action, message_text.as_ptr()) }
}

#[unicode_fn]
pub fn fatal_app_exit_w(action: UINT, message_text: &WStr) {
    unsafe { FatalAppExitW(action, message_text.as_ptr()) }
}

pub fn get_thread_error_mode() -> ErrorMode {
    ErrorMode::from(GetThreadErrorMode())
}

pub fn set_thread_error_mode(err_mode: ErrorMode) -> OsResult<ErrorMode> {
    unsafe {
        let mut old_mode = 0;
        SetThreadErrorMode(err_mode as u32, &mut old_mode)?;
        Ok(ErrorMode::from(old_mode))
    }
}

/// Represents VECTORED_EXCEPTION_HANDLER function.
pub type VectoredExceptionHandler =
    extern "system" fn(&mut ExceptionPointers) -> ExceptionHandler;

pub fn add_vectored_exception_handler(
    is_first: bool,
    func: VectoredExceptionHandler,
) -> Result<VEHHandle, ()> {
    unsafe {
        let func = std::mem::transmute::<_, PVECTORED_EXCEPTION_HANDLER>(func);
        Ok(VEHHandle::new(AddVectoredExceptionHandler(
            is_first.into(),
            func,
        )?))
    }
}

pub unsafe fn remove_vectored_exception_handler(
    handle: VEHHandle,
) -> Result<(), ()> {
    RemoveVectoredExceptionHandler(handle.as_c_handle())?;
    std::mem::forget(handle);
    Ok(())
}

pub fn add_vectored_continue_handler(
    is_first: bool,
    func: VectoredExceptionHandler,
) -> Result<VCHHandle, ()> {
    unsafe {
        let func = std::mem::transmute::<_, PVECTORED_EXCEPTION_HANDLER>(func);
        Ok(VCHHandle::new(AddVectoredContinueHandler(
            is_first.into(),
            func,
        )?))
    }
}

pub unsafe fn remove_vectored_continue_handler(
    handle: VCHHandle,
) -> Result<(), ()> {
    RemoveVectoredContinueHandler(handle.as_c_handle())?;
    std::mem::forget(handle);
    Ok(())
}
