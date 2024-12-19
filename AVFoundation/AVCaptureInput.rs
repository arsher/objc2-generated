//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureinput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureInput;
);

unsafe impl NSObjectProtocol for AVCaptureInput {}

extern_methods!(
    unsafe impl AVCaptureInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other ports)]
        pub unsafe fn ports(&self) -> Retained<NSArray<AVCaptureInputPort>>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureinputportformatdescriptiondidchangenotification?language=objc)
    pub static AVCaptureInputPortFormatDescriptionDidChangeNotification:
        &'static NSNotificationName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureinputport?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureInputPort;
);

unsafe impl NSObjectProtocol for AVCaptureInputPort {}

extern_methods!(
    unsafe impl AVCaptureInputPort {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other input)]
        pub unsafe fn input(&self) -> Retained<AVCaptureInput>;

        #[cfg(feature = "AVMediaFormat")]
        #[method_id(@__retain_semantics Other mediaType)]
        pub unsafe fn mediaType(&self) -> Retained<AVMediaType>;

        #[cfg(feature = "objc2-core-media")]
        #[method(formatDescription)]
        pub unsafe fn formatDescription(&self) -> CMFormatDescriptionRef;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "objc2-core-media")]
        #[method(clock)]
        pub unsafe fn clock(&self) -> CMClockRef;

        #[cfg(feature = "AVCaptureDevice")]
        #[method_id(@__retain_semantics Other sourceDeviceType)]
        pub unsafe fn sourceDeviceType(&self) -> Option<Retained<AVCaptureDeviceType>>;

        #[cfg(feature = "AVCaptureDevice")]
        #[method(sourceDevicePosition)]
        pub unsafe fn sourceDevicePosition(&self) -> AVCaptureDevicePosition;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturemultichannelaudiomode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptureMultichannelAudioMode(pub NSInteger);
impl AVCaptureMultichannelAudioMode {
    #[doc(alias = "AVCaptureMultichannelAudioModeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "AVCaptureMultichannelAudioModeStereo")]
    pub const Stereo: Self = Self(1);
    #[doc(alias = "AVCaptureMultichannelAudioModeFirstOrderAmbisonics")]
    pub const FirstOrderAmbisonics: Self = Self(2);
}

unsafe impl Encode for AVCaptureMultichannelAudioMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptureMultichannelAudioMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturedeviceinput?language=objc)
    #[unsafe(super(AVCaptureInput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeviceInput;
);

unsafe impl NSObjectProtocol for AVCaptureDeviceInput {}

extern_methods!(
    unsafe impl AVCaptureDeviceInput {
        #[cfg(feature = "AVCaptureDevice")]
        #[method_id(@__retain_semantics Other deviceInputWithDevice:error:_)]
        pub unsafe fn deviceInputWithDevice_error(
            device: &AVCaptureDevice,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "AVCaptureDevice")]
        #[method_id(@__retain_semantics Init initWithDevice:error:_)]
        pub unsafe fn initWithDevice_error(
            this: Allocated<Self>,
            device: &AVCaptureDevice,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "AVCaptureDevice")]
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Retained<AVCaptureDevice>;

        #[method(unifiedAutoExposureDefaultsEnabled)]
        pub unsafe fn unifiedAutoExposureDefaultsEnabled(&self) -> bool;

        #[method(setUnifiedAutoExposureDefaultsEnabled:)]
        pub unsafe fn setUnifiedAutoExposureDefaultsEnabled(
            &self,
            unified_auto_exposure_defaults_enabled: bool,
        );

        #[cfg(all(feature = "AVCaptureDevice", feature = "AVMediaFormat"))]
        #[method_id(@__retain_semantics Other portsWithMediaType:sourceDeviceType:sourceDevicePosition:)]
        pub unsafe fn portsWithMediaType_sourceDeviceType_sourceDevicePosition(
            &self,
            media_type: Option<&AVMediaType>,
            source_device_type: Option<&AVCaptureDeviceType>,
            source_device_position: AVCaptureDevicePosition,
        ) -> Retained<NSArray<AVCaptureInputPort>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(videoMinFrameDurationOverride)]
        pub unsafe fn videoMinFrameDurationOverride(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setVideoMinFrameDurationOverride:)]
        pub unsafe fn setVideoMinFrameDurationOverride(
            &self,
            video_min_frame_duration_override: CMTime,
        );

        #[method(isMultichannelAudioModeSupported:)]
        pub unsafe fn isMultichannelAudioModeSupported(
            &self,
            multichannel_audio_mode: AVCaptureMultichannelAudioMode,
        ) -> bool;

        #[method(multichannelAudioMode)]
        pub unsafe fn multichannelAudioMode(&self) -> AVCaptureMultichannelAudioMode;

        #[method(setMultichannelAudioMode:)]
        pub unsafe fn setMultichannelAudioMode(
            &self,
            multichannel_audio_mode: AVCaptureMultichannelAudioMode,
        );

        #[method(isWindNoiseRemovalSupported)]
        pub unsafe fn isWindNoiseRemovalSupported(&self) -> bool;

        #[method(isWindNoiseRemovalEnabled)]
        pub unsafe fn isWindNoiseRemovalEnabled(&self) -> bool;

        #[method(setWindNoiseRemovalEnabled:)]
        pub unsafe fn setWindNoiseRemovalEnabled(&self, wind_noise_removal_enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `AVCaptureInput`
    unsafe impl AVCaptureDeviceInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturescreeninput?language=objc)
    #[unsafe(super(AVCaptureInput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureScreenInput;
);

