//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSFileTypeForHFSTypeCode(hfs_file_type_code: OSType) -> *mut NSString;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSHFSTypeCodeFromFileType(file_type_string: Option<&NSString>) -> OSType;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSHFSTypeOfFile(full_file_path: Option<&NSString>) -> *mut NSString;
}
