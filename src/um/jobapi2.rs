use crate::*;
use crate::handle::*;
use crate::raw::um::jobapi2::*;
use crate::string::*;
use crate::um::minwinbase::SecurityAttributes;
use std::ptr::null_mut;
use winapi::shared::basetsd::LONG64;
use winapi::shared::minwindef::UINT;
use winapi::shared::ntdef::ULONG;
use winapi::um::jobapi2::JOBOBJECT_IO_RATE_CONTROL_INFORMATION;
use winwrap_derive::*;

#[unicode_fn]
pub fn create_job_object_w<'a, SA>(
    sec_attr: SA,
    name: &WStr,
) -> OsResult<JobHandle>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>> {
    unsafe {
        CreateJobObjectW(
            sec_attr.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            name.as_ptr(),
        ).and_then(|x| Ok(JobHandle::new(x)))
    }
}

bitflags::bitflags! {
pub struct JobAccessRights: u32{
    const DELETE = winapi::um::winnt::DELETE;
    const READ_CONTROL = winapi::um::winnt::READ_CONTROL;
    const SYNCHRONIZE = winapi::um::winnt::SYNCHRONIZE;
    const WRITE_DAC = winapi::um::winnt::WRITE_DAC;
    const WRITE_OWNER = winapi::um::winnt::WRITE_OWNER;
    const ALL_ACCESS = winapi::um::winnt::JOB_OBJECT_ALL_ACCESS;
    const ASSIGN_PROCESS = winapi::um::winnt::JOB_OBJECT_ASSIGN_PROCESS;
    const QUERY = winapi::um::winnt::JOB_OBJECT_QUERY;
    const SET_ATTRIBUTES = winapi::um::winnt::JOB_OBJECT_SET_ATTRIBUTES;
    const SET_SECURITY_ATTRIBUTES = winapi::um::winnt::JOB_OBJECT_SET_SECURITY_ATTRIBUTES;
    const TERMINATE = winapi::um::winnt::JOB_OBJECT_TERMINATE;
    const ACCESS_SYSTEM_SECURITY = winapi::um::winnt::ACCESS_SYSTEM_SECURITY;
}}


#[unicode_fn]
pub fn open_job_object_w(
    desired_access: DWORD,
    inherit_handle: bool,
    job_name: &WStr,
) -> OsResult<JobHandle> {
    unsafe {
        OpenJobObjectW(
            desired_access,
            i32::from(inherit_handle),
            job_name.as_ptr(),
        ).and_then(|x| Ok(JobHandle::new(x)))
    }
}

pub fn assign_process_to_job_object(
    job_handle: &JobHandle,
    proc_handle: &ProcessHandle,
) -> OsResult<()> {
    unsafe {
        AssignProcessToJobObject(
            job_handle.as_c_handle(),
            proc_handle.as_c_handle(),
        )
    }
}

pub fn terminate_job_object(
    job_handle: &JobHandle,
    exit_code: UINT,
) -> OsResult<()> {
    unsafe {
        TerminateJobObject(
            job_handle.as_c_handle(),
            exit_code,
        )
    }
}

bitflags::bitflags! {
pub struct ControlFlags: ULONG {
    const JOB_OBJECT_IO_RATE_CONTROL_ENABLE = winapi::um::winnt::JOB_OBJECT_IO_RATE_CONTROL_ENABLE;
}}

make_struct! {JOBOBJECT_IO_RATE_CONTROL_INFORMATION,
#[derive(Debug, Clone)]
pub struct JobObjectIoRateControlInformation<'a>{
    pub max_iops: LONG64,
    pub max_band_width: LONG64,
    pub reservation_iops: LONG64,
    pub volume_name: Option<&'a WString>,
    pub base_io_size: ULONG,
    pub control_flags: ControlFlags,
}}

make_struct! {JOBOBJECT_IO_RATE_CONTROL_INFORMATION,
pub struct JobObjectIoRateControlInformations<'a>{
    inner: &'a mut [JobObjectIoRateControlInformation<'a>],
}}
impl<'a> JobObjectIoRateControlInformations<'a> {
    pub unsafe fn new(job_infos: *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, len: usize) -> Self {
        Self {
            inner: std::slice::from_raw_parts_mut(job_infos as *mut JobObjectIoRateControlInformation, len)
        }
    }
}

impl<'a> Drop for JobObjectIoRateControlInformations<'a> {
    fn drop(&mut self) {
        unsafe { free_memory_job_object(self.inner); }
    }
}


pub fn query_io_rate_control_information_job_object<'a, VN>(
    handle: &JobHandle,
    volume_name: VN,
) -> OsResult<JobObjectIoRateControlInformations>
    where
        VN: Into<Option<&'a WStr>>
{
    unsafe {
        let mut info_block_count = 0;
        let mut p = null_mut();
        QueryIoRateControlInformationJobObject(
            handle.as_c_handle(),
            volume_name.into().map_or(null_mut(), |x| x.as_ptr()),
            &mut p,
            &mut info_block_count,
        )?;
        debug_assert_ne!(p, null_mut());
        Ok(JobObjectIoRateControlInformations::new(p, info_block_count as usize))
    }
}

pub unsafe fn free_memory_job_object(
    job_io_infos: &mut [JobObjectIoRateControlInformation]
) {
    FreeMemoryJobObject(job_io_infos.as_mut_ptr() as *mut _)
}
