//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessionruntimeerrornotification?language=objc)
    pub static AVCaptureSessionRuntimeErrorNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessionerrorkey?language=objc)
    pub static AVCaptureSessionErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessiondidstartrunningnotification?language=objc)
    pub static AVCaptureSessionDidStartRunningNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessiondidstoprunningnotification?language=objc)
    pub static AVCaptureSessionDidStopRunningNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessionwasinterruptednotification?language=objc)
    pub static AVCaptureSessionWasInterruptedNotification: &'static NSNotificationName;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessioninterruptionreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptureSessionInterruptionReason(pub NSInteger);
impl AVCaptureSessionInterruptionReason {
    #[doc(alias = "AVCaptureSessionInterruptionReasonVideoDeviceNotAvailableInBackground")]
    pub const VideoDeviceNotAvailableInBackground: Self = Self(1);
    #[doc(alias = "AVCaptureSessionInterruptionReasonAudioDeviceInUseByAnotherClient")]
    pub const AudioDeviceInUseByAnotherClient: Self = Self(2);
    #[doc(alias = "AVCaptureSessionInterruptionReasonVideoDeviceInUseByAnotherClient")]
    pub const VideoDeviceInUseByAnotherClient: Self = Self(3);
    #[doc(
        alias = "AVCaptureSessionInterruptionReasonVideoDeviceNotAvailableWithMultipleForegroundApps"
    )]
    pub const VideoDeviceNotAvailableWithMultipleForegroundApps: Self = Self(4);
    #[doc(alias = "AVCaptureSessionInterruptionReasonVideoDeviceNotAvailableDueToSystemPressure")]
    pub const VideoDeviceNotAvailableDueToSystemPressure: Self = Self(5);
}

unsafe impl Encode for AVCaptureSessionInterruptionReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptureSessionInterruptionReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessioninterruptionreasonkey?language=objc)
    pub static AVCaptureSessionInterruptionReasonKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessioninterruptionsystempressurestatekey?language=objc)
    pub static AVCaptureSessionInterruptionSystemPressureStateKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessioninterruptionendednotification?language=objc)
    pub static AVCaptureSessionInterruptionEndedNotification: &'static NSNotificationName;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturevideoorientation?language=objc)
// NS_ENUM
#[deprecated = "Use AVCaptureDeviceRotationCoordinator instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptureVideoOrientation(pub NSInteger);
impl AVCaptureVideoOrientation {
    #[deprecated = "Use AVCaptureDeviceRotationCoordinator instead"]
    #[doc(alias = "AVCaptureVideoOrientationPortrait")]
    pub const Portrait: Self = Self(1);
    #[deprecated = "Use AVCaptureDeviceRotationCoordinator instead"]
    #[doc(alias = "AVCaptureVideoOrientationPortraitUpsideDown")]
    pub const PortraitUpsideDown: Self = Self(2);
    #[deprecated = "Use AVCaptureDeviceRotationCoordinator instead"]
    #[doc(alias = "AVCaptureVideoOrientationLandscapeRight")]
    pub const LandscapeRight: Self = Self(3);
    #[deprecated = "Use AVCaptureDeviceRotationCoordinator instead"]
    #[doc(alias = "AVCaptureVideoOrientationLandscapeLeft")]
    pub const LandscapeLeft: Self = Self(4);
}

unsafe impl Encode for AVCaptureVideoOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptureVideoOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesession?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureSession;
);

unsafe impl NSObjectProtocol for AVCaptureSession {}

