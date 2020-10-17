use crate::*;
use crate::handle::*;
use crate::raw::um::tlhelp32::*;
use crate::string::*;
use std::mem::{MaybeUninit, size_of, ManuallyDrop};
use std::ptr::null_mut;
use winapi::shared::basetsd::{ULONG_PTR, SIZE_T};
use winapi::shared::minwindef::{DWORD, BYTE, MAX_PATH};
use winapi::um::tlhelp32::{MAX_MODULE_NAME32, MODULEENTRY32, THREADENTRY32, MODULEENTRY32W, HEAPENTRY32, HEAPLIST32, PROCESSENTRY32, PROCESSENTRY32W};
use winapi::um::winnt::{CHAR, WCHAR, LONG, HANDLE};

bitflags::bitflags! {
    pub struct TH32CSFlags:u32 {
        const INHERIT = winapi::um::tlhelp32::TH32CS_INHERIT;
        const SNAPALL = winapi::um::tlhelp32::TH32CS_SNAPALL;
        const SNAPHEAPLIST = winapi::um::tlhelp32::TH32CS_SNAPHEAPLIST;
        const SNAPMODULE = winapi::um::tlhelp32::TH32CS_SNAPMODULE;
        const SNAPMODULE32 = winapi::um::tlhelp32::TH32CS_SNAPMODULE32;
        const SNAPPROCESS = winapi::um::tlhelp32::TH32CS_SNAPPROCESS;
        const SNAPTHREAD = winapi::um::tlhelp32::TH32CS_SNAPTHREAD;
    }
}

bitflags::bitflags! {
    pub struct HF32Flags:u32 {
        const DEFAULT = winapi::um::tlhelp32::HF32_DEFAULT;
        const SHARED = winapi::um::tlhelp32::HF32_SHARED;
    }
}

make_struct! {HEAPLIST32,
#[derive(Debug, Clone)]
pub struct HeapList32 {
    /// dwSize
    pub size: SIZE_T,
    /// th32ProcessID
    pub process_id: DWORD,
    /// th32HeapID
    pub heap_id: ULONG_PTR,
    /// dwFlags
    pub flags: HF32Flags,
}}

impl Default for HeapList32 {
    fn default() -> Self {
        Self {
            size: size_of::<HEAPLIST32>(),
            process_id: 0,
            heap_id: 0,
            flags: HF32Flags::empty(),
        }
    }
}

bitflags::bitflags! {
pub struct LF32Flags: u32 {
    const FIXED = winapi::um::tlhelp32::LF32_FIXED;
    const FREE = winapi::um::tlhelp32::LF32_FREE;
    const MOVEABLE = winapi::um::tlhelp32::LF32_MOVEABLE;
}
}

make_struct! {HEAPENTRY32,
pub struct HeapEntry32 {
    /// dwSize
    pub size: SIZE_T,
    /// hHandle
    pub heap_handle: HANDLE,
    /// dwAccess
    pub address: ULONG_PTR,
    /// dwBlockSize
    pub block_size: SIZE_T,
    /// dwFlags
    pub flags: LF32Flags,
    /// dwLockCount
    pub lock_count: DWORD,
    /// dwResvd
    pub resvd: DWORD,
    /// th32ProcessID
    pub process_id: DWORD,
    /// th32HeapID
    pub heap_id: ULONG_PTR,
}}

impl Default for HeapEntry32 {
    fn default() -> Self {
        Self {
            size: size_of::<HEAPENTRY32>(),
            heap_handle: null_mut(),
            address: 0,
            block_size: 0,
            flags: LF32Flags::empty(),
            lock_count: 0,
            resvd: 0,
            process_id: 0,
            heap_id: 0,
        }
    }
}

