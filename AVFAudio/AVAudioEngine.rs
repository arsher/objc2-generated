//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-audio-toolbox")]
#[cfg(not(target_os = "watchos"))]
use objc2_audio_toolbox::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioenginemanualrenderingerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAudioEngineManualRenderingError(pub OSStatus);
impl AVAudioEngineManualRenderingError {
    #[doc(alias = "AVAudioEngineManualRenderingErrorInvalidMode")]
    pub const InvalidMode: Self = Self(-80800);
    #[doc(alias = "AVAudioEngineManualRenderingErrorInitialized")]
    pub const Initialized: Self = Self(-80801);
    #[doc(alias = "AVAudioEngineManualRenderingErrorNotRunning")]
    pub const NotRunning: Self = Self(-80802);
}

unsafe impl Encode for AVAudioEngineManualRenderingError {
    const ENCODING: Encoding = OSStatus::ENCODING;
}

unsafe impl RefEncode for AVAudioEngineManualRenderingError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioenginemanualrenderingstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAudioEngineManualRenderingStatus(pub NSInteger);
impl AVAudioEngineManualRenderingStatus {
    #[doc(alias = "AVAudioEngineManualRenderingStatusError")]
    pub const Error: Self = Self(-1);
    #[doc(alias = "AVAudioEngineManualRenderingStatusSuccess")]
    pub const Success: Self = Self(0);
    #[doc(alias = "AVAudioEngineManualRenderingStatusInsufficientDataFromInputNode")]
    pub const InsufficientDataFromInputNode: Self = Self(1);
    #[doc(alias = "AVAudioEngineManualRenderingStatusCannotDoInCurrentContext")]
    pub const CannotDoInCurrentContext: Self = Self(2);
}

unsafe impl Encode for AVAudioEngineManualRenderingStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVAudioEngineManualRenderingStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioenginemanualrenderingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAudioEngineManualRenderingMode(pub NSInteger);
impl AVAudioEngineManualRenderingMode {
    #[doc(alias = "AVAudioEngineManualRenderingModeOffline")]
    pub const Offline: Self = Self(0);
    #[doc(alias = "AVAudioEngineManualRenderingModeRealtime")]
    pub const Realtime: Self = Self(1);
}

unsafe impl Encode for AVAudioEngineManualRenderingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVAudioEngineManualRenderingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioenginemanualrenderingblock?language=objc)
#[cfg(all(
    feature = "AVAudioTypes",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub type AVAudioEngineManualRenderingBlock = *mut block2::Block<
    dyn Fn(
        AVAudioFrameCount,
        NonNull<AudioBufferList>,
        *mut OSStatus,
    ) -> AVAudioEngineManualRenderingStatus,
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioengine?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioEngine;
);

unsafe impl NSObjectProtocol for AVAudioEngine {}

