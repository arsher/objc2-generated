//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionlocation?language=objc)
// NS_TYPED_ENUM
pub type AVAudioSessionLocation = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionlocationupper?language=objc)
    pub static AVAudioSessionLocationUpper: &'static AVAudioSessionLocation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionlocationlower?language=objc)
    pub static AVAudioSessionLocationLower: &'static AVAudioSessionLocation;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientation?language=objc)
// NS_TYPED_ENUM
pub type AVAudioSessionOrientation = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationtop?language=objc)
    pub static AVAudioSessionOrientationTop: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationbottom?language=objc)
    pub static AVAudioSessionOrientationBottom: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationfront?language=objc)
    pub static AVAudioSessionOrientationFront: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationback?language=objc)
    pub static AVAudioSessionOrientationBack: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationleft?language=objc)
    pub static AVAudioSessionOrientationLeft: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationright?language=objc)
    pub static AVAudioSessionOrientationRight: &'static AVAudioSessionOrientation;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpattern?language=objc)
// NS_TYPED_ENUM
pub type AVAudioSessionPolarPattern = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpatternomnidirectional?language=objc)
    pub static AVAudioSessionPolarPatternOmnidirectional: &'static AVAudioSessionPolarPattern;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpatterncardioid?language=objc)
    pub static AVAudioSessionPolarPatternCardioid: &'static AVAudioSessionPolarPattern;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpatternsubcardioid?language=objc)
    pub static AVAudioSessionPolarPatternSubcardioid: &'static AVAudioSessionPolarPattern;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpatternstereo?language=objc)
    pub static AVAudioSessionPolarPatternStereo: &'static AVAudioSessionPolarPattern;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionchanneldescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSessionChannelDescription;
);

unsafe impl Send for AVAudioSessionChannelDescription {}

unsafe impl Sync for AVAudioSessionChannelDescription {}

unsafe impl NSObjectProtocol for AVAudioSessionChannelDescription {}

extern_methods!(
    unsafe impl AVAudioSessionChannelDescription {
        #[method_id(@__retain_semantics Other channelName)]
        pub unsafe fn channelName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other owningPortUID)]
        pub unsafe fn owningPortUID(&self) -> Retained<NSString>;

        #[method(channelNumber)]
        pub unsafe fn channelNumber(&self) -> NSUInteger;

        #[cfg(feature = "objc2-core-audio-types")]
        #[method(channelLabel)]
        pub unsafe fn channelLabel(&self) -> AudioChannelLabel;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSessionChannelDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessiondatasourcedescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSessionDataSourceDescription;
);

unsafe impl Send for AVAudioSessionDataSourceDescription {}

unsafe impl Sync for AVAudioSessionDataSourceDescription {}

unsafe impl NSObjectProtocol for AVAudioSessionDataSourceDescription {}

extern_methods!(
    unsafe impl AVAudioSessionDataSourceDescription {
        #[method_id(@__retain_semantics Other dataSourceID)]
        pub unsafe fn dataSourceID(&self) -> Retained<NSNumber>;

        #[method_id(@__retain_semantics Other dataSourceName)]
        pub unsafe fn dataSourceName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Retained<AVAudioSessionLocation>>;

        #[method_id(@__retain_semantics Other orientation)]
        pub unsafe fn orientation(&self) -> Option<Retained<AVAudioSessionOrientation>>;

        #[method_id(@__retain_semantics Other supportedPolarPatterns)]
        pub unsafe fn supportedPolarPatterns(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionPolarPattern>>>;

        #[method_id(@__retain_semantics Other selectedPolarPattern)]
        pub unsafe fn selectedPolarPattern(&self) -> Option<Retained<AVAudioSessionPolarPattern>>;

        #[method_id(@__retain_semantics Other preferredPolarPattern)]
        pub unsafe fn preferredPolarPattern(&self) -> Option<Retained<AVAudioSessionPolarPattern>>;

        #[method(setPreferredPolarPattern:error:_)]
        pub unsafe fn setPreferredPolarPattern_error(
            &self,
            pattern: Option<&AVAudioSessionPolarPattern>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSessionDataSourceDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionportdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSessionPortDescription;
);

unsafe impl Send for AVAudioSessionPortDescription {}

unsafe impl Sync for AVAudioSessionPortDescription {}

unsafe impl NSObjectProtocol for AVAudioSessionPortDescription {}

extern_methods!(
    unsafe impl AVAudioSessionPortDescription {
        #[cfg(feature = "AVAudioSessionTypes")]
        #[method_id(@__retain_semantics Other portType)]
        pub unsafe fn portType(&self) -> Retained<AVAudioSessionPort>;

        #[method_id(@__retain_semantics Other portName)]
        pub unsafe fn portName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other UID)]
        pub unsafe fn UID(&self) -> Retained<NSString>;

        #[method(hasHardwareVoiceCallProcessing)]
        pub unsafe fn hasHardwareVoiceCallProcessing(&self) -> bool;

        #[method(isSpatialAudioEnabled)]
        pub unsafe fn isSpatialAudioEnabled(&self) -> bool;

        #[method_id(@__retain_semantics Other channels)]
        pub unsafe fn channels(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionChannelDescription>>>;

        #[method_id(@__retain_semantics Other dataSources)]
        pub unsafe fn dataSources(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionDataSourceDescription>>>;

        #[method_id(@__retain_semantics Other selectedDataSource)]
        pub unsafe fn selectedDataSource(
            &self,
        ) -> Option<Retained<AVAudioSessionDataSourceDescription>>;

        #[method_id(@__retain_semantics Other preferredDataSource)]
        pub unsafe fn preferredDataSource(
            &self,
        ) -> Option<Retained<AVAudioSessionDataSourceDescription>>;

        #[method(setPreferredDataSource:error:_)]
        pub unsafe fn setPreferredDataSource_error(
            &self,
            data_source: Option<&AVAudioSessionDataSourceDescription>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSessionPortDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionroutedescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSessionRouteDescription;
);

unsafe impl Send for AVAudioSessionRouteDescription {}

unsafe impl Sync for AVAudioSessionRouteDescription {}

unsafe impl NSObjectProtocol for AVAudioSessionRouteDescription {}

extern_methods!(
    unsafe impl AVAudioSessionRouteDescription {
        #[method_id(@__retain_semantics Other inputs)]
        pub unsafe fn inputs(&self) -> Retained<NSArray<AVAudioSessionPortDescription>>;

        #[method_id(@__retain_semantics Other outputs)]
        pub unsafe fn outputs(&self) -> Retained<NSArray<AVAudioSessionPortDescription>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSessionRouteDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
