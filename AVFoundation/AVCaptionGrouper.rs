//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptiongrouper?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionGrouper;
);

unsafe impl NSObjectProtocol for AVCaptionGrouper {}

extern_methods!(
    unsafe impl AVCaptionGrouper {
        #[cfg(feature = "AVCaption")]
        #[method(addCaption:)]
        pub unsafe fn addCaption(&self, input: &AVCaption);

        #[cfg(all(feature = "AVCaptionGroup", feature = "objc2-core-media"))]
        #[method_id(@__retain_semantics Other flushAddedCaptionsIntoGroupsUpToTime:)]
        pub unsafe fn flushAddedCaptionsIntoGroupsUpToTime(
            &self,
            up_to_time: CMTime,
        ) -> Retained<NSArray<AVCaptionGroup>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCaptionGrouper {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
