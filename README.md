# Winwrap
Rust-friendly Windows API Wrappers.

# Features

- Safe Windows API bindings
- Unsafe raw APIs
    - Unsafe raw APIs wrap only the error handling.
- TCHAR and TString support.
    - By default, TCHAR is WCHAR. If you want to use ANSI, ansi feature on.

```
[dependencies.winwrap]
version = "0.1.0"
features = ["ansi"]
```

# Examples

```rust
use winwrap::um::fileapi::*;
use winwrap::winapi::shared::winerror::ERROR_NO_MORE_FILES;

fn enumerate_files_w() {
    use winwrap::string::WString;
    let path = WString::from(r".\*.*");
    let (handle, mut data) = find_first_file_w(&path).unwrap();
    loop {
        println!("name: {:?}", data.get_file_name().to_string_lossy());
        println!("\tflags: {:?}", data.file_attributes);
        println!("\talternate file name: {}", data.get_alternate_file_name().to_string_lossy());
        println!("----------------------------");
        data = match find_next_file_w(&handle) {
            Ok(x) => x,
            Err(ERROR_NO_MORE_FILES) => {
                println!("All files enumerated!");
                break;
            }
            Err(x) => panic!("Unknown Error: {}", x),
        };
    }
}

fn main(){
    enumerate_files_w();
}
```

# License
This software is released under the MIT License, see LICENSE.
