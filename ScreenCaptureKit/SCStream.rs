//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCStreamOutputType(pub NSInteger);
impl SCStreamOutputType {
    #[doc(alias = "SCStreamOutputTypeScreen")]
    pub const Screen: Self = Self(0);
    #[doc(alias = "SCStreamOutputTypeAudio")]
    pub const Audio: Self = Self(1);
    #[doc(alias = "SCStreamOutputTypeMicrophone")]
    pub const Microphone: Self = Self(2);
}

unsafe impl Encode for SCStreamOutputType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCStreamOutputType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCFrameStatus(pub NSInteger);
impl SCFrameStatus {
    #[doc(alias = "SCFrameStatusComplete")]
    pub const Complete: Self = Self(0);
    #[doc(alias = "SCFrameStatusIdle")]
    pub const Idle: Self = Self(1);
    #[doc(alias = "SCFrameStatusBlank")]
    pub const Blank: Self = Self(2);
    #[doc(alias = "SCFrameStatusSuspended")]
    pub const Suspended: Self = Self(3);
    #[doc(alias = "SCFrameStatusStarted")]
    pub const Started: Self = Self(4);
    #[doc(alias = "SCFrameStatusStopped")]
    pub const Stopped: Self = Self(5);
}

unsafe impl Encode for SCFrameStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCFrameStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCPresenterOverlayAlertSetting(pub NSInteger);
impl SCPresenterOverlayAlertSetting {
    #[doc(alias = "SCPresenterOverlayAlertSettingSystem")]
    pub const System: Self = Self(0);
    #[doc(alias = "SCPresenterOverlayAlertSettingNever")]
    pub const Never: Self = Self(1);
    #[doc(alias = "SCPresenterOverlayAlertSettingAlways")]
    pub const Always: Self = Self(2);
}

