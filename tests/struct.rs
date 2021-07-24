// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
#[cfg(test)]
pub mod struct_size_tests {
    use winwrap::{
        raw::um::{verrsrc::*, winver::*},
        shared::minwindef::*,
        um::{
            jobapi2::JobObjectIoRateControlInformation, minwinbase::*,
            processthreadsapi::*, tlhelp32::*, winnt::*, winver::*,
        },
        winapi::{
            shared::minwindef::FILETIME,
            um::{
                jobapi2::JOBOBJECT_IO_RATE_CONTROL_INFORMATION,
                minwinbase::*,
                processthreadsapi::*,
                tlhelp32::{
                    HEAPENTRY32, HEAPLIST32, MODULEENTRY32, MODULEENTRY32W,
                    PROCESSENTRY32, PROCESSENTRY32W,
                },
                winnt::*,
            },
        },
    };

    macro_rules! assert_size {
        ($x:ty, $y:ty) => {
            assert_eq!(
                std::mem::size_of::<$x>(),
                std::mem::size_of::<$y>(),
                "Failed to size check: {} and {}",
                stringify!($x),
                stringify!($y)
            );
            assert_eq!(
                std::mem::align_of::<$x>(),
                std::mem::align_of::<$y>(),
                "Failed to align check: {} and {}",
                stringify!($x),
                stringify!($y)
            )
        };
    }

    #[test]
    fn test_struct_size() {
        // minwinbase
        assert_size!(Win32FindDataA, WIN32_FIND_DATAA);
        assert_size!(SidIdentifierAuthority, SID_IDENTIFIER_AUTHORITY);
        assert_size!(SecurityAttributes, SECURITY_ATTRIBUTES);
        assert_size!(Sid, SID);
        assert_size!(Acl, ACL);
        assert_size!(SecurityDescriptor, SECURITY_DESCRIPTOR);
        assert_size!(Overlapped, OVERLAPPED);

        // processthreadsapi
        assert_size!(StartupInfoA, STARTUPINFOA);
        assert_size!(ProcessInformation, PROCESS_INFORMATION);

        // winnt
        assert_size!(ProcessorNumber, PROCESSOR_NUMBER);
        assert_size!(GroupAffinity, GROUP_AFFINITY);
        assert_size!(ExceptionPointers, EXCEPTION_POINTERS);
        assert_size!(ExceptionRecord, EXCEPTION_RECORD);

        // minwindef
        assert_size!(FileTime, FILETIME);
        assert_size!(SystemTime, SYSTEMTIME);

        // tlhelp32
        assert_size!(HeapList32, HEAPLIST32);
        assert_size!(HeapEntry32, HEAPENTRY32);
        assert_size!(ProcessEntry32, PROCESSENTRY32);
        assert_size!(ProcessEntry32W, PROCESSENTRY32W);
        assert_size!(ModuleEntry32, MODULEENTRY32);
        assert_size!(ModuleEntry32W, MODULEENTRY32W);

        // jobapi2
        assert_size!(
            JobObjectIoRateControlInformation,
            JOBOBJECT_IO_RATE_CONTROL_INFORMATION
        );

        // verrsrc
        assert_size!(VSFixedFileInfo, VS_FIXEDFILEINFO);

        // winver
        assert_size!(LangAndCodePage, LANGANDCODEPAGE);
    }

    #[test]
    fn test_default() {
        let mut data = Win32FindDataA::default();
        assert_eq!(&"", &data.get_file_name().to_string_lossy());
        assert_eq!(&"", &data.get_alternate_file_name().to_string_lossy());
        let mut data = Win32FindDataW::default();
        assert_eq!(&"", &data.get_file_name().to_string());
        assert_eq!(&"", &data.get_alternate_file_name().to_string());
        let mut data = ProcessEntry32::default();
        assert_eq!(&"", &data.get_exe_file().to_string_lossy());
        let mut data = ProcessEntry32W::default();
        assert_eq!(&"", &data.get_exe_file().to_string_lossy());
    }
}
