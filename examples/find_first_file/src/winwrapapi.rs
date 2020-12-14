use winwrap::prelude::*;
use winwrap::OsError::Win32;
use winwrap::winapi::shared::winerror::ERROR_NO_MORE_FILES;

fn enumerate_files_a() {
    let path = AString::from_str_lossy(r".\*.*");
    let (handle, mut data) = find_first_file_a(&path).unwrap();
    loop {
        println!("name: {:?}", data.get_file_name().to_string_lossy());
        println!("\tflags: {:?}", data.file_attributes);
        println!("\talternate file name: {:?}", data.get_alternate_file_name().to_string_lossy());
        println!("----------------------------");
        data = match find_next_file_a(&handle) {
            Ok(x) => x,
            Err(Win32(ERROR_NO_MORE_FILES)) => {
                println!("All files enumerated!");
                break;
            }
            Err(x) => panic!("Unknown Error: {}", x),
        };
    }
}

fn enumerate_files_w() {
    let path = WString::from_str_lossy(r".\*.*");
    let (handle, mut data) = find_first_file_w(&path).unwrap();
    loop {
        println!("name: {:?}", data.get_file_name().to_string_lossy());
        println!("\tflags: {:?}", data.file_attributes);
        println!("\talternate file name: {}", data.get_alternate_file_name().to_string_lossy());
        println!("----------------------------");
        data = match find_next_file_w(&handle) {
            Ok(x) => x,
            Err(Win32(ERROR_NO_MORE_FILES)) => {
                println!("All files enumerated!");
                break;
            }
            Err(x) => panic!("Unknown Error: {}", x),
        };
    }
}

fn enumerate_files_t() {
    let path = TString::from_str_lossy(r".\*.*");
    let (handle, mut data) = find_first_file(&path).unwrap();
    loop {
        println!("name: {:?}", data.get_file_name().to_string());
        println!("\tflags: {:?}", data.file_attributes);
        println!("\talternate file name: {}", data.get_alternate_file_name().to_string_lossy());
        println!("----------------------------");
        data = match find_next_file(&handle) {
            Ok(x) => x,
            Err(Win32(ERROR_NO_MORE_FILES)) => {
                println!("All files enumerated!");
                break;
            }
            Err(x) => panic!("Unknown Error: {}", x),
        };
    }
}

fn main() {
    println!("***** Enumerate files [A] *****");
    enumerate_files_a();
    println!("\n***** Enumerate files [W] *****");
    enumerate_files_w();
    println!("\n***** Enumerate files [T] *****");
    enumerate_files_t();
}
