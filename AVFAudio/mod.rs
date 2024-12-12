// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "AVFAudio", kind = "framework")]
extern "C" {}

#[cfg(feature = "AVAudioApplication")]
#[path = "AVAudioApplication.rs"]
mod __AVAudioApplication;
#[cfg(feature = "AVAudioBuffer")]
#[path = "AVAudioBuffer.rs"]
mod __AVAudioBuffer;
#[cfg(feature = "AVAudioChannelLayout")]
#[path = "AVAudioChannelLayout.rs"]
mod __AVAudioChannelLayout;
#[cfg(feature = "AVAudioConnectionPoint")]
#[path = "AVAudioConnectionPoint.rs"]
mod __AVAudioConnectionPoint;
#[cfg(feature = "AVAudioConverter")]
#[path = "AVAudioConverter.rs"]
mod __AVAudioConverter;
#[cfg(feature = "AVAudioEngine")]
#[path = "AVAudioEngine.rs"]
mod __AVAudioEngine;
#[cfg(feature = "AVAudioEnvironmentNode")]
#[path = "AVAudioEnvironmentNode.rs"]
mod __AVAudioEnvironmentNode;
#[cfg(feature = "AVAudioFile")]
#[path = "AVAudioFile.rs"]
mod __AVAudioFile;
#[cfg(feature = "AVAudioFormat")]
#[path = "AVAudioFormat.rs"]
mod __AVAudioFormat;
#[cfg(feature = "AVAudioIONode")]
#[path = "AVAudioIONode.rs"]
mod __AVAudioIONode;
#[cfg(feature = "AVAudioMixerNode")]
#[path = "AVAudioMixerNode.rs"]
mod __AVAudioMixerNode;
#[cfg(feature = "AVAudioMixing")]
#[path = "AVAudioMixing.rs"]
mod __AVAudioMixing;
#[cfg(feature = "AVAudioNode")]
#[path = "AVAudioNode.rs"]
mod __AVAudioNode;
#[cfg(feature = "AVAudioPlayer")]
#[path = "AVAudioPlayer.rs"]
mod __AVAudioPlayer;
#[cfg(feature = "AVAudioPlayerNode")]
#[path = "AVAudioPlayerNode.rs"]
mod __AVAudioPlayerNode;
#[cfg(feature = "AVAudioRecorder")]
#[path = "AVAudioRecorder.rs"]
mod __AVAudioRecorder;
#[cfg(feature = "AVAudioRoutingArbiter")]
#[path = "AVAudioRoutingArbiter.rs"]
mod __AVAudioRoutingArbiter;
#[cfg(feature = "AVAudioSequencer")]
#[path = "AVAudioSequencer.rs"]
mod __AVAudioSequencer;
#[cfg(feature = "AVAudioSession")]
#[path = "AVAudioSession.rs"]
mod __AVAudioSession;
#[cfg(feature = "AVAudioSessionDeprecated")]
#[path = "AVAudioSessionDeprecated.rs"]
mod __AVAudioSessionDeprecated;
#[cfg(feature = "AVAudioSessionRoute")]
#[path = "AVAudioSessionRoute.rs"]
mod __AVAudioSessionRoute;
#[cfg(feature = "AVAudioSessionTypes")]
#[path = "AVAudioSessionTypes.rs"]
mod __AVAudioSessionTypes;
#[cfg(feature = "AVAudioSettings")]
#[path = "AVAudioSettings.rs"]
mod __AVAudioSettings;
#[cfg(feature = "AVAudioSinkNode")]
#[path = "AVAudioSinkNode.rs"]
mod __AVAudioSinkNode;
#[cfg(feature = "AVAudioSourceNode")]
#[path = "AVAudioSourceNode.rs"]
mod __AVAudioSourceNode;
#[cfg(feature = "AVAudioTime")]
#[path = "AVAudioTime.rs"]
mod __AVAudioTime;
#[cfg(feature = "AVAudioTypes")]
#[path = "AVAudioTypes.rs"]
mod __AVAudioTypes;
#[cfg(feature = "AVAudioUnit")]
#[path = "AVAudioUnit.rs"]
mod __AVAudioUnit;
#[cfg(feature = "AVAudioUnitComponent")]
#[path = "AVAudioUnitComponent.rs"]
mod __AVAudioUnitComponent;
#[cfg(feature = "AVAudioUnitDelay")]
#[path = "AVAudioUnitDelay.rs"]
mod __AVAudioUnitDelay;
#[cfg(feature = "AVAudioUnitDistortion")]
#[path = "AVAudioUnitDistortion.rs"]
mod __AVAudioUnitDistortion;
#[cfg(feature = "AVAudioUnitEQ")]
#[path = "AVAudioUnitEQ.rs"]
mod __AVAudioUnitEQ;
#[cfg(feature = "AVAudioUnitEffect")]
#[path = "AVAudioUnitEffect.rs"]
mod __AVAudioUnitEffect;
#[cfg(feature = "AVAudioUnitGenerator")]
#[path = "AVAudioUnitGenerator.rs"]
mod __AVAudioUnitGenerator;
#[cfg(feature = "AVAudioUnitMIDIInstrument")]
#[path = "AVAudioUnitMIDIInstrument.rs"]
mod __AVAudioUnitMIDIInstrument;
#[cfg(feature = "AVAudioUnitReverb")]
#[path = "AVAudioUnitReverb.rs"]
mod __AVAudioUnitReverb;
#[cfg(feature = "AVAudioUnitSampler")]
#[path = "AVAudioUnitSampler.rs"]
mod __AVAudioUnitSampler;
#[cfg(feature = "AVAudioUnitTimeEffect")]
#[path = "AVAudioUnitTimeEffect.rs"]
mod __AVAudioUnitTimeEffect;
#[cfg(feature = "AVAudioUnitTimePitch")]
#[path = "AVAudioUnitTimePitch.rs"]
mod __AVAudioUnitTimePitch;
#[cfg(feature = "AVAudioUnitVarispeed")]
#[path = "AVAudioUnitVarispeed.rs"]
mod __AVAudioUnitVarispeed;
#[cfg(feature = "AVMIDIPlayer")]
#[path = "AVMIDIPlayer.rs"]
mod __AVMIDIPlayer;
#[cfg(feature = "AVMusicEvents")]
#[path = "AVMusicEvents.rs"]
mod __AVMusicEvents;
#[cfg(feature = "AVSpeechSynthesis")]
#[path = "AVSpeechSynthesis.rs"]
mod __AVSpeechSynthesis;
#[cfg(feature = "AVSpeechSynthesisProvider")]
#[path = "AVSpeechSynthesisProvider.rs"]
mod __AVSpeechSynthesisProvider;

