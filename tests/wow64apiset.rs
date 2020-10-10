#[cfg(test)]
pub mod wow64apiset_tests {
    use winwrap::um::processthreadsapi::get_current_process;
    use winwrap::um::wow64apiset::{is_wow64_process, is_wow64_process2, get_system_wow64_directory_a, get_system_wow64_directory_w};

    #[test]
    fn is_wow64_process_test() {
        let h = get_current_process();
        println!("{:?}", is_wow64_process(&h));
        println!("{:?}", is_wow64_process2(&h));
    }

    #[test]
    fn wow64_dir_test() {
        let a = get_system_wow64_directory_a();
        let w = get_system_wow64_directory_w();
        if let (Ok(a), Ok(w)) = (&a, &w) {
            assert_eq!(a.to_string_lossy(), w.to_string_lossy());
        } else if let (Err(a), Err(w)) = (&a, &w) {
            assert_eq!(a, w);
        } else {
            panic!("{:?} != {:?}", a, w);
        }
    }
}
