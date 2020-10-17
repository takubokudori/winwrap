use crate::*;
use crate::handle::*;
use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::minwindef::DWORD;
use winapi::um::minwinbase::OVERLAPPED;

make_struct! {OVERLAPPED,
#[derive(Debug)]
pub struct Overlapped {
    pub internal: ULONG_PTR,
    pub internal_high: ULONG_PTR,
    pub offset: DWORD,
    pub offset_high:DWORD,
    // pointer: LPVOID, // union
    pub event: EventHandle,
}}
