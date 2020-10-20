use crate::*;
use crate::um::stringapiset::{wide_char_to_multi_byte, CodePage, WCFlags, multi_byte_to_wide_char, MBFlags};
use std::cmp::Ordering;
use std::ops;
use std::string::FromUtf16Error;
use winapi::ctypes::wchar_t;

#[cfg(not(feature = "ansi"))]
pub type TString = WString;

#[cfg(not(feature = "ansi"))]
pub type TStr = WStr;

#[cfg(feature = "ansi")]
pub type TString = AString;

#[cfg(feature = "ansi")]
pub type TStr = AString;

#[allow(non_snake_case)]
extern "C" {
    pub fn wcslen(s: *const wchar_t) -> usize;

    pub fn strlen(s: *const u8) -> usize;

    pub fn wcsnlen(s: *const wchar_t, len: usize) -> usize;

    pub fn strnlen(s: *const u8, len: usize) -> usize;
}

fn wide_char_to_multi_byte_wrap(
    code_page: CodePage,
    wc_flags: WCFlags,
    x: &[u16],
) -> OsResult<Vec<u8>> {
    // get the required buffer size.
    let l = wide_char_to_multi_byte(
        code_page,
        wc_flags,
        x,
        &mut [],
        None,
        None,
    )?;
    let mut ret: Vec<u8> = Vec::with_capacity(l);
    unsafe {
        ret.set_len(l);

        let l2 = wide_char_to_multi_byte(
            code_page,
            wc_flags,
            x,
            ret.as_mut_slice(),
            None,
            None,
        )?;
        assert_eq!(l, l2);
    }
    Ok(ret)
}

fn multi_byte_to_wide_char_wrap(
    code_page: CodePage,
    mb_flags: MBFlags,
    x: &[u8],
) -> OsResult<Vec<u16>> {
    let l = multi_byte_to_wide_char(
        code_page,
        mb_flags,
        x,
        &mut [],
    )?;
    let mut ret: Vec<u16> = Vec::with_capacity(l);
    unsafe {
        ret.set_len(l);

        let l2 = multi_byte_to_wide_char(
            code_page,
            mb_flags,
            x,
            ret.as_mut_slice(),
        )?;
        assert_eq!(l, l2);
    }
    Ok(ret)
}

#[repr(C)]
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct WString {
    inner: Box<[wchar_t]>,
}

impl WString {
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u16] { &self.inner }

    #[inline]
    pub fn as_bytes(&self) -> &[u16] { &self.as_bytes_with_nul()[..self.inner.len() - 1] }

    #[inline]
    pub fn as_u8_bytes_with_nul(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner.as_ptr() as *const u8, self.inner.len() * 2) }
    }

    #[inline]
    pub fn as_u8_bytes(&self) -> &[u8] { &self.as_u8_bytes_with_nul()[..self.inner.len() - 2] }

    #[inline]
    pub fn as_c_str(&self) -> &WStr { &*self }

    #[inline]
    pub fn as_mut_c_str(&mut self) -> &mut WStr { &mut *self }

    #[inline]
    pub fn as_ptr(&self) -> *const u16 { self.inner.as_ptr() }

    #[inline]
    pub fn len(&self) -> usize { self.inner.len() }

    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(self.as_bytes())
    }

    pub fn to_string_lossy(&self) -> String { String::from_utf16_lossy(self.as_bytes()) }

    #[inline]
    pub fn new<T: Into<Vec<u16>>>(v: T) -> Self { Self::_new(v.into()) }

    #[inline]
    pub fn new_c<T: Into<Vec<u8>>>(v: T) -> Self { Self::_new2(v.into()) }

    #[inline]
    fn _new(mut v: Vec<u16>) -> Self {
        unsafe {
            let len = wcsnlen(v.as_ptr(), v.len());
            if len == v.len() {
                // append NULL.
                v.reserve_exact(1);
                v.push(0);
            }
            v.set_len(len + 1);
            Self::new_nul_unchecked(v)
        }
    }

    fn from_astr(x: &AStr) -> OsResult<Self> {
        let wc = multi_byte_to_wide_char_wrap(
            CodePage::ACP,
            MBFlags::PRECOMPOSED | MBFlags::ERR_INVALID_CHARS,
            x.to_bytes_with_nul(),
        )?;
        Ok(Self::_new(wc))
    }

    fn from_astr_lossy(x: &AStr) -> Self {
        let wc = multi_byte_to_wide_char_wrap(
            CodePage::ACP,
            MBFlags::PRECOMPOSED,
            x.to_bytes_with_nul(),
        ).unwrap();
        Self::_new(wc)
    }

    #[inline]
    fn _new2(mut v: Vec<u8>) -> Self {
        if v.len() & 1 == 1 { v.push(0); } // Make the length even.
        unsafe {
            let v = v.leak();
            let x = Vec::from_raw_parts(v.as_ptr() as *mut u16, v.len() / 2, v.len() / 2);
            Self::_new(x)
        }
    }

    /// # Safety
    /// `v` must be a null-terminated UNICODE string.
    pub unsafe fn new_nul_unchecked(v: Vec<u16>) -> Self {
        Self { inner: v.into_boxed_slice() }
    }

    /// # Safety
    /// `ptr` must be a null-terminated UNICODE string.
    pub unsafe fn from_raw(ptr: *mut wchar_t) -> Self {
        let len = wcslen(ptr);
        let slice = std::slice::from_raw_parts_mut(ptr, len as usize + 1);
        Self { inner: Box::from_raw(slice) }
    }

    pub unsafe fn from_raw_s(ptr: *mut u16, len: usize) -> Self {
        let v = Vec::from_raw_parts(ptr, len, len);
        Self::_new(v)
    }
}

