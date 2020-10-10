use crate::*;
use crate::raw::um::stringapiset::*;
use std::ptr::{null, null_mut};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum CodePage {
    ACP,
    MACCP,
    OEMCP,
    SYMBOL,
    THREAD_ACP,
    UTF7,
    UTF8,
    CODEPAGE(u32),
}

impl From<u32> for CodePage {
    fn from(x: u32) -> Self {
        match x {
            winapi::um::winnls::CP_ACP => Self::ACP,
            winapi::um::winnls::CP_MACCP => Self::MACCP,
            winapi::um::winnls::CP_OEMCP => Self::OEMCP,
            winapi::um::winnls::CP_SYMBOL => Self::SYMBOL,
            winapi::um::winnls::CP_THREAD_ACP => Self::THREAD_ACP,
            winapi::um::winnls::CP_UTF7 => Self::UTF7,
            winapi::um::winnls::CP_UTF8 => Self::UTF8,
            e => Self::CODEPAGE(e),
        }
    }
}

impl Into<u32> for CodePage {
    fn into(self) -> u32 {
        match self {
            Self::ACP => winapi::um::winnls::CP_ACP,
            Self::MACCP => winapi::um::winnls::CP_MACCP,
            Self::OEMCP => winapi::um::winnls::CP_OEMCP,
            Self::SYMBOL => winapi::um::winnls::CP_SYMBOL,
            Self::THREAD_ACP => winapi::um::winnls::CP_THREAD_ACP,
            Self::UTF7 => winapi::um::winnls::CP_UTF7,
            Self::UTF8 => winapi::um::winnls::CP_UTF8,
            Self::CODEPAGE(x) => x,
        }
    }
}
bitflags::bitflags! {
    pub struct MBFlags: u32 {
        const ERR_INVALID_CHARS = winapi::um::winnls::MB_ERR_INVALID_CHARS;
        const USEGLYPHCHARS     = winapi::um::winnls::MB_USEGLYPHCHARS;
        const COMPOSITE         = winapi::um::winnls::MB_COMPOSITE;
        const PRECOMPOSED         = winapi::um::winnls::MB_PRECOMPOSED;
    }
}

/// If use_composite is true, MB_COMPOSITE is specified.
pub fn multi_byte_to_wide_char(
    code_page: CodePage,
    mb_flags: MBFlags,
    mb_bytes: &[u8],
    wc_bytes: &mut [u16],
) -> OsResult<usize> {
    unsafe {
        MultiByteToWideChar(
            code_page.into(),
            mb_flags.bits,
            mb_bytes.as_ptr() as *const i8,
            mb_bytes.len() as i32,
            wc_bytes.as_mut_ptr(),
            wc_bytes.len() as i32,
        ).and_then(|x| Ok(x as usize))
    }
}

bitflags::bitflags! {
    pub struct WCFlags: u32 {
        const COMPOSITECHECK = winapi::um::winnls::WC_COMPOSITECHECK;
        const DISCARDNS = winapi::um::winnls::WC_DISCARDNS;
        const SEPCHARS = winapi::um::winnls::WC_SEPCHARS;
        const DEFAULTCHAR = winapi::um::winnls::WC_DEFAULTCHAR;
        const ERR_INVALID_CHARS     = winapi::um::winnls::WC_ERR_INVALID_CHARS;
        const NO_BEST_FIT_CHARS     = winapi::um::winnls::WC_NO_BEST_FIT_CHARS;
    }
}

pub fn wide_char_to_multi_byte<'a, DC, UDC>(
    code_page: CodePage,
    wc_flags: WCFlags,
    wc_bytes: &[u16],
    mb_bytes: &mut [u8],
    defalut_char: DC,
    used_default_char: UDC,
) -> OsResult<usize>
    where
        DC: Into<Option<char>>,
        UDC: Into<Option<&'a mut bool>> {
    let dc = defalut_char.into().map_or(null(), |x| &x);
    unsafe {
        WideCharToMultiByte(
            code_page.into(),
            wc_flags.bits,
            wc_bytes.as_ptr(),
            wc_bytes.len() as i32,
            mb_bytes.as_mut_ptr() as *mut i8,
            mb_bytes.len() as i32,
            dc as *const i8,
            used_default_char.into().map_or(null_mut(), |x| x as *mut _ as *mut _),
        ).and_then(|x| Ok(x as usize))
    }
}
