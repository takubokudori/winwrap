// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
#[cfg(test)]
pub mod tests {
    use winwrap::string::*;
    use winwrap::um::processenv::get_environment_variable_a;
    use winwrap::winapi::shared::winerror::ERROR_ENVVAR_NOT_FOUND;
    use winwrap::{OsError, MAKE_QWORD, SEP_QWORD};
    use winwrap::OsError::Win32;

    #[test]
    fn test_qw() {
        assert_eq!(0x1, MAKE_QWORD(0, 1));
        assert_eq!(0xFFFFFFFF_FFFFFFFF, MAKE_QWORD(0xFFFFFFFF, 0xFFFFFFFF));
        assert_eq!(0x1_00000000, MAKE_QWORD(1, 0));
        assert_eq!(0x1_00000001, MAKE_QWORD(1, 1));
        assert_eq!((0, 1), SEP_QWORD(1));
        assert_eq!((1, 0), SEP_QWORD(0x1_0000_0000));
        assert_eq!((0xDEADBEEF, 0xCAFEBABE), SEP_QWORD(0xDEADBEEF_CAFEBABE));
        assert_eq!((0xFFFFFFFF, 0xFFFFFFFF), SEP_QWORD(0xFFFFFFFF_FFFFFFFF));
    }

    #[test]
    #[allow(overflowing_literals)]
    fn test_os_error() {
        macro_rules! ios { ($x:expr) => { std::io::Error::from_raw_os_error($x) }}
        macro_rules! win { ($x:expr) => { OsError::Win32($x) }}
        macro_rules! nts { ($x:expr) => { OsError::NtStatus($x) }}
        assert_eq!(win!(1), ios!(1));
        assert_ne!(nts!(1), ios!(1));
        assert_eq!(nts!(0), ios!(0));
        assert_eq!(nts!(0xC00000B0), ios!(233));
    }

    #[test]
    fn test_processenv() {
        let name = AString::from_str_lossy("ThisDoesNotExist");
        let value = get_environment_variable_a(name.as_c_str());
        assert_eq!(Err(Win32(ERROR_ENVVAR_NOT_FOUND)), value);
        let name = AString::from_str_lossy("NUMBER_OF_PROCESSORS");
        get_environment_variable_a(name.as_c_str()).expect("Failed to get the value");
    }
}
