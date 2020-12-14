use crate::*;
use crate::handle::*;
use crate::raw::um::processthreadsapi::*;
use crate::shared::minwindef::FileTime;
use crate::string::*;
use crate::um::minwinbase::SecurityAttributes;
use std::mem::MaybeUninit;
use std::ptr::{null_mut, null};
use winapi::ctypes::c_void;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{LPBYTE, WORD, DWORD, BOOL, UINT, LPVOID};
use winapi::shared::ntdef::{LPSTR, LPWSTR};
use winapi::um::processthreadsapi::{STARTUPINFOA, STARTUPINFOW, PROCESS_INFORMATION};
use winwrap_derive::*;

#[cfg(feature = "ansi")]
pub type StartupInfo<'a> = StartupInfoA<'a>;

#[cfg(not(feature = "ansi"))]
pub type StartupInfo<'a> = StartupInfoW<'a>;

make_struct! {STARTUPINFOA,
#[derive(Debug)]
pub struct StartupInfoA<'a> {
    pub cb: DWORD,
    reserved: LPSTR,
    pub desktop: Option<&'a AString>,
    pub title: Option<&'a AString>,
    pub x: DWORD,
    pub y: DWORD,
    pub x_size: DWORD,
    pub y_size: DWORD,
    pub x_count_chars: DWORD,
    pub y_count_chars: DWORD,
    pub fill_attributes: DWORD,
    pub flags: DWORD,
    pub show_window: WORD,
    cb_reserved2: WORD,
    lp_reserved2: LPBYTE,
    pub std_input: StdHandle,
    pub std_output: StdHandle,
    pub std_error: StdHandle,
}}

impl<'a> Default for StartupInfoA<'a> {
    fn default() -> Self {
        Self {
            cb: std::mem::size_of::<Self>() as u32,
            reserved: null_mut(),
            desktop: None,
            title: None,
            x: 0,
            y: 0,
            x_size: 0,
            y_size: 0,
            x_count_chars: 0,
            y_count_chars: 0,
            fill_attributes: 0,
            flags: 0,
            show_window: 0,
            cb_reserved2: 0,
            lp_reserved2: null_mut(),
            std_input: Default::default(),
            std_output: Default::default(),
            std_error: Default::default(),
        }
    }
}

make_struct! {STARTUPINFOW,
#[derive(Debug)]
pub struct StartupInfoW<'a> {
    pub cb: DWORD,
    reserved: LPWSTR,
    pub desktop: Option<&'a WString>,
    pub title: Option<&'a WString>,
    pub x: DWORD,
    pub y: DWORD,
    pub x_size: DWORD,
    pub y_size: DWORD,
    pub x_count_chars: DWORD,
    pub y_count_chars: DWORD,
    pub fill_attributes: DWORD,
    pub flags: DWORD,
    pub show_window: WORD,
    cb_reserved2: WORD,
    lp_reserved2: LPBYTE,
    pub std_input: StdHandle,
    pub std_output: StdHandle,
    pub std_error: StdHandle,
}}

impl<'a> Default for StartupInfoW<'a> {
    fn default() -> Self {
        Self {
            cb: std::mem::size_of::<Self>() as u32,
            reserved: null_mut(),
            desktop: None,
            title: None,
            x: 0,
            y: 0,
            x_size: 0,
            y_size: 0,
            x_count_chars: 0,
            y_count_chars: 0,
            fill_attributes: 0,
            flags: 0,
            show_window: 0,
            cb_reserved2: 0,
            lp_reserved2: null_mut(),
            std_input: Default::default(),
            std_output: Default::default(),
            std_error: Default::default(),
        }
    }
}

make_struct! {PROCESS_INFORMATION,
pub struct ProcessInformation {
    pub process: ProcessHandle,
    pub thread: ThreadHandle,
    pub process_id: DWORD,
    pub thread_id: DWORD,
}}

impl Default for ProcessInformation {
    fn default() -> Self {
        Self {
            process: Default::default(),
            thread: Default::default(),
            process_id: 0,
            thread_id: 0,
        }
    }
}

