//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AXRequest;
);

unsafe impl NSCoding for AXRequest {}

unsafe impl NSCopying for AXRequest {}

unsafe impl CopyingHelper for AXRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AXRequest {}

unsafe impl NSSecureCoding for AXRequest {}

extern_methods!(
    unsafe impl AXRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other currentRequest)]
        pub unsafe fn currentRequest() -> Option<Retained<AXRequest>>;

        #[cfg(feature = "AXTechnology")]
        #[method_id(@__retain_semantics Other technology)]
        pub unsafe fn technology(&self) -> Retained<AXTechnology>;
    }
);
