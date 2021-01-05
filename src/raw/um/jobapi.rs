// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{shared::minwindef::PBOOL, um::winnt::HANDLE};

make_func2! {winapi::um::jobapi,
pub fn IsProcessInJob(
    ProcessHandle: HANDLE,
    JobHandle: HANDLE,
    Result: PBOOL,
) -> BOOL;0}
