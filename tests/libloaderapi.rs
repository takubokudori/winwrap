// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
#[cfg(test)]
pub mod libloaderapi_tests {
    use winwrap::um::libloaderapi::{
        get_module_file_name_a, get_module_file_name_w,
    };

    #[test]
    fn test_get_module_name() {
        assert_eq!(
            get_module_file_name_a(None).unwrap().to_string_lossy(),
            get_module_file_name_w(None).unwrap().to_string_lossy()
        );
    }
}
