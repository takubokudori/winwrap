// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
#[cfg(test)]
pub mod wow64apiset_tests {
    use winwrap::um::{
        processthreadsapi::get_current_process,
        wow64apiset::{
            get_system_wow64_directory_a, get_system_wow64_directory_w,
            is_wow64_process, is_wow64_process2,
        },
    };

    #[test]
    fn test_is_wow64_process() {
        let h = get_current_process();
        let _wow1 = is_wow64_process(&h);
        let _wow2 = is_wow64_process2(&h);
    }

    #[test]
    fn test_wow64_dir() {
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
