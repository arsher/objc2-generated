//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-avf-audio")]
use objc2_avf_audio::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/speech/sfspeechrecognitionrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSpeechRecognitionRequest;
);

unsafe impl NSObjectProtocol for SFSpeechRecognitionRequest {}

extern_methods!(
    unsafe impl SFSpeechRecognitionRequest {
        #[cfg(feature = "SFSpeechRecognitionTaskHint")]
        #[method(taskHint)]
        pub unsafe fn taskHint(&self) -> SFSpeechRecognitionTaskHint;

        #[cfg(feature = "SFSpeechRecognitionTaskHint")]
        #[method(setTaskHint:)]
        pub unsafe fn setTaskHint(&self, task_hint: SFSpeechRecognitionTaskHint);

        #[method(shouldReportPartialResults)]
        pub unsafe fn shouldReportPartialResults(&self) -> bool;

        #[method(setShouldReportPartialResults:)]
        pub unsafe fn setShouldReportPartialResults(&self, should_report_partial_results: bool);

        #[method_id(@__retain_semantics Other contextualStrings)]
        pub unsafe fn contextualStrings(&self) -> Retained<NSArray<NSString>>;

        #[method(setContextualStrings:)]
        pub unsafe fn setContextualStrings(&self, contextual_strings: &NSArray<NSString>);

        #[deprecated = "Not used anymore"]
        #[method_id(@__retain_semantics Other interactionIdentifier)]
        pub unsafe fn interactionIdentifier(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Not used anymore"]
        #[method(setInteractionIdentifier:)]
        pub unsafe fn setInteractionIdentifier(&self, interaction_identifier: Option<&NSString>);

        #[method(requiresOnDeviceRecognition)]
        pub unsafe fn requiresOnDeviceRecognition(&self) -> bool;

        #[method(setRequiresOnDeviceRecognition:)]
        pub unsafe fn setRequiresOnDeviceRecognition(&self, requires_on_device_recognition: bool);

        #[method(addsPunctuation)]
        pub unsafe fn addsPunctuation(&self) -> bool;

        #[method(setAddsPunctuation:)]
        pub unsafe fn setAddsPunctuation(&self, adds_punctuation: bool);

        #[cfg(feature = "SFSpeechLanguageModel")]
        #[method_id(@__retain_semantics Other customizedLanguageModel)]
        pub unsafe fn customizedLanguageModel(
            &self,
        ) -> Option<Retained<SFSpeechLanguageModelConfiguration>>;

        #[cfg(feature = "SFSpeechLanguageModel")]
        #[method(setCustomizedLanguageModel:)]
        pub unsafe fn setCustomizedLanguageModel(
            &self,
            customized_language_model: Option<&SFSpeechLanguageModelConfiguration>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFSpeechRecognitionRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/speech/sfspeechurlrecognitionrequest?language=objc)
    #[unsafe(super(SFSpeechRecognitionRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSpeechURLRecognitionRequest;
);

unsafe impl NSObjectProtocol for SFSpeechURLRecognitionRequest {}

extern_methods!(
    unsafe impl SFSpeechURLRecognitionRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFSpeechURLRecognitionRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/speech/sfspeechaudiobufferrecognitionrequest?language=objc)
    #[unsafe(super(SFSpeechRecognitionRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSpeechAudioBufferRecognitionRequest;
);

unsafe impl NSObjectProtocol for SFSpeechAudioBufferRecognitionRequest {}

extern_methods!(
    unsafe impl SFSpeechAudioBufferRecognitionRequest {
        #[cfg(feature = "objc2-avf-audio")]
        #[method_id(@__retain_semantics Other nativeAudioFormat)]
        pub unsafe fn nativeAudioFormat(&self) -> Retained<AVAudioFormat>;

        #[cfg(feature = "objc2-avf-audio")]
        #[method(appendAudioPCMBuffer:)]
        pub unsafe fn appendAudioPCMBuffer(&self, audio_pcm_buffer: &AVAudioPCMBuffer);

        #[cfg(feature = "objc2-core-media")]
        #[method(appendAudioSampleBuffer:)]
        pub unsafe fn appendAudioSampleBuffer(&self, sample_buffer: CMSampleBufferRef);

        #[method(endAudio)]
        pub unsafe fn endAudio(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFSpeechAudioBufferRecognitionRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
