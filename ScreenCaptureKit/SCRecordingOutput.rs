//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-av-foundation")]
use objc2_av_foundation::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/screcordingoutputconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCRecordingOutputConfiguration;
);

unsafe impl NSObjectProtocol for SCRecordingOutputConfiguration {}

extern_methods!(
    unsafe impl SCRecordingOutputConfiguration {
        #[method_id(@__retain_semantics Other outputURL)]
        pub unsafe fn outputURL(&self) -> Retained<NSURL>;

        #[method(setOutputURL:)]
        pub unsafe fn setOutputURL(&self, output_url: &NSURL);

        #[cfg(feature = "objc2-av-foundation")]
        #[method_id(@__retain_semantics Other videoCodecType)]
        pub unsafe fn videoCodecType(&self) -> Retained<AVVideoCodecType>;

        #[cfg(feature = "objc2-av-foundation")]
        #[method(setVideoCodecType:)]
        pub unsafe fn setVideoCodecType(&self, video_codec_type: &AVVideoCodecType);

        #[cfg(feature = "objc2-av-foundation")]
        #[method_id(@__retain_semantics Other outputFileType)]
        pub unsafe fn outputFileType(&self) -> Retained<AVFileType>;

        #[cfg(feature = "objc2-av-foundation")]
        #[method(setOutputFileType:)]
        pub unsafe fn setOutputFileType(&self, output_file_type: &AVFileType);

        #[cfg(feature = "objc2-av-foundation")]
        #[method_id(@__retain_semantics Other availableVideoCodecTypes)]
        pub unsafe fn availableVideoCodecTypes(&self) -> Retained<NSArray<AVVideoCodecType>>;

        #[cfg(feature = "objc2-av-foundation")]
        #[method_id(@__retain_semantics Other availableOutputFileTypes)]
        pub unsafe fn availableOutputFileTypes(&self) -> Retained<NSArray<AVFileType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCRecordingOutputConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/screcordingoutputdelegate?language=objc)
    pub unsafe trait SCRecordingOutputDelegate: NSObjectProtocol {
        #[optional]
        #[method(recordingOutputDidStartRecording:)]
        unsafe fn recordingOutputDidStartRecording(&self, recording_output: &SCRecordingOutput);

        #[optional]
        #[method(recordingOutput:didFailWithError:)]
        unsafe fn recordingOutput_didFailWithError(
            &self,
            recording_output: &SCRecordingOutput,
            error: &NSError,
        );

        #[optional]
        #[method(recordingOutputDidFinishRecording:)]
        unsafe fn recordingOutputDidFinishRecording(&self, recording_output: &SCRecordingOutput);
    }

    unsafe impl ProtocolType for dyn SCRecordingOutputDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/screcordingoutput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCRecordingOutput;
);

unsafe impl NSObjectProtocol for SCRecordingOutput {}

extern_methods!(
    unsafe impl SCRecordingOutput {
        #[cfg(feature = "objc2-core-media")]
        #[method(recordedDuration)]
        pub unsafe fn recordedDuration(&self) -> CMTime;

        #[method(recordedFileSize)]
        pub unsafe fn recordedFileSize(&self) -> NSInteger;

        #[method_id(@__retain_semantics Init initWithConfiguration:delegate:)]
        pub unsafe fn initWithConfiguration_delegate(
            this: Allocated<Self>,
            recording_output_configuration: &SCRecordingOutputConfiguration,
            delegate: &ProtocolObject<dyn SCRecordingOutputDelegate>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCRecordingOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
