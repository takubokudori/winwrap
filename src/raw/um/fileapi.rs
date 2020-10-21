use crate::*;
use winapi::shared::minwindef::{DWORD, LPDWORD, LPCVOID, LPVOID, FILETIME, LPFILETIME, BOOL, UINT, PDWORD};
use winapi::shared::ntdef::{LONG, LPWSTR, LPSTR, PLONG, LONGLONG, LPWCH, PUCHAR, ULONG, PWSTR};
use winapi::shared::winerror::NO_ERROR;
use winapi::um::fileapi::{LPBY_HANDLE_FILE_INFORMATION, LPCREATEFILE2_EXTENDED_PARAMETERS, INVALID_FILE_ATTRIBUTES, INVALID_FILE_SIZE, INVALID_SET_FILE_POINTER, STREAM_INFO_LEVELS};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES, LPWIN32_FIND_DATAA, LPWIN32_FIND_DATAW, FINDEX_INFO_LEVELS, FINDEX_SEARCH_OPS, GET_FILEEX_INFO_LEVELS, LPOVERLAPPED_COMPLETION_ROUTINE, FILE_INFO_BY_HANDLE_CLASS};
use winapi::um::winbase::FILE_TYPE_UNKNOWN;
use winapi::um::winnt::{HANDLE, LPCSTR, LPCWSTR, FILE_SEGMENT_ELEMENT, LARGE_INTEGER, PULARGE_INTEGER, PLARGE_INTEGER};

tp_func! {winapi::um::fileapi,
pub fn CompareFileTime(
    lpFileTime1: *const FILETIME,
    lpFileTime2: *const FILETIME,
) -> LONG;}

