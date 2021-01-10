// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
//! # WinWrap
//! Rust-friendly Windows API wrappers
//!
//! # Features
//!
//! - Safe Windows API bindings
//! - Unsafe raw APIs
//! - Unsafe raw APIs wrap only the error handling.
//! - TCHAR and TString support.
//! - By default, TCHAR is WCHAR. If you want to use ANSI, ansi feature on.
//!
//! ```toml
//! [dependencies.winwrap]
//! version = "0.1.0"
//! features = ["ansi"]
//! ```
//!
//! # Examples
//!
//! ```rust
//! use winwrap::{
//!     um::fileapi::*, winapi::shared::winerror::ERROR_NO_MORE_FILES,
//! };
//!
//! fn enumerate_files_w() {
//!     use winwrap::{string::WString, OsError::Win32};
//!     let path = WString::from_str_lossy(r".\*.*");
//!     let (handle, mut data) = find_first_file_w(&path).unwrap();
//!     loop {
//!         println!("name: {:?}", data.get_file_name().to_string_lossy());
//!         println!("\tflags: {:?}", data.file_attributes);
//!         println!(
//!             "\talternate file name: {}",
//!             data.get_alternate_file_name().to_string_lossy()
//!         );
//!         println!("----------------------------");
//!         data = match find_next_file_w(&handle) {
//!             Ok(x) => x,
//!             Err(Win32(ERROR_NO_MORE_FILES)) => {
//!                 println!("All files enumerated!");
//!                 break;
//!             }
//!             Err(x) => panic!("Unknown Error: {}", x),
//!         };
//!     }
//! }
//!
//! fn main() { enumerate_files_w(); }
//! ```
//!
//! # License
//! This software is released under the MIT License, see LICENSE.

#![cfg(windows)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::uninit_assumed_init)]
#![allow(clippy::missing_safety_doc)]

pub use winapi;
pub use windy as string;

pub mod handle;
pub mod prelude;
pub mod raw;
pub mod shared;
pub mod um;
pub mod vc;

#[macro_use]
pub mod macros;

pub use macros::*;

use std::fmt;
use winapi::shared::{
    minwindef::{DWORD, WORD},
    ntdef::{NTSTATUS, ULONG},
    winerror::ERROR_MR_MID_NOT_FOUND,
};
use windy::ConvertError;

#[cfg(feature = "ansi")]
pub type TString = windy::AString;
#[cfg(not(feature = "ansi"))]
pub type TString = windy::WString;

#[cfg(feature = "ansi")]
pub type TStr = windy::AStr;
#[cfg(not(feature = "ansi"))]
pub type TStr = windy::WStr;

pub type OsResult<T> = Result<T, OsError>;

/// Returns QWORD.
#[inline]
#[allow(non_snake_case)]
pub const fn MAKE_QWORD(high: DWORD, low: DWORD) -> u64 {
    (high as u64) << 32 | low as u64
}

/// Returns `(high, low)` DWORD values.
#[inline]
#[allow(non_snake_case)]
pub const fn SEP_QWORD(qw: u64) -> (u32, u32) {
    ((qw >> 32) as u32, (qw & 0xFFFFFFFF) as u32)
}

#[link(name = "ntdll")]
extern "system" {
    fn RtlNtStatusToDosError(status: NTSTATUS) -> ULONG;
}

/// Represents a Windows OS error code.
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OsError {
    /// Represents a system error code (a.k.a. `Win32Error`, `DosError`).
    Win32(ULONG),
    /// Represents a NTSTATUS error code.
    NtStatus(NTSTATUS),
}

impl fmt::Debug for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsError::Win32(win32_err) => {
                let e = std::io::Error::from_raw_os_error(*win32_err as i32);
                f.debug_struct("Win32").field("", &e).finish()
            }
            OsError::NtStatus(nt_status) => {
                match Self::nt_status_to_win32_error(*nt_status) {
                    Some(win32_err) => {
                        let e =
                            std::io::Error::from_raw_os_error(win32_err as i32);
                        f.debug_struct("NtStatus")
                            .field("", &e)
                            .field("NTSTATUS", nt_status)
                            .finish()
                    }
                    None => f
                        .debug_struct("NtStatus")
                        .field("NTSTATUS", nt_status)
                        .finish(),
                }
            }
        }
    }
}

impl PartialEq<std::io::Error> for OsError {
    fn eq(&self, other: &std::io::Error) -> bool {
        let x = self.to_win32_error();
        let y = other.raw_os_error();
        if let (Some(x), Some(y)) = (x, y) {
            x == y as u32
        } else {
            false
        }
    }
}

impl OsError {
    /// Wraps NTSTATUS up OsError
    #[inline]
    pub fn from_nt_status(x: NTSTATUS) -> Self { Self::NtStatus(x) }

    /// Wraps Win32Error (DosError) up OsError.
    #[inline]
    pub fn from_win32_error(x: ULONG) -> Self { Self::Win32(x) }

    pub fn last_os_error() -> Self {
        Self::Win32(crate::raw::um::errhandlingapi::GetLastError())
    }

    /// Converts NTSTATUS to Win32Error.
    #[inline]
    pub fn nt_status_to_win32_error(x: NTSTATUS) -> Option<ULONG> {
        unsafe {
            match RtlNtStatusToDosError(x) {
                ERROR_MR_MID_NOT_FOUND => None,
                x => Some(x),
            }
        }
    }

    /// Gets Win32Error code.
    ///
    /// If NtStatus is contained, Converts Win32Error code.
    #[inline]
    pub fn to_win32_error(&self) -> Option<ULONG> {
        match self {
            Self::Win32(x) => Some(*x),
            Self::NtStatus(x) => Self::nt_status_to_win32_error(*x),
        }
    }

    /// Returns the NTSTATUS error code if `NtStatus` contained.
    #[inline]
    pub fn get_nt_status(&self) -> Option<NTSTATUS> {
        match self {
            Self::Win32(_) => None,
            Self::NtStatus(x) => Some(*x),
        }
    }

    /// Gets the raw error code.
    #[inline]
    pub fn get_error_code(&self) -> u32 {
        match self {
            OsError::Win32(x) => *x,
            OsError::NtStatus(x) => *x as u32,
        }
    }

    /// Converts OsError to std::io::Error.
    #[inline]
    pub fn into_io_error(self) -> std::io::Error { self.into() }
}

/// Converts OsError to std::io::Error.
///
/// Panics if there is no corresponding Win32Error code.
impl Into<std::io::Error> for OsError {
    fn into(self) -> std::io::Error {
        std::io::Error::from_raw_os_error(self.to_win32_error().unwrap() as i32)
    }
}

impl From<std::io::Error> for OsError {
    fn from(x: std::io::Error) -> Self {
        Self::Win32(x.raw_os_error().unwrap() as u32)
    }
}

impl From<windy::ConvertError> for OsError {
    fn from(x: ConvertError) -> Self { Self::Win32(x.to_error_code()) }
}

impl Into<u32> for OsError {
    fn into(self) -> u32 { self.to_win32_error().unwrap() }
}
