#[cfg(test)]
pub mod tests {
    use winwrap::string::*;
    use winwrap::um::processenv::get_environment_variable_a;
    use winwrap::winapi::shared::winerror::ERROR_ENVVAR_NOT_FOUND;
    use winapi::shared::winerror::ERROR_NO_UNICODE_TRANSLATION;

    #[test]
    fn test_wstring() {
        let x = WString::new(vec![]); // empty vec
        assert_eq!(&[0x00], x.as_bytes_with_nul());
        let x = WString::new(vec![0x00]); // empty vec
        assert_eq!(&[0x00], x.as_bytes_with_nul());
        let x = WString::new(vec![0x74, 0x65, 0x73, 0x74]); // test
        assert_eq!(&[0x74, 0x65, 0x73, 0x74, 0x00], x.as_bytes_with_nul());
        let x = WString::new(vec![0x74, 0x65, 0x73, 0x74, 0x00]); // test\0
        assert_eq!(&[0x74, 0x65, 0x73, 0x74, 0x00], x.as_bytes_with_nul());
        let x = WString::new(vec![0x74, 0x65, 0x00, 0x73, 0x74]); // te\0st
        assert_eq!(&[0x74, 0x65, 0x00], x.as_bytes_with_nul());
        let x = WString::new_c(vec![0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74, 0x00]); // test
        assert_eq!(&[0x74, 0x65, 0x73, 0x74, 0x00], x.as_bytes_with_nul());
        let x = WString::new_c(vec![0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74, 0x00, 0x00]); // test\0 (odd)
        assert_eq!(&[0x74, 0x65, 0x73, 0x74, 0x00], x.as_bytes_with_nul());
        let x = WString::new_c(vec![0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74, 0x00, 0x00, 0x00]); // test\0\0 (even)
        assert_eq!(&[0x74, 0x65, 0x73, 0x74, 0x00], x.as_bytes_with_nul());
        assert_eq!(x.as_bytes_with_nul(), x.as_c_str().to_bytes_with_nul());
        assert_eq!(x, WString::from("test"));
        assert_ne!(x, WString::from("Test"));
        let x = &mut [0x0074, 0x0065, 0x0073, 0x0074, 0x0000];
        unsafe {
            assert_eq!("test", WString::from_raw(x.as_mut_ptr()).to_string_lossy());
        }
    }

    #[test]
    fn test_astring() {
        let x = AString::new(vec![]); // empty vec
        assert_eq!(&[0x00], x.as_bytes_with_nul());
        let x = AString::new(vec![0x74, 0x65, 0x73, 0x74]); // test
        assert_eq!(&[0x74, 0x65, 0x73, 0x74, 0x00], x.as_bytes_with_nul());
        let x = AString::new(vec![0x74, 0x65, 0x73, 0x74, 0x00]); // test\0
        assert_eq!(&[0x74, 0x65, 0x73, 0x74, 0x00], x.as_bytes_with_nul());
        let x = AString::new(vec![0x74, 0x65, 0x00, 0x73, 0x74, 0x00]); // te\0st\0
        assert_eq!(&[0x74, 0x65, 0x00], x.as_bytes_with_nul());
        assert_eq!(x.as_bytes_with_nul(), x.as_c_str().to_bytes_with_nul());
        assert_eq!(x, AString::from("te"));
        assert_ne!(x, AString::from("Te"));
        let x = AString::new(vec![0xff]); // invalid byte
        assert_eq!("\u{f8f3}", &x.to_string_lossy());
        assert_eq!(Err(ERROR_NO_UNICODE_TRANSLATION), x.to_string());
        let x = &mut [0x74, 0x65, 0x73, 0x74, 0x00];
        unsafe {
            assert_eq!("test", AString::from_raw(x.as_mut_ptr()).to_string_lossy());
        }
    }

    #[test]
    fn test_string_conversion() {
        let s = WString::from("testテスト");
        assert_eq!("testテスト", s.to_string_lossy());
        let s = AString::from(s);
        assert_eq!("testテスト", s.to_string_lossy());
        let s = WString::from(s);
        assert_eq!("testテスト", s.to_string_lossy());
        let s = WString::from("");
        assert_eq!(&[0x00], s.as_bytes_with_nul());
        assert_eq!("", s.to_string_lossy());
        let s = AString::from(s);
        assert_eq!(&[0x00], s.as_bytes_with_nul());
        assert_eq!("", s.to_string_lossy());
        let s = WString::from(s);
        assert_eq!(&[0x00], s.as_bytes_with_nul());
        assert_eq!("", s.to_string_lossy());
    }

    #[test]
    fn test_processenv() {
        let name = AString::from("ThisDoesNotExist");
        let value = get_environment_variable_a(name.as_c_str());
        assert_eq!(Err(ERROR_ENVVAR_NOT_FOUND), value);
        let name = AString::from("NUMBER_OF_PROCESSORS");
        get_environment_variable_a(name.as_c_str()).expect("Failed to get the value");
    }
}
