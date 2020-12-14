// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use crate::*;
use winapi::STRUCT;

#[allow(non_camel_case_types)]
pub type tagVS_FIXEDFILEINFO = VS_FIXEDFILEINFO;

STRUCT! {#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_snake_case)]
struct VS_FIXEDFILEINFO{
  dwSignature: DWORD,
  dwStrucVersion: DWORD,
  dwFileVersionMS: DWORD,
  dwFileVersionLS: DWORD,
  dwProductVersionMS: DWORD,
  dwProductVersionLS: DWORD,
  dwFileFlagsMask: DWORD,
  dwFileFlags: DWORD,
  dwFileOS: DWORD,
  dwFileType: DWORD,
  dwFileSubtype: DWORD,
  dwFileDateMS: DWORD,
  dwFileDateLS: DWORD,
}}

make_struct! {VS_FIXEDFILEINFO,
#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VSFixedFileInfo {
    pub signature:DWORD,
    pub struct_version: DWORD,
    pub file_version_ms: DWORD,
    pub file_version_ls:DWORD,
    pub product_version_ms: DWORD,
    pub product_version_ls: DWORD,
    pub file_flags_mask: DWORD,
    pub file_flags: DWORD,
    pub file_os: DWORD,
    pub file_type: DWORD,
    pub file_sub_type: DWORD,
    pub file_date_ms:DWORD,
    pub file_date_ls:DWORD,
}}

impl VSFixedFileInfo {
    pub fn get_file_version(&self) -> u64 {
        MAKE_QWORD(self.file_version_ms, self.file_version_ls)
    }

    pub fn get_product_version(&self) -> u64 {
        MAKE_QWORD(self.product_version_ms, self.product_version_ls)
    }
}
