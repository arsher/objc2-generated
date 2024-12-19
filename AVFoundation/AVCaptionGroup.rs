//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptiongroup?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionGroup;
);

unsafe impl NSObjectProtocol for AVCaptionGroup {}

extern_methods!(
    unsafe impl AVCaptionGroup {
        #[cfg(all(feature = "AVCaption", feature = "objc2-core-media"))]
        #[method_id(@__retain_semantics Init initWithCaptions:timeRange:)]
        pub unsafe fn initWithCaptions_timeRange(
            this: Allocated<Self>,
            captions: &NSArray<AVCaption>,
            time_range: CMTimeRange,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method_id(@__retain_semantics Init initWithTimeRange:)]
        pub unsafe fn initWithTimeRange(
            this: Allocated<Self>,
            time_range: CMTimeRange,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method(timeRange)]
        pub unsafe fn timeRange(&self) -> CMTimeRange;

        #[cfg(feature = "AVCaption")]
        #[method_id(@__retain_semantics Other captions)]
        pub unsafe fn captions(&self) -> Retained<NSArray<AVCaption>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCaptionGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
