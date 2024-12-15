//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetreaderoutput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetReaderOutput;
);

unsafe impl NSObjectProtocol for AVAssetReaderOutput {}

extern_methods!(
    unsafe impl AVAssetReaderOutput {
        #[cfg(feature = "AVMediaFormat")]
        #[method_id(@__retain_semantics Other mediaType)]
        pub unsafe fn mediaType(&self) -> Retained<AVMediaType>;

        #[method(alwaysCopiesSampleData)]
        pub unsafe fn alwaysCopiesSampleData(&self) -> bool;

        #[method(setAlwaysCopiesSampleData:)]
        pub unsafe fn setAlwaysCopiesSampleData(&self, always_copies_sample_data: bool);

        #[cfg(feature = "objc2-core-media")]
        #[method(copyNextSampleBuffer)]
        pub unsafe fn copyNextSampleBuffer(&self) -> CMSampleBufferRef;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAssetReaderOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVAssetReaderOutputRandomAccess
    unsafe impl AVAssetReaderOutput {
        #[method(supportsRandomAccess)]
        pub unsafe fn supportsRandomAccess(&self) -> bool;

        #[method(setSupportsRandomAccess:)]
        pub unsafe fn setSupportsRandomAccess(&self, supports_random_access: bool);

        #[method(resetForReadingTimeRanges:)]
        pub unsafe fn resetForReadingTimeRanges(&self, time_ranges: &NSArray<NSValue>);

        #[method(markConfigurationAsFinal)]
        pub unsafe fn markConfigurationAsFinal(&self);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetreadertrackoutput?language=objc)
    #[unsafe(super(AVAssetReaderOutput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetReaderTrackOutput;
);

unsafe impl NSObjectProtocol for AVAssetReaderTrackOutput {}

extern_methods!(
    unsafe impl AVAssetReaderTrackOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other assetReaderTrackOutputWithTrack:outputSettings:)]
        pub unsafe fn assetReaderTrackOutputWithTrack_outputSettings(
            track: &AVAssetTrack,
            output_settings: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Init initWithTrack:outputSettings:)]
        pub unsafe fn initWithTrack_outputSettings(
            this: Allocated<Self>,
            track: &AVAssetTrack,
            output_settings: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other track)]
        pub unsafe fn track(&self) -> Retained<AVAssetTrack>;

        #[method_id(@__retain_semantics Other outputSettings)]
        pub unsafe fn outputSettings(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(feature = "AVAudioProcessingSettings")]
        #[method_id(@__retain_semantics Other audioTimePitchAlgorithm)]
        pub unsafe fn audioTimePitchAlgorithm(&self) -> Retained<AVAudioTimePitchAlgorithm>;

        #[cfg(feature = "AVAudioProcessingSettings")]
        #[method(setAudioTimePitchAlgorithm:)]
        pub unsafe fn setAudioTimePitchAlgorithm(
            &self,
            audio_time_pitch_algorithm: &AVAudioTimePitchAlgorithm,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetreaderaudiomixoutput?language=objc)
    #[unsafe(super(AVAssetReaderOutput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetReaderAudioMixOutput;
);

unsafe impl NSObjectProtocol for AVAssetReaderAudioMixOutput {}

extern_methods!(
    unsafe impl AVAssetReaderAudioMixOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other assetReaderAudioMixOutputWithAudioTracks:audioSettings:)]
        pub unsafe fn assetReaderAudioMixOutputWithAudioTracks_audioSettings(
            audio_tracks: &NSArray<AVAssetTrack>,
            audio_settings: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Init initWithAudioTracks:audioSettings:)]
        pub unsafe fn initWithAudioTracks_audioSettings(
            this: Allocated<Self>,
            audio_tracks: &NSArray<AVAssetTrack>,
            audio_settings: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other audioTracks)]
        pub unsafe fn audioTracks(&self) -> Retained<NSArray<AVAssetTrack>>;

        #[method_id(@__retain_semantics Other audioSettings)]
        pub unsafe fn audioSettings(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(feature = "AVAudioMix")]
        #[method_id(@__retain_semantics Other audioMix)]
        pub unsafe fn audioMix(&self) -> Option<Retained<AVAudioMix>>;

        #[cfg(feature = "AVAudioMix")]
        #[method(setAudioMix:)]
        pub unsafe fn setAudioMix(&self, audio_mix: Option<&AVAudioMix>);

        #[cfg(feature = "AVAudioProcessingSettings")]
        #[method_id(@__retain_semantics Other audioTimePitchAlgorithm)]
        pub unsafe fn audioTimePitchAlgorithm(&self) -> Retained<AVAudioTimePitchAlgorithm>;

        #[cfg(feature = "AVAudioProcessingSettings")]
        #[method(setAudioTimePitchAlgorithm:)]
        pub unsafe fn setAudioTimePitchAlgorithm(
            &self,
            audio_time_pitch_algorithm: &AVAudioTimePitchAlgorithm,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetreadervideocompositionoutput?language=objc)
    #[unsafe(super(AVAssetReaderOutput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetReaderVideoCompositionOutput;
);

unsafe impl NSObjectProtocol for AVAssetReaderVideoCompositionOutput {}

