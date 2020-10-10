use crate::*;
use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::minwindef::DWORD;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::winnt::HANDLE;

make_struct! {OVERLAPPED,
#[derive(Debug)]
pub struct Overlapped {
    internal: ULONG_PTR,
    internal_high: ULONG_PTR,
    offset: DWORD,
    offset_high:DWORD,
    // pointer: LPVOID, // union
    event: HANDLE,
}}