bitflags::bitflags! {
pub struct CreationFlags: DWORD{
    const CREATE_BREAKAWAY_FROM_JOB=winapi::um::winbase::CREATE_BREAKAWAY_FROM_JOB;
    const CREATE_DEFAULT_ERROR_MODE=winapi::um::winbase::CREATE_DEFAULT_ERROR_MODE;
    const CREATE_NEW_CONSOLE=winapi::um::winbase::CREATE_NEW_CONSOLE;
    const CREATE_NEW_PROCESS_GROUP=winapi::um::winbase::CREATE_NEW_PROCESS_GROUP;
    const CREATE_NO_WINDOW=winapi::um::winbase::CREATE_NO_WINDOW;
    const CREATE_PROTECTED_PROCESS=winapi::um::winbase::CREATE_PROTECTED_PROCESS;
    const CREATE_PRESERVE_CODE_AUTHZ_LEVEL=winapi::um::winbase::CREATE_PRESERVE_CODE_AUTHZ_LEVEL;
    const CREATE_SEPARATE_WOW_VDM=winapi::um::winbase::CREATE_SEPARATE_WOW_VDM;
    const CREATE_SHARED_WOW_VDM=winapi::um::winbase::CREATE_SHARED_WOW_VDM;
    const CREATE_SUSPENDED=winapi::um::winbase::CREATE_SUSPENDED;
    const CREATE_UNICODE_ENVIRONMENT=winapi::um::winbase::CREATE_UNICODE_ENVIRONMENT;
    const DEBUG_ONLY_THIS_PROCESS=winapi::um::winbase::DEBUG_ONLY_THIS_PROCESS;
    const DEBUG_PROCESS=winapi::um::winbase::DEBUG_PROCESS;
    const DETACHED_PROCESS=winapi::um::winbase::DETACHED_PROCESS;
    const EXTENDED_STARTUPINFO_PRESENT=winapi::um::winbase::EXTENDED_STARTUPINFO_PRESENT;
    const INHERIT_PARENT_AFFINITY=winapi::um::winbase::INHERIT_PARENT_AFFINITY;
    const CREATE_SECURE_PROCESS = 0x400000;
}}

pub fn get_current_process() -> ProcessHandle { ProcessHandle(GetCurrentProcess()) }

pub fn get_current_process_id() -> DWORD { GetCurrentProcessId() }

/// command_line must be null-terminated string.
///
/// ```no_run
///
/// use winwrap::um::processthreadsapi::{create_process_a, CreationFlags, StartupInfoA};
/// use winwrap::string::AString;
///
/// let mut command_line = AString::from_str_lossy(r"C:\Windows\System32\calc.exe");
/// let mut si=StartupInfoA::default();
/// let pi = create_process_a(
///     None,
///     unsafe { command_line.as_mut_c_str() },
///     None,
///     None,
///     false,
///     CreationFlags::empty(),
///     None,
///     None,
///     &mut si,
/// ).unwrap();
/// ```
#[ansi_fn]
#[inline]
pub fn create_process_a<'a, AN, CL, PA, TA, EV, CD>(
    application_name: AN,
    command_line: CL,
    process_attributes: PA,
    thread_attributes: TA,
    inherit_handle: bool,
    creation_flags: CreationFlags,
    env: EV,
    current_directory: CD,
    si: &mut StartupInfoA,
) -> OsResult<ProcessInformation>
    where
        AN: Into<Option<&'a AStr>>,
        CL: Into<Option<&'a mut AStr>>,
        PA: Into<Option<&'a mut SecurityAttributes<'a>>>,
        TA: Into<Option<&'a mut SecurityAttributes<'a>>>,
        EV: Into<Option<&'a mut [u8]>>,
        CD: Into<Option<&'a AStr>>,
{
    unsafe {
        // null-terminated check
        let mut pi: ProcessInformation = std::mem::zeroed();
        CreateProcessA(
            application_name.into().map_or(null(), |x| x.as_ptr()),
            command_line.into().map_or(null_mut(), |x| x.as_mut_ptr()),
            process_attributes.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            thread_attributes.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            inherit_handle as BOOL,
            creation_flags.bits,
            env.into().map_or(null_mut(), |x| x.as_mut_ptr() as *mut _),
            current_directory.into().map_or(null(), |x| x.as_ptr()),
            si.as_mut_c_ptr(),
            pi.as_mut_ptr() as *mut _,
        )?;
        Ok(pi)
    }
}

#[unicode_fn]
#[inline]
pub fn create_process_w<'a, AN, CL, PA, TA, EV, CD>(
    application_name: AN,
    command_line: CL,
    process_attributes: PA,
    thread_attributes: TA,
    inherit_handle: bool,
    creation_flags: CreationFlags,
    env: EV,
    current_directory: CD,
    si: &mut StartupInfoW,
) -> OsResult<ProcessInformation>
    where
        AN: Into<Option<&'a WStr>>,
        CL: Into<Option<&'a mut WStr>>,
        PA: Into<Option<&'a mut SecurityAttributes<'a>>>,
        TA: Into<Option<&'a mut SecurityAttributes<'a>>>,
        EV: Into<Option<&'a mut [u8]>>,
        CD: Into<Option<&'a WStr>>,
{
    unsafe {
        // null-terminated check
        let mut pi: ProcessInformation = std::mem::zeroed();
        CreateProcessW(
            application_name.into().map_or(null(), |x| x.as_ptr()),
            command_line.into().map_or(null_mut(), |x| x.as_mut_ptr()),
            process_attributes.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            thread_attributes.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
            inherit_handle as BOOL,
            creation_flags.bits,
            env.into().map_or(null_mut(), |x| x.as_mut_ptr() as *mut _),
            current_directory.into().map_or(null(), |x| x.as_ptr()),
            si.as_mut_c_ptr(),
            pi.as_mut_ptr() as *mut _,
        )?;
        Ok(pi)
    }
}

