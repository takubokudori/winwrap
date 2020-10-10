#[cfg(test)]
pub mod namedpipeapi_tests {
    use winwrap::um::fileapi::{write_file, read_file};
    use winwrap::um::namedpipeapi::*;

    #[test]
    fn rw_test() {
        let (read_pipe, write_pipe) = create_pipe(None, 0).unwrap();
        write_file(&write_pipe, &[1, 2, 3], None).unwrap();
        let mut buf = [0; 3];
        let r = read_file(&read_pipe, &mut buf, None).unwrap();
        assert_eq!(3, r);
        assert_eq!([1, 2, 3], buf);
        write_file(&write_pipe, &[1, 2, 3], None).unwrap();
        write_file(&write_pipe, &[5, 6, 7, 8], None).unwrap();
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
