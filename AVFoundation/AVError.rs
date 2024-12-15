//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfoundationerrordomain?language=objc)
    pub static AVFoundationErrorDomain: Option<&'static NSErrorDomain>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrordevicekey?language=objc)
    pub static AVErrorDeviceKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrortimekey?language=objc)
    pub static AVErrorTimeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrorfilesizekey?language=objc)
    pub static AVErrorFileSizeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrorpidkey?language=objc)
    pub static AVErrorPIDKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrorrecordingsuccessfullyfinishedkey?language=objc)
    pub static AVErrorRecordingSuccessfullyFinishedKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrormediatypekey?language=objc)
    pub static AVErrorMediaTypeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrormediasubtypekey?language=objc)
    pub static AVErrorMediaSubTypeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrorpresentationtimestampkey?language=objc)
    pub static AVErrorPresentationTimeStampKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrorpersistenttrackidkey?language=objc)
    pub static AVErrorPersistentTrackIDKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrorfiletypekey?language=objc)
    pub static AVErrorFileTypeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averrordiscontinuityflagskey?language=objc)
    pub static AVErrorDiscontinuityFlagsKey: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/averror?language=objc)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVError(pub NSInteger);
impl AVError {
    #[doc(alias = "AVErrorUnknown")]
    pub const Unknown: Self = Self(-11800);
    #[doc(alias = "AVErrorOutOfMemory")]
    pub const OutOfMemory: Self = Self(-11801);
    #[doc(alias = "AVErrorSessionNotRunning")]
    pub const SessionNotRunning: Self = Self(-11803);
    #[doc(alias = "AVErrorDeviceAlreadyUsedByAnotherSession")]
    pub const DeviceAlreadyUsedByAnotherSession: Self = Self(-11804);
    #[doc(alias = "AVErrorNoDataCaptured")]
    pub const NoDataCaptured: Self = Self(-11805);
    #[doc(alias = "AVErrorSessionConfigurationChanged")]
    pub const SessionConfigurationChanged: Self = Self(-11806);
    #[doc(alias = "AVErrorDiskFull")]
    pub const DiskFull: Self = Self(-11807);
    #[doc(alias = "AVErrorDeviceWasDisconnected")]
    pub const DeviceWasDisconnected: Self = Self(-11808);
    #[doc(alias = "AVErrorMediaChanged")]
    pub const MediaChanged: Self = Self(-11809);
    #[doc(alias = "AVErrorMaximumDurationReached")]
    pub const MaximumDurationReached: Self = Self(-11810);
    #[doc(alias = "AVErrorMaximumFileSizeReached")]
    pub const MaximumFileSizeReached: Self = Self(-11811);
    #[doc(alias = "AVErrorMediaDiscontinuity")]
    pub const MediaDiscontinuity: Self = Self(-11812);
    #[doc(alias = "AVErrorMaximumNumberOfSamplesForFileFormatReached")]
    pub const MaximumNumberOfSamplesForFileFormatReached: Self = Self(-11813);
    #[doc(alias = "AVErrorDeviceNotConnected")]
    pub const DeviceNotConnected: Self = Self(-11814);
    #[doc(alias = "AVErrorDeviceInUseByAnotherApplication")]
    pub const DeviceInUseByAnotherApplication: Self = Self(-11815);
    #[doc(alias = "AVErrorDeviceLockedForConfigurationByAnotherProcess")]
    pub const DeviceLockedForConfigurationByAnotherProcess: Self = Self(-11817);
    #[doc(alias = "AVErrorExportFailed")]
    pub const ExportFailed: Self = Self(-11820);
    #[doc(alias = "AVErrorDecodeFailed")]
    pub const DecodeFailed: Self = Self(-11821);
    #[doc(alias = "AVErrorInvalidSourceMedia")]
    pub const InvalidSourceMedia: Self = Self(-11822);
    #[doc(alias = "AVErrorFileAlreadyExists")]
    pub const FileAlreadyExists: Self = Self(-11823);
    #[doc(alias = "AVErrorCompositionTrackSegmentsNotContiguous")]
    pub const CompositionTrackSegmentsNotContiguous: Self = Self(-11824);
    #[doc(alias = "AVErrorInvalidCompositionTrackSegmentDuration")]
    pub const InvalidCompositionTrackSegmentDuration: Self = Self(-11825);
    #[doc(alias = "AVErrorInvalidCompositionTrackSegmentSourceStartTime")]
    pub const InvalidCompositionTrackSegmentSourceStartTime: Self = Self(-11826);
    #[doc(alias = "AVErrorInvalidCompositionTrackSegmentSourceDuration")]
    pub const InvalidCompositionTrackSegmentSourceDuration: Self = Self(-11827);
    #[doc(alias = "AVErrorFileFormatNotRecognized")]
    pub const FileFormatNotRecognized: Self = Self(-11828);
    #[doc(alias = "AVErrorFileFailedToParse")]
    pub const FileFailedToParse: Self = Self(-11829);
    #[doc(alias = "AVErrorMaximumStillImageCaptureRequestsExceeded")]
    pub const MaximumStillImageCaptureRequestsExceeded: Self = Self(-11830);
    #[doc(alias = "AVErrorContentIsProtected")]
    pub const ContentIsProtected: Self = Self(-11831);
    #[doc(alias = "AVErrorNoImageAtTime")]
    pub const NoImageAtTime: Self = Self(-11832);
    #[doc(alias = "AVErrorDecoderNotFound")]
    pub const DecoderNotFound: Self = Self(-11833);
    #[doc(alias = "AVErrorEncoderNotFound")]
    pub const EncoderNotFound: Self = Self(-11834);
    #[doc(alias = "AVErrorContentIsNotAuthorized")]
    pub const ContentIsNotAuthorized: Self = Self(-11835);
    #[doc(alias = "AVErrorApplicationIsNotAuthorized")]
    pub const ApplicationIsNotAuthorized: Self = Self(-11836);
    #[doc(alias = "AVErrorOperationNotSupportedForAsset")]
    pub const OperationNotSupportedForAsset: Self = Self(-11838);
    #[doc(alias = "AVErrorDecoderTemporarilyUnavailable")]
    pub const DecoderTemporarilyUnavailable: Self = Self(-11839);
    #[doc(alias = "AVErrorEncoderTemporarilyUnavailable")]
    pub const EncoderTemporarilyUnavailable: Self = Self(-11840);
    #[doc(alias = "AVErrorInvalidVideoComposition")]
    pub const InvalidVideoComposition: Self = Self(-11841);
    #[doc(alias = "AVErrorReferenceForbiddenByReferencePolicy")]
    pub const ReferenceForbiddenByReferencePolicy: Self = Self(-11842);
    #[doc(alias = "AVErrorInvalidOutputURLPathExtension")]
    pub const InvalidOutputURLPathExtension: Self = Self(-11843);
    #[doc(alias = "AVErrorScreenCaptureFailed")]
    pub const ScreenCaptureFailed: Self = Self(-11844);
    #[doc(alias = "AVErrorDisplayWasDisabled")]
    pub const DisplayWasDisabled: Self = Self(-11845);
    #[doc(alias = "AVErrorTorchLevelUnavailable")]
    pub const TorchLevelUnavailable: Self = Self(-11846);
    #[doc(alias = "AVErrorIncompatibleAsset")]
    pub const IncompatibleAsset: Self = Self(-11848);
    #[doc(alias = "AVErrorFailedToLoadMediaData")]
    pub const FailedToLoadMediaData: Self = Self(-11849);
    #[doc(alias = "AVErrorServerIncorrectlyConfigured")]
    pub const ServerIncorrectlyConfigured: Self = Self(-11850);
    #[doc(alias = "AVErrorApplicationIsNotAuthorizedToUseDevice")]
    pub const ApplicationIsNotAuthorizedToUseDevice: Self = Self(-11852);
    #[doc(alias = "AVErrorFailedToParse")]
    pub const FailedToParse: Self = Self(-11853);
    #[doc(alias = "AVErrorFileTypeDoesNotSupportSampleReferences")]
    pub const FileTypeDoesNotSupportSampleReferences: Self = Self(-11854);
    #[doc(alias = "AVErrorUndecodableMediaData")]
    pub const UndecodableMediaData: Self = Self(-11855);
    #[doc(alias = "AVErrorAirPlayControllerRequiresInternet")]
    pub const AirPlayControllerRequiresInternet: Self = Self(-11856);
    #[doc(alias = "AVErrorAirPlayReceiverRequiresInternet")]
    pub const AirPlayReceiverRequiresInternet: Self = Self(-11857);
    #[doc(alias = "AVErrorVideoCompositorFailed")]
    pub const VideoCompositorFailed: Self = Self(-11858);
    #[doc(alias = "AVErrorRecordingAlreadyInProgress")]
    pub const RecordingAlreadyInProgress: Self = Self(-11859);
    #[doc(alias = "AVErrorCreateContentKeyRequestFailed")]
    pub const CreateContentKeyRequestFailed: Self = Self(-11860);
    #[doc(alias = "AVErrorUnsupportedOutputSettings")]
    pub const UnsupportedOutputSettings: Self = Self(-11861);
    #[doc(alias = "AVErrorOperationNotAllowed")]
    pub const OperationNotAllowed: Self = Self(-11862);
    #[doc(alias = "AVErrorContentIsUnavailable")]
    pub const ContentIsUnavailable: Self = Self(-11863);
    #[doc(alias = "AVErrorFormatUnsupported")]
    pub const FormatUnsupported: Self = Self(-11864);
    #[doc(alias = "AVErrorMalformedDepth")]
    pub const MalformedDepth: Self = Self(-11865);
    #[doc(alias = "AVErrorContentNotUpdated")]
    pub const ContentNotUpdated: Self = Self(-11866);
    #[doc(alias = "AVErrorNoLongerPlayable")]
    pub const NoLongerPlayable: Self = Self(-11867);
    #[doc(alias = "AVErrorNoCompatibleAlternatesForExternalDisplay")]
    pub const NoCompatibleAlternatesForExternalDisplay: Self = Self(-11868);
    #[doc(alias = "AVErrorNoSourceTrack")]
    pub const NoSourceTrack: Self = Self(-11869);
    #[doc(alias = "AVErrorExternalPlaybackNotSupportedForAsset")]
    pub const ExternalPlaybackNotSupportedForAsset: Self = Self(-11870);
    #[doc(alias = "AVErrorOperationNotSupportedForPreset")]
    pub const OperationNotSupportedForPreset: Self = Self(-11871);
    #[doc(alias = "AVErrorSessionHardwareCostOverage")]
    pub const SessionHardwareCostOverage: Self = Self(-11872);
    #[doc(alias = "AVErrorUnsupportedDeviceActiveFormat")]
    pub const UnsupportedDeviceActiveFormat: Self = Self(-11873);
    #[doc(alias = "AVErrorIncorrectlyConfigured")]
    pub const IncorrectlyConfigured: Self = Self(-11875);
    #[doc(alias = "AVErrorSegmentStartedWithNonSyncSample")]
    pub const SegmentStartedWithNonSyncSample: Self = Self(-11876);
    #[doc(alias = "AVErrorRosettaNotInstalled")]
    pub const RosettaNotInstalled: Self = Self(-11877);
    #[doc(alias = "AVErrorOperationCancelled")]
    pub const OperationCancelled: Self = Self(-11878);
    #[doc(alias = "AVErrorContentKeyRequestCancelled")]
    pub const ContentKeyRequestCancelled: Self = Self(-11879);
    #[doc(alias = "AVErrorInvalidSampleCursor")]
    pub const InvalidSampleCursor: Self = Self(-11880);
    #[doc(alias = "AVErrorFailedToLoadSampleData")]
    pub const FailedToLoadSampleData: Self = Self(-11881);
    #[doc(alias = "AVErrorAirPlayReceiverTemporarilyUnavailable")]
    pub const AirPlayReceiverTemporarilyUnavailable: Self = Self(-11882);
    #[doc(alias = "AVErrorEncodeFailed")]
    pub const EncodeFailed: Self = Self(-11883);
    #[doc(alias = "AVErrorSandboxExtensionDenied")]
    pub const SandboxExtensionDenied: Self = Self(-11884);
    #[doc(alias = "AVErrorToneMappingFailed")]
    pub const ToneMappingFailed: Self = Self(-11885);
    #[doc(alias = "AVErrorMediaExtensionDisabled")]
    pub const MediaExtensionDisabled: Self = Self(-11886);
    #[doc(alias = "AVErrorMediaExtensionConflict")]
    pub const MediaExtensionConflict: Self = Self(-11887);
}

unsafe impl Encode for AVError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}