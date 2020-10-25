use crate::raw::um::fileapi::FindClose;
use crate::raw::um::handleapi::CloseHandle;
use std::fmt;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HKEY, HMODULE};
use winapi::shared::windef::HWND;
use winapi::um::errhandlingapi::{RemoveVectoredExceptionHandler, RemoveVectoredContinueHandler};
use winapi::um::winnt::HANDLE;
use winwrap_derive::*;

#[macro_export]
macro_rules! make_handle {
    ($(#[$outer:meta])*
    pub struct $name:ident ; $($close_func:ident)?) => (
        #[repr(C)]
        $(#[$outer])*
        #[derive(Eq, PartialEq)]
        pub struct $name(pub(crate) winapi::shared::ntdef::HANDLE);

        impl Handle for $name {
            #[inline]
            fn new(handle:HANDLE)->Self{
                Self(handle)
            }

            #[inline]
            fn as_c_handle(&self)->HANDLE{
                self.0
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (self.0 as usize).fmt(f)
            }
        }

        $(
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { let _ = $close_func(self.0); }
            }
        }
        )?

        impl Default for $name {
            fn default() -> Self {
                Self(winapi::um::handleapi::INVALID_HANDLE_VALUE)
            }
        }
    )
}

#[macro_export]
macro_rules! bfi {
    ($x:ident,$t:ty) => ( impl Into<$t> for $x {
        fn into(self) -> $t {
            self.bits
        }
    })
}

pub trait Handle {
    fn new(handle: HANDLE) -> Self;
    fn as_c_handle(&self) -> HANDLE;
}

/// Can `ReadFile()`.
pub trait ReadableHandle: Handle {
    fn hello() {}
}

/// Can `WriteFile()`.
pub trait WritableHandle: Handle {}

/// Can `WaitForSingleObject()`.
pub trait WaitableHandle: Handle {}

/// Can `CancelIo()`.
pub trait CancelableIoHandle: Handle {}

/// Can `DuplicateHandle()`.
pub trait DuplicatableHandle: Handle {}

make_handle! {
#[derive(ReadableHandle, WritableHandle, WaitableHandle, CancelableIoHandle, DuplicatableHandle)]
pub struct PipeHandle; CloseHandle}

make_handle! {pub struct FindFileHandle; FindClose}

pub trait HKeyType {
    fn as_c_hkey(&self) -> HKEY;
}

/// Represents HKEY handle wrapper.
#[repr(C)]
pub struct HKey(pub(crate) HKEY);

impl fmt::Debug for HKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0 as usize).fmt(f)
    }
}

impl HKey {
    pub fn new(hkey: HKEY) -> Self { Self(hkey) }
}

impl HKeyType for HKey {
    fn as_c_hkey(&self) -> HKEY { self.0 }
}

impl Drop for HKey {
    fn drop(&mut self) {
        let _ = crate::raw::um::winreg::RegCloseKey(self.0);
    }
}

make_handle! {
#[derive(ReadableHandle, WritableHandle, WaitableHandle, CancelableIoHandle, DuplicatableHandle)]
pub struct FileHandle; CloseHandle }

make_handle! { pub struct FileMappingHandle; CloseHandle }

make_handle! {
#[derive(WaitableHandle, DuplicatableHandle)]
pub struct ProcessHandle; CloseHandle }

make_handle! { pub struct TokenHandle; CloseHandle }

make_handle! {
#[derive(WaitableHandle, DuplicatableHandle)]
pub struct ThreadHandle; CloseHandle }

make_handle! { pub struct StdHandle; CloseHandle }

make_handle! {
#[derive(WaitableHandle, DuplicatableHandle)]
pub struct EventHandle; CloseHandle }

make_handle! { pub struct MutexHandle; CloseHandle }

make_handle! { pub struct JobHandle; CloseHandle }

make_handle! {
/// Vectored Exception Handler handle
pub struct VEHHandle; RemoveVectoredExceptionHandler }

make_handle! {
/// Vectored Continue Handler handle
pub struct VCHHandle; RemoveVectoredContinueHandler }

pub struct HModule(pub(crate) HMODULE);

impl fmt::Debug for HModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0 as usize).fmt(f)
    }
}

impl HModule {
    pub fn new(hmodule: HMODULE) -> Self { Self(hmodule) }
    pub fn as_c_hmodule(&self) -> HMODULE { self.0 }
}

impl Default for HModule {
    fn default() -> Self { Self(null_mut()) }
}

pub struct HWnd(pub(crate) HWND);

impl fmt::Debug for HWnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0 as usize).fmt(f)
    }
}

impl HWnd {
    pub fn new(hwnd: HWND) -> Self { Self(hwnd) }
    pub fn as_c_hwnd(&self) -> HWND { self.0 }
}

make_handle! { pub struct SnapshotHandle; CloseHandle }
