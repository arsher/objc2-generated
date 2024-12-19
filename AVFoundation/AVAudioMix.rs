//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avaudiomix?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioMix;
);

unsafe impl NSCopying for AVAudioMix {}

unsafe impl CopyingHelper for AVAudioMix {
    type Result = Self;
}

unsafe impl NSMutableCopying for AVAudioMix {}

unsafe impl MutableCopyingHelper for AVAudioMix {
    type Result = AVMutableAudioMix;
}

unsafe impl NSObjectProtocol for AVAudioMix {}

extern_methods!(
    unsafe impl AVAudioMix {
        #[method_id(@__retain_semantics Other inputParameters)]
        pub unsafe fn inputParameters(&self) -> Retained<NSArray<AVAudioMixInputParameters>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioMix {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmutableaudiomix?language=objc)
    #[unsafe(super(AVAudioMix, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMutableAudioMix;
);

unsafe impl NSCopying for AVMutableAudioMix {}

unsafe impl CopyingHelper for AVMutableAudioMix {
    type Result = AVAudioMix;
}

unsafe impl NSMutableCopying for AVMutableAudioMix {}

unsafe impl MutableCopyingHelper for AVMutableAudioMix {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVMutableAudioMix {}

extern_methods!(
    unsafe impl AVMutableAudioMix {
        #[method_id(@__retain_semantics Other audioMix)]
        pub unsafe fn audioMix() -> Retained<Self>;

        #[method_id(@__retain_semantics Other inputParameters)]
        pub unsafe fn inputParameters(&self) -> Retained<NSArray<AVAudioMixInputParameters>>;

        #[method(setInputParameters:)]
        pub unsafe fn setInputParameters(
            &self,
            input_parameters: &NSArray<AVAudioMixInputParameters>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMutableAudioMix {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avaudiomixinputparameters?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioMixInputParameters;
);

unsafe impl NSCopying for AVAudioMixInputParameters {}

unsafe impl CopyingHelper for AVAudioMixInputParameters {
    type Result = Self;
}

unsafe impl NSMutableCopying for AVAudioMixInputParameters {}

unsafe impl MutableCopyingHelper for AVAudioMixInputParameters {
    type Result = AVMutableAudioMixInputParameters;
}

unsafe impl NSObjectProtocol for AVAudioMixInputParameters {}

extern_methods!(
    unsafe impl AVAudioMixInputParameters {
        #[cfg(feature = "objc2-core-media")]
        #[method(trackID)]
        pub unsafe fn trackID(&self) -> CMPersistentTrackID;

        #[cfg(feature = "AVAudioProcessingSettings")]
        #[method_id(@__retain_semantics Other audioTimePitchAlgorithm)]
        pub unsafe fn audioTimePitchAlgorithm(&self)
            -> Option<Retained<AVAudioTimePitchAlgorithm>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(getVolumeRampForTime:startVolume:endVolume:timeRange:)]
        pub unsafe fn getVolumeRampForTime_startVolume_endVolume_timeRange(
            &self,
            time: CMTime,
            start_volume: *mut c_float,
            end_volume: *mut c_float,
            time_range: *mut CMTimeRange,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioMixInputParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmutableaudiomixinputparameters?language=objc)
    #[unsafe(super(AVAudioMixInputParameters, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMutableAudioMixInputParameters;
);

unsafe impl NSCopying for AVMutableAudioMixInputParameters {}

unsafe impl CopyingHelper for AVMutableAudioMixInputParameters {
    type Result = AVAudioMixInputParameters;
}

unsafe impl NSMutableCopying for AVMutableAudioMixInputParameters {}

unsafe impl MutableCopyingHelper for AVMutableAudioMixInputParameters {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVMutableAudioMixInputParameters {}

extern_methods!(
    unsafe impl AVMutableAudioMixInputParameters {
        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other audioMixInputParametersWithTrack:)]
        pub unsafe fn audioMixInputParametersWithTrack(
            track: Option<&AVAssetTrack>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other audioMixInputParameters)]
        pub unsafe fn audioMixInputParameters() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method(trackID)]
        pub unsafe fn trackID(&self) -> CMPersistentTrackID;

        #[cfg(feature = "objc2-core-media")]
        #[method(setTrackID:)]
        pub unsafe fn setTrackID(&self, track_id: CMPersistentTrackID);

        #[cfg(feature = "AVAudioProcessingSettings")]
        #[method_id(@__retain_semantics Other audioTimePitchAlgorithm)]
        pub unsafe fn audioTimePitchAlgorithm(&self)
            -> Option<Retained<AVAudioTimePitchAlgorithm>>;

        #[cfg(feature = "AVAudioProcessingSettings")]
        #[method(setAudioTimePitchAlgorithm:)]
        pub unsafe fn setAudioTimePitchAlgorithm(
            &self,
            audio_time_pitch_algorithm: Option<&AVAudioTimePitchAlgorithm>,
        );

        #[cfg(feature = "objc2-core-media")]
        #[method(setVolumeRampFromStartVolume:toEndVolume:timeRange:)]
        pub unsafe fn setVolumeRampFromStartVolume_toEndVolume_timeRange(
            &self,
            start_volume: c_float,
            end_volume: c_float,
            time_range: CMTimeRange,
        );

        #[cfg(feature = "objc2-core-media")]
        #[method(setVolume:atTime:)]
        pub unsafe fn setVolume_atTime(&self, volume: c_float, time: CMTime);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMutableAudioMixInputParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