unsafe impl Encode for SCPresenterOverlayAlertSetting {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCPresenterOverlayAlertSetting {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[deprecated = "Use SCShareableContentStyle instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCStreamType(pub NSInteger);
impl SCStreamType {
    #[deprecated = "Use SCShareableContentStyle instead"]
    #[doc(alias = "SCStreamTypeWindow")]
    pub const Window: Self = Self(0);
    #[deprecated = "Use SCShareableContentStyle instead"]
    #[doc(alias = "SCStreamTypeDisplay")]
    pub const Display: Self = Self(1);
}

unsafe impl Encode for SCStreamType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCStreamType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCCaptureResolutionType(pub NSInteger);
impl SCCaptureResolutionType {
    pub const SCCaptureResolutionAutomatic: Self = Self(0);
    pub const SCCaptureResolutionBest: Self = Self(1);
    pub const SCCaptureResolutionNominal: Self = Self(2);
}

unsafe impl Encode for SCCaptureResolutionType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCCaptureResolutionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCCaptureDynamicRange(pub NSInteger);
impl SCCaptureDynamicRange {
    #[doc(alias = "SCCaptureDynamicRangeSDR")]
    pub const SDR: Self = Self(0);
    #[doc(alias = "SCCaptureDynamicRangeHDRLocalDisplay")]
    pub const HDRLocalDisplay: Self = Self(1);
    #[doc(alias = "SCCaptureDynamicRangeHDRCanonicalDisplay")]
    pub const HDRCanonicalDisplay: Self = Self(2);
}

unsafe impl Encode for SCCaptureDynamicRange {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCCaptureDynamicRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCContentFilter;

    unsafe impl ClassType for SCContentFilter {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for SCContentFilter {}

extern_methods!(
    unsafe impl SCContentFilter {
        #[deprecated = "Use style instead"]
        #[method(streamType)]
        pub unsafe fn streamType(&self) -> SCStreamType;

        #[cfg(feature = "SCShareableContent")]
        #[method(style)]
        pub unsafe fn style(&self) -> SCShareableContentStyle;

        #[method(pointPixelScale)]
        pub unsafe fn pointPixelScale(&self) -> c_float;

        #[method(contentRect)]
        pub unsafe fn contentRect(&self) -> CGRect;

        #[method(includeMenuBar)]
        pub unsafe fn includeMenuBar(&self) -> bool;

        #[method(setIncludeMenuBar:)]
        pub unsafe fn setIncludeMenuBar(&self, include_menu_bar: bool);

        #[cfg(feature = "SCShareableContent")]
        #[method_id(@__retain_semantics Init initWithDesktopIndependentWindow:)]
        pub unsafe fn initWithDesktopIndependentWindow(
            this: Allocated<Self>,
            window: &SCWindow,
        ) -> Retained<Self>;

        #[cfg(feature = "SCShareableContent")]
        #[method_id(@__retain_semantics Init initWithDisplay:excludingWindows:)]
        pub unsafe fn initWithDisplay_excludingWindows(
            this: Allocated<Self>,
            display: &SCDisplay,
            excluded: &NSArray<SCWindow>,
        ) -> Retained<Self>;

        #[cfg(feature = "SCShareableContent")]
        #[method_id(@__retain_semantics Init initWithDisplay:includingWindows:)]
        pub unsafe fn initWithDisplay_includingWindows(
            this: Allocated<Self>,
            display: &SCDisplay,
            included_windows: &NSArray<SCWindow>,
        ) -> Retained<Self>;

        #[cfg(feature = "SCShareableContent")]
        #[method_id(@__retain_semantics Init initWithDisplay:includingApplications:exceptingWindows:)]
        pub unsafe fn initWithDisplay_includingApplications_exceptingWindows(
            this: Allocated<Self>,
            display: &SCDisplay,
            applications: &NSArray<SCRunningApplication>,
            excepting_windows: &NSArray<SCWindow>,
        ) -> Retained<Self>;

        #[cfg(feature = "SCShareableContent")]
        #[method_id(@__retain_semantics Init initWithDisplay:excludingApplications:exceptingWindows:)]
        pub unsafe fn initWithDisplay_excludingApplications_exceptingWindows(
            this: Allocated<Self>,
            display: &SCDisplay,
            applications: &NSArray<SCRunningApplication>,
            excepting_windows: &NSArray<SCWindow>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCContentFilter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCStreamConfigurationPreset(pub NSInteger);
impl SCStreamConfigurationPreset {
    #[doc(alias = "SCStreamConfigurationPresetCaptureHDRStreamLocalDisplay")]
    pub const CaptureHDRStreamLocalDisplay: Self = Self(0);
    #[doc(alias = "SCStreamConfigurationPresetCaptureHDRStreamCanonicalDisplay")]
    pub const CaptureHDRStreamCanonicalDisplay: Self = Self(1);
    #[doc(alias = "SCStreamConfigurationPresetCaptureHDRScreenshotLocalDisplay")]
    pub const CaptureHDRScreenshotLocalDisplay: Self = Self(2);
    #[doc(alias = "SCStreamConfigurationPresetCaptureHDRScreenshotCanonicalDisplay")]
    pub const CaptureHDRScreenshotCanonicalDisplay: Self = Self(3);
}

unsafe impl Encode for SCStreamConfigurationPreset {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCStreamConfigurationPreset {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCStreamConfiguration;

    unsafe impl ClassType for SCStreamConfiguration {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for SCStreamConfiguration {}

extern_methods!(
    unsafe impl SCStreamConfiguration {
        #[method(width)]
        pub unsafe fn width(&self) -> usize;

        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: usize);

        #[method(height)]
        pub unsafe fn height(&self) -> usize;

        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: usize);

        #[method(pixelFormat)]
        pub unsafe fn pixelFormat(&self) -> OSType;

        #[method(setPixelFormat:)]
        pub unsafe fn setPixelFormat(&self, pixel_format: OSType);

        #[method(scalesToFit)]
        pub unsafe fn scalesToFit(&self) -> bool;

        #[method(setScalesToFit:)]
        pub unsafe fn setScalesToFit(&self, scales_to_fit: bool);

        #[method(preservesAspectRatio)]
        pub unsafe fn preservesAspectRatio(&self) -> bool;

        #[method(setPreservesAspectRatio:)]
        pub unsafe fn setPreservesAspectRatio(&self, preserves_aspect_ratio: bool);

        #[method_id(@__retain_semantics Other streamName)]
        pub unsafe fn streamName(&self) -> Option<Retained<NSString>>;

        #[method(setStreamName:)]
        pub unsafe fn setStreamName(&self, stream_name: Option<&NSString>);

        #[method(showsCursor)]
        pub unsafe fn showsCursor(&self) -> bool;

        #[method(setShowsCursor:)]
        pub unsafe fn setShowsCursor(&self, shows_cursor: bool);

        #[method(showMouseClicks)]
        pub unsafe fn showMouseClicks(&self) -> bool;

        #[method(setShowMouseClicks:)]
        pub unsafe fn setShowMouseClicks(&self, show_mouse_clicks: bool);

        #[method(sourceRect)]
        pub unsafe fn sourceRect(&self) -> CGRect;

        #[method(setSourceRect:)]
        pub unsafe fn setSourceRect(&self, source_rect: CGRect);

        #[method(destinationRect)]
        pub unsafe fn destinationRect(&self) -> CGRect;

        #[method(setDestinationRect:)]
        pub unsafe fn setDestinationRect(&self, destination_rect: CGRect);

        #[method(queueDepth)]
        pub unsafe fn queueDepth(&self) -> NSInteger;

        #[method(setQueueDepth:)]
        pub unsafe fn setQueueDepth(&self, queue_depth: NSInteger);

        #[method(capturesAudio)]
        pub unsafe fn capturesAudio(&self) -> bool;

        #[method(setCapturesAudio:)]
        pub unsafe fn setCapturesAudio(&self, captures_audio: bool);

        #[method(sampleRate)]
        pub unsafe fn sampleRate(&self) -> NSInteger;

        #[method(setSampleRate:)]
        pub unsafe fn setSampleRate(&self, sample_rate: NSInteger);

        #[method(channelCount)]
        pub unsafe fn channelCount(&self) -> NSInteger;

        #[method(setChannelCount:)]
        pub unsafe fn setChannelCount(&self, channel_count: NSInteger);

        #[method(excludesCurrentProcessAudio)]
        pub unsafe fn excludesCurrentProcessAudio(&self) -> bool;

        #[method(setExcludesCurrentProcessAudio:)]
        pub unsafe fn setExcludesCurrentProcessAudio(&self, excludes_current_process_audio: bool);

        #[method(ignoreShadowsDisplay)]
        pub unsafe fn ignoreShadowsDisplay(&self) -> bool;

        #[method(setIgnoreShadowsDisplay:)]
        pub unsafe fn setIgnoreShadowsDisplay(&self, ignore_shadows_display: bool);

        #[method(ignoreShadowsSingleWindow)]
        pub unsafe fn ignoreShadowsSingleWindow(&self) -> bool;

        #[method(setIgnoreShadowsSingleWindow:)]
        pub unsafe fn setIgnoreShadowsSingleWindow(&self, ignore_shadows_single_window: bool);

        #[method(captureResolution)]
        pub unsafe fn captureResolution(&self) -> SCCaptureResolutionType;

        #[method(setCaptureResolution:)]
        pub unsafe fn setCaptureResolution(&self, capture_resolution: SCCaptureResolutionType);

        #[method(capturesShadowsOnly)]
        pub unsafe fn capturesShadowsOnly(&self) -> bool;

        #[method(setCapturesShadowsOnly:)]
        pub unsafe fn setCapturesShadowsOnly(&self, captures_shadows_only: bool);

        #[method(shouldBeOpaque)]
        pub unsafe fn shouldBeOpaque(&self) -> bool;

        #[method(setShouldBeOpaque:)]
        pub unsafe fn setShouldBeOpaque(&self, should_be_opaque: bool);

        #[method(ignoreGlobalClipDisplay)]
        pub unsafe fn ignoreGlobalClipDisplay(&self) -> bool;

        #[method(setIgnoreGlobalClipDisplay:)]
        pub unsafe fn setIgnoreGlobalClipDisplay(&self, ignore_global_clip_display: bool);

        #[method(ignoreGlobalClipSingleWindow)]
        pub unsafe fn ignoreGlobalClipSingleWindow(&self) -> bool;

        #[method(setIgnoreGlobalClipSingleWindow:)]
        pub unsafe fn setIgnoreGlobalClipSingleWindow(
            &self,
            ignore_global_clip_single_window: bool,
        );

        #[method(presenterOverlayPrivacyAlertSetting)]
        pub unsafe fn presenterOverlayPrivacyAlertSetting(&self) -> SCPresenterOverlayAlertSetting;

        #[method(setPresenterOverlayPrivacyAlertSetting:)]
        pub unsafe fn setPresenterOverlayPrivacyAlertSetting(
            &self,
            presenter_overlay_privacy_alert_setting: SCPresenterOverlayAlertSetting,
        );

        #[method(includeChildWindows)]
        pub unsafe fn includeChildWindows(&self) -> bool;

        #[method(setIncludeChildWindows:)]
        pub unsafe fn setIncludeChildWindows(&self, include_child_windows: bool);

        #[method(captureMicrophone)]
        pub unsafe fn captureMicrophone(&self) -> bool;

        #[method(setCaptureMicrophone:)]
        pub unsafe fn setCaptureMicrophone(&self, capture_microphone: bool);

        #[method_id(@__retain_semantics Other microphoneCaptureDeviceID)]
        pub unsafe fn microphoneCaptureDeviceID(&self) -> Option<Retained<NSString>>;

        #[method(setMicrophoneCaptureDeviceID:)]
        pub unsafe fn setMicrophoneCaptureDeviceID(
            &self,
            microphone_capture_device_id: Option<&NSString>,
        );

        #[method(captureDynamicRange)]
        pub unsafe fn captureDynamicRange(&self) -> SCCaptureDynamicRange;

        #[method(setCaptureDynamicRange:)]
        pub unsafe fn setCaptureDynamicRange(&self, capture_dynamic_range: SCCaptureDynamicRange);

        #[method_id(@__retain_semantics Other streamConfigurationWithPreset:)]
        pub unsafe fn streamConfigurationWithPreset(
            preset: SCStreamConfigurationPreset,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCStreamConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_TYPED_ENUM
pub type SCStreamFrameInfo = NSString;

extern "C" {
    pub static SCStreamFrameInfoStatus: &'static SCStreamFrameInfo;
}

extern "C" {
    pub static SCStreamFrameInfoDisplayTime: &'static SCStreamFrameInfo;
}

extern "C" {
    pub static SCStreamFrameInfoScaleFactor: &'static SCStreamFrameInfo;
}

extern "C" {
    pub static SCStreamFrameInfoContentScale: &'static SCStreamFrameInfo;
}

extern "C" {
    pub static SCStreamFrameInfoContentRect: &'static SCStreamFrameInfo;
}

extern "C" {
    pub static SCStreamFrameInfoDirtyRects: &'static SCStreamFrameInfo;
}

extern "C" {
    pub static SCStreamFrameInfoScreenRect: &'static SCStreamFrameInfo;
}

extern "C" {
    pub static SCStreamFrameInfoBoundingRect: &'static SCStreamFrameInfo;
}

extern "C" {
    pub static SCStreamFrameInfoPresenterOverlayContentRect: &'static SCStreamFrameInfo;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCStream;

    unsafe impl ClassType for SCStream {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for SCStream {}

extern_methods!(
    unsafe impl SCStream {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithFilter:configuration:delegate:)]
        pub unsafe fn initWithFilter_configuration_delegate(
            this: Allocated<Self>,
            content_filter: &SCContentFilter,
            stream_config: &SCStreamConfiguration,
            delegate: Option<&ProtocolObject<dyn SCStreamDelegate>>,
        ) -> Retained<Self>;

        #[method(removeStreamOutput:type:error:_)]
        pub unsafe fn removeStreamOutput_type_error(
            &self,
            output: &ProtocolObject<dyn SCStreamOutput>,
            r#type: SCStreamOutputType,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method(updateContentFilter:completionHandler:)]
        pub unsafe fn updateContentFilter_completionHandler(
            &self,
            content_filter: &SCContentFilter,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(updateConfiguration:completionHandler:)]
        pub unsafe fn updateConfiguration_completionHandler(
            &self,
            stream_config: &SCStreamConfiguration,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(startCaptureWithCompletionHandler:)]
        pub unsafe fn startCaptureWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(stopCaptureWithCompletionHandler:)]
        pub unsafe fn stopCaptureWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "SCRecordingOutput")]
        #[method(addRecordingOutput:error:_)]
        pub unsafe fn addRecordingOutput_error(
            &self,
            recording_output: &SCRecordingOutput,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "SCRecordingOutput")]
        #[method(removeRecordingOutput:error:_)]
        pub unsafe fn removeRecordingOutput_error(
            &self,
            recording_output: &SCRecordingOutput,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_protocol!(
    pub unsafe trait SCStreamOutput: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn SCStreamOutput {}
);

extern_protocol!(
    pub unsafe trait SCStreamDelegate: NSObjectProtocol {
        #[optional]
        #[method(stream:didStopWithError:)]
        unsafe fn stream_didStopWithError(&self, stream: &SCStream, error: &NSError);

        #[optional]
        #[method(outputVideoEffectDidStartForStream:)]
        unsafe fn outputVideoEffectDidStartForStream(&self, stream: &SCStream);

        #[optional]
        #[method(outputVideoEffectDidStopForStream:)]
        unsafe fn outputVideoEffectDidStopForStream(&self, stream: &SCStream);
    }

    unsafe impl ProtocolType for dyn SCStreamDelegate {}
);
