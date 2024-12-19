//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-av-foundation")]
use objc2_av_foundation::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phcontenteditinginput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHContentEditingInput;
);

unsafe impl NSObjectProtocol for PHContentEditingInput {}

extern_methods!(
    unsafe impl PHContentEditingInput {
        #[cfg(feature = "PhotosTypes")]
        #[method(mediaType)]
        pub unsafe fn mediaType(&self) -> PHAssetMediaType;

        #[cfg(feature = "PhotosTypes")]
        #[method(mediaSubtypes)]
        pub unsafe fn mediaSubtypes(&self) -> PHAssetMediaSubtype;

        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Retained<CLLocation>>;

        #[method_id(@__retain_semantics Other uniformTypeIdentifier)]
        pub unsafe fn uniformTypeIdentifier(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "PhotosTypes")]
        #[method(playbackStyle)]
        pub unsafe fn playbackStyle(&self) -> PHAssetPlaybackStyle;

        #[cfg(feature = "PHAdjustmentData")]
        #[method_id(@__retain_semantics Other adjustmentData)]
        pub unsafe fn adjustmentData(&self) -> Option<Retained<PHAdjustmentData>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other displaySizeImage)]
        pub unsafe fn displaySizeImage(&self) -> Option<Retained<NSImage>>;

        #[method_id(@__retain_semantics Other fullSizeImageURL)]
        pub unsafe fn fullSizeImageURL(&self) -> Option<Retained<NSURL>>;

        #[method(fullSizeImageOrientation)]
        pub unsafe fn fullSizeImageOrientation(&self) -> c_int;

        #[cfg(feature = "objc2-av-foundation")]
        #[deprecated]
        #[method_id(@__retain_semantics Other avAsset)]
        pub unsafe fn avAsset(&self) -> Option<Retained<AVAsset>>;

        #[cfg(feature = "objc2-av-foundation")]
        #[method_id(@__retain_semantics Other audiovisualAsset)]
        pub unsafe fn audiovisualAsset(&self) -> Option<Retained<AVAsset>>;

        #[cfg(feature = "PHLivePhoto")]
        #[method_id(@__retain_semantics Other livePhoto)]
        pub unsafe fn livePhoto(&self) -> Option<Retained<PHLivePhoto>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHContentEditingInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
