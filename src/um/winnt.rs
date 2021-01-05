// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{
    shared::{
        basetsd::{KAFFINITY, ULONG_PTR},
        minwindef::{BYTE, WORD},
        ntdef::GROUP_AFFINITY,
    },
    um::winnt::{
        CONTEXT, EXCEPTION_MAXIMUM_PARAMETERS, EXCEPTION_POINTERS,
        EXCEPTION_RECORD, PROCESSOR_NUMBER, PVOID,
    },
};

make_struct! {PROCESSOR_NUMBER,
#[derive(Debug)]
pub struct ProcessorNumber {
    group_number: WORD,
    number: BYTE,
    reserved: BYTE,
}}

make_struct! {GROUP_AFFINITY,
#[derive(Debug)]
pub struct GroupAffinity {
    mask: KAFFINITY,
    group: WORD,
    reserved: [WORD; 3],
}}

/// # Examples
///
/// ```no_run
/// use winwrap::handle::HModule;
/// use winwrap::um::winnt::DllReason;
/// use winwrap::winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL};
///
/// #[no_mangle]
/// #[allow(non_snake_case)]
/// pub unsafe extern "system" fn DllMain(
///     hinstDLL: HINSTANCE,
///     fdwReason: DWORD,
///     _lpReserved: LPVOID
/// ) -> BOOL {
///     dll_main(HModule::new(hinstDLL), DllReason::from(fdwReason)).into()
/// }
///
/// fn dll_main(dll_instance: HModule, reason: DllReason) -> bool {
///     match reason {
///         DllReason::PROCESS_ATTACH => { /* Write your code */ }
///         DllReason::PROCESS_DETACH => { /* Write your code */ }
///         DllReason::THREAD_ATTACH => { /* Write your code */ }
///         DllReason::THREAD_DETACH => { /* Write your code */ }
///     }
///     true
/// }
/// ```
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DllReason {
    /// DLL_PROCESS_ATTACH
    PROCESS_ATTACH = 1,
    /// DLL_PROCESS_DETACH
    PROCESS_DETACH = 0,
    /// DLL_THREAD_ATTACH
    THREAD_ATTACH = 2,
    /// DLL_THREAD_DETACH
    THREAD_DETACH = 3,
}

impl From<DWORD> for DllReason {
    fn from(x: u32) -> Self {
        match x {
            1 => Self::PROCESS_ATTACH,
            0 => Self::PROCESS_DETACH,
            2 => Self::THREAD_ATTACH,
            3 => Self::THREAD_DETACH,
            e => panic!("Unknown DLL reason: {}", e),
        }
    }
}

make_struct! {EXCEPTION_POINTERS,
pub struct ExceptionPointers<'a> {
    pub exception_record:&'a mut ExceptionRecord<'a>,
    pub context_record:&'a mut CONTEXT,
}}

make_struct! {EXCEPTION_RECORD,
pub struct ExceptionRecord<'a>{
    pub exception_code: DWORD,
    pub exception_flags: DWORD,
    pub exception_record: Option<&'a mut EXCEPTION_RECORD>,
    pub exception_address: PVOID,
    pub number_parameters: DWORD,
    pub exception_information: [ULONG_PTR; EXCEPTION_MAXIMUM_PARAMETERS],
}}