make_struct! {PROCESSENTRY32W,
pub struct ProcessEntry32W {
    /// dwSize
    pub size: DWORD,
    /// cntUsage
    pub cnt_usage: DWORD,
    /// th32ProcessID
    pub process_id: DWORD,
    /// th32DefaultHeapID
    pub default_heap_id: ULONG_PTR,
    /// th32ModuleID
    pub module_id: DWORD,
    /// cntThreads
    pub cnt_threads: DWORD,
    /// th32ParentProcessID
    pub parent_process_id: DWORD,
    /// pcPriClassBase
    pub pri_class_base: LONG,
    /// dwFlags
    pub flags: DWORD,
    /// szExeFile
    exe_file: [WCHAR; MAX_PATH],
}}

impl Default for ProcessEntry32W {
    fn default() -> Self {
        unsafe {
            let mut this = Self {
                size: size_of::<PROCESSENTRY32W>() as u32,
                cnt_usage: 0,
                process_id: 0,
                default_heap_id: 0,
                module_id: 0,
                cnt_threads: 0,
                parent_process_id: 0,
                pri_class_base: 0,
                flags: 0,
                exe_file: MaybeUninit::uninit().assume_init(),
            };
            *this.exe_file.get_unchecked_mut(0) = 0;
            this
        }
    }
}

impl ProcessEntry32W {
    pub fn get_exe_file(&mut self) -> ManuallyDrop<WString> {
        unsafe {
            ManuallyDrop::new(WString::from_raw(self.exe_file.as_mut_ptr() as *mut _))
        }
    }
}

make_struct! {PROCESSENTRY32,
pub struct ProcessEntry32 {
    /// dwSize
    pub size: DWORD,
    /// cntUsage
    pub cnt_usage: DWORD,
    /// th32ProcessID
    pub process_id: DWORD,
    /// th32DefaultHeapID
    pub default_heap_id: ULONG_PTR,
    /// th32ModuleID
    pub module_id: DWORD,
    /// cntThreads
    pub cnt_threads: DWORD,
    /// th32ParentProcessID
    pub parent_process_id: DWORD,
    /// pcPriClassBase
    pub pri_class_base: LONG,
    /// dwFlags
    pub flags: DWORD,
    /// szExeFile
    exe_file: [CHAR; MAX_PATH],
}}

impl Default for ProcessEntry32 {
    fn default() -> Self {
        unsafe {
            let mut this = Self {
                size: size_of::<PROCESSENTRY32>() as u32,
                cnt_usage: 0,
                process_id: 0,
                default_heap_id: 0,
                module_id: 0,
                cnt_threads: 0,
                parent_process_id: 0,
                pri_class_base: 0,
                flags: 0,
                exe_file: MaybeUninit::uninit().assume_init(),
            };
            *this.exe_file.get_unchecked_mut(0) = 0;
            this
        }
    }
}

impl ProcessEntry32 {
    pub fn get_exe_file(&mut self) -> ManuallyDrop<AString> {
        unsafe {
            ManuallyDrop::new(AString::from_raw(self.exe_file.as_mut_ptr() as *mut _))
        }
    }
}

make_struct! {MODULEENTRY32,
pub struct ModuleEntry32{
    pub size: DWORD,
    pub module_id: DWORD,
    pub process_id: DWORD,
    pub glbl_cnt_usage: DWORD,
    pub proc_cnt_usage: DWORD,
    pub mod_base_addr: *mut BYTE,
    pub mod_base_size: DWORD,
    pub mod_handle: HModule,
    module_name: [CHAR; MAX_MODULE_NAME32 + 1],
    exe_path: [CHAR; MAX_PATH],
}}

impl Default for ModuleEntry32 {
    fn default() -> Self {
        unsafe {
            let mut this = Self {
                size: size_of::<MODULEENTRY32>() as u32,
                module_id: 0,
                process_id: 0,
                glbl_cnt_usage: 0,
                proc_cnt_usage: 0,
                mod_base_addr: null_mut(),
                mod_base_size: 0,
                mod_handle: Default::default(),
                module_name: MaybeUninit::uninit().assume_init(),
                exe_path: MaybeUninit::uninit().assume_init(),
            };
            *this.module_name.get_unchecked_mut(0) = 0;
            *this.exe_path.get_unchecked_mut(0) = 0;
            this
        }
    }
}

