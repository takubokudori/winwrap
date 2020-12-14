// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::shared::minwindef::FILETIME;
use winapi::um::minwinbase::SYSTEMTIME;

make_struct! {FILETIME,
#[derive(Default, Clone, Eq, PartialEq)]
pub struct FileTime{
    pub low_date_time: DWORD,
    pub high_date_time: DWORD,
}}

make_struct! {SYSTEMTIME,
#[derive(Default, Clone, Eq, PartialEq)]
pub struct SystemTime{
    pub year: WORD,
    pub month: WORD,
    pub day_of_week: WORD,
    pub day: WORD,
    pub hour: WORD,
    pub minute: WORD,
    pub second: WORD,
    pub milliseconds: WORD,
}}
