use winwrap::um::errhandlingapi::*;
use winwrap::um::winnt::ExceptionPointers;
use winwrap::vc::excpt::*;

/// The number of times veh_handler function was called.
static mut NUM: i32 = 0;

extern "system" fn veh_handler(x: &mut ExceptionPointers) -> ExceptionHandler {
    println!("In handler");
    unsafe { NUM += 1; }
    #[cfg(target_arch = "x86_64")]
        {
            println!("rip: 0x{:X}", x.context_record.Rip);
            x.context_record.Rip += 1;
        }
    #[cfg(target_arch = "x86")]
        {
            println!("eip: 0x{:X}", x.context_record.Eip);
            x.context_record.Eip += 1;
        }
    ExceptionHandler::CONTINUE_EXECUTION
}

fn main() {
    // add_vectored_exception_handler(true, veh_handler).unwrap(); // NG
    let _h = add_vectored_exception_handler(true, veh_handler).unwrap(); // OK
    // std::mem::forget(add_vectored_exception_handler(true, veh_handler).unwrap()); // OK
    let x = 0x4649;
    unsafe {
        println!("NUM1: 0x{:X}", NUM);
        *(x as *mut i32) = 0x4649; // ACCESS VIOLATION
        println!("NUM2: 0x{:X}", NUM);
    }
    println!("hello world!!");
}
