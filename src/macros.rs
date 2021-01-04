// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.

/// Returns `Ok(ret)`.
#[macro_export]
macro_rules! handle {
    ($x:expr,$y:pat) => { match $x {
        $y => Err($crate::OsError::last_os_error()),
        x=> Ok(x),
    }
    };
    // Type conversion
    ($x:expr,$match_ty:ty,$y:pat,$ret_ty:ty) => { match $x as $match_ty {
        $y => Err($crate::OsError::last_os_error()),
        x=> Ok(x as $ret_ty),
    }
    };
}

/// Returns `Ok(())`.
#[macro_export]
macro_rules! handle2 {
    ($x:expr,$err_pat:pat) => { match $x {
        $err_pat => Err($crate::OsError::last_os_error()),
        _=> Ok(()),
    }
    };
    // Type conversion
    ($x:expr,$match_ty:ty,$err_pat:pat) => { match $x as $match_ty {
        $err_pat => Err($crate::OsError::last_os_error()),
        _=> Ok(()),
    }
    };
}

/// Returns `Result<ret,()>`.
#[macro_export]
macro_rules! e_handle_internal {
    ($x:expr,$y:pat) => { match $x {
        $y => Err(()),
        x=> Ok(x),
    }
    };
    // Type conversion
    ($x:expr,$match_ty:ty,$y:pat,$ret_ty:ty) => { match $x as $match_ty {
        $y => Err(()),
        x=> Ok(x as $ret_ty),
    }
    };
}

/// Returns `Ok(())`.
#[macro_export]
macro_rules! e_handle2_internal {
    ($x:expr,$err_pat:pat) => { match $x {
        $err_pat => Err(()),
        _=> Ok(()),
    }
    };
    // Type conversion
    ($x:expr,$match_ty:ty,$err_pat:pat) => { match $x as $match_ty {
        $err_pat => Err(()),
        _=> Ok(()),
    }
    };
}

