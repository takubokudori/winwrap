//! An example of a DLL injector.
//!
//! ```
//! dll_injector.exe [pid] [absolute DLL path]
//! ```
use windy_macros::{astr, wstr};
use winwrap::{
    string::*,
    um::{
        libloaderapi::{get_proc_address, load_library_w, ProcAddress},
        memoryapi::{
            virtual_alloc_ex, write_process_memory, MemAllocationFlags,
            MemProtectionFlags,
        },
        processthreadsapi::{
            create_remote_thread, open_process, ProcessAccessRights,
        },
    },
    winapi::shared::minwindef::{FARPROC, LPVOID},
    OsResult,
};

fn main() {
    /// Gets the address of LoadLibraryW.
    fn get_ll_addr() -> OsResult<FARPROC> {
        let ll = load_library_w(wstr!("kernel32.dll"))?;
        get_proc_address(&ll, ProcAddress::ByName(astr!("LoadLibraryW")))
    }
    let ll_addr = get_ll_addr().expect("Failed to get LoadLibraryW ptr");

    let args: Vec<String> = std::env::args().collect();
    let pid: u32 = args[1].parse().expect("Failed to parse pid");
    let lib_path = WString::from_str(args[2].as_str())
        .expect("Failed to convert lib_path");

    let p = open_process(
        ProcessAccessRights::CREATE_THREAD
            | ProcessAccessRights::VM_OPERATION
            | ProcessAccessRights::VM_WRITE,
        false,
        pid,
    )
    .expect("Failed to open a process");

    unsafe {
        let mem = virtual_alloc_ex(
            &p,
            None,
            lib_path.as_u8_bytes_with_nul().len(),
            MemAllocationFlags::COMMIT,
            None,
            MemProtectionFlags::READWRITE,
        )
        .expect("Failed to virtual_alloc_ex");
        write_process_memory(
            &p,
            mem.as_ptr() as *mut u8,
            lib_path.as_u8_bytes_with_nul(),
        )
        .expect("Failed to write_process_memory");
        create_remote_thread(
            &p,
            None,
            0,
            std::mem::transmute(ll_addr),
            mem.as_ptr() as LPVOID,
            None,
        )
        .expect("Failed to create_remote_thread");

        // VirtualMemory will be freed when the drop function is called.
        // It prevents VirtualMemory from being freed before the thread is executed,
        // since VirtualMemory may be freed before the thread is executed.
        std::mem::forget(mem);
    }

    println!("Injected {:?} to {}!!", lib_path, pid);
}
