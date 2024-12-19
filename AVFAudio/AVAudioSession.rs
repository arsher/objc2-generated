//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosession?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSession;
);

unsafe impl Send for AVAudioSession {}

unsafe impl Sync for AVAudioSession {}

unsafe impl NSObjectProtocol for AVAudioSession {}

extern_methods!(
    unsafe impl AVAudioSession {
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Retained<AVAudioSession>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method_id(@__retain_semantics Other availableCategories)]
        pub unsafe fn availableCategories(&self) -> Retained<NSArray<AVAudioSessionCategory>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(setCategory:error:_)]
        pub unsafe fn setCategory_error(
            &self,
            category: &AVAudioSessionCategory,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(setCategory:withOptions:error:_)]
        pub unsafe fn setCategory_withOptions_error(
            &self,
            category: &AVAudioSessionCategory,
            options: AVAudioSessionCategoryOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(setCategory:mode:options:error:_)]
        pub unsafe fn setCategory_mode_options_error(
            &self,
            category: &AVAudioSessionCategory,
            mode: &AVAudioSessionMode,
            options: AVAudioSessionCategoryOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(setCategory:mode:routeSharingPolicy:options:error:_)]
        pub unsafe fn setCategory_mode_routeSharingPolicy_options_error(
            &self,
            category: &AVAudioSessionCategory,
            mode: &AVAudioSessionMode,
            policy: AVAudioSessionRouteSharingPolicy,
            options: AVAudioSessionCategoryOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Retained<AVAudioSessionCategory>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(categoryOptions)]
        pub unsafe fn categoryOptions(&self) -> AVAudioSessionCategoryOptions;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(routeSharingPolicy)]
        pub unsafe fn routeSharingPolicy(&self) -> AVAudioSessionRouteSharingPolicy;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method_id(@__retain_semantics Other availableModes)]
        pub unsafe fn availableModes(&self) -> Retained<NSArray<AVAudioSessionMode>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(setMode:error:_)]
        pub unsafe fn setMode_error(
            &self,
            mode: &AVAudioSessionMode,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method_id(@__retain_semantics Other mode)]
        pub unsafe fn mode(&self) -> Retained<AVAudioSessionMode>;

        #[method(setAllowHapticsAndSystemSoundsDuringRecording:error:_)]
        pub unsafe fn setAllowHapticsAndSystemSoundsDuringRecording_error(
            &self,
            in_value: bool,
        ) -> Result<(), Retained<NSError>>;

        #[method(allowHapticsAndSystemSoundsDuringRecording)]
        pub unsafe fn allowHapticsAndSystemSoundsDuringRecording(&self) -> bool;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[deprecated = "Please use AVAudioApplication recordPermission"]
        #[method(recordPermission)]
        pub unsafe fn recordPermission(&self) -> AVAudioSessionRecordPermission;

        #[cfg(feature = "block2")]
        #[deprecated = "Please use AVAudioApplication requestRecordPermissionWithCompletionHandler"]
        #[method(requestRecordPermission:)]
        pub unsafe fn requestRecordPermission(&self, response: &block2::Block<dyn Fn(Bool)>);

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(overrideOutputAudioPort:error:_)]
        pub unsafe fn overrideOutputAudioPort_error(
            &self,
            port_override: AVAudioSessionPortOverride,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method(setPreferredInput:error:_)]
        pub unsafe fn setPreferredInput_error(
            &self,
            in_port: Option<&AVAudioSessionPortDescription>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other preferredInput)]
        pub unsafe fn preferredInput(&self) -> Option<Retained<AVAudioSessionPortDescription>>;

        #[method(setPrefersNoInterruptionsFromSystemAlerts:error:_)]
        pub unsafe fn setPrefersNoInterruptionsFromSystemAlerts_error(
            &self,
            in_value: bool,
        ) -> Result<(), Retained<NSError>>;

        #[method(prefersNoInterruptionsFromSystemAlerts)]
        pub unsafe fn prefersNoInterruptionsFromSystemAlerts(&self) -> bool;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(renderingMode)]
        pub unsafe fn renderingMode(&self) -> AVAudioSessionRenderingMode;
    }
);