extern_methods!(
    unsafe impl AVAssetReaderVideoCompositionOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other assetReaderVideoCompositionOutputWithVideoTracks:videoSettings:)]
        pub unsafe fn assetReaderVideoCompositionOutputWithVideoTracks_videoSettings(
            video_tracks: &NSArray<AVAssetTrack>,
            video_settings: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Init initWithVideoTracks:videoSettings:)]
        pub unsafe fn initWithVideoTracks_videoSettings(
            this: Allocated<Self>,
            video_tracks: &NSArray<AVAssetTrack>,
            video_settings: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other videoTracks)]
        pub unsafe fn videoTracks(&self) -> Retained<NSArray<AVAssetTrack>>;

        #[method_id(@__retain_semantics Other videoSettings)]
        pub unsafe fn videoSettings(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

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
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetreaderoutputmetadataadaptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetReaderOutputMetadataAdaptor;
);

unsafe impl NSObjectProtocol for AVAssetReaderOutputMetadataAdaptor {}

extern_methods!(
    unsafe impl AVAssetReaderOutputMetadataAdaptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other assetReaderOutputMetadataAdaptorWithAssetReaderTrackOutput:)]
        pub unsafe fn assetReaderOutputMetadataAdaptorWithAssetReaderTrackOutput(
            track_output: &AVAssetReaderTrackOutput,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAssetReaderTrackOutput:)]
        pub unsafe fn initWithAssetReaderTrackOutput(
            this: Allocated<Self>,
            track_output: &AVAssetReaderTrackOutput,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other assetReaderTrackOutput)]
        pub unsafe fn assetReaderTrackOutput(&self) -> Retained<AVAssetReaderTrackOutput>;

        #[cfg(feature = "AVTimedMetadataGroup")]
        #[method_id(@__retain_semantics Other nextTimedMetadataGroup)]
        pub unsafe fn nextTimedMetadataGroup(&self) -> Option<Retained<AVTimedMetadataGroup>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetreaderoutputcaptionadaptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetReaderOutputCaptionAdaptor;
);

unsafe impl NSObjectProtocol for AVAssetReaderOutputCaptionAdaptor {}

extern_methods!(
    unsafe impl AVAssetReaderOutputCaptionAdaptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other assetReaderOutputCaptionAdaptorWithAssetReaderTrackOutput:)]
        pub unsafe fn assetReaderOutputCaptionAdaptorWithAssetReaderTrackOutput(
            track_output: &AVAssetReaderTrackOutput,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAssetReaderTrackOutput:)]
        pub unsafe fn initWithAssetReaderTrackOutput(
            this: Allocated<Self>,
            track_output: &AVAssetReaderTrackOutput,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other assetReaderTrackOutput)]
        pub unsafe fn assetReaderTrackOutput(&self) -> Retained<AVAssetReaderTrackOutput>;

        #[cfg(feature = "AVCaptionGroup")]
        #[method_id(@__retain_semantics Other nextCaptionGroup)]
        pub unsafe fn nextCaptionGroup(&self) -> Option<Retained<AVCaptionGroup>>;

        #[cfg(all(feature = "AVCaption", feature = "AVCaptionGroup"))]
        #[method_id(@__retain_semantics Other captionsNotPresentInPreviousGroupsInCaptionGroup:)]
        pub unsafe fn captionsNotPresentInPreviousGroupsInCaptionGroup(
            &self,
            caption_group: &AVCaptionGroup,
        ) -> Retained<NSArray<AVCaption>>;
    }
);

extern_methods!(
    /// AVAssetReaderCaptionValidation
    unsafe impl AVAssetReaderOutputCaptionAdaptor {
        #[method_id(@__retain_semantics Other validationDelegate)]
        pub unsafe fn validationDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVAssetReaderCaptionValidationHandling>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setValidationDelegate:)]
        pub unsafe fn setValidationDelegate(
            &self,
            validation_delegate: Option<
                &ProtocolObject<dyn AVAssetReaderCaptionValidationHandling>,
            >,
        );
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetreadercaptionvalidationhandling?language=objc)
    pub unsafe trait AVAssetReaderCaptionValidationHandling: NSObjectProtocol {
        #[cfg(feature = "AVCaption")]
        #[optional]
        #[method(captionAdaptor:didVendCaption:skippingUnsupportedSourceSyntaxElements:)]
        unsafe fn captionAdaptor_didVendCaption_skippingUnsupportedSourceSyntaxElements(
            &self,
            adaptor: &AVAssetReaderOutputCaptionAdaptor,
            caption: &AVCaption,
            syntax_elements: &NSArray<NSString>,
        );
    }

    unsafe impl ProtocolType for dyn AVAssetReaderCaptionValidationHandling {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetreadersamplereferenceoutput?language=objc)
    #[unsafe(super(AVAssetReaderOutput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetReaderSampleReferenceOutput;
);

unsafe impl NSObjectProtocol for AVAssetReaderSampleReferenceOutput {}

extern_methods!(
    unsafe impl AVAssetReaderSampleReferenceOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other assetReaderSampleReferenceOutputWithTrack:)]
        pub unsafe fn assetReaderSampleReferenceOutputWithTrack(
            track: &AVAssetTrack,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Init initWithTrack:)]
        pub unsafe fn initWithTrack(this: Allocated<Self>, track: &AVAssetTrack) -> Retained<Self>;

        #[cfg(feature = "AVAssetTrack")]
        #[method_id(@__retain_semantics Other track)]
        pub unsafe fn track(&self) -> Retained<AVAssetTrack>;
    }
);