extern_methods!(
    unsafe impl AVCaptureSession {
        #[cfg(feature = "AVCaptureSessionPreset")]
        #[method(canSetSessionPreset:)]
        pub unsafe fn canSetSessionPreset(&self, preset: &AVCaptureSessionPreset) -> bool;

        #[cfg(feature = "AVCaptureSessionPreset")]
        #[method_id(@__retain_semantics Other sessionPreset)]
        pub unsafe fn sessionPreset(&self) -> Retained<AVCaptureSessionPreset>;

        #[cfg(feature = "AVCaptureSessionPreset")]
        #[method(setSessionPreset:)]
        pub unsafe fn setSessionPreset(&self, session_preset: &AVCaptureSessionPreset);

        #[cfg(feature = "AVCaptureInput")]
        #[method_id(@__retain_semantics Other inputs)]
        pub unsafe fn inputs(&self) -> Retained<NSArray<AVCaptureInput>>;

        #[cfg(feature = "AVCaptureInput")]
        #[method(canAddInput:)]
        pub unsafe fn canAddInput(&self, input: &AVCaptureInput) -> bool;

        #[cfg(feature = "AVCaptureInput")]
        #[method(addInput:)]
        pub unsafe fn addInput(&self, input: &AVCaptureInput);

        #[cfg(feature = "AVCaptureInput")]
        #[method(removeInput:)]
        pub unsafe fn removeInput(&self, input: &AVCaptureInput);

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method_id(@__retain_semantics Other outputs)]
        pub unsafe fn outputs(&self) -> Retained<NSArray<AVCaptureOutput>>;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method(canAddOutput:)]
        pub unsafe fn canAddOutput(&self, output: &AVCaptureOutput) -> bool;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method(addOutput:)]
        pub unsafe fn addOutput(&self, output: &AVCaptureOutput);

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method(removeOutput:)]
        pub unsafe fn removeOutput(&self, output: &AVCaptureOutput);

        #[cfg(feature = "AVCaptureInput")]
        #[method(addInputWithNoConnections:)]
        pub unsafe fn addInputWithNoConnections(&self, input: &AVCaptureInput);

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method(addOutputWithNoConnections:)]
        pub unsafe fn addOutputWithNoConnections(&self, output: &AVCaptureOutput);

        #[method_id(@__retain_semantics Other connections)]
        pub unsafe fn connections(&self) -> Retained<NSArray<AVCaptureConnection>>;

        #[method(canAddConnection:)]
        pub unsafe fn canAddConnection(&self, connection: &AVCaptureConnection) -> bool;

        #[method(addConnection:)]
        pub unsafe fn addConnection(&self, connection: &AVCaptureConnection);

        #[method(removeConnection:)]
        pub unsafe fn removeConnection(&self, connection: &AVCaptureConnection);

        #[method(supportsControls)]
        pub unsafe fn supportsControls(&self) -> bool;

        #[method(maxControlsCount)]
        pub unsafe fn maxControlsCount(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other controlsDelegate)]
        pub unsafe fn controlsDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVCaptureSessionControlsDelegate>>>;

        #[cfg(feature = "AVCaptureControl")]
        #[method_id(@__retain_semantics Other controls)]
        pub unsafe fn controls(&self) -> Retained<NSArray<AVCaptureControl>>;

        #[cfg(feature = "AVCaptureControl")]
        #[method(canAddControl:)]
        pub unsafe fn canAddControl(&self, control: &AVCaptureControl) -> bool;

        #[cfg(feature = "AVCaptureControl")]
        #[method(addControl:)]
        pub unsafe fn addControl(&self, control: &AVCaptureControl);

        #[cfg(feature = "AVCaptureControl")]
        #[method(removeControl:)]
        pub unsafe fn removeControl(&self, control: &AVCaptureControl);

        #[method(beginConfiguration)]
        pub unsafe fn beginConfiguration(&self);

        #[method(commitConfiguration)]
        pub unsafe fn commitConfiguration(&self);

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(isInterrupted)]
        pub unsafe fn isInterrupted(&self) -> bool;

        #[method(isMultitaskingCameraAccessSupported)]
        pub unsafe fn isMultitaskingCameraAccessSupported(&self) -> bool;

        #[method(isMultitaskingCameraAccessEnabled)]
        pub unsafe fn isMultitaskingCameraAccessEnabled(&self) -> bool;

        #[method(setMultitaskingCameraAccessEnabled:)]
        pub unsafe fn setMultitaskingCameraAccessEnabled(
            &self,
            multitasking_camera_access_enabled: bool,
        );

        #[method(usesApplicationAudioSession)]
        pub unsafe fn usesApplicationAudioSession(&self) -> bool;

        #[method(setUsesApplicationAudioSession:)]
        pub unsafe fn setUsesApplicationAudioSession(&self, uses_application_audio_session: bool);

        #[method(automaticallyConfiguresApplicationAudioSession)]
        pub unsafe fn automaticallyConfiguresApplicationAudioSession(&self) -> bool;

        #[method(setAutomaticallyConfiguresApplicationAudioSession:)]
        pub unsafe fn setAutomaticallyConfiguresApplicationAudioSession(
            &self,
            automatically_configures_application_audio_session: bool,
        );

        #[method(configuresApplicationAudioSessionToMixWithOthers)]
        pub unsafe fn configuresApplicationAudioSessionToMixWithOthers(&self) -> bool;

        #[method(setConfiguresApplicationAudioSessionToMixWithOthers:)]
        pub unsafe fn setConfiguresApplicationAudioSessionToMixWithOthers(
            &self,
            configures_application_audio_session_to_mix_with_others: bool,
        );

        #[method(automaticallyConfiguresCaptureDeviceForWideColor)]
        pub unsafe fn automaticallyConfiguresCaptureDeviceForWideColor(&self) -> bool;

        #[method(setAutomaticallyConfiguresCaptureDeviceForWideColor:)]
        pub unsafe fn setAutomaticallyConfiguresCaptureDeviceForWideColor(
            &self,
            automatically_configures_capture_device_for_wide_color: bool,
        );

        #[method(startRunning)]
        pub unsafe fn startRunning(&self);

        #[method(stopRunning)]
        pub unsafe fn stopRunning(&self);

        #[cfg(feature = "objc2-core-media")]
        #[method(synchronizationClock)]
        pub unsafe fn synchronizationClock(&self) -> CMClockRef;

        #[cfg(feature = "objc2-core-media")]
        #[deprecated]
        #[method(masterClock)]
        pub unsafe fn masterClock(&self) -> CMClockRef;

        #[method(hardwareCost)]
        pub unsafe fn hardwareCost(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCaptureSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesessioncontrolsdelegate?language=objc)
    pub unsafe trait AVCaptureSessionControlsDelegate: NSObjectProtocol {
        #[method(sessionControlsDidBecomeActive:)]
        unsafe fn sessionControlsDidBecomeActive(&self, session: &AVCaptureSession);

        #[method(sessionControlsWillEnterFullscreenAppearance:)]
        unsafe fn sessionControlsWillEnterFullscreenAppearance(&self, session: &AVCaptureSession);

        #[method(sessionControlsWillExitFullscreenAppearance:)]
        unsafe fn sessionControlsWillExitFullscreenAppearance(&self, session: &AVCaptureSession);

        #[method(sessionControlsDidBecomeInactive:)]
        unsafe fn sessionControlsDidBecomeInactive(&self, session: &AVCaptureSession);
    }

    unsafe impl ProtocolType for dyn AVCaptureSessionControlsDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturemulticamsession?language=objc)
    #[unsafe(super(AVCaptureSession, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureMultiCamSession;
);