pub fn resume_thread(
    handle: &ThreadHandle,
) -> OsResult<u32> {
    unsafe { ResumeThread(handle.as_c_handle()) }
}

pub fn terminate_process(
    handle: &ProcessHandle,
    exit_code: UINT,
) -> OsResult<()> {
    unsafe { TerminateProcess(handle.as_c_handle(), exit_code) }
}

pub fn terminate_thread(
    handle: &ThreadHandle,
    exit_code: UINT,
) -> OsResult<()> {
    unsafe { TerminateThread(handle.as_c_handle(), exit_code) }
}

pub fn get_exit_code_process(
    handle: &ProcessHandle,
) -> OsResult<DWORD> {
    unsafe {
        let mut exit_code = 0;
        GetExitCodeProcess(handle.as_c_handle(), &mut exit_code)?;
        Ok(exit_code)
    }
}

pub fn get_exit_code_thread(
    handle: &ThreadHandle,
) -> OsResult<DWORD> {
    unsafe {
        let mut exit_code = 0;
        GetExitCodeThread(handle.as_c_handle(), &mut exit_code)?;
        Ok(exit_code)
    }
}

bitflags::bitflags! {
pub struct ProcessAccessRights: u32 {
    const DELETE                    = winapi::um::winnt::DELETE;
    const READ_CONTROL              = winapi::um::winnt::READ_CONTROL;
    const SYNCHRONIZE               = winapi::um::winnt::SYNCHRONIZE;
    const WRITE_DAC                 = winapi::um::winnt::WRITE_DAC;
    const WRITE_OWNER               = winapi::um::winnt::WRITE_OWNER;
    /// PROCESS_ALL_ACCESS
    const ALL_ACCESS                = winapi::um::winnt::PROCESS_ALL_ACCESS;
    /// PROCESS_CREATE_PROCESS
    const CREATE_PROCESS            = winapi::um::winnt::PROCESS_CREATE_PROCESS;
    /// PROCESS_CREATE_THREAD
    const CREATE_THREAD             = winapi::um::winnt::PROCESS_CREATE_THREAD;
    /// PROCESS_DUP_HANDLE
    const DUP_HANDLE                = winapi::um::winnt::PROCESS_DUP_HANDLE;
    /// PROCESS_QUERY_INFORMATION
    const QUERY_INFORMATION         = winapi::um::winnt::PROCESS_QUERY_INFORMATION;
    /// PROCESS_QUERY_LIMITED_INFORMATION
    const QUERY_LIMITED_INFORMATION = winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
    /// PROCESS_SET_INFORMATION
    const SET_INFORMATION           = winapi::um::winnt::PROCESS_SET_INFORMATION;
    /// PROCESS_SET_QUOTA
    const SET_QUOTA                 = winapi::um::winnt::PROCESS_SET_QUOTA;
    /// PROCESS_SUSPEND_RESUME
    const SUSPEND_RESUME            = winapi::um::winnt::PROCESS_SUSPEND_RESUME;
    /// PROCESS_TERMINATE
    const TERMINATE                 = winapi::um::winnt::PROCESS_TERMINATE;
    /// PROCESS_VM_OPERATION
    const VM_OPERATION              = winapi::um::winnt::PROCESS_VM_OPERATION;
    /// PROCESS_VM_READ
    const VM_READ                   = winapi::um::winnt::PROCESS_VM_READ;
    /// PROCESS_VM_WRITE
    const VM_WRITE                  = winapi::um::winnt::PROCESS_VM_WRITE;
}}

pub fn open_process(
    desired_access: ProcessAccessRights,
    is_inherit_handle: bool,
    pid: DWORD,
) -> OsResult<ProcessHandle> {
    Ok(ProcessHandle::new(
        OpenProcess(
            desired_access.bits,
            is_inherit_handle.into(),
            pid,
        )?
    ))
}

