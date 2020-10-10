use crate::*;
use crate::handle::*;
use crate::raw::um::jobapi::*;

pub fn is_process_in_job(
    proc_handle: &ProcessHandle,
    job_handle: &JobHandle,
) -> OsResult<bool> {
    unsafe {
        let mut res = 0;
        IsProcessInJob(
            proc_handle.as_c_handle(),
            job_handle.as_c_handle(),
            &mut res)?;
        Ok(res != 0)
    }
}