unsafe impl NSObjectProtocol for AVCaptureMultiCamSession {}

extern_methods!(
    unsafe impl AVCaptureMultiCamSession {
        #[method(isMultiCamSupported)]
        pub unsafe fn isMultiCamSupported() -> bool;

        #[method(hardwareCost)]
        pub unsafe fn hardwareCost(&self) -> c_float;

        #[method(systemPressureCost)]
        pub unsafe fn systemPressureCost(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCaptureMultiCamSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideofieldmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVVideoFieldMode(pub NSInteger);
impl AVVideoFieldMode {
    #[doc(alias = "AVVideoFieldModeBoth")]
    pub const Both: Self = Self(0);
    #[doc(alias = "AVVideoFieldModeTopOnly")]
    pub const TopOnly: Self = Self(1);
    #[doc(alias = "AVVideoFieldModeBottomOnly")]
    pub const BottomOnly: Self = Self(2);
    #[doc(alias = "AVVideoFieldModeDeinterlace")]
    pub const Deinterlace: Self = Self(3);
}

unsafe impl Encode for AVVideoFieldMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVVideoFieldMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureconnection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureConnection;
);

unsafe impl NSObjectProtocol for AVCaptureConnection {}

extern_methods!(
    unsafe impl AVCaptureConnection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(feature = "AVCaptureInput", feature = "AVCaptureOutputBase"))]
        #[method_id(@__retain_semantics Other connectionWithInputPorts:output:)]
        pub unsafe fn connectionWithInputPorts_output(
            ports: &NSArray<AVCaptureInputPort>,
            output: &AVCaptureOutput,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "AVCaptureInput",
            feature = "AVCaptureVideoPreviewLayer",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other connectionWithInputPort:videoPreviewLayer:)]
        pub unsafe fn connectionWithInputPort_videoPreviewLayer(
            port: &AVCaptureInputPort,
            layer: &AVCaptureVideoPreviewLayer,
        ) -> Retained<Self>;

        #[cfg(all(feature = "AVCaptureInput", feature = "AVCaptureOutputBase"))]
        #[method_id(@__retain_semantics Init initWithInputPorts:output:)]
        pub unsafe fn initWithInputPorts_output(
            this: Allocated<Self>,
            ports: &NSArray<AVCaptureInputPort>,
            output: &AVCaptureOutput,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "AVCaptureInput",
            feature = "AVCaptureVideoPreviewLayer",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Init initWithInputPort:videoPreviewLayer:)]
        pub unsafe fn initWithInputPort_videoPreviewLayer(
            this: Allocated<Self>,
            port: &AVCaptureInputPort,
            layer: &AVCaptureVideoPreviewLayer,
        ) -> Retained<Self>;

        #[cfg(feature = "AVCaptureInput")]
        #[method_id(@__retain_semantics Other inputPorts)]
        pub unsafe fn inputPorts(&self) -> Retained<NSArray<AVCaptureInputPort>>;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method_id(@__retain_semantics Other output)]
        pub unsafe fn output(&self) -> Option<Retained<AVCaptureOutput>>;

        #[cfg(all(feature = "AVCaptureVideoPreviewLayer", feature = "objc2-quartz-core"))]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other videoPreviewLayer)]
        pub unsafe fn videoPreviewLayer(&self) -> Option<Retained<AVCaptureVideoPreviewLayer>>;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method_id(@__retain_semantics Other audioChannels)]
        pub unsafe fn audioChannels(&self) -> Retained<NSArray<AVCaptureAudioChannel>>;

        #[method(isVideoMirroringSupported)]
        pub unsafe fn isVideoMirroringSupported(&self) -> bool;

        #[method(isVideoMirrored)]
        pub unsafe fn isVideoMirrored(&self) -> bool;

        #[method(setVideoMirrored:)]
        pub unsafe fn setVideoMirrored(&self, video_mirrored: bool);

        #[method(automaticallyAdjustsVideoMirroring)]
        pub unsafe fn automaticallyAdjustsVideoMirroring(&self) -> bool;

        #[method(setAutomaticallyAdjustsVideoMirroring:)]
        pub unsafe fn setAutomaticallyAdjustsVideoMirroring(
            &self,
            automatically_adjusts_video_mirroring: bool,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(isVideoRotationAngleSupported:)]
        pub unsafe fn isVideoRotationAngleSupported(&self, video_rotation_angle: CGFloat) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(videoRotationAngle)]
        pub unsafe fn videoRotationAngle(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setVideoRotationAngle:)]
        pub unsafe fn setVideoRotationAngle(&self, video_rotation_angle: CGFloat);

        #[deprecated = "Use -isVideoRotationAngleSupported: instead"]
        #[method(isVideoOrientationSupported)]
        pub unsafe fn isVideoOrientationSupported(&self) -> bool;

        #[deprecated = "Use -videoRotationAngle instead"]
        #[method(videoOrientation)]
        pub unsafe fn videoOrientation(&self) -> AVCaptureVideoOrientation;

        #[deprecated = "Use -videoRotationAngle instead"]
        #[method(setVideoOrientation:)]
        pub unsafe fn setVideoOrientation(&self, video_orientation: AVCaptureVideoOrientation);

        #[method(isVideoFieldModeSupported)]
        pub unsafe fn isVideoFieldModeSupported(&self) -> bool;

        #[method(videoFieldMode)]
        pub unsafe fn videoFieldMode(&self) -> AVVideoFieldMode;

        #[method(setVideoFieldMode:)]
        pub unsafe fn setVideoFieldMode(&self, video_field_mode: AVVideoFieldMode);

        #[deprecated = "Use AVCaptureDevice's activeFormat.videoSupportedFrameRateRanges instead."]
        #[method(isVideoMinFrameDurationSupported)]
        pub unsafe fn isVideoMinFrameDurationSupported(&self) -> bool;

        #[cfg(feature = "objc2-core-media")]
        #[deprecated = "Use AVCaptureDevice's activeVideoMinFrameDuration instead."]
        #[method(videoMinFrameDuration)]
        pub unsafe fn videoMinFrameDuration(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[deprecated = "Use AVCaptureDevice's activeVideoMinFrameDuration instead."]
        #[method(setVideoMinFrameDuration:)]
        pub unsafe fn setVideoMinFrameDuration(&self, video_min_frame_duration: CMTime);

        #[deprecated = "Use AVCaptureDevice's activeFormat.videoSupportedFrameRateRanges instead."]
        #[method(isVideoMaxFrameDurationSupported)]
        pub unsafe fn isVideoMaxFrameDurationSupported(&self) -> bool;

        #[cfg(feature = "objc2-core-media")]
        #[deprecated = "Use AVCaptureDevice's activeVideoMaxFrameDuration instead."]
        #[method(videoMaxFrameDuration)]
        pub unsafe fn videoMaxFrameDuration(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[deprecated = "Use AVCaptureDevice's activeVideoMaxFrameDuration instead."]
        #[method(setVideoMaxFrameDuration:)]
        pub unsafe fn setVideoMaxFrameDuration(&self, video_max_frame_duration: CMTime);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(videoMaxScaleAndCropFactor)]
        pub unsafe fn videoMaxScaleAndCropFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(videoScaleAndCropFactor)]
        pub unsafe fn videoScaleAndCropFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setVideoScaleAndCropFactor:)]
        pub unsafe fn setVideoScaleAndCropFactor(&self, video_scale_and_crop_factor: CGFloat);

        #[cfg(feature = "AVCaptureDevice")]
        #[method(preferredVideoStabilizationMode)]
        pub unsafe fn preferredVideoStabilizationMode(&self) -> AVCaptureVideoStabilizationMode;

        #[cfg(feature = "AVCaptureDevice")]
        #[method(setPreferredVideoStabilizationMode:)]
        pub unsafe fn setPreferredVideoStabilizationMode(
            &self,
            preferred_video_stabilization_mode: AVCaptureVideoStabilizationMode,
        );

        #[cfg(feature = "AVCaptureDevice")]
        #[method(activeVideoStabilizationMode)]
        pub unsafe fn activeVideoStabilizationMode(&self) -> AVCaptureVideoStabilizationMode;

        #[method(isVideoStabilizationSupported)]
        pub unsafe fn isVideoStabilizationSupported(&self) -> bool;

        #[deprecated = "Use activeVideoStabilizationMode instead."]
        #[method(isVideoStabilizationEnabled)]
        pub unsafe fn isVideoStabilizationEnabled(&self) -> bool;

        #[deprecated = "Use preferredVideoStabilizationMode instead."]
        #[method(enablesVideoStabilizationWhenAvailable)]
        pub unsafe fn enablesVideoStabilizationWhenAvailable(&self) -> bool;

        #[deprecated = "Use preferredVideoStabilizationMode instead."]
        #[method(setEnablesVideoStabilizationWhenAvailable:)]
        pub unsafe fn setEnablesVideoStabilizationWhenAvailable(
            &self,
            enables_video_stabilization_when_available: bool,
        );

        #[method(isCameraIntrinsicMatrixDeliverySupported)]
        pub unsafe fn isCameraIntrinsicMatrixDeliverySupported(&self) -> bool;

        #[method(isCameraIntrinsicMatrixDeliveryEnabled)]
        pub unsafe fn isCameraIntrinsicMatrixDeliveryEnabled(&self) -> bool;

        #[method(setCameraIntrinsicMatrixDeliveryEnabled:)]
        pub unsafe fn setCameraIntrinsicMatrixDeliveryEnabled(
            &self,
            camera_intrinsic_matrix_delivery_enabled: bool,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureaudiochannel?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureAudioChannel;
);

unsafe impl NSObjectProtocol for AVCaptureAudioChannel {}

extern_methods!(
    unsafe impl AVCaptureAudioChannel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(averagePowerLevel)]
        pub unsafe fn averagePowerLevel(&self) -> c_float;

        #[method(peakHoldLevel)]
        pub unsafe fn peakHoldLevel(&self) -> c_float;

        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);