impl ModuleEntry32 {
    pub fn get_module_name(&mut self) -> ManuallyDrop<AString> {
        unsafe {
            ManuallyDrop::new(AString::from_raw(self.module_name.as_mut_ptr() as *mut _))
        }
    }

    pub fn get_exe_path(&mut self) -> ManuallyDrop<AString> {
        unsafe {
            ManuallyDrop::new(AString::from_raw(self.exe_path.as_mut_ptr() as *mut _))
        }
    }
}

make_struct! {THREADENTRY32,
#[derive(Debug, Default, Clone)]
pub struct ThreadEntry32 {
    pub size: DWORD,
    pub cnt_usage: DWORD,
    pub thread_id: DWORD,
    pub owner_process_id: DWORD,
    pub tp_base_pri: LONG,
    pub tp_delta_pri: LONG,
    pub flags: DWORD,
}}

make_struct! {MODULEENTRY32W,
pub struct ModuleEntry32W{
    pub size: DWORD,
    pub module_id: DWORD,
    pub process_id: DWORD,
    pub glbl_cnt_usage: DWORD,
    pub proc_cnt_usage: DWORD,
    pub mod_base_addr: *mut BYTE,
    pub mod_base_size: DWORD,
    pub mod_handle: HModule,
    module_name: [WCHAR; MAX_MODULE_NAME32 + 1],
    exe_path: [WCHAR; MAX_PATH],
}}

impl ModuleEntry32W {
    pub fn get_module_name(&mut self) -> ManuallyDrop<WString> {
        unsafe {
            ManuallyDrop::new(WString::from_raw(self.module_name.as_mut_ptr() as *mut _))
        }
    }

    pub fn get_exe_path(&mut self) -> ManuallyDrop<WString> {
        unsafe {
            ManuallyDrop::new(WString::from_raw(self.exe_path.as_mut_ptr() as *mut _))
        }
    }
}

impl Default for ModuleEntry32W {
    fn default() -> Self {
        unsafe {
            let mut this = Self {
                size: size_of::<MODULEENTRY32W>() as u32,
                module_id: 0,
                process_id: 0,
                glbl_cnt_usage: 0,
                proc_cnt_usage: 0,
                mod_base_addr: null_mut(),
                mod_base_size: 0,
                mod_handle: Default::default(),
                module_name: MaybeUninit::uninit().assume_init(),
                exe_path: MaybeUninit::uninit().assume_init(),
            };
            *this.module_name.get_unchecked_mut(0) = 0;
            *this.exe_path.get_unchecked_mut(0) = 0;
            this
        }
    }
}

/// # Example
///
/// ```rust
/// fn main(){
///     use winwrap::um::tlhelp32::{create_tool_help32_snapshot, TH32CSFlags};
///     use winwrap::um::processthreadsapi::get_current_process_id;
///     let handle = create_tool_help32_snapshot(TH32CSFlags::SNAPMODULE32, get_current_process_id()).unwrap();
/// }
/// ```
pub fn create_tool_help32_snapshot(
    flags: TH32CSFlags,
    pid: DWORD,
) -> OsResult<SnapshotHandle> {
    unsafe {
        CreateToolhelp32Snapshot(flags.bits(), pid)
            .and_then(|x| Ok(SnapshotHandle::new(x)))
    }
}

pub fn heap32list_first(
    handle: &SnapshotHandle,
    entry: &mut HeapList32,
) -> OsResult<()> {
    unsafe {
        entry.size = size_of::<HEAPLIST32>();
        Heap32ListFirst(
            handle.as_c_handle(),
            entry.as_mut_c_ptr(),
        )
    }
}

pub fn heap32list_next(
    handle: &SnapshotHandle,
    entry: &mut HeapList32,
) -> OsResult<()> {
    unsafe {
        entry.size = size_of::<HEAPLIST32>();
        Heap32ListNext(
            handle.as_c_handle(),
            entry.as_mut_c_ptr(),
        )
    }
}

