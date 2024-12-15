//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratoraperturemode?language=objc)
// NS_TYPED_ENUM
pub type AVAssetImageGeneratorApertureMode = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratoraperturemodecleanaperture?language=objc)
    pub static AVAssetImageGeneratorApertureModeCleanAperture:
        &'static AVAssetImageGeneratorApertureMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratoraperturemodeproductionaperture?language=objc)
    pub static AVAssetImageGeneratorApertureModeProductionAperture:
        &'static AVAssetImageGeneratorApertureMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratoraperturemodeencodedpixels?language=objc)
    pub static AVAssetImageGeneratorApertureModeEncodedPixels:
        &'static AVAssetImageGeneratorApertureMode;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratordynamicrangepolicy?language=objc)
// NS_TYPED_ENUM
pub type AVAssetImageGeneratorDynamicRangePolicy = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratordynamicrangepolicyforcesdr?language=objc)
    pub static AVAssetImageGeneratorDynamicRangePolicyForceSDR:
        &'static AVAssetImageGeneratorDynamicRangePolicy;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratordynamicrangepolicymatchsource?language=objc)
    pub static AVAssetImageGeneratorDynamicRangePolicyMatchSource:
        &'static AVAssetImageGeneratorDynamicRangePolicy;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratorresult?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAssetImageGeneratorResult(pub NSInteger);
impl AVAssetImageGeneratorResult {
    pub const AVAssetImageGeneratorSucceeded: Self = Self(0);
    pub const AVAssetImageGeneratorFailed: Self = Self(1);
    pub const AVAssetImageGeneratorCancelled: Self = Self(2);
}

unsafe impl Encode for AVAssetImageGeneratorResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVAssetImageGeneratorResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegeneratorcompletionhandler?language=objc)
#[cfg(all(
    feature = "block2",
    feature = "objc2-core-graphics",
    feature = "objc2-core-media"
))]
pub type AVAssetImageGeneratorCompletionHandler = *mut block2::Block<
    dyn Fn(CMTime, CGImageRef, CMTime, AVAssetImageGeneratorResult, *mut NSError),
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetimagegenerator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetImageGenerator;
);

unsafe impl NSObjectProtocol for AVAssetImageGenerator {}

extern_methods!(
    unsafe impl AVAssetImageGenerator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVAsset")]
        #[method_id(@__retain_semantics Other asset)]
        pub unsafe fn asset(&self) -> Retained<AVAsset>;

        #[method(appliesPreferredTrackTransform)]
        pub unsafe fn appliesPreferredTrackTransform(&self) -> bool;

        #[method(setAppliesPreferredTrackTransform:)]
        pub unsafe fn setAppliesPreferredTrackTransform(
            &self,
            applies_preferred_track_transform: bool,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(maximumSize)]
        pub unsafe fn maximumSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMaximumSize:)]
        pub unsafe fn setMaximumSize(&self, maximum_size: CGSize);

        #[method_id(@__retain_semantics Other apertureMode)]
        pub unsafe fn apertureMode(&self) -> Option<Retained<AVAssetImageGeneratorApertureMode>>;

        #[method(setApertureMode:)]
        pub unsafe fn setApertureMode(
            &self,
            aperture_mode: Option<&AVAssetImageGeneratorApertureMode>,
        );

        #[method_id(@__retain_semantics Other dynamicRangePolicy)]
        pub unsafe fn dynamicRangePolicy(
            &self,
        ) -> Retained<AVAssetImageGeneratorDynamicRangePolicy>;

        #[method(setDynamicRangePolicy:)]
        pub unsafe fn setDynamicRangePolicy(
            &self,
            dynamic_range_policy: &AVAssetImageGeneratorDynamicRangePolicy,
        );

        #[cfg(feature = "AVVideoComposition")]
        #[method_id(@__retain_semantics Other videoComposition)]
        pub unsafe fn videoComposition(&self) -> Option<Retained<AVVideoComposition>>;

        #[cfg(feature = "AVVideoComposition")]
        #[method(setVideoComposition:)]
        pub unsafe fn setVideoComposition(&self, video_composition: Option<&AVVideoComposition>);

        #[cfg(feature = "AVVideoCompositing")]
        #[method_id(@__retain_semantics Other customVideoCompositor)]
        pub unsafe fn customVideoCompositor(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVVideoCompositing>>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(requestedTimeToleranceBefore)]
        pub unsafe fn requestedTimeToleranceBefore(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setRequestedTimeToleranceBefore:)]
        pub unsafe fn setRequestedTimeToleranceBefore(
            &self,
            requested_time_tolerance_before: CMTime,
        );

        #[cfg(feature = "objc2-core-media")]
        #[method(requestedTimeToleranceAfter)]
        pub unsafe fn requestedTimeToleranceAfter(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setRequestedTimeToleranceAfter:)]
        pub unsafe fn setRequestedTimeToleranceAfter(&self, requested_time_tolerance_after: CMTime);

        #[cfg(feature = "AVAsset")]
        #[method_id(@__retain_semantics Other assetImageGeneratorWithAsset:)]
        pub unsafe fn assetImageGeneratorWithAsset(asset: &AVAsset) -> Retained<Self>;

        #[cfg(feature = "AVAsset")]
        #[method_id(@__retain_semantics Init initWithAsset:)]
        pub unsafe fn initWithAsset(this: Allocated<Self>, asset: &AVAsset) -> Retained<Self>;

        #[cfg(all(
            feature = "block2",
            feature = "objc2-core-graphics",
            feature = "objc2-core-media"
        ))]
        #[method(generateCGImagesAsynchronouslyForTimes:completionHandler:)]
        pub unsafe fn generateCGImagesAsynchronouslyForTimes_completionHandler(
            &self,
            requested_times: &NSArray<NSValue>,
            handler: AVAssetImageGeneratorCompletionHandler,
        );

        #[cfg(all(
            feature = "block2",
            feature = "objc2-core-graphics",
            feature = "objc2-core-media"
        ))]
        #[method(generateCGImageAsynchronouslyForTime:completionHandler:)]
        pub unsafe fn generateCGImageAsynchronouslyForTime_completionHandler(
            &self,
            requested_time: CMTime,
            handler: &block2::Block<dyn Fn(CGImageRef, CMTime, *mut NSError)>,
        );

        #[method(cancelAllCGImageGeneration)]
        pub unsafe fn cancelAllCGImageGeneration(&self);
    }
);