#[cfg(feature = "AVAudioApplication")]
pub use self::__AVAudioApplication::AVAudioApplication;
#[cfg(feature = "AVAudioApplication")]
pub use self::__AVAudioApplication::AVAudioApplicationInputMuteStateChangeNotification;
#[cfg(feature = "AVAudioApplication")]
pub use self::__AVAudioApplication::AVAudioApplicationMicrophoneInjectionPermission;
#[cfg(feature = "AVAudioApplication")]
pub use self::__AVAudioApplication::AVAudioApplicationMuteStateKey;
#[cfg(feature = "AVAudioApplication")]
pub use self::__AVAudioApplication::AVAudioApplicationRecordPermission;
#[cfg(feature = "AVAudioBuffer")]
pub use self::__AVAudioBuffer::AVAudioBuffer;
#[cfg(feature = "AVAudioBuffer")]
pub use self::__AVAudioBuffer::AVAudioCompressedBuffer;
#[cfg(feature = "AVAudioBuffer")]
pub use self::__AVAudioBuffer::AVAudioPCMBuffer;
#[cfg(feature = "AVAudioChannelLayout")]
pub use self::__AVAudioChannelLayout::AVAudioChannelLayout;
#[cfg(feature = "AVAudioConnectionPoint")]
pub use self::__AVAudioConnectionPoint::AVAudioConnectionPoint;
#[cfg(feature = "AVAudioConverter")]
pub use self::__AVAudioConverter::AVAudioConverter;
#[cfg(all(
    feature = "AVAudioBuffer",
    feature = "AVAudioConverter",
    feature = "AVAudioTypes",
    feature = "block2"
))]
pub use self::__AVAudioConverter::AVAudioConverterInputBlock;
#[cfg(feature = "AVAudioConverter")]
pub use self::__AVAudioConverter::AVAudioConverterInputStatus;
#[cfg(feature = "AVAudioConverter")]
pub use self::__AVAudioConverter::AVAudioConverterOutputStatus;
#[cfg(all(feature = "AVAudioConverter", feature = "AVAudioTypes"))]
pub use self::__AVAudioConverter::AVAudioConverterPrimeInfo;
#[cfg(feature = "AVAudioConverter")]
pub use self::__AVAudioConverter::AVAudioConverterPrimeMethod;
#[cfg(feature = "AVAudioEngine")]
pub use self::__AVAudioEngine::AVAudioEngine;
#[cfg(feature = "AVAudioEngine")]
pub use self::__AVAudioEngine::AVAudioEngineConfigurationChangeNotification;
#[cfg(all(
    feature = "AVAudioEngine",
    feature = "AVAudioTypes",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub use self::__AVAudioEngine::AVAudioEngineManualRenderingBlock;