bitflags::bitflags! {
    pub struct TokenAccessRights: u32 {
        const ADJUST_DEFAULT = winapi::um::winnt::TOKEN_ADJUST_DEFAULT;
        const ADJUST_GROUPS = winapi::um::winnt::TOKEN_ADJUST_GROUPS;
        const ADJUST_PRIVILEGES = winapi::um::winnt::TOKEN_ADJUST_PRIVILEGES;
        const ADJUST_SESSIONID = winapi::um::winnt::TOKEN_ADJUST_SESSIONID;
        const ASSIGN_PRIMARY = winapi::um::winnt::TOKEN_ASSIGN_PRIMARY;
        const DUPLICATE = winapi::um::winnt::TOKEN_DUPLICATE;
        const EXECUTE = winapi::um::winnt::TOKEN_EXECUTE;
        const IMPRERSONATE = winapi::um::winnt::TOKEN_IMPERSONATE;
        const QUERY = winapi::um::winnt::TOKEN_QUERY;
        const QUERY_SOURCE = winapi::um::winnt::TOKEN_QUERY_SOURCE;
        const READ = winapi::um::winnt::TOKEN_READ;
        const WRITE = winapi::um::winnt::TOKEN_WRITE;
        const ALL_ACCESS = winapi::um::winnt::TOKEN_ALL_ACCESS;
    }
}

pub fn open_process_token(
    handle: &ProcessHandle,
    access: TokenAccessRights,
) -> OsResult<TokenHandle> {
    unsafe {
        let mut ret = null_mut();
        OpenProcessToken(
            handle.as_c_handle(),
            access.bits,
            &mut ret,
        )?;
        Ok(TokenHandle::new(ret))
    }
}

pub fn get_process_id(
    handle: &ProcessHandle,
) -> OsResult<DWORD> {
    unsafe { GetProcessId(handle.as_c_handle()) }
}

pub fn get_thread_id(
    handle: &ThreadHandle,
) -> OsResult<DWORD> {
    unsafe { GetThreadId(handle.as_c_handle()) }
}

pub fn get_process_id_of_thread(
    handle: &ThreadHandle,
) -> OsResult<DWORD> {
    unsafe { GetProcessIdOfThread(handle.as_c_handle()) }
}

pub fn get_system_times() -> OsResult<(FileTime, FileTime, FileTime)> {
    unsafe {
        let mut idle_file_time = MaybeUninit::<FileTime>::uninit();
        let mut kernel_file_time = MaybeUninit::<FileTime>::uninit();
        let mut user_file_time = MaybeUninit::<FileTime>::uninit();
        GetSystemTimes(
            idle_file_time.as_mut_ptr() as *mut _,
            kernel_file_time.as_mut_ptr() as *mut _,
            user_file_time.as_mut_ptr() as *mut _,
        )?;
        Ok((idle_file_time.assume_init(), kernel_file_time.assume_init(), user_file_time.assume_init()))
    }
}

pub fn is_process_critical(
    handle: &ProcessHandle,
) -> OsResult<bool> {
    unsafe {
        let mut is_critical = 0;
        IsProcessCritical(
            handle.as_c_handle(),
            &mut is_critical,
        )?;
        Ok(is_critical != 0)
    }
}

pub type ThreadProc = extern "system" fn(param: LPVOID) -> DWORD;

bitflags::bitflags! {
pub struct RemoteThreadFlags: u32{
    const CREATE_SUSPENDED                  = winapi::um::winbase::CREATE_SUSPENDED;
    const STACK_SIZE_PARAM_IS_A_RESERVATION = 0x10000 /* winapi::um::winbase::STACK_SIZE_PARAM_IS_A_RESERVATION */;
}}

pub type ThreadStartRoutine = extern "system" fn(x: *mut c_void) -> u32;

pub unsafe fn create_remote_thread<'a, SA, RT>(
    proc_handle: &ProcessHandle,
    th_attrs: SA,
    stack_size: SIZE_T,
    start_address: ThreadStartRoutine,
    param: LPVOID,
    creation_flags: RT,
) -> OsResult<(ThreadHandle, DWORD)>
    where
        SA: Into<Option<&'a mut SecurityAttributes<'a>>>,
        RT: Into<Option<RemoteThreadFlags>>,
{
    let mut thread_id = 0;
    let handle = CreateRemoteThread(
        proc_handle.as_c_handle(),
        th_attrs.into().map_or(null_mut(), |x| x.as_mut_c_ptr()),
        stack_size,
        Some(start_address),
        param,
        creation_flags.into().map_or(0, |x| x.bits),
        &mut thread_id,
    )?;
    Ok((ThreadHandle::new(handle), thread_id))
}

pub unsafe fn flush_instruction_cache<BA, T>(
    proc_handle: &ProcessHandle,
    base_address: BA,
) -> OsResult<()>
    where
        BA: Into<Option<(*mut T, usize)>>,
{
    let (addr, size) = base_address.into().map_or((null_mut(), 0), |x| x);
    FlushInstructionCache(
        proc_handle.as_c_handle(),
        addr as *mut _,
        size,
    )
}
