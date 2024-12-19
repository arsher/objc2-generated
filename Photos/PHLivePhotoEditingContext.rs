//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-image")]
use objc2_core_image::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoframeprocessingblock?language=objc)
#[cfg(all(feature = "block2", feature = "objc2-core-image"))]
pub type PHLivePhotoFrameProcessingBlock = *mut block2::Block<
    dyn Fn(NonNull<ProtocolObject<dyn PHLivePhotoFrame>>, NonNull<*mut NSError>) -> *mut CIImage,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoeditingoption?language=objc)
// NS_TYPED_ENUM
pub type PHLivePhotoEditingOption = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoeditingcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHLivePhotoEditingContext;
);

unsafe impl NSObjectProtocol for PHLivePhotoEditingContext {}

extern_methods!(
    unsafe impl PHLivePhotoEditingContext {
        #[cfg(feature = "PHContentEditingInput")]
        #[method_id(@__retain_semantics Init initWithLivePhotoEditingInput:)]
        pub unsafe fn initWithLivePhotoEditingInput(
            this: Allocated<Self>,
            live_photo_input: &PHContentEditingInput,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-image")]
        #[method_id(@__retain_semantics Other fullSizeImage)]
        pub unsafe fn fullSizeImage(&self) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-media")]
        #[method(duration)]
        pub unsafe fn duration(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(photoTime)]
        pub unsafe fn photoTime(&self) -> CMTime;

        #[cfg(all(feature = "block2", feature = "objc2-core-image"))]
        #[method(frameProcessor)]
        pub unsafe fn frameProcessor(&self) -> PHLivePhotoFrameProcessingBlock;

        #[cfg(all(feature = "block2", feature = "objc2-core-image"))]
        #[method(setFrameProcessor:)]
        pub unsafe fn setFrameProcessor(&self, frame_processor: PHLivePhotoFrameProcessingBlock);

        #[method(audioVolume)]
        pub unsafe fn audioVolume(&self) -> c_float;

        #[method(setAudioVolume:)]
        pub unsafe fn setAudioVolume(&self, audio_volume: c_float);

        #[cfg(all(
            feature = "PHLivePhoto",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        #[method(prepareLivePhotoForPlaybackWithTargetSize:options:completionHandler:)]
        pub unsafe fn prepareLivePhotoForPlaybackWithTargetSize_options_completionHandler(
            &self,
            target_size: CGSize,
            options: Option<&NSDictionary<NSString, AnyObject>>,
            handler: &block2::Block<dyn Fn(*mut PHLivePhoto, *mut NSError)>,
        );

        #[cfg(all(feature = "PHContentEditingOutput", feature = "block2"))]
        #[method(saveLivePhotoToOutput:options:completionHandler:)]
        pub unsafe fn saveLivePhotoToOutput_options_completionHandler(
            &self,
            output: &PHContentEditingOutput,
            options: Option<&NSDictionary<NSString, AnyObject>>,
            handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHLivePhotoEditingContext {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoframetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHLivePhotoFrameType(pub NSInteger);
impl PHLivePhotoFrameType {
    #[doc(alias = "PHLivePhotoFrameTypePhoto")]
    pub const Photo: Self = Self(0);
    #[doc(alias = "PHLivePhotoFrameTypeVideo")]
    pub const Video: Self = Self(1);
}

unsafe impl Encode for PHLivePhotoFrameType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHLivePhotoFrameType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoframe?language=objc)
    pub unsafe trait PHLivePhotoFrame {
        #[cfg(feature = "objc2-core-image")]
        #[method_id(@__retain_semantics Other image)]
        unsafe fn image(&self) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-media")]
        #[method(time)]
        unsafe fn time(&self) -> CMTime;

        #[method(type)]
        unsafe fn r#type(&self) -> PHLivePhotoFrameType;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(renderScale)]
        unsafe fn renderScale(&self) -> CGFloat;
    }

    unsafe impl ProtocolType for dyn PHLivePhotoFrame {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoshouldrenderatplaybacktime?language=objc)
    pub static PHLivePhotoShouldRenderAtPlaybackTime: &'static PHLivePhotoEditingOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoeditingerrordomain?language=objc)
    pub static PHLivePhotoEditingErrorDomain: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoeditingerrorcode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHLivePhotoEditingErrorCode(pub NSInteger);
impl PHLivePhotoEditingErrorCode {
    #[deprecated]
    #[doc(alias = "PHLivePhotoEditingErrorCodeUnknown")]
    pub const Unknown: Self = Self(0);
    #[deprecated]
    #[doc(alias = "PHLivePhotoEditingErrorCodeAborted")]
    pub const Aborted: Self = Self(1);
}

unsafe impl Encode for PHLivePhotoEditingErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHLivePhotoEditingErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