extern_methods!(
    /// Activation
    unsafe impl AVAudioSession {
        #[method(setActive:error:_)]
        pub unsafe fn setActive_error(&self, active: bool) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(setActive:withOptions:error:_)]
        pub unsafe fn setActive_withOptions_error(
            &self,
            active: bool,
            options: AVAudioSessionSetActiveOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "AVAudioSessionTypes", feature = "block2"))]
        #[method(activateWithOptions:completionHandler:)]
        pub unsafe fn activateWithOptions_completionHandler(
            &self,
            options: AVAudioSessionActivationOptions,
            handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// AVAudioSessionHardwareConfiguration
    unsafe impl AVAudioSession {
        #[method(setPreferredSampleRate:error:_)]
        pub unsafe fn setPreferredSampleRate_error(
            &self,
            sample_rate: c_double,
        ) -> Result<(), Retained<NSError>>;

        #[method(preferredSampleRate)]
        pub unsafe fn preferredSampleRate(&self) -> c_double;

        #[method(setPreferredIOBufferDuration:error:_)]
        pub unsafe fn setPreferredIOBufferDuration_error(
            &self,
            duration: NSTimeInterval,
        ) -> Result<(), Retained<NSError>>;

        #[method(preferredIOBufferDuration)]
        pub unsafe fn preferredIOBufferDuration(&self) -> NSTimeInterval;

        #[method(setPreferredInputNumberOfChannels:error:_)]
        pub unsafe fn setPreferredInputNumberOfChannels_error(
            &self,
            count: NSInteger,
        ) -> Result<(), Retained<NSError>>;

        #[method(preferredInputNumberOfChannels)]
        pub unsafe fn preferredInputNumberOfChannels(&self) -> NSInteger;

        #[method(setPreferredOutputNumberOfChannels:error:_)]
        pub unsafe fn setPreferredOutputNumberOfChannels_error(
            &self,
            count: NSInteger,
        ) -> Result<(), Retained<NSError>>;

        #[method(preferredOutputNumberOfChannels)]
        pub unsafe fn preferredOutputNumberOfChannels(&self) -> NSInteger;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(setPreferredInputOrientation:error:_)]
        pub unsafe fn setPreferredInputOrientation_error(
            &self,
            orientation: AVAudioStereoOrientation,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(preferredInputOrientation)]
        pub unsafe fn preferredInputOrientation(&self) -> AVAudioStereoOrientation;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(inputOrientation)]
        pub unsafe fn inputOrientation(&self) -> AVAudioStereoOrientation;

        #[method(maximumInputNumberOfChannels)]
        pub unsafe fn maximumInputNumberOfChannels(&self) -> NSInteger;

        #[method(maximumOutputNumberOfChannels)]
        pub unsafe fn maximumOutputNumberOfChannels(&self) -> NSInteger;

        #[method(setInputGain:error:_)]
        pub unsafe fn setInputGain_error(&self, gain: c_float) -> Result<(), Retained<NSError>>;

        #[method(inputGain)]
        pub unsafe fn inputGain(&self) -> c_float;

        #[method(isInputGainSettable)]
        pub unsafe fn isInputGainSettable(&self) -> bool;

        #[method(isInputAvailable)]
        pub unsafe fn isInputAvailable(&self) -> bool;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other inputDataSources)]
        pub unsafe fn inputDataSources(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionDataSourceDescription>>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other inputDataSource)]
        pub unsafe fn inputDataSource(
            &self,
        ) -> Option<Retained<AVAudioSessionDataSourceDescription>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method(setInputDataSource:error:_)]
        pub unsafe fn setInputDataSource_error(
            &self,
            data_source: Option<&AVAudioSessionDataSourceDescription>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other outputDataSources)]
        pub unsafe fn outputDataSources(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionDataSourceDescription>>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other outputDataSource)]
        pub unsafe fn outputDataSource(
            &self,
        ) -> Option<Retained<AVAudioSessionDataSourceDescription>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method(setOutputDataSource:error:_)]
        pub unsafe fn setOutputDataSource_error(
            &self,
            data_source: Option<&AVAudioSessionDataSourceDescription>,
        ) -> Result<(), Retained<NSError>>;

        #[method(sampleRate)]
        pub unsafe fn sampleRate(&self) -> c_double;

        #[method(inputNumberOfChannels)]
        pub unsafe fn inputNumberOfChannels(&self) -> NSInteger;

        #[method(outputNumberOfChannels)]
        pub unsafe fn outputNumberOfChannels(&self) -> NSInteger;

        #[method(inputLatency)]
        pub unsafe fn inputLatency(&self) -> NSTimeInterval;

        #[method(outputLatency)]
        pub unsafe fn outputLatency(&self) -> NSTimeInterval;

        #[method(IOBufferDuration)]
        pub unsafe fn IOBufferDuration(&self) -> NSTimeInterval;

        #[cfg(feature = "AVAudioChannelLayout")]
        #[method_id(@__retain_semantics Other supportedOutputChannelLayouts)]
        pub unsafe fn supportedOutputChannelLayouts(
            &self,
        ) -> Retained<NSArray<AVAudioChannelLayout>>;
    }
);

