// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
#[cfg(test)]
pub mod namedpipeapi_tests {
    use winwrap::um::fileapi::{write_file, read_file};
    use winwrap::um::namedpipeapi::*;

    #[test]
    fn test_pipe() {
        let (read_pipe, write_pipe) = create_pipe(None, 0).unwrap();
        write_file(&write_pipe, &[1u8, 2, 3], None).unwrap();
        let mut buf = [0u8; 3];
        let r = read_file(&read_pipe, &mut buf, None).unwrap();
        assert_eq!(3, r);
        assert_eq!([1, 2, 3], buf);
        write_file(&write_pipe, &[1u8, 2, 3], None).unwrap();
        write_file(&write_pipe, &[5u8, 6, 7, 8], None).unwrap();
        let r = read_file(&read_pipe, &mut buf, None).unwrap();
        assert_eq!(3, r);
        assert_eq!([1, 2, 3], buf);
        let r = read_file(&read_pipe, &mut buf, None).unwrap();
        assert_eq!(3, r);
        assert_eq!([5, 6, 7], buf);
        let r = read_file(&read_pipe, &mut buf, None).unwrap();
        assert_eq!(1, r);
        assert_eq!([8, 6, 7], buf);
    }
}
