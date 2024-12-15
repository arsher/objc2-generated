//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspreset?language=objc)
// NS_TYPED_ENUM
pub type AVOutputSettingsPreset = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspreset640x480?language=objc)
    pub static AVOutputSettingsPreset640x480: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspreset960x540?language=objc)
    pub static AVOutputSettingsPreset960x540: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspreset1280x720?language=objc)
    pub static AVOutputSettingsPreset1280x720: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspreset1920x1080?language=objc)
    pub static AVOutputSettingsPreset1920x1080: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspreset3840x2160?language=objc)
    pub static AVOutputSettingsPreset3840x2160: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspresethevc1920x1080?language=objc)
    pub static AVOutputSettingsPresetHEVC1920x1080: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspresethevc1920x1080withalpha?language=objc)
    pub static AVOutputSettingsPresetHEVC1920x1080WithAlpha: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspresethevc3840x2160?language=objc)
    pub static AVOutputSettingsPresetHEVC3840x2160: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspresethevc3840x2160withalpha?language=objc)
    pub static AVOutputSettingsPresetHEVC3840x2160WithAlpha: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspresethevc7680x4320?language=objc)
    pub static AVOutputSettingsPresetHEVC7680x4320: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspresetmvhevc960x960?language=objc)
    pub static AVOutputSettingsPresetMVHEVC960x960: &'static AVOutputSettingsPreset;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingspresetmvhevc1440x1440?language=objc)
    pub static AVOutputSettingsPresetMVHEVC1440x1440: &'static AVOutputSettingsPreset;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avoutputsettingsassistant?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVOutputSettingsAssistant;
);

unsafe impl NSObjectProtocol for AVOutputSettingsAssistant {}

extern_methods!(
    unsafe impl AVOutputSettingsAssistant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other availableOutputSettingsPresets)]
        pub unsafe fn availableOutputSettingsPresets() -> Retained<NSArray<AVOutputSettingsPreset>>;

        #[method_id(@__retain_semantics Other outputSettingsAssistantWithPreset:)]
        pub unsafe fn outputSettingsAssistantWithPreset(
            preset_identifier: &AVOutputSettingsPreset,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other audioSettings)]
        pub unsafe fn audioSettings(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method_id(@__retain_semantics Other videoSettings)]
        pub unsafe fn videoSettings(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(feature = "AVMediaFormat")]
        #[method_id(@__retain_semantics Other outputFileType)]
        pub unsafe fn outputFileType(&self) -> Retained<AVFileType>;
    }
);

extern_methods!(
    /// AVOutputSettingsAssistant_SourceInformation
    unsafe impl AVOutputSettingsAssistant {
        #[cfg(feature = "objc2-core-media")]
        #[method(sourceAudioFormat)]
        pub unsafe fn sourceAudioFormat(&self) -> CMAudioFormatDescriptionRef;

        #[cfg(feature = "objc2-core-media")]
        #[method(setSourceAudioFormat:)]
        pub unsafe fn setSourceAudioFormat(&self, source_audio_format: CMAudioFormatDescriptionRef);

        #[cfg(feature = "objc2-core-media")]
        #[method(sourceVideoFormat)]
        pub unsafe fn sourceVideoFormat(&self) -> CMVideoFormatDescriptionRef;

        #[cfg(feature = "objc2-core-media")]
        #[method(setSourceVideoFormat:)]
        pub unsafe fn setSourceVideoFormat(&self, source_video_format: CMVideoFormatDescriptionRef);

        #[cfg(feature = "objc2-core-media")]
        #[method(sourceVideoAverageFrameDuration)]
        pub unsafe fn sourceVideoAverageFrameDuration(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setSourceVideoAverageFrameDuration:)]
        pub unsafe fn setSourceVideoAverageFrameDuration(
            &self,
            source_video_average_frame_duration: CMTime,
        );

        #[cfg(feature = "objc2-core-media")]
        #[method(sourceVideoMinFrameDuration)]
        pub unsafe fn sourceVideoMinFrameDuration(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setSourceVideoMinFrameDuration:)]
        pub unsafe fn setSourceVideoMinFrameDuration(
            &self,
            source_video_min_frame_duration: CMTime,
        );
    }
);
