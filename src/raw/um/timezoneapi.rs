// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::{
    shared::{
        minwindef::{FILETIME, LPDWORD, LPFILETIME},
        ntdef::USHORT,
    },
    um::{
        minwinbase::{LPSYSTEMTIME, SYSTEMTIME},
        timezoneapi::{
            DYNAMIC_TIME_ZONE_INFORMATION, LPTIME_ZONE_INFORMATION,
            PDYNAMIC_TIME_ZONE_INFORMATION, TIME_ZONE_ID_INVALID,
            TIME_ZONE_INFORMATION,
        },
    },
};

make_func2! {winapi::um::timezoneapi,
pub fn SystemTimeToTzSpecificLocalTime(
    lpTimeZoneInformation: *const TIME_ZONE_INFORMATION,
    lpUniversalTime: *const SYSTEMTIME,
    lpLocalTime: LPSYSTEMTIME,
) -> BOOL;0}

make_func2! {winapi::um::timezoneapi,
pub fn TzSpecificLocalTimeToSystemTime(
    lpTimeZoneInformation: *const TIME_ZONE_INFORMATION,
    lpLocalTime: *const SYSTEMTIME,
    lpUniversalTime: LPSYSTEMTIME,
) -> BOOL;0}

make_func2! {winapi::um::timezoneapi,
pub fn FileTimeToSystemTime(
    lpFileTime: *const FILETIME,
    lpSystemTime: LPSYSTEMTIME,
) -> BOOL;0}

make_func2! {winapi::um::timezoneapi,
pub fn SystemTimeToFileTime(
    lpSystemTime: *const SYSTEMTIME,
    lpFileTime: LPFILETIME,
) -> BOOL;0}

make_func! {winapi::um::timezoneapi,
pub fn GetTimeZoneInformation(
    lpTimeZoneInformation: LPTIME_ZONE_INFORMATION,
) -> DWORD;TIME_ZONE_ID_INVALID}

make_func2! {winapi::um::timezoneapi,
pub fn SetTimeZoneInformation(
    lpTimeZoneInformation: *const TIME_ZONE_INFORMATION,
) -> BOOL;0}

make_func2! {winapi::um::timezoneapi,
pub fn SetDynamicTimeZoneInformation(
    lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
) -> BOOL;0}

make_func! {winapi::um::timezoneapi,
pub fn GetDynamicTimeZoneInformation(
    pTimeZoneInformation: PDYNAMIC_TIME_ZONE_INFORMATION,
) -> DWORD;TIME_ZONE_ID_INVALID}

make_func2! {winapi::um::timezoneapi,
pub fn GetTimeZoneInformationForYear(
    wYear: USHORT,
    pdtzi: PDYNAMIC_TIME_ZONE_INFORMATION,
    ptzi: LPTIME_ZONE_INFORMATION,
) -> BOOL;0}

tp_func! {winapi::um::timezoneapi,
pub fn EnumDynamicTimeZoneInformation(
    dwIndex: DWORD,
    lpTimeZoneInformation: PDYNAMIC_TIME_ZONE_INFORMATION,
) -> DWORD;}

tp_func! {winapi::um::timezoneapi,
pub fn GetDynamicTimeZoneInformationEffectiveYears(
    lpTimeZoneInformation: PDYNAMIC_TIME_ZONE_INFORMATION,
    FirstYear: LPDWORD,
    LastYear: LPDWORD,
) -> DWORD;}

make_func2! {winapi::um::timezoneapi,
pub fn SystemTimeToTzSpecificLocalTimeEx(
    lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
    lpUniversalTime: *const SYSTEMTIME,
    lpLocalTime: LPSYSTEMTIME,
) -> BOOL;0}

make_func2! {winapi::um::timezoneapi,
pub fn TzSpecificLocalTimeToSystemTimeEx(
    lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
    lpLocalTime: *const SYSTEMTIME,
    lpUniversalTime: LPSYSTEMTIME,
) -> BOOL;0}