#[cfg(feature = "AVAudioEngine")]
pub use self::__AVAudioEngine::AVAudioEngineManualRenderingError;
#[cfg(feature = "AVAudioEngine")]
pub use self::__AVAudioEngine::AVAudioEngineManualRenderingMode;
#[cfg(feature = "AVAudioEngine")]
pub use self::__AVAudioEngine::AVAudioEngineManualRenderingStatus;
#[cfg(feature = "AVAudioEnvironmentNode")]
pub use self::__AVAudioEnvironmentNode::AVAudioEnvironmentDistanceAttenuationModel;
#[cfg(feature = "AVAudioEnvironmentNode")]
pub use self::__AVAudioEnvironmentNode::AVAudioEnvironmentDistanceAttenuationParameters;
#[cfg(all(feature = "AVAudioEnvironmentNode", feature = "AVAudioNode"))]
pub use self::__AVAudioEnvironmentNode::AVAudioEnvironmentNode;
#[cfg(feature = "AVAudioEnvironmentNode")]
pub use self::__AVAudioEnvironmentNode::AVAudioEnvironmentOutputType;
#[cfg(feature = "AVAudioEnvironmentNode")]
pub use self::__AVAudioEnvironmentNode::AVAudioEnvironmentReverbParameters;
#[cfg(feature = "AVAudioFile")]
pub use self::__AVAudioFile::AVAudioFile;
#[cfg(feature = "AVAudioFormat")]
pub use self::__AVAudioFormat::AVAudioCommonFormat;
#[cfg(feature = "AVAudioFormat")]
pub use self::__AVAudioFormat::AVAudioFormat;
#[cfg(all(feature = "AVAudioIONode", feature = "AVAudioNode"))]
pub use self::__AVAudioIONode::AVAudioIONode;
#[cfg(all(
    feature = "AVAudioIONode",
    feature = "AVAudioTypes",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub use self::__AVAudioIONode::AVAudioIONodeInputBlock;
