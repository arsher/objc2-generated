//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

extern_category!(
    /// Category "NSNibAwaking" on [`NSObject`].
    #[doc(alias = "NSNibAwaking")]
    pub unsafe trait NSObjectNSNibAwaking {
        #[method(awakeFromNib)]
        unsafe fn awakeFromNib(&self);

        #[method(prepareForInterfaceBuilder)]
        unsafe fn prepareForInterfaceBuilder(&self);
    }

    unsafe impl NSObjectNSNibAwaking for NSObject {}
);
