//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avkeyvaluestatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVKeyValueStatus(pub NSInteger);
impl AVKeyValueStatus {
    #[doc(alias = "AVKeyValueStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "AVKeyValueStatusLoading")]
    pub const Loading: Self = Self(1);
    #[doc(alias = "AVKeyValueStatusLoaded")]
    pub const Loaded: Self = Self(2);
    #[doc(alias = "AVKeyValueStatusFailed")]
    pub const Failed: Self = Self(3);
    #[doc(alias = "AVKeyValueStatusCancelled")]
    pub const Cancelled: Self = Self(4);
}

unsafe impl Encode for AVKeyValueStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVKeyValueStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avasynchronouskeyvalueloading?language=objc)
    pub unsafe trait AVAsynchronousKeyValueLoading {
        #[cfg(feature = "block2")]
        #[method(loadValuesAsynchronouslyForKeys:completionHandler:)]
        unsafe fn loadValuesAsynchronouslyForKeys_completionHandler(
            &self,
            keys: &NSArray<NSString>,
            handler: Option<&block2::Block<dyn Fn()>>,
        );
    }

    unsafe impl ProtocolType for dyn AVAsynchronousKeyValueLoading {}
);