impl ops::Deref for WString {
    type Target = WStr;

    fn deref(&self) -> &Self::Target {
        unsafe { WStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }
}

impl ops::DerefMut for WString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { WStr::from_bytes_with_nul_unchecked_mut(self.as_bytes_with_nul()) }
    }
}

impl ops::Index<ops::RangeFull> for WString {
    type Output = WStr;

    #[inline]
    fn index(&self, _: ops::RangeFull) -> &Self::Output {
        self
    }
}

impl From<&AStr> for WString {
    /// Converts &AStr to WString. Panic if an input cannot be converted to WCHAR.
    fn from(x: &AStr) -> Self {
        let wc = multi_byte_to_wide_char_wrap(
            CodePage::ACP,
            MBFlags::PRECOMPOSED | MBFlags::ERR_INVALID_CHARS,
            x.to_bytes_with_nul(),
        ).unwrap();
        Self::_new(wc)
    }
}

impl From<AString> for WString {
    /// Converts AString to WString. Panic if an input cannot be converted to WCHAR.
    #[inline]
    fn from(x: AString) -> Self { Self::from(x.as_c_str()) }
}

impl From<Vec<u16>> for WString {
    fn from(x: Vec<u16>) -> Self { Self::_new(x) }
}

impl From<&str> for WString {
    fn from(x: &str) -> Self { Self::_new(x.encode_utf16().collect()) }
}

impl From<String> for WString {
    fn from(x: String) -> Self {
        Self::from(x.as_str())
    }
}

impl Drop for WString {
    fn drop(&mut self) {
        unsafe {
            *self.inner.as_mut_ptr() = 0;
            std::mem::forget(self)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct WStr {
    inner: [wchar_t],
}

impl WStr {
    #[inline]
    pub unsafe fn from_bytes_with_nul_unchecked(bytes: &[u16]) -> &Self {
        &*(bytes as *const [u16] as *const Self)
    }

    #[inline]
    pub unsafe fn from_bytes_with_nul_unchecked_mut(bytes: &[u16]) -> &mut Self {
        &mut *(bytes as *const [u16] as *mut Self)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const wchar_t { self.inner.as_ptr() }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut wchar_t { self.inner.as_mut_ptr() }

    #[inline]
    pub fn len(&self) -> usize { self.inner.len() }

    #[inline]
    pub fn to_bytes_with_nul(&self) -> &[u16] { &self.inner }

    pub fn to_bytes(&self) -> &[u16] {
        let bytes = self.to_bytes_with_nul();
        &bytes[..bytes.len() - 1]
    }

    #[inline]
    pub fn to_u8_bytes_with_nul(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner.as_ptr() as *const u8, self.inner.len() * 2) }
    }

    pub fn to_u8_bytes(&self) -> &[u8] {
        let bytes = self.to_u8_bytes_with_nul();
        &bytes[..bytes.len() - 2]
    }
}

impl PartialEq for WStr {
    fn eq(&self, other: &Self) -> bool { self.to_bytes().eq(other.to_bytes()) }
}

impl Eq for WStr {}

impl PartialOrd for WStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_bytes().partial_cmp(&other.to_bytes())
    }
}

impl Ord for WStr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_bytes().cmp(&other.to_bytes())
    }
}

/// ANSI string
#[repr(C)]
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct AString {
    inner: Box<[u8]>,
}

impl From<&str> for AString {
    fn from(x: &str) -> Self {
        // UTF8 -> UNICODE -> ANSI
        let ws = WString::from(x);
        AString::from(ws)
    }
}

impl From<String> for AString {
    fn from(x: String) -> Self {
        Self::from(x.as_str())
    }
}

impl From<&WStr> for AString {
    fn from(x: &WStr) -> Self {
        let mb = wide_char_to_multi_byte_wrap(
            CodePage::ACP,
            WCFlags::empty(),
            x.to_bytes_with_nul(),
        ).unwrap();
        Self::_new(mb)
    }
}

impl From<WString> for AString {
    /// Converts WString to AString. Panic if an input cannot be converted to CHAR.
    fn from(x: WString) -> Self {
        Self::from(x.as_c_str())
    }
}