extern_methods!(
    unsafe impl AVAudioEngine {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "AVAudioNode")]
        #[method(attachNode:)]
        pub unsafe fn attachNode(&self, node: &AVAudioNode);

        #[cfg(feature = "AVAudioNode")]
        #[method(detachNode:)]
        pub unsafe fn detachNode(&self, node: &AVAudioNode);

        #[cfg(all(
            feature = "AVAudioFormat",
            feature = "AVAudioNode",
            feature = "AVAudioTypes"
        ))]
        #[method(connect:to:fromBus:toBus:format:)]
        pub unsafe fn connect_to_fromBus_toBus_format(
            &self,
            node1: &AVAudioNode,
            node2: &AVAudioNode,
            bus1: AVAudioNodeBus,
            bus2: AVAudioNodeBus,
            format: Option<&AVAudioFormat>,
        );

        #[cfg(all(feature = "AVAudioFormat", feature = "AVAudioNode"))]
        #[method(connect:to:format:)]
        pub unsafe fn connect_to_format(
            &self,
            node1: &AVAudioNode,
            node2: &AVAudioNode,
            format: Option<&AVAudioFormat>,
        );

        #[cfg(all(
            feature = "AVAudioConnectionPoint",
            feature = "AVAudioFormat",
            feature = "AVAudioNode",
            feature = "AVAudioTypes"
        ))]
        #[method(connect:toConnectionPoints:fromBus:format:)]
        pub unsafe fn connect_toConnectionPoints_fromBus_format(
            &self,
            source_node: &AVAudioNode,
            dest_nodes: &NSArray<AVAudioConnectionPoint>,
            source_bus: AVAudioNodeBus,
            format: Option<&AVAudioFormat>,
        );

        #[cfg(all(feature = "AVAudioNode", feature = "AVAudioTypes"))]
        #[method(disconnectNodeInput:bus:)]
        pub unsafe fn disconnectNodeInput_bus(&self, node: &AVAudioNode, bus: AVAudioNodeBus);

        #[cfg(feature = "AVAudioNode")]
        #[method(disconnectNodeInput:)]
        pub unsafe fn disconnectNodeInput(&self, node: &AVAudioNode);

        #[cfg(all(feature = "AVAudioNode", feature = "AVAudioTypes"))]
        #[method(disconnectNodeOutput:bus:)]
        pub unsafe fn disconnectNodeOutput_bus(&self, node: &AVAudioNode, bus: AVAudioNodeBus);

        #[cfg(feature = "AVAudioNode")]
        #[method(disconnectNodeOutput:)]
        pub unsafe fn disconnectNodeOutput(&self, node: &AVAudioNode);

        #[method(prepare)]
        pub unsafe fn prepare(&self);

        #[method(startAndReturnError:_)]
        pub unsafe fn startAndReturnError(&self) -> Result<(), Retained<NSError>>;

        #[method(pause)]
        pub unsafe fn pause(&self);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(stop)]
        pub unsafe fn stop(&self);

        #[cfg(all(
            feature = "AVAudioConnectionPoint",
            feature = "AVAudioNode",
            feature = "AVAudioTypes"
        ))]
        #[method_id(@__retain_semantics Other inputConnectionPointForNode:inputBus:)]
        pub unsafe fn inputConnectionPointForNode_inputBus(
            &self,
            node: &AVAudioNode,
            bus: AVAudioNodeBus,
        ) -> Option<Retained<AVAudioConnectionPoint>>;

        #[cfg(all(
            feature = "AVAudioConnectionPoint",
            feature = "AVAudioNode",
            feature = "AVAudioTypes"
        ))]
        #[method_id(@__retain_semantics Other outputConnectionPointsForNode:outputBus:)]
        pub unsafe fn outputConnectionPointsForNode_outputBus(
            &self,
            node: &AVAudioNode,
            bus: AVAudioNodeBus,
        ) -> Retained<NSArray<AVAudioConnectionPoint>>;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method(musicSequence)]
        pub unsafe fn musicSequence(&self) -> MusicSequence;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method(setMusicSequence:)]
        pub unsafe fn setMusicSequence(&self, music_sequence: MusicSequence);

        #[cfg(all(feature = "AVAudioIONode", feature = "AVAudioNode"))]
        #[method_id(@__retain_semantics Other outputNode)]
        pub unsafe fn outputNode(&self) -> Retained<AVAudioOutputNode>;

        #[cfg(all(feature = "AVAudioIONode", feature = "AVAudioNode"))]
        #[method_id(@__retain_semantics Other inputNode)]
        pub unsafe fn inputNode(&self) -> Retained<AVAudioInputNode>;

        #[cfg(all(feature = "AVAudioMixerNode", feature = "AVAudioNode"))]
        #[method_id(@__retain_semantics Other mainMixerNode)]
        pub unsafe fn mainMixerNode(&self) -> Retained<AVAudioMixerNode>;

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(isAutoShutdownEnabled)]
        pub unsafe fn isAutoShutdownEnabled(&self) -> bool;

        #[method(setAutoShutdownEnabled:)]
        pub unsafe fn setAutoShutdownEnabled(&self, auto_shutdown_enabled: bool);

        #[cfg(feature = "AVAudioNode")]
        #[method_id(@__retain_semantics Other attachedNodes)]
        pub unsafe fn attachedNodes(&self) -> Retained<NSSet<AVAudioNode>>;

        #[cfg(all(feature = "AVAudioFormat", feature = "AVAudioTypes"))]
        #[method(enableManualRenderingMode:format:maximumFrameCount:error:_)]
        pub unsafe fn enableManualRenderingMode_format_maximumFrameCount_error(
            &self,
            mode: AVAudioEngineManualRenderingMode,
            pcm_format: &AVAudioFormat,
            maximum_frame_count: AVAudioFrameCount,
        ) -> Result<(), Retained<NSError>>;

        #[method(disableManualRenderingMode)]
        pub unsafe fn disableManualRenderingMode(&self);

        #[cfg(all(
            feature = "AVAudioTypes",
            feature = "block2",
            feature = "objc2-core-audio-types"
        ))]
        #[method(manualRenderingBlock)]
        pub unsafe fn manualRenderingBlock(&self) -> AVAudioEngineManualRenderingBlock;

        #[method(isInManualRenderingMode)]
        pub unsafe fn isInManualRenderingMode(&self) -> bool;

        #[method(manualRenderingMode)]
        pub unsafe fn manualRenderingMode(&self) -> AVAudioEngineManualRenderingMode;

        #[cfg(feature = "AVAudioFormat")]
        #[method_id(@__retain_semantics Other manualRenderingFormat)]
        pub unsafe fn manualRenderingFormat(&self) -> Retained<AVAudioFormat>;

        #[cfg(feature = "AVAudioTypes")]
        #[method(manualRenderingMaximumFrameCount)]
        pub unsafe fn manualRenderingMaximumFrameCount(&self) -> AVAudioFrameCount;

        #[cfg(feature = "AVAudioTypes")]
        #[method(manualRenderingSampleTime)]
        pub unsafe fn manualRenderingSampleTime(&self) -> AVAudioFramePosition;

        #[cfg(all(
            feature = "AVAudioFormat",
            feature = "AVAudioNode",
            feature = "block2",
            feature = "objc2-audio-toolbox"
        ))]
        #[cfg(not(target_os = "watchos"))]
        #[deprecated]
        #[method(connectMIDI:to:format:block:)]
        pub unsafe fn connectMIDI_to_format_block(
            &self,
            source_node: &AVAudioNode,
            destination_node: &AVAudioNode,
            format: Option<&AVAudioFormat>,
            tap_block: AUMIDIOutputEventBlock,
        );

        #[cfg(all(
            feature = "AVAudioFormat",
            feature = "AVAudioNode",
            feature = "block2",
            feature = "objc2-audio-toolbox"
        ))]
        #[cfg(not(target_os = "watchos"))]
        #[deprecated]
        #[method(connectMIDI:toNodes:format:block:)]
        pub unsafe fn connectMIDI_toNodes_format_block(
            &self,
            source_node: &AVAudioNode,
            destination_nodes: &NSArray<AVAudioNode>,
            format: Option<&AVAudioFormat>,
            tap_block: AUMIDIOutputEventBlock,
        );

        #[cfg(feature = "AVAudioNode")]
        #[method(disconnectMIDI:from:)]
        pub unsafe fn disconnectMIDI_from(
            &self,
            source_node: &AVAudioNode,
            destination_node: &AVAudioNode,
        );

        #[cfg(feature = "AVAudioNode")]
        #[method(disconnectMIDI:fromNodes:)]
        pub unsafe fn disconnectMIDI_fromNodes(
            &self,
            source_node: &AVAudioNode,
            destination_nodes: &NSArray<AVAudioNode>,
        );

        #[cfg(feature = "AVAudioNode")]
        #[method(disconnectMIDIInput:)]
        pub unsafe fn disconnectMIDIInput(&self, node: &AVAudioNode);

        #[cfg(feature = "AVAudioNode")]
        #[method(disconnectMIDIOutput:)]
        pub unsafe fn disconnectMIDIOutput(&self, node: &AVAudioNode);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioEngine {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioengineconfigurationchangenotification?language=objc)
    pub static AVAudioEngineConfigurationChangeNotification: &'static NSString;
}