extern_methods!(
    /// Observation
    unsafe impl AVAudioSession {
        #[method(isOtherAudioPlaying)]
        pub unsafe fn isOtherAudioPlaying(&self) -> bool;

        #[method(secondaryAudioShouldBeSilencedHint)]
        pub unsafe fn secondaryAudioShouldBeSilencedHint(&self) -> bool;

        #[method(outputVolume)]
        pub unsafe fn outputVolume(&self) -> c_float;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(promptStyle)]
        pub unsafe fn promptStyle(&self) -> AVAudioSessionPromptStyle;
    }
);

extern_methods!(
    /// RoutingConfiguration
    unsafe impl AVAudioSession {
        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other availableInputs)]
        pub unsafe fn availableInputs(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionPortDescription>>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other currentRoute)]
        pub unsafe fn currentRoute(&self) -> Retained<AVAudioSessionRouteDescription>;

        #[cfg(feature = "AVAudioSessionTypes")]
        #[method(setAggregatedIOPreference:error:_)]
        pub unsafe fn setAggregatedIOPreference_error(
            &self,
            in_io_type: AVAudioSessionIOType,
        ) -> Result<(), Retained<NSError>>;

        #[method(setSupportsMultichannelContent:error:_)]
        pub unsafe fn setSupportsMultichannelContent_error(
            &self,
            in_value: bool,
        ) -> Result<(), Retained<NSError>>;

        #[method(supportsMultichannelContent)]
        pub unsafe fn supportsMultichannelContent(&self) -> bool;

        #[method(setPrefersInterruptionOnRouteDisconnect:error:_)]
        pub unsafe fn setPrefersInterruptionOnRouteDisconnect_error(
            &self,
            in_value: bool,
        ) -> Result<(), Retained<NSError>>;

        #[method(prefersInterruptionOnRouteDisconnect)]
        pub unsafe fn prefersInterruptionOnRouteDisconnect(&self) -> bool;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessioninterruptionnotification?language=objc)
    pub static AVAudioSessionInterruptionNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionroutechangenotification?language=objc)
    pub static AVAudioSessionRouteChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionmediaserviceswerelostnotification?language=objc)
    pub static AVAudioSessionMediaServicesWereLostNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionmediaserviceswereresetnotification?language=objc)
    pub static AVAudioSessionMediaServicesWereResetNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionsilencesecondaryaudiohintnotification?language=objc)
    pub static AVAudioSessionSilenceSecondaryAudioHintNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionspatialplaybackcapabilitieschangednotification?language=objc)
    pub static AVAudioSessionSpatialPlaybackCapabilitiesChangedNotification:
        &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionrenderingmodechangenotification?language=objc)
    pub static AVAudioSessionRenderingModeChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionrenderingcapabilitieschangenotification?language=objc)
    pub static AVAudioSessionRenderingCapabilitiesChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionspatialaudioenabledkey?language=objc)
    pub static AVAudioSessionSpatialAudioEnabledKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessioninterruptiontypekey?language=objc)
    pub static AVAudioSessionInterruptionTypeKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessioninterruptionoptionkey?language=objc)
    pub static AVAudioSessionInterruptionOptionKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessioninterruptionreasonkey?language=objc)
    pub static AVAudioSessionInterruptionReasonKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessioninterruptionwassuspendedkey?language=objc)
    pub static AVAudioSessionInterruptionWasSuspendedKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionroutechangereasonkey?language=objc)
    pub static AVAudioSessionRouteChangeReasonKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionroutechangepreviousroutekey?language=objc)
    pub static AVAudioSessionRouteChangePreviousRouteKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionsilencesecondaryaudiohinttypekey?language=objc)
    pub static AVAudioSessionSilenceSecondaryAudioHintTypeKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionrenderingmodenewrenderingmodekey?language=objc)
    pub static AVAudioSessionRenderingModeNewRenderingModeKey: &'static NSString;
}