#[cfg(all(feature = "AVAudioIONode", feature = "AVAudioNode"))]
pub use self::__AVAudioIONode::AVAudioInputNode;
#[cfg(all(feature = "AVAudioIONode", feature = "AVAudioNode"))]
pub use self::__AVAudioIONode::AVAudioOutputNode;
#[cfg(feature = "AVAudioIONode")]
pub use self::__AVAudioIONode::AVAudioVoiceProcessingOtherAudioDuckingLevel;
#[cfg(feature = "AVAudioIONode")]
pub use self::__AVAudioIONode::AVAudioVoiceProcessingSpeechActivityEvent;
#[cfg(all(feature = "AVAudioMixerNode", feature = "AVAudioNode"))]
pub use self::__AVAudioMixerNode::AVAudioMixerNode;
#[cfg(feature = "AVAudioMixing")]
pub use self::__AVAudioMixing::AVAudio3DMixing;
#[cfg(feature = "AVAudioMixing")]
pub use self::__AVAudioMixing::AVAudio3DMixingPointSourceInHeadMode;
#[cfg(feature = "AVAudioMixing")]
pub use self::__AVAudioMixing::AVAudio3DMixingRenderingAlgorithm;
#[cfg(feature = "AVAudioMixing")]
pub use self::__AVAudioMixing::AVAudio3DMixingSourceMode;
#[cfg(feature = "AVAudioMixing")]
pub use self::__AVAudioMixing::AVAudioMixing;
#[cfg(feature = "AVAudioMixing")]
pub use self::__AVAudioMixing::AVAudioMixingDestination;
#[cfg(feature = "AVAudioMixing")]
pub use self::__AVAudioMixing::AVAudioStereoMixing;
#[cfg(feature = "AVAudioNode")]
pub use self::__AVAudioNode::AVAudioNode;
#[cfg(all(
    feature = "AVAudioBuffer",
    feature = "AVAudioNode",
    feature = "AVAudioTime",
    feature = "block2"
))]
pub use self::__AVAudioNode::AVAudioNodeTapBlock;
#[cfg(feature = "AVAudioPlayer")]
pub use self::__AVAudioPlayer::AVAudioPlayer;
#[cfg(feature = "AVAudioPlayer")]
pub use self::__AVAudioPlayer::AVAudioPlayerDelegate;
#[cfg(all(feature = "AVAudioNode", feature = "AVAudioPlayerNode"))]
pub use self::__AVAudioPlayerNode::AVAudioPlayerNode;
#[cfg(feature = "AVAudioPlayerNode")]
pub use self::__AVAudioPlayerNode::AVAudioPlayerNodeBufferOptions;
#[cfg(feature = "AVAudioPlayerNode")]
pub use self::__AVAudioPlayerNode::AVAudioPlayerNodeCompletionCallbackType;
#[cfg(all(feature = "AVAudioPlayerNode", feature = "block2"))]
pub use self::__AVAudioPlayerNode::AVAudioPlayerNodeCompletionHandler;
#[cfg(feature = "AVAudioRecorder")]
pub use self::__AVAudioRecorder::AVAudioRecorder;
#[cfg(feature = "AVAudioRecorder")]
pub use self::__AVAudioRecorder::AVAudioRecorderDelegate;
#[cfg(feature = "AVAudioRoutingArbiter")]
pub use self::__AVAudioRoutingArbiter::AVAudioRoutingArbiter;
#[cfg(feature = "AVAudioRoutingArbiter")]
pub use self::__AVAudioRoutingArbiter::AVAudioRoutingArbitrationCategory;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencer;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKey;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyAlbum;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyApproximateDurationInSeconds;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyArtist;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyChannelLayout;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyComments;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyComposer;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyCopyright;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyEncodingApplication;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyGenre;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyISRC;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyKeySignature;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyLyricist;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyNominalBitRate;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyRecordedDate;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeySourceBitDepth;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeySourceEncoder;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeySubTitle;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyTempo;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyTimeSignature;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyTitle;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyTrackNumber;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVAudioSequencerInfoDictionaryKeyYear;
#[cfg(all(
    feature = "AVAudioSequencer",
    feature = "AVAudioTypes",
    feature = "block2"
))]
pub use self::__AVAudioSequencer::AVAudioSequencerUserCallback;
#[cfg(all(feature = "AVAudioSequencer", feature = "AVAudioTypes"))]
pub use self::__AVAudioSequencer::AVBeatRange;
#[cfg(all(
    feature = "AVAudioSequencer",
    feature = "AVAudioTypes",
    feature = "AVMusicEvents",
    feature = "block2"
))]
pub use self::__AVAudioSequencer::AVMusicEventEnumerationBlock;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVMusicSequenceLoadOptions;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVMusicTrack;
#[cfg(feature = "AVAudioSequencer")]
pub use self::__AVAudioSequencer::AVMusicTrackLoopCount;
#[cfg(all(feature = "AVAudioSequencer", feature = "AVAudioTypes"))]
pub(crate) use self::__AVAudioSequencer::_AVBeatRange;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSession;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionInterruptionNotification;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionInterruptionOptionKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionInterruptionReasonKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionInterruptionTypeKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionInterruptionWasSuspendedKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionMediaServicesWereLostNotification;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionMediaServicesWereResetNotification;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionMicrophoneInjectionCapabilitiesChangeNotification;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionMicrophoneInjectionIsAvailableKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionRenderingCapabilitiesChangeNotification;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionRenderingModeChangeNotification;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionRenderingModeNewRenderingModeKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionRouteChangeNotification;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionRouteChangePreviousRouteKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionRouteChangeReasonKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionSilenceSecondaryAudioHintNotification;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionSilenceSecondaryAudioHintTypeKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionSpatialAudioEnabledKey;
#[cfg(feature = "AVAudioSession")]
pub use self::__AVAudioSession::AVAudioSessionSpatialPlaybackCapabilitiesChangedNotification;
#[cfg(feature = "AVAudioSessionDeprecated")]
pub use self::__AVAudioSessionDeprecated::AVAudioSessionDelegate;
#[cfg(feature = "AVAudioSessionDeprecated")]
pub use self::__AVAudioSessionDeprecated::AVAudioSessionInterruptionFlags_ShouldResume;
#[cfg(feature = "AVAudioSessionDeprecated")]
pub use self::__AVAudioSessionDeprecated::AVAudioSessionSetActiveFlags_NotifyOthersOnDeactivation;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionChannelDescription;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionDataSourceDescription;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionLocation;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionLocationLower;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionLocationUpper;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionOrientation;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionOrientationBack;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionOrientationBottom;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionOrientationFront;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionOrientationLeft;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionOrientationRight;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionOrientationTop;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionPolarPattern;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionPolarPatternCardioid;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionPolarPatternOmnidirectional;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionPolarPatternStereo;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionPolarPatternSubcardioid;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionPortDescription;
#[cfg(feature = "AVAudioSessionRoute")]
pub use self::__AVAudioSessionRoute::AVAudioSessionRouteDescription;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionActivationOptions;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategory;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategoryAmbient;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategoryAudioProcessing;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategoryMultiRoute;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategoryOptions;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategoryPlayAndRecord;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategoryPlayback;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategoryRecord;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionCategorySoloAmbient;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionIOType;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionInterruptionOptions;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionInterruptionReason;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionInterruptionType;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionMicrophoneInjectionMode;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionMode;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeDefault;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeGameChat;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeMeasurement;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeMoviePlayback;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeSpokenAudio;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeVideoChat;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeVideoRecording;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeVoiceChat;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionModeVoicePrompt;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPort;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortAVB;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortAirPlay;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortBluetoothA2DP;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortBluetoothHFP;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortBluetoothLE;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortBuiltInMic;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortBuiltInReceiver;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortBuiltInSpeaker;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortCarAudio;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortContinuityMicrophone;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortDisplayPort;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortFireWire;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortHDMI;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortHeadphones;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortHeadsetMic;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortLineIn;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortLineOut;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortOverride;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortPCI;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortThunderbolt;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortUSBAudio;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPortVirtual;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionPromptStyle;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionRecordPermission;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionRenderingMode;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionRouteChangeReason;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionRouteSharingPolicy;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionSetActiveOptions;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioSessionSilenceSecondaryAudioHintType;
#[cfg(feature = "AVAudioSessionTypes")]
pub use self::__AVAudioSessionTypes::AVAudioStereoOrientation;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVAudioBitRateStrategy_Constant;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVAudioBitRateStrategy_LongTermAverage;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVAudioBitRateStrategy_Variable;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVAudioBitRateStrategy_VariableConstrained;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVAudioFileTypeKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVAudioQuality;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVChannelLayoutKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVEncoderAudioQualityForVBRKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVEncoderAudioQualityKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVEncoderBitDepthHintKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVEncoderBitRateKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVEncoderBitRatePerChannelKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVEncoderBitRateStrategyKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVFormatIDKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVLinearPCMBitDepthKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVLinearPCMIsBigEndianKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVLinearPCMIsFloatKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVLinearPCMIsNonInterleaved;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVNumberOfChannelsKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVSampleRateConverterAlgorithmKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVSampleRateConverterAlgorithm_Mastering;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVSampleRateConverterAlgorithm_MinimumPhase;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVSampleRateConverterAlgorithm_Normal;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVSampleRateConverterAudioQualityKey;
#[cfg(feature = "AVAudioSettings")]
pub use self::__AVAudioSettings::AVSampleRateKey;
#[cfg(all(feature = "AVAudioNode", feature = "AVAudioSinkNode"))]
pub use self::__AVAudioSinkNode::AVAudioSinkNode;
#[cfg(all(
    feature = "AVAudioSinkNode",
    feature = "AVAudioTypes",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub use self::__AVAudioSinkNode::AVAudioSinkNodeReceiverBlock;