/// Returns `OsResult<$ret>`.
#[macro_export]
macro_rules! make_func {
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) -> $crate::OsResult<$ret> {
        $crate::handle!($($pa::)*$func($($p,)*),$err_pat)
    }
    );
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub unsafe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) -> $crate::OsResult<$ret> {
        $crate::handle!($($pa::)*$func($($p,)*),$err_pat)
    }
    );
    // safe
    ($($pa:ident)::*, $(#[$outer:meta])* pub safe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[inline]
    $(#[$outer])*
    pub fn $func ($($p: $t,)*) -> $crate::OsResult<$ret> {
        unsafe { $crate::handle!($($pa::)*$func($($p,)*),$err_pat) }
    }
    );
}

/// Returns `OsResult<()>`.
#[macro_export]
macro_rules! make_func2 {
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) -> $crate::OsResult<()> {
        $crate::handle2!($($pa::)*$func($($p,)*),$err_pat)
    }
    );
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub unsafe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) -> $crate::OsResult<()> {
        $crate::handle2!($($pa::)*$func($($p,)*),$err_pat)
    }
    );
    // safe
    ($($pa:ident)::*, $(#[$outer:meta])* pub safe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[inline]
    $(#[$outer])*
    pub fn $func ($($p: $t,)*) -> $crate::OsResult<()> {
        unsafe { handle2!($($pa::)*$func($($p,)*),$err_pat) }
    }
    );
}

/// Returns the returned value as they are.
#[macro_export]
macro_rules! tp_func {
    // safe
    ($($pa:ident)::*, $(#[$outer:meta])* pub fn $func:ident($($p:ident: $t:ty,)*) $(-> $ret:ty)?$(;)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) $(-> $ret)? { $($pa::)*$func($($p,)*) }
    );
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub unsafe fn $func:ident($($p:ident: $t:ty,)*) $(-> $ret:ty)?$(;)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) $(-> $ret)? { $($pa::)*$func($($p,)*) }
    );
    // safe wrapper
    ($($pa:ident)::*, $(#[$outer:meta])* pub safe fn $func:ident($($p:ident: $t:ty,)*) $(-> $ret:ty)?$(;)?) => (
    #[allow(non_snake_case)]
    #[inline]
    $(#[$outer])*
    pub fn $func ($($p: $t,)*) $(-> $ret)? { unsafe {$($pa::)*$func($($p,)*)} }
    );
}

#[macro_export]
macro_rules! make_struct {
    ($c_type: ty,
    $(#[$outer:meta])*
     $sv:vis struct $name:ident $(<$($attr:tt),+>)? {
        $(
            $(#[$inner:ident $($args:tt)*])*
            $vis:vis $v:ident :  $t:ty,
        )*
    }) => (
        #[repr(C)]
        $(#[$outer])*
        $sv struct $name $(<$($attr),+>)?{
            $(
                $(#[$inner $($args)*])*
                $vis $v: $t,
            )*
        }

        impl$(<$($attr),+>)? $name $(<$($attr),+>)?{
            pub fn as_ptr(&self) -> *const Self{
                self as *const _
            }
            pub fn as_mut_ptr(&mut self) -> *mut Self{
                self as *mut _
            }
            pub fn as_c_ptr(&self) -> *const $c_type {
                self.as_ptr() as *const _
            }
            pub fn as_mut_c_ptr(&mut self) -> *mut $c_type {
                self.as_mut_ptr() as *mut _
            }
        }
    )
}

/// Returns `Result<$ret,()>`.
///
/// This does not use `GetLastError()`.
#[macro_export]
macro_rules! e_make_func {
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) -> Result<$ret, ()> {
        $crate::e_handle_internal!($($pa::)*$func($($p,)*),$err_pat)
    }
    );
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub unsafe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) -> Result<$ret, ()> {
        $crate::e_handle_internal!($($pa::)*$func($($p,)*),$err_pat)
    }
    );
    // safe
    ($($pa:ident)::*, $(#[$outer:meta])* pub safe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub fn $func ($($p: $t,)*) -> Result<$ret, ()> {
        unsafe { $crate::e_handle_internal!($($pa::)*$func($($p,)*),$err_pat) }
    }
    );
}

/// Returns `Result<(),()>`.
///
/// This does not use `GetLastError()`.
#[macro_export]
macro_rules! e_make_func2 {
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) -> Result<(), ()> {
        $crate::e_handle2_internal!($($pa::)*$func($($p,)*),$err_pat)
    }
    );
    // unsafe
    ($($pa:ident)::*, $(#[$outer:meta])* pub unsafe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    $(#[$outer])*
    pub unsafe fn $func ($($p: $t,)*) -> Result<(), ()> {
        $crate::e_handle2_internal!($($pa::)*$func($($p,)*),$err_pat)
    }
    );
    // safe
    ($($pa:ident)::*, $(#[$outer:meta])* pub safe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;$err_pat:pat$(,)?) => (
    #[allow(non_snake_case)]
    #[inline]
    $(#[$outer])*
    pub fn $func ($($p: $t,)*) -> Result<(), ()> {
        unsafe { e_handle2_internal!($($pa::)*$func($($p,)*),$err_pat) }
    }
    );
}

#[macro_export]
macro_rules! make_func_hresult {
    // unsafe
    ($($pa:ident)::*, pub $(unsafe)? fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;) => (
    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn $func ($($p: $t,)*) -> $crate::OsResult<()> {
        match $($pa::)*$func($($p,)*) as i32 {
            winapi::shared::winerror::S_OK => Ok(()),
            x => Err($crate::OsError::Win32(x as u32)),
        }
    }
    );
    // safe
    ($($pa:ident)::*, pub safe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;) => (
    #[allow(non_snake_case)]
    #[inline]
    pub fn $func ($($p: $t,)*) -> $crate::OsResult<()> {
    unsafe {
            match $($pa::)*$func($($p,)*) as i32 {
                winapi::shared::winerror::S_OK => Ok(()),
                x => Err($crate::OsError::Win32(x as u32)),
            }
        }
    }
    );
}
