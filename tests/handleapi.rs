#[cfg(test)]
pub mod handleapi_tests {
    use winwrap::um::handleapi::*;
    use winwrap::um::processthreadsapi::get_current_process;

    #[test]
    fn test_get_handle_information() {
        let proc1 = get_current_process();
        let proc2 = get_current_process();
        let info1 = get_handle_information(&proc1);
        let info2 = get_handle_information(&proc2);
        assert_eq!(info1, info2);
    }
}
