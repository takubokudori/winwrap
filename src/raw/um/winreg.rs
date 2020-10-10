use crate::*;
use winapi::shared::minwindef::{HKEY, PHKEY, LPDWORD, BOOL, PFILETIME, LPBYTE, BYTE, LPCVOID, DWORD};
use winapi::shared::ntdef::{HANDLE, LPCSTR, LPCWSTR, ULONG, LPSTR, PVOID, LPWSTR, PLONG};
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::winnt::{SECURITY_INFORMATION, PSECURITY_DESCRIPTOR};
use winapi::um::winreg::{REGSAM, PVALENTA, PVALENTW};

macro_rules! make_func_reg {
    // unsafe
    ($($pa:ident)::*, pub $(unsafe)? fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;) => (
    #[allow(non_snake_case)]
    pub unsafe fn $func ($($p: $t,)*) -> crate::OsResult<()> {
        match $($pa::)*$func($($p,)*) as u32 {
            winapi::shared::winerror::ERROR_SUCCESS => Ok(()),
            x => Err(x),
        }
    }
    );
    // safe
    ($($pa:ident)::*, pub safe fn $func:ident($($p:ident: $t:ty,)*) -> $ret:ty;) => (
    #[allow(non_snake_case)]
    pub fn $func ($($p: $t,)*) -> crate::OsResult<()> {
    unsafe {
            match $($pa::)*$func($($p,)*) as u32 {
                winapi::shared::winerror::ERROR_SUCCESS => Ok(()),
                x => Err(x),
            }
        }
    }
    );
}