impl AString {
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u8] { &self.inner }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] { &self.as_bytes_with_nul()[..self.inner.len() - 1] }

    #[inline]
    pub fn as_c_str(&self) -> &AStr { &*self }

    #[inline]
    pub fn as_mut_c_str(&mut self) -> &mut AStr { &mut *self }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 { self.inner.as_ptr() }

    #[inline]
    pub fn len(&self) -> usize { self.inner.len() }

    pub fn to_string(&self) -> OsResult<String> {
        // ANSI -> UNICODE -> UTF8
        let x = WString::from_astr(self.as_c_str())?;
        unsafe {
            let mut mb = wide_char_to_multi_byte_wrap(
                CodePage::UTF8,
                WCFlags::ERR_INVALID_CHARS,
                x.as_bytes_with_nul(),
            )?;
            mb.set_len(mb.len() - 1); // remove NULL
            Ok(String::from_utf8_unchecked(mb))
        }
    }

    pub fn to_string_lossy(&self) -> String {
        // ANSI -> UNICODE -> UTF8
        let x = WString::from_astr_lossy(self.as_c_str());
        unsafe {
            let mut mb = wide_char_to_multi_byte_wrap(
                CodePage::UTF8,
                WCFlags::empty(),
                x.as_bytes_with_nul(),
            ).unwrap();
            mb.set_len(mb.len() - 1); // remove NULL
            String::from_utf8_unchecked(mb)
        }
    }

    #[inline]
    pub fn new(v: Vec<u8>) -> Self { Self::_new(v.into()) }

    /// Converts &str to AString without an encoding check.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use winapi::shared::minwindef::FARPROC;
    /// use winwrap::OsResult;
    /// use winwrap::string::*;
    /// use winwrap::um::libloaderapi::{get_proc_address, load_library_w, ProcAddress};
    ///
    /// /// It is assumed that a function name does not contains any multi-byte character.
    /// pub unsafe fn load_and_get_proc_address(dll_name: &str, func_name: &str) -> OsResult<FARPROC> {
    ///    let m = load_library_w(&WString::from(dll_name))?;
    ///    get_proc_address(&m, ProcAddress::ByName(&AString::from_str_unchecked(func_name)))
    /// }
    /// ```
    pub unsafe fn from_str_unchecked(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }

    fn _new(mut v: Vec<u8>) -> Self {
        unsafe {
            let len = strnlen(v.as_ptr(), v.len());
            if len == v.len() {
                v.reserve_exact(1);
                v.push(0);
            }
            v.set_len(len + 1);
            Self::new_nul_unchecked(v)
        }
    }

    /// # Safety
    /// `v` must be a null-terminated ANSI string.
    #[inline]
    pub fn new_nul_unchecked(v: Vec<u8>) -> Self {
        Self { inner: v.into_boxed_slice() }
    }

    /// # Safety
    /// `ptr` must be a null-terminated ANSI string.
    pub unsafe fn from_raw(ptr: *mut u8) -> Self {
        let len = strlen(ptr);
        let slice = std::slice::from_raw_parts_mut(ptr, len as usize + 1);
        Self { inner: Box::from_raw(slice) }
    }

    pub unsafe fn from_raw_s(ptr: *mut u8, len: usize) -> Self {
        let v = Vec::from_raw_parts(ptr, len, len);
        Self::_new(v)
    }
}

impl ops::Deref for AString {
    type Target = AStr;

    fn deref(&self) -> &Self::Target {
        unsafe { AStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }
}

impl ops::DerefMut for AString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { AStr::from_bytes_with_nul_unchecked_mut(self.as_bytes_with_nul()) }
    }
}

impl ops::Index<ops::RangeFull> for AString {
    type Output = AStr;

    #[inline]
    fn index(&self, _: ops::RangeFull) -> &Self::Output {
        self
    }
}

impl From<Vec<u8>> for AString {
    #[inline]
    fn from(x: Vec<u8>) -> Self { Self::new(x) }
}

impl Drop for AString {
    fn drop(&mut self) {
        unsafe {
            *self.inner.as_mut_ptr() = 0;
            std::mem::forget(self)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AStr {
    inner: [u8],
}

impl AStr {
    #[inline]
    pub unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &Self {
        &*(bytes as *const [u8] as *const Self)
    }

    #[inline]
    pub unsafe fn from_bytes_with_nul_unchecked_mut(bytes: &[u8]) -> &mut Self {
        &mut *(bytes as *const [u8] as *mut Self)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const i8 { self.inner.as_ptr() as *const i8 }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut i8 { self.inner.as_mut_ptr() as *mut i8 }

    #[inline]
    pub fn as_u8_ptr(&self) -> *const u8 { self.inner.as_ptr() }

    #[inline]
    pub fn as_mut_u8_ptr(&mut self) -> *mut u8 { self.inner.as_mut_ptr() }

    #[inline]
    pub fn len(&self) -> usize { self.inner.len() }

    #[inline]
    pub fn to_bytes_with_nul(&self) -> &[u8] { &self.inner }

    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        let bytes = self.to_bytes_with_nul();
        &bytes[..bytes.len() - 1]
    }
}

impl PartialEq for AStr {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes().eq(other.to_bytes())
    }
}

impl Eq for AStr {}

impl PartialOrd for AStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_bytes().partial_cmp(&other.to_bytes())
    }
}

impl Ord for AStr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_bytes().cmp(&other.to_bytes())
    }
}