pub fn heap32_first(
    entry: &mut HeapEntry32,
    pid: u32,
    heap_id: ULONG_PTR,
) -> OsResult<()> {
    unsafe {
        Heap32First(
            entry.as_mut_c_ptr(),
            pid,
            heap_id,
        )
    }
}

pub fn heap32_next(
    entry: &mut HeapEntry32,
) -> OsResult<()> {
    unsafe {
        Heap32Next(
            entry.as_mut_c_ptr(),
        )
    }
}

pub fn process32_first_w(
    handle: &SnapshotHandle,
    entry: &mut ProcessEntry32W,
) -> OsResult<()> {
    unsafe {
        Process32FirstW(
            handle.as_c_handle(),
            entry.as_mut_c_ptr())?;
        Ok(())
    }
}

pub fn process32_next_w(
    handle: &SnapshotHandle,
    entry: &mut ProcessEntry32W,
) -> OsResult<()> {
    unsafe {
        Process32NextW(
            handle.as_c_handle(),
            entry.as_mut_c_ptr())?;
        Ok(())
    }
}

pub fn process32_first(
    handle: &SnapshotHandle,
    entry: &mut ProcessEntry32,
) -> OsResult<()> {
    unsafe {
        Process32First(
            handle.as_c_handle(),
            entry.as_mut_c_ptr())?;
        Ok(())
    }
}

pub fn process32_next(
    handle: &SnapshotHandle,
    entry: &mut ProcessEntry32,
) -> OsResult<()> {
    unsafe {
        Process32Next(
            handle.as_c_handle(),
            entry.as_mut_c_ptr())?;
        Ok(())
    }
}

pub fn thread32_first(
    handle: &SnapshotHandle,
    entry: &mut ThreadEntry32,
) -> OsResult<()> {
    unsafe {
        Thread32First(
            handle.as_c_handle(),
            entry.as_mut_c_ptr(),
        )
    }
}

pub fn thread32_next(
    handle: &SnapshotHandle,
    entry: &mut ThreadEntry32,
) -> OsResult<()> {
    unsafe {
        Thread32Next(
            handle.as_c_handle(),
            entry.as_mut_c_ptr(),
        )
    }
}

pub fn module32_first_w(
    handle: &SnapshotHandle,
    entry: &mut ModuleEntry32W,
) -> OsResult<()> {
    unsafe {
        entry.size = size_of::<MODULEENTRY32W>() as u32;
        Module32FirstW(
            handle.as_c_handle(),
            entry.as_mut_c_ptr(),
        )
    }
}

pub fn module32_next_w(
    handle: &SnapshotHandle,
    entry: &mut ModuleEntry32W,
) -> OsResult<()> {
    unsafe {
        entry.size = size_of::<MODULEENTRY32W>() as u32;
        Module32NextW(
            handle.as_c_handle(),
            entry.as_mut_c_ptr(),
        )
    }
}

/// # Example
///
/// ```no_run
/// fn main(){
///     use winwrap::um::tlhelp32::{create_tool_help32_snapshot, module32_first, TH32CSFlags};
///     use winwrap::um::processthreadsapi::get_current_process_id;
///     let handle = create_tool_help32_snapshot(TH32CSFlags::SNAPMODULE32, get_current_process_id()).unwrap();
///     let mut entry=Default::default();
///     module32_first(&handle,&mut entry).unwrap();
///     println!("{}", entry.mod_base_addr as usize);
/// }
/// ```
pub fn module32_first(
    handle: &SnapshotHandle,
    entry: &mut ModuleEntry32,
) -> OsResult<()> {
    unsafe {
        entry.size = size_of::<MODULEENTRY32>() as u32;
        Module32First(
            handle.as_c_handle(),
            entry.as_mut_c_ptr())?;
        Ok(())
    }
}

pub fn module32_next(
    handle: &SnapshotHandle,
    entry: &mut ModuleEntry32,
) -> OsResult<()> {
    unsafe {
        entry.size = size_of::<MODULEENTRY32>() as u32;
        Module32Next(
            handle.as_c_handle(),
            entry.as_mut_c_ptr(),
        )
    }
}