unsafe impl NSObjectProtocol for AVCaptureScreenInput {}

extern_methods!(
    unsafe impl AVCaptureScreenInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Init initWithDisplayID:)]
        pub unsafe fn initWithDisplayID(
            this: Allocated<Self>,
            display_id: CGDirectDisplayID,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(minFrameDuration)]
        pub unsafe fn minFrameDuration(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setMinFrameDuration:)]
        pub unsafe fn setMinFrameDuration(&self, min_frame_duration: CMTime);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(cropRect)]
        pub unsafe fn cropRect(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setCropRect:)]
        pub unsafe fn setCropRect(&self, crop_rect: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(scaleFactor)]
        pub unsafe fn scaleFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setScaleFactor:)]
        pub unsafe fn setScaleFactor(&self, scale_factor: CGFloat);

        #[method(capturesMouseClicks)]
        pub unsafe fn capturesMouseClicks(&self) -> bool;

        #[method(setCapturesMouseClicks:)]
        pub unsafe fn setCapturesMouseClicks(&self, captures_mouse_clicks: bool);

        #[method(capturesCursor)]
        pub unsafe fn capturesCursor(&self) -> bool;

        #[method(setCapturesCursor:)]
        pub unsafe fn setCapturesCursor(&self, captures_cursor: bool);

        #[deprecated = "No longer supported."]
        #[method(removesDuplicateFrames)]
        pub unsafe fn removesDuplicateFrames(&self) -> bool;

        #[deprecated = "No longer supported."]
        #[method(setRemovesDuplicateFrames:)]
        pub unsafe fn setRemovesDuplicateFrames(&self, removes_duplicate_frames: bool);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturemetadatainput?language=objc)
    #[unsafe(super(AVCaptureInput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureMetadataInput;
);

unsafe impl NSObjectProtocol for AVCaptureMetadataInput {}

extern_methods!(
    unsafe impl AVCaptureMetadataInput {
        #[cfg(feature = "objc2-core-media")]
        #[method_id(@__retain_semantics Other metadataInputWithFormatDescription:clock:)]
        pub unsafe fn metadataInputWithFormatDescription_clock(
            desc: CMMetadataFormatDescriptionRef,
            clock: CMClockRef,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method_id(@__retain_semantics Init initWithFormatDescription:clock:)]
        pub unsafe fn initWithFormatDescription_clock(
            this: Allocated<Self>,
            desc: CMMetadataFormatDescriptionRef,
            clock: CMClockRef,
        ) -> Retained<Self>;

        #[cfg(feature = "AVTimedMetadataGroup")]
        #[method(appendTimedMetadataGroup:error:_)]
        pub unsafe fn appendTimedMetadataGroup_error(
            &self,
            metadata: &AVTimedMetadataGroup,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVCaptureInput`
    unsafe impl AVCaptureMetadataInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
