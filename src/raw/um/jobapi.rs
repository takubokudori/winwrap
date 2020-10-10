use crate::*;
use winapi::shared::minwindef::PBOOL;
use winapi::um::winnt::HANDLE;

make_func2! {winapi::um::jobapi,
pub fn IsProcessInJob(
    ProcessHandle: HANDLE,
    JobHandle: HANDLE,
    Result: PBOOL,
) -> BOOL;0}