make_func2! {winapi::um::fileapi,
pub fn CreateDirectoryA(
    lpPathName: LPCSTR,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn CreateDirectoryW(
    lpPathName: LPCWSTR,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub unsafe fn CreateFileA(
    lpFileName: LPCSTR,
    dwDesiredAccess: DWORD,
    dwShareMode: DWORD,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    dwCreationDisposition: DWORD,
    dwFlagsAndAttributes: DWORD,
    hTemplateFile: HANDLE,
    ) -> HANDLE; INVALID_HANDLE_VALUE}

make_func! {winapi::um::fileapi,
    pub unsafe fn CreateFileW(
        lpFileName: LPCWSTR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD,
        hTemplateFile: HANDLE,
    ) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::fileapi,
pub fn DefineDosDeviceW(
    dwFlags: DWORD,
    lpDeviceName: LPCWSTR,
    lpTargetPath: LPCWSTR,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn DeleteFileA(
    lpFileName: LPCSTR,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn DeleteFileW(
    lpFileName: LPCWSTR,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn DeleteVolumeMountPointW(
    lpszVolumeMountPoint: LPCWSTR,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn FileTimeToLocalFileTime(
    lpFileTime: *const FILETIME,
    lpLocalFileTime: LPFILETIME,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn FindClose(
    hFindFile: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn FindCloseChangeNotification(
    hChangeHandle: HANDLE,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub fn FindFirstChangeNotificationA(
    lpPathName: LPCSTR,
    bWatchSubtree: BOOL,
    dwNotifyFilter: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func! {winapi::um::fileapi,
pub fn FindFirstChangeNotificationW(
    lpPathName: LPCWSTR,
    bWatchSubtree: BOOL,
    dwNotifyFilter: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func! {winapi::um::fileapi,
pub fn FindFirstFileA(
    lpFileName: LPCSTR,
    lpFindFileData: LPWIN32_FIND_DATAA,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func! {winapi::um::fileapi,
pub fn FindFirstFileW(
    lpFileName: LPCWSTR,
    lpFindFileData: LPWIN32_FIND_DATAW,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func! {winapi::um::fileapi,
pub fn FindFirstFileExA(
    lpFileName: LPCSTR,
    fInfoLevelId: FINDEX_INFO_LEVELS,
    lpFindFileData: LPVOID,
    fSearchOp: FINDEX_SEARCH_OPS,
    lpSearchFilter: LPVOID,
    dwAdditionalFlags: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func! {winapi::um::fileapi,
pub fn FindFirstFileExW(
    lpFileName: LPCWSTR,
    fInfoLevelId: FINDEX_INFO_LEVELS,
    lpFindFileData: LPVOID,
    fSearchOp: FINDEX_SEARCH_OPS,
    lpSearchFilter: LPVOID,
    dwAdditionalFlags: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func! {winapi::um::fileapi,
pub fn FindFirstVolumeW(
    lpszVolumeName: LPWSTR,
    cchBufferLength: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::fileapi,
pub fn FindNextChangeNotification(
    hChangeHandle: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn FindNextFileA(
    hFindFile: HANDLE,
    lpFindFileData: LPWIN32_FIND_DATAA,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn FindNextFileW(
    hFindFile: HANDLE,
    lpFindFileData: LPWIN32_FIND_DATAW,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn FindNextVolumeW(
    hFindVolume: HANDLE,
    lpszVolumeName: LPWSTR,
    cchBufferLength: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn FindVolumeClose(
    hFindVolume: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn FlushFileBuffers(
    hFile: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetDiskFreeSpaceA(
    lpRootPathName: LPCSTR,
    lpSectorsPerCluster: LPDWORD,
    lpBytesPerSector: LPDWORD,
    lpNumberOfFreeClusters: LPDWORD,
    lpTotalNumberOfClusters: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetDiskFreeSpaceW(
    lpRootPathName: LPCWSTR,
    lpSectorsPerCluster: LPDWORD,
    lpBytesPerSector: LPDWORD,
    lpNumberOfFreeClusters: LPDWORD,
    lpTotalNumberOfClusters: LPDWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetDiskFreeSpaceExA(
    lpDirectoryName: LPCSTR,
    lpFreeBytesAvailableToCaller: PULARGE_INTEGER,
    lpTotalNumberOfBytes: PULARGE_INTEGER,
    lpTotalNumberOfFreeBytes: PULARGE_INTEGER,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetDiskFreeSpaceExW(
    lpDirectoryName: LPCWSTR,
    lpFreeBytesAvailableToCaller: PULARGE_INTEGER,
    lpTotalNumberOfBytes: PULARGE_INTEGER,
    lpTotalNumberOfFreeBytes: PULARGE_INTEGER,
) -> BOOL;0}

tp_func! {winapi::um::fileapi,
pub fn GetDriveTypeA(
    lpRootPathName: LPCSTR,
) -> UINT;}

tp_func! {winapi::um::fileapi,
pub fn GetDriveTypeW(
    lpRootPathName: LPCWSTR,
) -> UINT;}

make_func! {winapi::um::fileapi,
pub fn GetFileAttributesA(
    lpFileName: LPCSTR,
) -> DWORD;INVALID_FILE_ATTRIBUTES}

make_func! {winapi::um::fileapi,
pub fn GetFileAttributesW(
    lpFileName: LPCWSTR,
) -> DWORD;INVALID_FILE_ATTRIBUTES}

make_func2! {winapi::um::fileapi,
pub fn GetFileAttributesExA(
    lpFileName: LPCSTR,
    fInfoLevelId: GET_FILEEX_INFO_LEVELS,
    lpFileInformation: LPVOID,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetFileAttributesExW(
    lpFileName: LPCWSTR,
    fInfoLevelId: GET_FILEEX_INFO_LEVELS,
    lpFileInformation: LPVOID,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetFileInformationByHandle(
    hFile: HANDLE,
    lpFileInformation: LPBY_HANDLE_FILE_INFORMATION,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub fn GetFileSize(
    hFile: HANDLE,
    lpFileSizeHigh: LPDWORD,
) -> DWORD;INVALID_FILE_SIZE}

make_func2! {winapi::um::fileapi,
pub fn GetFileSizeEx(
    hFile: HANDLE,
    lpFileSize: PLARGE_INTEGER,
) -> BOOL;0}

#[allow(non_snake_case)]
pub fn GetFileType(
    hFile: HANDLE,
) -> OsResult<DWORD> {
    unsafe {
        match winapi::um::fileapi::GetFileType(hFile) {
            FILE_TYPE_UNKNOWN => {
                match winapi::um::errhandlingapi::GetLastError() {
                    NO_ERROR => Ok(FILE_TYPE_UNKNOWN),
                    x => Err(OsError::from_win32_error(x)),
                }
            }
            x => Ok(x),
        }
    }
}

make_func! {winapi::um::fileapi,
pub fn GetFinalPathNameByHandleA(
    hFile: HANDLE,
    lpszFilePath: LPSTR,
    cchFilePath: DWORD,
    dwFlags: DWORD,
) -> DWORD;0}

make_func! {winapi::um::fileapi,
pub fn GetFinalPathNameByHandleW(
    hFile: HANDLE,
    lpszFilePath: LPWSTR,
    cchFilePath: DWORD,
    dwFlags: DWORD,
) -> DWORD;0}

make_func2! {winapi::um::fileapi,
pub fn GetFileTime(
    hFile: HANDLE,
    lpCreationTime: LPFILETIME,
    lpLastAccessTime: LPFILETIME,
    lpLastWriteTime: LPFILETIME,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub fn GetFullPathNameW(
    lpFileName: LPCWSTR,
    nBufferLength: DWORD,
    lpBuffer: LPWSTR,
    lpFilePart: *mut LPWSTR,
) -> DWORD;0}

make_func! {winapi::um::fileapi,
pub fn GetFullPathNameA(
    lpFileName: LPCSTR,
    nBufferLength: DWORD,
    lpBuffer: LPSTR,
    lpFilePart: *mut LPSTR,
) -> DWORD;0}

make_func! {winapi::um::fileapi,
pub safe fn GetLogicalDrives() -> DWORD;0}

make_func! {winapi::um::fileapi,
pub fn GetLogicalDriveStringsW(
    nBufferLength: DWORD,
    lpBuffer: LPWSTR,
) -> DWORD;0}

make_func! {winapi::um::fileapi,
pub fn GetLongPathNameA(
    lpszShortPath: LPCSTR,
    lpszLongPath: LPSTR,
    cchBuffer: DWORD,
) -> DWORD;0}

make_func! {winapi::um::fileapi,
pub fn GetLongPathNameW(
    lpszShortPath: LPCWSTR,
    lpszLongPath: LPWSTR,
    cchBuffer: DWORD,
) -> DWORD;0}

make_func! {winapi::um::fileapi,
pub fn GetShortPathNameW(
    lpszLongPath: LPCWSTR,
    lpszShortPath: LPWSTR,
    cchBuffer: DWORD,
) -> DWORD;0}

make_func! {winapi::um::fileapi,
pub fn GetTempFileNameW(
    lpPathName: LPCWSTR,
    lpPrefixString: LPCWSTR,
    uUnique: UINT,
    lpTempFileName: LPWSTR,
) -> UINT;0}

make_func2! {winapi::um::fileapi,
pub fn GetVolumeInformationByHandleW(
    hFile: HANDLE,
    lpVolumeNameBuffer: LPWSTR,
    nVolumeNameSize: DWORD,
    lpVolumeSerialNumber: LPDWORD,
    lpMaximumComponentLength: LPDWORD,
    lpFileSystemFlags: LPDWORD,
    lpFileSystemNameBuffer: LPWSTR,
    nFileSystemNameSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetVolumeInformationW(
    lpRootPathName: LPCWSTR,
    lpVolumeNameBuffer: LPWSTR,
    nVolumeNameSize: DWORD,
    lpVolumeSerialNumber: LPDWORD,
    lpMaximumComponentLength: LPDWORD,
    lpFileSystemFlags: LPDWORD,
    lpFileSystemNameBuffer: LPWSTR,
    nFileSystemNameSize: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetVolumePathNameW(
    lpszFileName: LPCWSTR,
    lpszVolumePathName: LPWSTR,
    cchBufferLength: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn LocalFileTimeToFileTime(
    lpLocalFileTime: *const FILETIME,
    lpFileTime: LPFILETIME,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn LockFile(
    hFile: HANDLE,
    dwFileOffsetLow: DWORD,
    dwFileOffsetHigh: DWORD,
    nNumberOfBytesToLockLow: DWORD,
    nNumberOfBytesToLockHigh: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn LockFileEx(
    hFile: HANDLE,
    dwFlags: DWORD,
    dwReserved: DWORD,
    nNumberOfBytesToLockLow: DWORD,
    nNumberOfBytesToLockHigh: DWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub fn QueryDosDeviceW(
    lpDeviceName: LPCWSTR,
    lpTargetPath: LPWSTR,
    ucchMax: DWORD,
) -> DWORD;0}

make_func2! {winapi::um::fileapi,
pub fn ReadFile(
    hFile: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfBytesToRead: DWORD,
    lpNumberOfBytesRead: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn ReadFileEx(
    hFile: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfBytesToRead: DWORD,
    lpOverlapped: LPOVERLAPPED,
    lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn ReadFileScatter(
    hFile: HANDLE,
    aSegmentArray: *mut FILE_SEGMENT_ELEMENT,
    nNumberOfBytesToRead: DWORD,
    lpReserved: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn RemoveDirectoryA(
    lpPathName: LPCSTR,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn RemoveDirectoryW(
    lpPathName: LPCWSTR,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn SetEndOfFile(
    hFile: HANDLE,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn SetFileAttributesA(
    lpFileName: LPCSTR,
    dwFileAttributes: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn SetFileAttributesW(
    lpFileName: LPCWSTR,
    dwFileAttributes: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn SetFileInformationByHandle(
    hFile: HANDLE,
    FileInformationClass: FILE_INFO_BY_HANDLE_CLASS,
    lpFileInformation: LPVOID,
    dwBufferSize: DWORD,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub fn SetFilePointer(
    hFile: HANDLE,
    lDistanceToMove: LONG,
    lpDistanceToMoveHigh: PLONG,
    dwMoveMethod: DWORD,
) -> DWORD;INVALID_SET_FILE_POINTER}

make_func2! {winapi::um::fileapi,
pub fn SetFilePointerEx(
    hFile: HANDLE,
    liDistanceToMove: LARGE_INTEGER,
    lpNewFilePointer: PLARGE_INTEGER,
    dwMoveMethod: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn SetFileTime(
    hFile: HANDLE,
    lpCreationTime: *const FILETIME,
    lpLastAccessTime: *const FILETIME,
    lpLastWriteTime: *const FILETIME,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn SetFileValidData(
    hFile: HANDLE,
    ValidDataLength: LONGLONG,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn UnlockFile(
    hFile: HANDLE,
    dwFileOffsetLow: DWORD,
    dwFileOffsetHigh: DWORD,
    nNumberOfBytesToUnlockLow: DWORD,
    nNumberOfBytesToUnlockHigh: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn UnlockFileEx(
    hFile: HANDLE,
    dwReserved: DWORD,
    nNumberOfBytesToUnlockLow: DWORD,
    nNumberOfBytesToUnlockHigh: DWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn WriteFile(
    hFile: HANDLE,
    lpBuffer: LPCVOID,
    nNumberOfBytesToWrite: DWORD,
    lpNumberOfBytesWritten: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn WriteFileEx(
    hFile: HANDLE,
    lpBuffer: LPCVOID,
    nNumberOfBytesToWrite: DWORD,
    lpOverlapped: LPOVERLAPPED,
    lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn WriteFileGather(
    hFile: HANDLE,
    aSegmentArray: *mut FILE_SEGMENT_ELEMENT,
    nNumberOfBytesToWrite: DWORD,
    lpReserved: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub fn GetTempPathW(
    nBufferLength: DWORD,
    lpBuffer: LPWSTR,
) -> DWORD;0}

make_func2! {winapi::um::fileapi,
pub fn GetVolumeNameForVolumeMountPointW(
    lpszVolumeMountPoint: LPCWSTR,
    lpszVolumeName: LPWSTR,
    cchBufferLength: DWORD,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetVolumePathNamesForVolumeNameW(
    lpszVolumeName: LPCWSTR,
    lpszVolumePathNames: LPWCH,
    cchBufferLength: DWORD,
    lpcchReturnLength: PDWORD,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub fn CreateFile2(
    lpFileName: LPCWSTR,
    dwDesiredAccess: DWORD,
    dwShareMode: DWORD,
    dwCreationDisposition: DWORD,
    pCreateExParams: LPCREATEFILE2_EXTENDED_PARAMETERS,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::fileapi,
pub fn SetFileIoOverlappedRange(
    FileHandle: HANDLE,
    OverlappedRangeStart: PUCHAR,
    Length: ULONG,
) -> BOOL;0}

#[allow(non_snake_case)]
pub unsafe fn GetCompressedFileSizeA(
    lpFileName: LPCSTR,
    lpFileSizeHigh: LPDWORD,
) -> OsResult<DWORD> {
    handle_err(winapi::um::fileapi::GetCompressedFileSizeA(lpFileName, lpFileSizeHigh), lpFileSizeHigh)
}

#[allow(non_snake_case)]
pub unsafe fn GetCompressedFileSizeW(
    lpFileName: LPCWSTR,
    lpFileSizeHigh: LPDWORD,
) -> OsResult<DWORD> {
    handle_err(winapi::um::fileapi::GetCompressedFileSizeW(lpFileName, lpFileSizeHigh), lpFileSizeHigh)
}

#[inline]
#[allow(non_snake_case)]
fn handle_err(x: DWORD, lpFileSizeHigh: LPDWORD) -> OsResult<DWORD> {
    match x {
        INVALID_FILE_SIZE => {
            let e = unsafe { winapi::um::errhandlingapi::GetLastError() };
            if lpFileSizeHigh.is_null() { return Err(OsError::from_win32_error(e)); }
            match e {
                NO_ERROR => Ok(INVALID_FILE_SIZE),
                x => Err(OsError::from_win32_error(x)),
            }
        }
        x => Ok(x),
    }
}

make_func! {winapi::um::fileapi,
pub fn FindFirstStreamW(
    lpFileName: LPCWSTR,
    InfoLevel: STREAM_INFO_LEVELS,
    lpFindStreamData: LPVOID,
    dwFlags: DWORD,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::fileapi,
pub fn FindNextStreamW(
    hFindStream: HANDLE,
    lpFindStreamData: LPVOID,
) -> BOOL;0}

tp_func! {winapi::um::fileapi,
pub fn AreFileApisANSI() -> BOOL;}

make_func! {winapi::um::fileapi,
pub fn GetTempPathA(
    nBufferLength: DWORD,
    lpBuffer: LPSTR,
) -> DWORD;0}

make_func! {winapi::um::fileapi,
pub fn FindFirstFileNameW(
    lpFileName: LPCWSTR,
    dwFlags: DWORD,
    StringLength: LPDWORD,
    LinkName: PWSTR,
) -> HANDLE;INVALID_HANDLE_VALUE}

make_func2! {winapi::um::fileapi,
pub fn FindNextFileNameW(
    hFindStream: HANDLE,
    StringLength: LPDWORD,
    LinkName: PWSTR,
) -> BOOL;0}

make_func2! {winapi::um::fileapi,
pub fn GetVolumeInformationA(
    lpRootPathName: LPCSTR,
    lpVolumeNameBuffer: LPSTR,
    nVolumeNameSize: DWORD,
    lpVolumeSerialNumber: LPDWORD,
    lpMaximumComponentLength: LPDWORD,
    lpFileSystemFlags: LPDWORD,
    lpFileSystemNameBuffer: LPSTR,
    nFileSystemNameSize: DWORD,
) -> BOOL;0}

make_func! {winapi::um::fileapi,
pub fn GetTempFileNameA(
    lpPathName: LPCSTR,
    lpPrefixString: LPCSTR,
    uUnique: UINT,
    lpTempFileName: LPSTR,
) -> UINT;0}

tp_func! {winapi::um::fileapi,
pub safe fn SetFileApisToOEM();}

tp_func! {winapi::um::fileapi,
pub safe fn SetFileApisToANSI();}
