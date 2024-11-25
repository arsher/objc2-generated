//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckserverchangetoken?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKServerChangeToken;
);

unsafe impl Send for CKServerChangeToken {}

unsafe impl Sync for CKServerChangeToken {}

unsafe impl NSCoding for CKServerChangeToken {}

unsafe impl NSCopying for CKServerChangeToken {}

unsafe impl CopyingHelper for CKServerChangeToken {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKServerChangeToken {}

unsafe impl NSSecureCoding for CKServerChangeToken {}

extern_methods!(
    unsafe impl CKServerChangeToken {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
