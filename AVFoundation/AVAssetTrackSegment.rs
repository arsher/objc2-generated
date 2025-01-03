//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassettracksegment?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetTrackSegment;
);

unsafe impl Send for AVAssetTrackSegment {}

unsafe impl Sync for AVAssetTrackSegment {}

unsafe impl NSObjectProtocol for AVAssetTrackSegment {}

extern_methods!(
    unsafe impl AVAssetTrackSegment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method(timeMapping)]
        pub unsafe fn timeMapping(&self) -> CMTimeMapping;

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;
    }
);
