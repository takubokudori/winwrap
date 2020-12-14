// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use crate::handle::*;
use crate::shared::minwindef::FileTime;
use crate::string::{WString, AString};
use crate::um::fileapi::{FileAttributeConstants};
use std::mem::{ManuallyDrop, MaybeUninit};
use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::minwindef::{WORD, DWORD, BYTE, MAX_PATH};
use winapi::um::minwinbase::{SECURITY_ATTRIBUTES, WIN32_FIND_DATAA, WIN32_FIND_DATAW, OVERLAPPED};
use winapi::um::winnt::{SECURITY_DESCRIPTOR_CONTROL, CHAR, SID_IDENTIFIER_AUTHORITY, SID, ACL, SECURITY_DESCRIPTOR, WCHAR};

make_struct! {SID_IDENTIFIER_AUTHORITY,
#[derive(Debug, Default)]
pub struct SidIdentifierAuthority {
    pub value: [u8; 6],
}}

make_struct! {SID,
#[derive(Debug, Default)]
pub struct Sid {
    pub revision: BYTE,
    pub sub_authority_count: BYTE,
    pub identifier_authority: SidIdentifierAuthority,
    pub sub_authority: [DWORD; 1],
}}

make_struct! {ACL,
/// Access control list
#[derive(Debug, Default)]
pub struct Acl {
    pub acl_revision: BYTE,
    pub sbz1: BYTE,
    pub acl_size: WORD,
    pub ace_count: WORD,
    pub sbz2: WORD,
}}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct SecurityDescriptorControl: SECURITY_DESCRIPTOR_CONTROL{
        /// SE_OWNER_DEFAULTED
        const OWNER_DEFAULTED = winapi::um::winnt::SE_OWNER_DEFAULTED;
        /// SE_GROUP_DEFAULTED
        const GROUP_DEFAULTED = winapi::um::winnt::SE_GROUP_DEFAULTED;
        /// SE_DACL_PRESENT
        const DACL_PRESENT = winapi::um::winnt::SE_DACL_PRESENT;
        /// SE_DACL_DEFAULTED
        const DACL_DEFAULTED = winapi::um::winnt::SE_DACL_DEFAULTED;
        /// SE_SACL_PRESENT
        const SACL_PRESENT = winapi::um::winnt::SE_SACL_PRESENT;
        /// SE_SACL_DEFAULTED
        const SACL_DEFAULTED = winapi::um::winnt::SE_SACL_DEFAULTED;
        /// SE_DACL_AUTO_INHERIT_REQ
        const DACL_AUTO_INHERIT_REQ = winapi::um::winnt::SE_DACL_AUTO_INHERIT_REQ;
        /// SE_SACL_AUTO_INHERIT_REQ
        const SACL_AUTO_INHERIT_REQ = winapi::um::winnt::SE_SACL_AUTO_INHERIT_REQ;
        /// SE_DACL_AUTO_INHERITED
        const DACL_AUTO_INHERITED = winapi::um::winnt::SE_DACL_AUTO_INHERITED;
        /// SE_SACL_AUTO_INHERITED
        const SACL_AUTO_INHERITED = winapi::um::winnt::SE_SACL_AUTO_INHERITED;
        /// SE_DACL_PROTECTED
        const DACL_PROTECTED = winapi::um::winnt::SE_DACL_PROTECTED;
        /// SE_SAcL_PROTECTED
        const SACL_PROTECTED = winapi::um::winnt::SE_SACL_PROTECTED;
        /// SE_RM_CONTROL_VALID
        const RM_CONTROL_VALID = winapi::um::winnt::SE_RM_CONTROL_VALID;
        /// SE_SELF_RELATIVE
        const SELF_RELATIVE = winapi::um::winnt::SE_SELF_RELATIVE;
    }
}

make_struct! {SECURITY_DESCRIPTOR,
#[derive(Debug)]
pub struct SecurityDescriptor<'a> {
    pub revision: BYTE,
    pub sbz1: BYTE,
    pub control: SecurityDescriptorControl,
    pub owner: Option<&'a Sid>,
    pub group: Option<&'a Sid>,
    pub s_acl: Option<&'a Acl>,
    pub d_acl: Option<&'a Acl>,
}}

