//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcontentauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVContentAuthorizationStatus(pub NSInteger);
impl AVContentAuthorizationStatus {
    pub const AVContentAuthorizationUnknown: Self = Self(0);
    pub const AVContentAuthorizationCompleted: Self = Self(1);
    pub const AVContentAuthorizationCancelled: Self = Self(2);
    pub const AVContentAuthorizationTimedOut: Self = Self(3);
    pub const AVContentAuthorizationBusy: Self = Self(4);
    pub const AVContentAuthorizationNotAvailable: Self = Self(5);
    pub const AVContentAuthorizationNotPossible: Self = Self(6);
}

unsafe impl Encode for AVContentAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVContentAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// AVPlayerItemProtectedContent
    #[cfg(feature = "AVPlayerItem")]
    unsafe impl AVPlayerItem {
        #[method(isAuthorizationRequiredForPlayback)]
        pub unsafe fn isAuthorizationRequiredForPlayback(&self) -> bool;

        #[method(isApplicationAuthorizedForPlayback)]
        pub unsafe fn isApplicationAuthorizedForPlayback(&self) -> bool;

        #[method(isContentAuthorizedForPlayback)]
        pub unsafe fn isContentAuthorizedForPlayback(&self) -> bool;

        #[cfg(feature = "block2")]
        #[method(requestContentAuthorizationAsynchronouslyWithTimeoutInterval:completionHandler:)]
        pub unsafe fn requestContentAuthorizationAsynchronouslyWithTimeoutInterval_completionHandler(
            &self,
            timeout_interval: NSTimeInterval,
            handler: &block2::Block<dyn Fn()>,
        );

        #[method(cancelContentAuthorizationRequest)]
        pub unsafe fn cancelContentAuthorizationRequest(&self);

        #[method(contentAuthorizationRequestStatus)]
        pub unsafe fn contentAuthorizationRequestStatus(&self) -> AVContentAuthorizationStatus;
    }
);
