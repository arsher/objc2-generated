//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnrequestrevisionproviding?language=objc)
    pub unsafe trait VNRequestRevisionProviding {
        /// The revision of the VNRequest subclass that was used to generate the object that implements this protocol.
        #[method(requestRevision)]
        unsafe fn requestRevision(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn VNRequestRevisionProviding {}
);