make_struct! {SECURITY_ATTRIBUTES,
#[derive(Debug)]
    pub struct SecurityAttributes<'a>{
        pub length: usize,
        pub security_descriptor: Option<&'a SecurityDescriptor<'a>>,
        pub inherit_handle: bool,
    }
}

make_struct! {WIN32_FIND_DATAA,
#[derive(Clone)]
pub struct Win32FindDataA {
    pub file_attributes: FileAttributeConstants,
    pub creation_tile: FileTime,
    pub last_access_time: FileTime,
    pub last_write_time: FileTime,
    pub file_size_high: DWORD,
    pub file_size_low: DWORD,
    pub reserved0: DWORD,
    pub reserved1: DWORD,
    file_name: [CHAR; MAX_PATH],
    alternate_file_name: [CHAR; 14],
}}

impl Win32FindDataA {
    pub fn get_file_name(&mut self) -> ManuallyDrop<AString> {
        unsafe {
            ManuallyDrop::new(AString::from_raw(self.file_name.as_mut_ptr() as *mut _))
        }
    }

    pub fn get_alternate_file_name(&mut self) -> ManuallyDrop<AString> {
        unsafe {
            ManuallyDrop::new(AString::from_raw(self.alternate_file_name.as_mut_ptr() as *mut _))
        }
    }

    pub fn get_file_size(&self) -> u64 {
        MAKE_QWORD(self.file_size_high, self.file_size_low)
    }
}

impl Default for Win32FindDataA {
    fn default() -> Self {
        unsafe {
            let mut this = Self {
                file_attributes: FileAttributeConstants::empty(),
                creation_tile: Default::default(),
                last_access_time: Default::default(),
                last_write_time: Default::default(),
                file_size_high: 0,
                file_size_low: 0,
                reserved0: 0,
                reserved1: 0,
                file_name: MaybeUninit::uninit().assume_init(),
                alternate_file_name: MaybeUninit::uninit().assume_init(),
            };
            *this.file_name.get_unchecked_mut(0) = 0; // null-terminated.
            *this.alternate_file_name.get_unchecked_mut(0) = 0; // null-terminated.
            this
        }
    }
}

make_struct! {WIN32_FIND_DATAW,
#[derive(Clone)]
pub struct Win32FindDataW {
    pub file_attributes: FileAttributeConstants,
    pub creation_tile: FileTime,
    pub last_access_time: FileTime,
    pub last_write_time: FileTime,
    pub file_size_high: DWORD,
    pub file_size_low: DWORD,
    pub reserved0: DWORD,
    pub reserved1: DWORD,
    pub file_name: [WCHAR; MAX_PATH],
    pub alternate_file_name: [WCHAR; 14],
}}

impl Win32FindDataW {
    pub fn get_file_name(&mut self) -> ManuallyDrop<WString> {
        unsafe {
            ManuallyDrop::new(WString::from_raw(self.file_name.as_mut_ptr() as *mut _))
        }
    }

    pub fn get_alternate_file_name(&mut self) -> ManuallyDrop<WString> {
        unsafe {
            ManuallyDrop::new(WString::from_raw(self.alternate_file_name.as_mut_ptr() as *mut _))
        }
    }

    pub fn get_file_size(&self) -> u64 {
        MAKE_QWORD(self.file_size_high, self.file_size_low)
    }
}

impl Default for Win32FindDataW {
    fn default() -> Self {
        unsafe {
            let mut this = Self {
                file_attributes: FileAttributeConstants::empty(),
                creation_tile: Default::default(),
                last_access_time: Default::default(),
                last_write_time: Default::default(),
                file_size_high: 0,
                file_size_low: 0,
                reserved0: 0,
                reserved1: 0,
                file_name: MaybeUninit::uninit().assume_init(),
                alternate_file_name: MaybeUninit::uninit().assume_init(),
            };
            *this.file_name.get_unchecked_mut(0) = 0; // null-terminated.
            *this.alternate_file_name.get_unchecked_mut(0) = 0; // null-terminated.
            this
        }
    }
}

make_struct! {OVERLAPPED,
#[derive(Debug, Default)]
pub struct Overlapped {
    pub internal: ULONG_PTR,
    pub internal_high: ULONG_PTR,
    pub offset: DWORD,
    pub offset_high:DWORD,
    // pointer: LPVOID, // union
    pub event: EventHandle,
}}
