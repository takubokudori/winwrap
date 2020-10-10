#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum ExceptionHandler {
    /// EXCEPTION_EXECUTE_HANDLER
    EXECUTE_HANDLER = 1,
    /// EXCEPTION_CONTINUE_SEARCH
    CONTINUE_SEARCH = 0,
    /// EXCEPTION_CONTINUE_EXECUTION
    CONTINUE_EXECUTION = -1,
}
