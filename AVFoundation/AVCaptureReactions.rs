//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontype?language=objc)
// NS_TYPED_ENUM
pub type AVCaptureReactionType = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontypethumbsup?language=objc)
    pub static AVCaptureReactionTypeThumbsUp: &'static AVCaptureReactionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontypethumbsdown?language=objc)
    pub static AVCaptureReactionTypeThumbsDown: &'static AVCaptureReactionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontypeballoons?language=objc)
    pub static AVCaptureReactionTypeBalloons: &'static AVCaptureReactionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontypeheart?language=objc)
    pub static AVCaptureReactionTypeHeart: &'static AVCaptureReactionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontypefireworks?language=objc)
    pub static AVCaptureReactionTypeFireworks: &'static AVCaptureReactionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontyperain?language=objc)
    pub static AVCaptureReactionTypeRain: &'static AVCaptureReactionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontypeconfetti?language=objc)
    pub static AVCaptureReactionTypeConfetti: &'static AVCaptureReactionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactiontypelasers?language=objc)
    pub static AVCaptureReactionTypeLasers: &'static AVCaptureReactionType;
}

extern "C-unwind" {
    pub fn AVCaptureReactionSystemImageNameForType(
        reaction_type: &AVCaptureReactionType,
    ) -> NonNull<NSString>;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturereactioneffectstate?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureReactionEffectState;
);

unsafe impl NSObjectProtocol for AVCaptureReactionEffectState {}

extern_methods!(
    unsafe impl AVCaptureReactionEffectState {
        #[method_id(@__retain_semantics Other reactionType)]
        pub unsafe fn reactionType(&self) -> Retained<AVCaptureReactionType>;

        #[cfg(feature = "objc2-core-media")]
        #[method(startTime)]
        pub unsafe fn startTime(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(endTime)]
        pub unsafe fn endTime(&self) -> CMTime;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCaptureReactionEffectState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);