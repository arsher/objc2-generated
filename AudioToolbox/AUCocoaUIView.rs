//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aucocoauibase?language=objc)
    pub unsafe trait AUCocoaUIBase {
        #[method(interfaceVersion)]
        unsafe fn interfaceVersion(&self) -> c_uint;
    }

    unsafe impl ProtocolType for dyn AUCocoaUIBase {}
);
