//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSNull")]
    pub struct NSNull;

    #[cfg(feature = "Foundation_NSNull")]
    unsafe impl ClassType for NSNull {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSNull")]
unsafe impl NSCoding for NSNull {}

#[cfg(feature = "Foundation_NSNull")]
unsafe impl NSObjectProtocol for NSNull {}

#[cfg(feature = "Foundation_NSNull")]
unsafe impl NSSecureCoding for NSNull {}

extern_methods!(
    #[cfg(feature = "Foundation_NSNull")]
    unsafe impl NSNull {
        #[method_id(@__retain_semantics Other null)]
        pub unsafe fn null() -> Id<NSNull, Shared>;
    }
);
