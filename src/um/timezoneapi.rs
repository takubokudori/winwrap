use crate::*;
use crate::raw::um::timezoneapi::*;
use crate::shared::minwindef::{FileTime, SystemTime};
use std::mem::MaybeUninit;

pub fn file_time_to_system_time(
    file_time: &FileTime,
) -> OsResult<SystemTime> {
    unsafe {
        let mut system_time = MaybeUninit::<SystemTime>::uninit();
        FileTimeToSystemTime(
            file_time.as_c_ptr(),
            system_time.as_mut_ptr() as *mut _,
        )?;
        Ok(system_time.assume_init())
    }
}

pub fn system_time_to_file_time(
    system_time: &SystemTime,
) -> OsResult<FileTime> {
    unsafe {
        let mut file_time = MaybeUninit::<FileTime>::uninit();
        SystemTimeToFileTime(
            system_time.as_c_ptr(),
            file_time.as_mut_ptr() as *mut _,
        )?;
        Ok(file_time.assume_init())
    }
}
