//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avrenderedcaptionimage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVRenderedCaptionImage;
);

unsafe impl NSObjectProtocol for AVRenderedCaptionImage {}

extern_methods!(
    unsafe impl AVRenderedCaptionImage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-video")]
        #[method(pixelBuffer)]
        pub unsafe fn pixelBuffer(&self) -> CVPixelBufferRef;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(position)]
        pub unsafe fn position(&self) -> CGPoint;
    }
);
