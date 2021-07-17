//! A simple DLL.
//!
//! Popup a message box when attach or detach this DLL.
use windy_macros::wstr;
use winwrap::{
    handle::HModule,
    string::*,
    um::{
        winnt::DllReason,
        winuser::{message_box_w, MBType},
    },
    winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID},
};

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: DWORD,
    _lpReserved: LPVOID,
) -> BOOL {
    dll_main(HModule::new(hinstDLL), DllReason::from(fdwReason)).into()
}

fn dll_main(_dll_instance: HModule, reason: DllReason) -> bool {
    match reason {
        DllReason::PROCESS_ATTACH => message_box_w(
            None,
            &wstr!("PROCESS_ATTACH"),
            &wstr!("A"),
            MBType::OK,
        )
        .expect("Failed to message_box_w"),
        DllReason::PROCESS_DETACH => message_box_w(
            None,
            wstr!("PROCESS_DETACH"),
            &wstr!("B"),
            MBType::OK,
        )
        .expect("Failed to message_box_w"),
        DllReason::THREAD_ATTACH => { /* Does nothing */ }
        DllReason::THREAD_DETACH => { /* Does nothing */ }
    }
    true
}
