use crate::*;
use crate::raw::um::verrsrc::VSFixedFileInfo;
use crate::raw::um::winver::*;
use crate::string::*;
use std::mem::{ManuallyDrop, size_of};
use std::ptr::null_mut;
use winapi::shared::minwindef::{LPCVOID, LPVOID};
use winwrap_derive::*;

make_struct! {LANGANDCODEPAGE,
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct LangAndCodePage{
    pub language:WORD,
    pub code_page:WORD,
}}

#[repr(C)]
pub struct FileVersionInfo {
    inner: Vec<u8>,
}

impl FileVersionInfo {
    pub fn new_a(file_name: &AStr) -> OsResult<Self> {
        let len = get_file_version_info_size_a(file_name)?;
        get_file_version_info_a(file_name, len)
    }

    pub fn new_w(file_name: &WStr) -> OsResult<Self> {
        let len = get_file_version_info_size_w(file_name)?;
        get_file_version_info_w(file_name, len)
    }

    #[inline]
    pub fn get_data(&self) -> &Vec<u8> { &self.inner }

    #[inline]
    unsafe fn _new(v: Vec<u8>) -> Self {
        Self { inner: v }
    }

    #[inline]
    pub fn as_c_ptr(&self) -> LPCVOID { self.inner.as_ptr() as *const _ }
}

#[ansi_fn]
pub fn get_file_version_info_size_a(
    file_name: &AStr,
) -> OsResult<DWORD> {
    unsafe {
        GetFileVersionInfoSizeA(
            file_name.as_ptr(),
            null_mut(),
        )
    }
}

#[unicode_fn]
pub fn get_file_version_info_size_w(
    file_name: &WStr,
) -> OsResult<DWORD> {
    unsafe {
        GetFileVersionInfoSizeW(
            file_name.as_ptr(),
            null_mut(),
        )
    }
}

#[ansi_fn]
pub fn get_file_version_info_a(
    file_name: &AStr,
    len: DWORD,
) -> OsResult<FileVersionInfo> {
    unsafe {
        let mut data: Vec<u8> = Vec::with_capacity(len as usize);
        GetFileVersionInfoA(
            file_name.as_ptr(),
            0, // ignored
            data.capacity() as DWORD,
            data.as_mut_ptr() as *mut _,
        )?;
        Ok(FileVersionInfo::_new(data))
    }
}

#[unicode_fn]
pub fn get_file_version_info_w(
    file_name: &WStr,
    len: DWORD,
) -> OsResult<FileVersionInfo> {
    unsafe {
        let mut data: Vec<u8> = Vec::with_capacity(len as usize);
        GetFileVersionInfoW(
            file_name.as_ptr(),
            0, // ignored
            data.capacity() as DWORD,
            data.as_mut_ptr() as *mut _,
        )?;
        Ok(FileVersionInfo::_new(data))
    }
}

#[repr(C)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum VerQuerySubBlock {
    /// `\`
    Root,
    /// `\VarFileInfo\Translation`
    VarFileInfo,
    /// `\StringFileInfo\lang-codepage\string-name`
    StringFileInfo(WORD, WORD, String),
}