#[cfg(all(feature = "AVAudioNode", feature = "AVAudioSourceNode"))]
pub use self::__AVAudioSourceNode::AVAudioSourceNode;
#[cfg(all(
    feature = "AVAudioSourceNode",
    feature = "AVAudioTypes",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub use self::__AVAudioSourceNode::AVAudioSourceNodeRenderBlock;
#[cfg(feature = "AVAudioTime")]
pub use self::__AVAudioTime::AVAudioTime;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudio3DAngularOrientation;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudio3DPoint;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudio3DVector;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudio3DVectorOrientation;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudioChannelCount;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudioFrameCount;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudioFramePosition;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudioNodeBus;
#[cfg(all(feature = "AVAudioTypes", feature = "block2"))]
pub use self::__AVAudioTypes::AVAudioNodeCompletionHandler;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVAudioPacketCount;
#[cfg(feature = "AVAudioTypes")]
pub use self::__AVAudioTypes::AVMusicTimeStamp;
#[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
pub use self::__AVAudioUnit::AVAudioUnit;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitComponent;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitComponentManager;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitComponentManagerRegistrationsChangedNotification;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitComponentTagsDidChangeNotification;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitManufacturerNameApple;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeEffect;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeFormatConverter;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeGenerator;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeMIDIProcessor;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeMixer;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeMusicDevice;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeMusicEffect;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeOfflineEffect;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypeOutput;
#[cfg(feature = "AVAudioUnitComponent")]
pub use self::__AVAudioUnitComponent::AVAudioUnitTypePanner;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitDelay",
    feature = "AVAudioUnitEffect"
))]
pub use self::__AVAudioUnitDelay::AVAudioUnitDelay;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitDistortion",
    feature = "AVAudioUnitEffect"
))]
pub use self::__AVAudioUnitDistortion::AVAudioUnitDistortion;
#[cfg(feature = "AVAudioUnitDistortion")]
pub use self::__AVAudioUnitDistortion::AVAudioUnitDistortionPreset;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitEQ",
    feature = "AVAudioUnitEffect"
))]
pub use self::__AVAudioUnitEQ::AVAudioUnitEQ;
#[cfg(feature = "AVAudioUnitEQ")]
pub use self::__AVAudioUnitEQ::AVAudioUnitEQFilterParameters;
#[cfg(feature = "AVAudioUnitEQ")]
pub use self::__AVAudioUnitEQ::AVAudioUnitEQFilterType;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitEffect"
))]
pub use self::__AVAudioUnitEffect::AVAudioUnitEffect;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitGenerator"
))]
pub use self::__AVAudioUnitGenerator::AVAudioUnitGenerator;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitMIDIInstrument"
))]
pub use self::__AVAudioUnitMIDIInstrument::AVAudioUnitMIDIInstrument;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitEffect",
    feature = "AVAudioUnitReverb"
))]
pub use self::__AVAudioUnitReverb::AVAudioUnitReverb;
#[cfg(feature = "AVAudioUnitReverb")]
pub use self::__AVAudioUnitReverb::AVAudioUnitReverbPreset;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitMIDIInstrument",
    feature = "AVAudioUnitSampler"
))]
pub use self::__AVAudioUnitSampler::AVAudioUnitSampler;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitTimeEffect"
))]
pub use self::__AVAudioUnitTimeEffect::AVAudioUnitTimeEffect;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitTimeEffect",
    feature = "AVAudioUnitTimePitch"
))]
pub use self::__AVAudioUnitTimePitch::AVAudioUnitTimePitch;
#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitTimeEffect",
    feature = "AVAudioUnitVarispeed"
))]
pub use self::__AVAudioUnitVarispeed::AVAudioUnitVarispeed;
#[cfg(feature = "AVMIDIPlayer")]
pub use self::__AVMIDIPlayer::AVMIDIPlayer;
#[cfg(all(feature = "AVMIDIPlayer", feature = "block2"))]
pub use self::__AVMIDIPlayer::AVMIDIPlayerCompletionHandler;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVAUPresetEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVExtendedNoteOnEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVExtendedNoteOnEventDefaultInstrument;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVExtendedTempoEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIChannelEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIChannelPressureEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIControlChangeEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIControlChangeMessageType;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIMetaEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIMetaEventType;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDINoteEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIPitchBendEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIPolyPressureEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDIProgramChangeEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMIDISysexEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMusicEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVMusicUserEvent;
#[cfg(feature = "AVMusicEvents")]
pub use self::__AVMusicEvents::AVParameterEvent;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechBoundary;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisAvailableVoicesDidChangeNotification;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisIPANotationAttribute;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisMarker;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisMarkerMark;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisPersonalVoiceAuthorizationStatus;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisVoice;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisVoiceGender;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisVoiceIdentifierAlex;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisVoiceQuality;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesisVoiceTraits;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesizer;
#[cfg(all(
    feature = "AVAudioBuffer",
    feature = "AVSpeechSynthesis",
    feature = "block2"
))]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesizerBufferCallback;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesizerDelegate;
#[cfg(all(feature = "AVSpeechSynthesis", feature = "block2"))]
pub use self::__AVSpeechSynthesis::AVSpeechSynthesizerMarkerCallback;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechUtterance;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechUtteranceDefaultSpeechRate;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechUtteranceMaximumSpeechRate;
#[cfg(feature = "AVSpeechSynthesis")]
pub use self::__AVSpeechSynthesis::AVSpeechUtteranceMinimumSpeechRate;
#[cfg(all(feature = "AVSpeechSynthesisProvider", feature = "objc2-audio-toolbox"))]
#[cfg(not(target_os = "watchos"))]
pub use self::__AVSpeechSynthesisProvider::AVSpeechSynthesisProviderAudioUnit;
#[cfg(all(
    feature = "AVSpeechSynthesis",
    feature = "AVSpeechSynthesisProvider",
    feature = "block2"
))]
pub use self::__AVSpeechSynthesisProvider::AVSpeechSynthesisProviderOutputBlock;
#[cfg(feature = "AVSpeechSynthesisProvider")]
pub use self::__AVSpeechSynthesisProvider::AVSpeechSynthesisProviderRequest;
#[cfg(feature = "AVSpeechSynthesisProvider")]
pub use self::__AVSpeechSynthesisProvider::AVSpeechSynthesisProviderVoice;