make_func_reg! {winapi::um::winreg,
pub safe fn RegCloseKey(
    hKey: HKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOverridePredefKey(
    hKey: HKEY,
    hNewHKey: HKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOpenUserClassesRoot(
    hToken: HANDLE,
    dwOptions: DWORD,
    samDesired: REGSAM,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOpenCurrentUser(
    samDesired: REGSAM,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDisablePredefinedCache() -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDisablePredefinedCacheEx() -> LSTATUS;}


make_func_reg! {winapi::um::winreg,
pub fn RegConnectRegistryA(
    lpMachineName: LPCSTR,
    hKey: HKEY,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegConnectRegistryW(
    lpMachineName: LPCWSTR,
    hKey: HKEY,
    phkResult: PHKEY,
) -> LSTATUS;}

// Undocumented
make_func_reg! {winapi::um::winreg,
pub fn RegConnectRegistryExA(
    lpMachineName: LPCSTR,
    hKey: HKEY,
    flags: ULONG,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegConnectRegistryExW(
    lpMachineName: LPCWSTR,
    hKey: HKEY,
    flags: ULONG,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegCreateKeyA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegCreateKeyW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegCreateKeyExA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    Reserved: DWORD,
    lpClass: LPSTR,
    dwOptions: DWORD,
    samDesired: REGSAM,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    phkResult: PHKEY,
    lpdwDisposition: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegCreateKeyExW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    Reserved: DWORD,
    lpClass: LPWSTR,
    dwOptions: DWORD,
    samDesired: REGSAM,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    phkResult: PHKEY,
    lpdwDisposition: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegCreateKeyTransactedA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    Reserved: DWORD,
    lpClass: LPSTR,
    dwOptions: DWORD,
    samDesired: REGSAM,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    phkResult: PHKEY,
    lpdwDisposition: LPDWORD,
    hTransaction: HANDLE,
    pExtendedParemeter: PVOID,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegCreateKeyTransactedW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    Reserved: DWORD,
    lpClass: LPWSTR,
    dwOptions: DWORD,
    samDesired: REGSAM,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    phkResult: PHKEY,
    lpdwDisposition: LPDWORD,
    hTransaction: HANDLE,
    pExtendedParemeter: PVOID,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteKeyA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteKeyW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteKeyExA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    samDesired: REGSAM,
    Reserved: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteKeyExW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    samDesired: REGSAM,
    Reserved: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteKeyTransactedA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    samDesired: REGSAM,
    Reserved: DWORD,
    hTransaction: HANDLE,
    pExtendedParemeter: PVOID,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteKeyTransactedW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    samDesired: REGSAM,
    Reserved: DWORD,
    hTransaction: HANDLE,
    pExtendedParemeter: PVOID,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDisableReflectionKey(
    hBase: HKEY,
) -> LONG;}

make_func_reg! {winapi::um::winreg,
pub fn RegEnableReflectionKey(
    hBase: HKEY,
) -> LONG;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryReflectionKey(
    hBase: HKEY,
    bIsReflectionDisabled: *mut BOOL,
) -> LONG;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteValueA(
    hKey: HKEY,
    lpValueName: LPCSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteValueW(
    hKey: HKEY,
    lpValueName: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegEnumKeyA(
    hKey: HKEY,
    dwIndex: DWORD,
    lpName: LPSTR,
    cchName: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegEnumKeyW(
    hKey: HKEY,
    dwIndex: DWORD,
    lpName: LPWSTR,
    cchName: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegEnumKeyExA(
    hKey: HKEY,
    dwIndex: DWORD,
    lpName: LPSTR,
    lpcName: LPDWORD,
    lpReserved: LPDWORD,
    lpClass: LPSTR,
    lpcClass: LPDWORD,
    lpftLastWriteTime: PFILETIME,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegEnumKeyExW(
    hKey: HKEY,
    dwIndex: DWORD,
    lpName: LPWSTR,
    lpcName: LPDWORD,
    lpReserved: LPDWORD,
    lpClass: LPWSTR,
    lpcClass: LPDWORD,
    lpftLastWriteTime: PFILETIME,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegEnumValueA(
    hKey: HKEY,
    dwIndex: DWORD,
    lpValueName: LPSTR,
    lpcchValueName: LPDWORD,
    lpReserved: LPDWORD,
    lpType: LPDWORD,
    lpData: LPBYTE,
    lpcbData: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegEnumValueW(
    hKey: HKEY,
    dwIndex: DWORD,
    lpValueName: LPWSTR,
    lpcchValueName: LPDWORD,
    lpReserved: LPDWORD,
    lpType: LPDWORD,
    lpData: LPBYTE,
    lpcbData: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegFlushKey(
    hKey: HKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegGetKeySecurity(
    hKey: HKEY,
    SecurityInformation: SECURITY_INFORMATION,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    lpcbSecurityDescriptor: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegLoadKeyA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    lpFile: LPCSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegLoadKeyW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    lpFile: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegNotifyChangeKeyValue(
    hKey: HKEY,
    bWatchSubtree: BOOL,
    dwNotifyFilter: DWORD,
    hEvent: HANDLE,
    fAsynchronous: BOOL,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOpenKeyA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOpenKeyW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOpenKeyExA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    ulOptions: DWORD,
    samDesired: REGSAM,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOpenKeyExW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    ulOptions: DWORD,
    samDesired: REGSAM,
    phkResult: PHKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOpenKeyTransactedA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    ulOptions: DWORD,
    samDesired: REGSAM,
    phkResult: PHKEY,
    hTransaction: HANDLE,
    pExtendedParemeter: PVOID,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegOpenKeyTransactedW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    ulOptions: DWORD,
    samDesired: REGSAM,
    phkResult: PHKEY,
    hTransaction: HANDLE,
    pExtendedParemeter: PVOID,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryInfoKeyA(
    hKey: HKEY,
    lpClass: LPSTR,
    lpcClass: LPDWORD,
    lpReserved: LPDWORD,
    lpcSubKeys: LPDWORD,
    lpcMaxSubKeyLen: LPDWORD,
    lpcMaxClassLen: LPDWORD,
    lpcValues: LPDWORD,
    lpcMaxValueNameLen: LPDWORD,
    lpcMaxValueLen: LPDWORD,
    lpcbSecurityDescriptor: LPDWORD,
    lpftLastWriteTime: PFILETIME,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryInfoKeyW(
    hKey: HKEY,
    lpClass: LPWSTR,
    lpcClass: LPDWORD,
    lpReserved: LPDWORD,
    lpcSubKeys: LPDWORD,
    lpcMaxSubKeyLen: LPDWORD,
    lpcMaxClassLen: LPDWORD,
    lpcValues: LPDWORD,
    lpcMaxValueNameLen: LPDWORD,
    lpcMaxValueLen: LPDWORD,
    lpcbSecurityDescriptor: LPDWORD,
    lpftLastWriteTime: PFILETIME,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryValueA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    lpData: LPSTR,
    lpcbData: PLONG,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryValueW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    lpData: LPWSTR,
    lpcbData: PLONG,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryMultipleValuesA(
    hKey: HKEY,
    val_list: PVALENTA,
    num_vals: DWORD,
    lpValueBuf: LPSTR,
    ldwTotsize: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryMultipleValuesW(
    hKey: HKEY,
    val_list: PVALENTW,
    num_vals: DWORD,
    lpValueBuf: LPWSTR,
    ldwTotsize: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryValueExA(
    hKey: HKEY,
    lpValueName: LPCSTR,
    lpReserved: LPDWORD,
    lpType: LPDWORD,
    lpData: LPBYTE,
    lpcbData: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegQueryValueExW(
    hKey: HKEY,
    lpValueName: LPCWSTR,
    lpReserved: LPDWORD,
    lpType: LPDWORD,
    lpData: LPBYTE,
    lpcbData: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegReplaceKeyA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    lpNewFile: LPCSTR,
    lpOldFile: LPCSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegReplaceKeyW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    lpNewFile: LPCWSTR,
    lpOldFile: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegRestoreKeyA(
    hKey: HKEY,
    lpFile: LPCSTR,
    dwFlags: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegRestoreKeyW(
    hKey: HKEY,
    lpFile: LPCWSTR,
    dwFlags: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegRenameKey(
    hKey: HKEY,
    lpSubKeyName: LPCWSTR,
    lpNewKeyName: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSaveKeyA(
    hKey: HKEY,
    lpFile: LPCSTR,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSaveKeyW(
    hKey: HKEY,
    lpFile: LPCWSTR,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSetKeySecurity(
    hKey: HKEY,
    SecurityInformation: SECURITY_INFORMATION,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSetValueA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    dwType: DWORD,
    lpData: LPCSTR,
    cbData: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSetValueW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    dwType: DWORD,
    lpData: LPCWSTR,
    cbData: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSetValueExA(
    hKey: HKEY,
    lpValueName: LPCSTR,
    Reserved: DWORD,
    dwType: DWORD,
    lpData: *const BYTE,
    cbData: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSetValueExW(
    hKey: HKEY,
    lpValueName: LPCWSTR,
    Reserved: DWORD,
    dwType: DWORD,
    lpData: *const BYTE,
    cbData: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegUnLoadKeyA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegUnLoadKeyW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteKeyValueA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    lpValueName: LPCSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteKeyValueW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    lpValueName: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSetKeyValueA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
    lpValueName: LPCSTR,
    dwType: DWORD,
    lpData: LPCVOID,
    cbData: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSetKeyValueW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
    lpValueName: LPCWSTR,
    dwType: DWORD,
    lpData: LPCVOID,
    cbData: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteTreeA(
    hKey: HKEY,
    lpSubKey: LPCSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegDeleteTreeW(
    hKey: HKEY,
    lpSubKey: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegCopyTreeA(
    hKeySrc: HKEY,
    lpSubKey: LPCSTR,
    hKeyDest: HKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegGetValueA(
    hkey: HKEY,
    lpSubKey: LPCSTR,
    lpValue: LPCSTR,
    dwFlags: DWORD,
    pdwType: LPDWORD,
    pvData: PVOID,
    pcbData: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegGetValueW(
    hkey: HKEY,
    lpSubKey: LPCWSTR,
    lpValue: LPCWSTR,
    dwFlags: DWORD,
    pdwType: LPDWORD,
    pvData: PVOID,
    pcbData: LPDWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegCopyTreeW(
    hKeySrc: HKEY,
    lpSubKey: LPCWSTR,
    hKeyDest: HKEY,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegLoadMUIStringA(
    hKey: HKEY,
    pszValue: LPCSTR,
    pszOutBuf: LPSTR,
    cbOutBuf: DWORD,
    pcbData: LPDWORD,
    Flags: DWORD,
    pszDirectory: LPCSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegLoadMUIStringW(
    hKey: HKEY,
    pszValue: LPCWSTR,
    pszOutBuf: LPWSTR,
    cbOutBuf: DWORD,
    pcbData: LPDWORD,
    Flags: DWORD,
    pszDirectory: LPCWSTR,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegLoadAppKeyA(
    lpFile: LPCSTR,
    phkResult: PHKEY,
    samDesired: REGSAM,
    dwOptions: DWORD,
    Reserved: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegLoadAppKeyW(
    lpFile: LPCWSTR,
    phkResult: PHKEY,
    samDesired: REGSAM,
    dwOptions: DWORD,
    Reserved: DWORD,
) -> LSTATUS;}

make_func2! {winapi::um::winreg,
pub fn InitiateSystemShutdownA(
    lpMachineName: LPSTR,
    lpMessage: LPSTR,
    dwTimeout: DWORD,
    bForceAppsClosed: BOOL,
    bRebootAfterShutdown: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::winreg,
pub fn InitiateSystemShutdownW(
    lpMachineName: LPWSTR,
    lpMessage: LPWSTR,
    dwTimeout: DWORD,
    bForceAppsClosed: BOOL,
    bRebootAfterShutdown: BOOL,
) -> BOOL;0}

make_func2! {winapi::um::winreg,
pub fn AbortSystemShutdownA(
    lpMachineName: LPSTR,
) -> BOOL;0}

make_func2! {winapi::um::winreg,
pub fn AbortSystemShutdownW(
    lpMachineName: LPWSTR,
) -> BOOL;0}

make_func2! {winapi::um::winreg,
pub fn InitiateSystemShutdownExA(
    lpMachineName: LPSTR,
    lpMessage: LPSTR,
    dwTimeout: DWORD,
    bForceAppsClosed: BOOL,
    bRebootAfterShutdown: BOOL,
    dwReason: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::winreg,
pub fn InitiateSystemShutdownExW(
    lpMachineName: LPWSTR,
    lpMessage: LPWSTR,
    dwTimeout: DWORD,
    bForceAppsClosed: BOOL,
    bRebootAfterShutdown: BOOL,
    dwReason: DWORD,
) -> BOOL;0}

make_func_reg! {winapi::um::winreg,
pub fn InitiateShutdownA(
    lpMachineName: LPSTR,
    lpMessage: LPSTR,
    dwGracePeriod: DWORD,
    dwShutdownFlags: DWORD,
    dwReason: DWORD,
) -> DWORD;}

make_func_reg! {winapi::um::winreg,
pub fn InitiateShutdownW(
    lpMachineName: LPWSTR,
    lpMessage: LPWSTR,
    dwGracePeriod: DWORD,
    dwShutdownFlags: DWORD,
    dwReason: DWORD,
) -> DWORD;}

// Undocumented function
/*
make_func_reg! {winapi::um::winreg,
pub fn CheckForHiberboot(
    pHiberboot: PBOOLEAN,
    bClearFlag: BOOLEAN,
) -> DWORD;}
 */

make_func_reg! {winapi::um::winreg,
pub fn RegSaveKeyExA(
    hKey: HKEY,
    lpFile: LPCSTR,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    Flags: DWORD,
) -> LSTATUS;}

make_func_reg! {winapi::um::winreg,
pub fn RegSaveKeyExW(
    hKey: HKEY,
    lpFile: LPCWSTR,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    Flags: DWORD,
) -> LSTATUS;}