impl VerQuerySubBlock {
    pub fn as_c_sub_block_a(&self) -> AString {
        match self {
            Self::Root => AString::from_str_lossy(r"\"),
            Self::VarFileInfo => AString::from_str_lossy(r"\VarFileInfo\Translation"),
            Self::StringFileInfo(lang, codepage, string_name) => {
                AString::from_str_lossy(
                    &format!(r"\StringFileInfo\{:04x}{:04x}\{}", lang, codepage, string_name).as_str()
                )
            }
        }
    }

    pub fn as_c_sub_block_w(&self) -> WString {
        match self {
            Self::Root => WString::from_str_lossy(r"\"),
            Self::VarFileInfo => WString::from_str_lossy(r"\VarFileInfo\Translation"),
            Self::StringFileInfo(lang, codepage, string_name) => {
                WString::from_str_lossy(
                    &format!(r"\StringFileInfo\{:04x}{:04x}\{}", lang, codepage, string_name).as_str()
                )
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum VerQueryValueA<'a> {
    Root(&'a VSFixedFileInfo),
    VarFileInfo(&'a LangAndCodePage),
    StringFileInfo(ManuallyDrop<AString>),
}

impl<'a> VerQueryValueA<'a> {
    unsafe fn from_raw(buf: LPVOID, len: u32, block: &VerQuerySubBlock) -> Self {
        match block {
            VerQuerySubBlock::Root => {
                assert_eq!(len, size_of::<VSFixedFileInfo>() as u32);
                Self::Root(&*(buf as *mut VSFixedFileInfo))
            }
            VerQuerySubBlock::VarFileInfo => {
                assert_eq!(len, size_of::<LangAndCodePage>() as u32);
                Self::VarFileInfo(&*(buf as *mut LangAndCodePage))
            }
            VerQuerySubBlock::StringFileInfo(_, _, _) => {
                let buf = buf as *mut u8;
                Self::StringFileInfo(ManuallyDrop::new(
                    AString::from_raw_s(buf as *mut _, len as usize)
                ))
            }
        }
    }

    /// # Examples
    ///
    /// ```
    /// use winwrap::um::winver::*;
    /// use winwrap::string::*;
    /// let file_name = AString::from_str_lossy("ntdll.dll");
    /// let len = get_file_version_info_size_a(&file_name).unwrap();
    /// let x = get_file_version_info_a(&file_name, len).unwrap();
    /// let root = ver_query_value_a(&x, VerQuerySubBlock::Root).unwrap();
    /// println!("{:?}", root.as_root());
    /// ```
    #[inline]
    pub fn as_root(&self) -> &VSFixedFileInfo {
        match self {
            VerQueryValueA::Root(x) => x,
            x => panic!("The item is not Root: {:?}", x),
        }
    }

    /// # examples
    ///
    /// ```
    /// use winwrap::um::winver::*;
    /// use winwrap::string::*;
    /// let file_name = AString::from_str_lossy("ntdll.dll");
    /// let len = get_file_version_info_size_a(&file_name).unwrap();
    /// let x = get_file_version_info_a(&file_name, len).unwrap();
    /// let vfi = ver_query_value_a(&x, VerQuerySubBlock::VarFileInfo).unwrap();
    /// println!("{:?}", vfi.as_var_file_info());
    /// ```
    #[inline]
    pub fn as_var_file_info(&self) -> &LangAndCodePage {
        match self {
            Self::VarFileInfo(x) => x,
            x => panic!("The item is not Root: {:?}", x),
        }
    }

    /// # examples
    ///
    /// ```
    /// use winwrap::um::winver::*;
    /// use winwrap::string::*;
    /// let file_name = AString::from_str_lossy("ntdll.dll");
    /// let len = get_file_version_info_size_a(&file_name).unwrap();
    /// let x = get_file_version_info_a(&file_name, len).unwrap();
    /// let vfi = ver_query_value_a(&x, VerQuerySubBlock::VarFileInfo).unwrap();
    /// let vfi = vfi.as_var_file_info();
    /// let (language, code_page) = (vfi.language, vfi.code_page);
    /// let sfi = ver_query_value_a(
    /// &x,
    /// VerQuerySubBlock::StringFileInfo(language, code_page, "ProductVersion".to_string()),
    /// ).unwrap();
    /// println!("{:?}", sfi.as_string_file_info().to_string_lossy());
    /// ```
    #[inline]
    pub fn as_string_file_info(&self) -> &ManuallyDrop<AString> {
        match self {
            Self::StringFileInfo(x) => x,
            x => panic!("The item is not StringFileInfo: {:?}", x),
        }
    }
}

#[repr(C)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum VerQueryValueW<'a> {
    Root(&'a VSFixedFileInfo),
    VarFileInfo(&'a LangAndCodePage),
    StringFileInfo(ManuallyDrop<WString>),
}

impl<'a> VerQueryValueW<'a> {
    unsafe fn from_raw(buf: LPVOID, len: u32, block: &VerQuerySubBlock) -> Self {
        match block {
            VerQuerySubBlock::Root => {
                assert_eq!(len, size_of::<VSFixedFileInfo>() as u32);
                Self::Root(&*(buf as *mut VSFixedFileInfo))
            }
            VerQuerySubBlock::VarFileInfo => {
                assert_eq!(len, size_of::<LangAndCodePage>() as u32);
                Self::VarFileInfo(&*(buf as *mut LangAndCodePage))
            }
            VerQuerySubBlock::StringFileInfo(_, _, _) => {
                let buf = buf as *mut u8;
                Self::StringFileInfo(ManuallyDrop::new(
                    WString::from_raw_s(buf as *mut _, len as usize)
                ))
            }
        }
    }

    /// # Examples
    ///
    /// ```
    /// use winwrap::um::winver::*;
    /// use winwrap::string::*;
    /// let file_name = WString::from_str_lossy("ntdll.dll");
    /// let len = get_file_version_info_size_w(&file_name).unwrap();
    /// let x = get_file_version_info_w(&file_name, len).unwrap();
    /// let root = ver_query_value_w(&x, VerQuerySubBlock::Root).unwrap();
    /// println!("{:?}", root.as_root());
    /// ```
    #[inline]
    pub fn as_root(&self) -> &VSFixedFileInfo {
        match self {
            Self::Root(x) => x,
            x => panic!("The item is not Root: {:?}", x),
        }
    }

    /// # examples
    ///
    /// ```
    /// use winwrap::um::winver::*;
    /// use winwrap::string::*;
    /// let file_name = WString::from_str_lossy("ntdll.dll");
    /// let len = get_file_version_info_size_w(&file_name).unwrap();
    /// let x = get_file_version_info_w(&file_name, len).unwrap();
    /// let vfi = ver_query_value_w(&x, VerQuerySubBlock::VarFileInfo).unwrap();
    /// println!("{:?}", vfi.as_var_file_info());
    /// ```
    #[inline]
    pub fn as_var_file_info(&self) -> &LangAndCodePage {
        match self {
            Self::VarFileInfo(x) => x,
            x => panic!("The item is not Root: {:?}", x),
        }
    }

    /// # examples
    ///
    /// ```
    /// use winwrap::um::winver::*;
    /// use winwrap::string::*;
    /// let file_name = WString::from_str_lossy("ntdll.dll");
    /// let len = get_file_version_info_size_w(&file_name).unwrap();
    /// let x = get_file_version_info_w(&file_name, len).unwrap();
    /// let vfi = ver_query_value_w(&x, VerQuerySubBlock::VarFileInfo).unwrap();
    /// let vfi = vfi.as_var_file_info();
    /// let (language, code_page) = (vfi.language, vfi.code_page);
    /// let sfi = ver_query_value_w(
    /// &x,
    /// VerQuerySubBlock::StringFileInfo(language, code_page, "ProductVersion".to_string()),
    /// ).unwrap();
    /// println!("{:?}", sfi.as_string_file_info().to_string_lossy());
    /// ```
    #[inline]
    pub fn as_string_file_info(&self) -> &ManuallyDrop<WString> {
        match self {
            VerQueryValueW::StringFileInfo(x) => x,
            x => panic!("The item is not StringFileInfo: {:?}", x),
        }
    }
}

#[ansi_fn]
pub fn ver_query_value_a<'a>(
    block: &FileVersionInfo,
    sub_block: VerQuerySubBlock,
) -> OsResult<VerQueryValueA> {
    unsafe {
        let sb = sub_block.as_c_sub_block_a();
        let mut buf = null_mut();
        let mut len = 0;
        VerQueryValueA(
            block.as_c_ptr(),
            sb.as_c_str().as_ptr(),
            &mut buf,
            &mut len,
        )?;
        Ok(VerQueryValueA::from_raw(buf, len, &sub_block))
    }
}

#[unicode_fn]
pub fn ver_query_value_w<'a>(
    block: &FileVersionInfo,
    sub_block: VerQuerySubBlock,
) -> OsResult<VerQueryValueW> {
    unsafe {
        let sb = sub_block.as_c_sub_block_w();
        let mut buf = null_mut();
        let mut len = 0;
        VerQueryValueW(
            block.as_c_ptr(),
            sb.as_c_str().as_ptr(),
            &mut buf,
            &mut len,
        )?;
        Ok(VerQueryValueW::from_raw(buf, len, &sub_block))
    }
}

#[ansi_fn]
pub fn ver_language_name_a(
    lang: DWORD,
) -> OsResult<AString>
{
    unsafe {
        let mut v: Vec<u8> = Vec::with_capacity(128);
        let len = VerLanguageNameA(
            lang,
            v.as_mut_ptr() as *mut _,
            128)?;
        v.set_len(len as usize);
        Ok(AString::new_unchecked(v))
    }
}

#[unicode_fn]
pub fn ver_language_name_w(
    lang: DWORD,
) -> OsResult<WString>
{
    unsafe {
        let mut v: Vec<u16> = Vec::with_capacity(128);
        let len = VerLanguageNameW(
            lang,
            v.as_mut_ptr() as *mut _,
            128)?;
        v.set_len(len as usize);
        Ok(WString::new_unchecked(v))
    }
}
