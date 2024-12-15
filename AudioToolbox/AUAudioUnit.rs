//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudioobjectid?language=objc)
pub type AUAudioObjectID = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/midichannelnumber?language=objc)
pub type MIDIChannelNumber = u8;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudiounitstatus?language=objc)
pub type AUAudioUnitStatus = OSStatus;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudioframecount?language=objc)
pub type AUAudioFrameCount = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudiochannelcount?language=objc)
pub type AUAudioChannelCount = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudiounitbustype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AUAudioUnitBusType(pub NSInteger);
impl AUAudioUnitBusType {
    #[doc(alias = "AUAudioUnitBusTypeInput")]
    pub const Input: Self = Self(1);
    #[doc(alias = "AUAudioUnitBusTypeOutput")]
    pub const Output: Self = Self(2);
}

unsafe impl Encode for AUAudioUnitBusType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AUAudioUnitBusType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aurenderpullinputblock?language=objc)
#[cfg(all(
    feature = "AUComponent",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub type AURenderPullInputBlock = *mut block2::Block<
    dyn Fn(
        NonNull<AudioUnitRenderActionFlags>,
        NonNull<AudioTimeStamp>,
        AUAudioFrameCount,
        NSInteger,
        NonNull<AudioBufferList>,
    ) -> AUAudioUnitStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aurenderblock?language=objc)
#[cfg(all(
    feature = "AUComponent",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub type AURenderBlock = *mut block2::Block<
    dyn Fn(
        NonNull<AudioUnitRenderActionFlags>,
        NonNull<AudioTimeStamp>,
        AUAudioFrameCount,
        NSInteger,
        NonNull<AudioBufferList>,
        AURenderPullInputBlock,
    ) -> AUAudioUnitStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aurenderobserver?language=objc)
#[cfg(all(
    feature = "AUComponent",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub type AURenderObserver = *mut block2::Block<
    dyn Fn(AudioUnitRenderActionFlags, NonNull<AudioTimeStamp>, AUAudioFrameCount, NSInteger),
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auscheduleparameterblock?language=objc)
#[cfg(all(
    feature = "AUParameters",
    feature = "AudioUnitProperties",
    feature = "block2"
))]
pub type AUScheduleParameterBlock =
    *mut block2::Block<dyn Fn(AUEventSampleTime, AUAudioFrameCount, AUParameterAddress, AUValue)>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auschedulemidieventblock?language=objc)
#[cfg(all(feature = "AudioUnitProperties", feature = "block2"))]
pub type AUScheduleMIDIEventBlock =
    *mut block2::Block<dyn Fn(AUEventSampleTime, u8, NSInteger, NonNull<u8>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aumidioutputeventblock?language=objc)
#[cfg(all(feature = "AudioUnitProperties", feature = "block2"))]
pub type AUMIDIOutputEventBlock =
    *mut block2::Block<dyn Fn(AUEventSampleTime, u8, NSInteger, NonNull<u8>) -> OSStatus>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auhostmusicalcontextblock?language=objc)
#[cfg(feature = "block2")]
pub type AUHostMusicalContextBlock = *mut block2::Block<
    dyn Fn(
        *mut c_double,
        *mut c_double,
        *mut NSInteger,
        *mut c_double,
        *mut NSInteger,
        *mut c_double,
    ) -> Bool,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auhosttransportstateflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AUHostTransportStateFlags(pub NSUInteger);
bitflags::bitflags! {
    impl AUHostTransportStateFlags: NSUInteger {
        const AUHostTransportStateChanged = 1;
        const AUHostTransportStateMoving = 2;
        const AUHostTransportStateRecording = 4;
        const AUHostTransportStateCycling = 8;
    }
}

unsafe impl Encode for AUHostTransportStateFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AUHostTransportStateFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auhosttransportstateblock?language=objc)
#[cfg(feature = "block2")]
pub type AUHostTransportStateBlock = *mut block2::Block<
    dyn Fn(*mut AUHostTransportStateFlags, *mut c_double, *mut c_double, *mut c_double) -> Bool,
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudiounit?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AUAudioUnit;
);

unsafe impl NSObjectProtocol for AUAudioUnit {}

extern_methods!(
    unsafe impl AUAudioUnit {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "AudioComponent")]
        #[method_id(@__retain_semantics Init initWithComponentDescription:options:error:_)]
        pub unsafe fn initWithComponentDescription_options_error(
            this: Allocated<Self>,
            component_description: AudioComponentDescription,
            options: AudioComponentInstantiationOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "AudioComponent")]
        #[method_id(@__retain_semantics Init initWithComponentDescription:error:_)]
        pub unsafe fn initWithComponentDescription_error(
            this: Allocated<Self>,
            component_description: AudioComponentDescription,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "AudioComponent", feature = "block2"))]
        #[method(instantiateWithComponentDescription:options:completionHandler:)]
        pub unsafe fn instantiateWithComponentDescription_options_completionHandler(
            component_description: AudioComponentDescription,
            options: AudioComponentInstantiationOptions,
            completion_handler: &block2::Block<dyn Fn(*mut AUAudioUnit, *mut NSError)>,
        );

        #[cfg(feature = "AudioComponent")]
        #[method(componentDescription)]
        pub unsafe fn componentDescription(&self) -> AudioComponentDescription;

        #[cfg(feature = "AudioComponent")]
        #[method(component)]
        pub unsafe fn component(&self) -> AudioComponent;

        #[method_id(@__retain_semantics Other componentName)]
        pub unsafe fn componentName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other audioUnitName)]
        pub unsafe fn audioUnitName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other manufacturerName)]
        pub unsafe fn manufacturerName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other audioUnitShortName)]
        pub unsafe fn audioUnitShortName(&self) -> Option<Retained<NSString>>;

        #[method(componentVersion)]
        pub unsafe fn componentVersion(&self) -> u32;

        #[method(allocateRenderResourcesAndReturnError:_)]
        pub unsafe fn allocateRenderResourcesAndReturnError(&self)
            -> Result<(), Retained<NSError>>;

        #[method(deallocateRenderResources)]
        pub unsafe fn deallocateRenderResources(&self);

        #[method(renderResourcesAllocated)]
        pub unsafe fn renderResourcesAllocated(&self) -> bool;

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method_id(@__retain_semantics Other inputBusses)]
        pub unsafe fn inputBusses(&self) -> Retained<AUAudioUnitBusArray>;

        #[method_id(@__retain_semantics Other outputBusses)]
        pub unsafe fn outputBusses(&self) -> Retained<AUAudioUnitBusArray>;

        #[cfg(all(
            feature = "AUComponent",
            feature = "block2",
            feature = "objc2-core-audio-types"
        ))]
        #[method(renderBlock)]
        pub unsafe fn renderBlock(&self) -> AURenderBlock;

        #[cfg(all(
            feature = "AUParameters",
            feature = "AudioUnitProperties",
            feature = "block2"
        ))]
        #[method(scheduleParameterBlock)]
        pub unsafe fn scheduleParameterBlock(&self) -> AUScheduleParameterBlock;

        #[cfg(all(
            feature = "AUComponent",
            feature = "block2",
            feature = "objc2-core-audio-types"
        ))]
        #[method(tokenByAddingRenderObserver:)]
        pub unsafe fn tokenByAddingRenderObserver(&self, observer: AURenderObserver) -> NSInteger;

        #[method(removeRenderObserver:)]
        pub unsafe fn removeRenderObserver(&self, token: NSInteger);

        #[method(maximumFramesToRender)]
        pub unsafe fn maximumFramesToRender(&self) -> AUAudioFrameCount;

        #[method(setMaximumFramesToRender:)]
        pub unsafe fn setMaximumFramesToRender(&self, maximum_frames_to_render: AUAudioFrameCount);

        #[cfg(feature = "AUParameters")]
        #[method_id(@__retain_semantics Other parameterTree)]
        pub unsafe fn parameterTree(&self) -> Option<Retained<AUParameterTree>>;

        #[cfg(feature = "AUParameters")]
        #[method(setParameterTree:)]
        pub unsafe fn setParameterTree(&self, parameter_tree: Option<&AUParameterTree>);

        #[method_id(@__retain_semantics Other parametersForOverviewWithCount:)]
        pub unsafe fn parametersForOverviewWithCount(
            &self,
            count: NSInteger,
        ) -> Retained<NSArray<NSNumber>>;

        #[method(allParameterValues)]
        pub unsafe fn allParameterValues(&self) -> bool;

        #[method(isMusicDeviceOrEffect)]
        pub unsafe fn isMusicDeviceOrEffect(&self) -> bool;

        #[method(virtualMIDICableCount)]
        pub unsafe fn virtualMIDICableCount(&self) -> NSInteger;

        #[cfg(all(feature = "AudioUnitProperties", feature = "block2"))]
        #[method(scheduleMIDIEventBlock)]
        pub unsafe fn scheduleMIDIEventBlock(&self) -> AUScheduleMIDIEventBlock;

        #[method_id(@__retain_semantics Other MIDIOutputNames)]
        pub unsafe fn MIDIOutputNames(&self) -> Retained<NSArray<NSString>>;

        #[method(providesUserInterface)]
        pub unsafe fn providesUserInterface(&self) -> bool;

        #[cfg(all(feature = "AudioUnitProperties", feature = "block2"))]
        #[method(MIDIOutputEventBlock)]
        pub unsafe fn MIDIOutputEventBlock(&self) -> AUMIDIOutputEventBlock;

        #[cfg(all(feature = "AudioUnitProperties", feature = "block2"))]
        #[method(setMIDIOutputEventBlock:)]
        pub unsafe fn setMIDIOutputEventBlock(
            &self,
            midi_output_event_block: AUMIDIOutputEventBlock,
        );

        #[method_id(@__retain_semantics Other fullState)]
        pub unsafe fn fullState(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(setFullState:)]
        pub unsafe fn setFullState(&self, full_state: Option<&NSDictionary<NSString, AnyObject>>);

        #[method_id(@__retain_semantics Other fullStateForDocument)]
        pub unsafe fn fullStateForDocument(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(setFullStateForDocument:)]
        pub unsafe fn setFullStateForDocument(
            &self,
            full_state_for_document: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[method_id(@__retain_semantics Other factoryPresets)]
        pub unsafe fn factoryPresets(&self) -> Option<Retained<NSArray<AUAudioUnitPreset>>>;

        #[method_id(@__retain_semantics Other userPresets)]
        pub unsafe fn userPresets(&self) -> Retained<NSArray<AUAudioUnitPreset>>;

        #[method(saveUserPreset:error:_)]
        pub unsafe fn saveUserPreset_error(
            &self,
            user_preset: &AUAudioUnitPreset,
        ) -> Result<(), Retained<NSError>>;

        #[method(deleteUserPreset:error:_)]
        pub unsafe fn deleteUserPreset_error(
            &self,
            user_preset: &AUAudioUnitPreset,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other presetStateFor:error:_)]
        pub unsafe fn presetStateFor_error(
            &self,
            user_preset: &AUAudioUnitPreset,
        ) -> Result<Retained<NSDictionary<NSString, AnyObject>>, Retained<NSError>>;

        #[method(supportsUserPresets)]
        pub unsafe fn supportsUserPresets(&self) -> bool;

        #[method(isLoadedInProcess)]
        pub unsafe fn isLoadedInProcess(&self) -> bool;

        #[method_id(@__retain_semantics Other currentPreset)]
        pub unsafe fn currentPreset(&self) -> Option<Retained<AUAudioUnitPreset>>;

        #[method(setCurrentPreset:)]
        pub unsafe fn setCurrentPreset(&self, current_preset: Option<&AUAudioUnitPreset>);

        #[method(latency)]
        pub unsafe fn latency(&self) -> NSTimeInterval;

        #[method(tailTime)]
        pub unsafe fn tailTime(&self) -> NSTimeInterval;

        #[method(renderQuality)]
        pub unsafe fn renderQuality(&self) -> NSInteger;

        #[method(setRenderQuality:)]
        pub unsafe fn setRenderQuality(&self, render_quality: NSInteger);

        #[method(shouldBypassEffect)]
        pub unsafe fn shouldBypassEffect(&self) -> bool;

        #[method(setShouldBypassEffect:)]
        pub unsafe fn setShouldBypassEffect(&self, should_bypass_effect: bool);

        #[method(canProcessInPlace)]
        pub unsafe fn canProcessInPlace(&self) -> bool;

        #[method(isRenderingOffline)]
        pub unsafe fn isRenderingOffline(&self) -> bool;

        #[method(setRenderingOffline:)]
        pub unsafe fn setRenderingOffline(&self, rendering_offline: bool);

        #[method_id(@__retain_semantics Other channelCapabilities)]
        pub unsafe fn channelCapabilities(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[cfg(feature = "block2")]
        #[method(musicalContextBlock)]
        pub unsafe fn musicalContextBlock(&self) -> AUHostMusicalContextBlock;

        #[cfg(feature = "block2")]
        #[method(setMusicalContextBlock:)]
        pub unsafe fn setMusicalContextBlock(
            &self,
            musical_context_block: AUHostMusicalContextBlock,
        );

        #[cfg(feature = "block2")]
        #[method(transportStateBlock)]
        pub unsafe fn transportStateBlock(&self) -> AUHostTransportStateBlock;

        #[cfg(feature = "block2")]
        #[method(setTransportStateBlock:)]
        pub unsafe fn setTransportStateBlock(
            &self,
            transport_state_block: AUHostTransportStateBlock,
        );

        #[method_id(@__retain_semantics Other contextName)]
        pub unsafe fn contextName(&self) -> Option<Retained<NSString>>;

        #[method(setContextName:)]
        pub unsafe fn setContextName(&self, context_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other migrateFromPlugin)]
        pub unsafe fn migrateFromPlugin(&self) -> Retained<NSArray>;

        #[method(supportsMPE)]
        pub unsafe fn supportsMPE(&self) -> bool;

        #[method_id(@__retain_semantics Other channelMap)]
        pub unsafe fn channelMap(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[method(setChannelMap:)]
        pub unsafe fn setChannelMap(&self, channel_map: Option<&NSArray<NSNumber>>);

        #[method_id(@__retain_semantics Other messageChannelFor:)]
        pub unsafe fn messageChannelFor(
            &self,
            channel_name: &NSString,
        ) -> Retained<ProtocolObject<dyn AUMessageChannel>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AUAudioUnit {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auinputhandler?language=objc)
#[cfg(all(
    feature = "AUComponent",
    feature = "block2",
    feature = "objc2-core-audio-types"
))]
pub type AUInputHandler = *mut block2::Block<
    dyn Fn(
        NonNull<AudioUnitRenderActionFlags>,
        NonNull<AudioTimeStamp>,
        AUAudioFrameCount,
        NSInteger,
    ),
>;

extern_methods!(
    /// AUAudioInputOutputUnit
    unsafe impl AUAudioUnit {
        #[method(canPerformInput)]
        pub unsafe fn canPerformInput(&self) -> bool;

        #[method(canPerformOutput)]
        pub unsafe fn canPerformOutput(&self) -> bool;

        #[method(isInputEnabled)]
        pub unsafe fn isInputEnabled(&self) -> bool;

        #[method(setInputEnabled:)]
        pub unsafe fn setInputEnabled(&self, input_enabled: bool);

        #[method(isOutputEnabled)]
        pub unsafe fn isOutputEnabled(&self) -> bool;

        #[method(setOutputEnabled:)]
        pub unsafe fn setOutputEnabled(&self, output_enabled: bool);

        #[cfg(all(
            feature = "AUComponent",
            feature = "block2",
            feature = "objc2-core-audio-types"
        ))]
        #[method(outputProvider)]
        pub unsafe fn outputProvider(&self) -> AURenderPullInputBlock;

        #[cfg(all(
            feature = "AUComponent",
            feature = "block2",
            feature = "objc2-core-audio-types"
        ))]
        #[method(setOutputProvider:)]
        pub unsafe fn setOutputProvider(&self, output_provider: AURenderPullInputBlock);

        #[cfg(all(
            feature = "AUComponent",
            feature = "block2",
            feature = "objc2-core-audio-types"
        ))]
        #[method(inputHandler)]
        pub unsafe fn inputHandler(&self) -> AUInputHandler;

        #[cfg(all(
            feature = "AUComponent",
            feature = "block2",
            feature = "objc2-core-audio-types"
        ))]
        #[method(setInputHandler:)]
        pub unsafe fn setInputHandler(&self, input_handler: AUInputHandler);

        #[method(deviceID)]
        pub unsafe fn deviceID(&self) -> AUAudioObjectID;

        #[method(setDeviceID:error:_)]
        pub unsafe fn setDeviceID_error(
            &self,
            device_id: AUAudioObjectID,
        ) -> Result<(), Retained<NSError>>;

        #[method(deviceInputLatency)]
        pub unsafe fn deviceInputLatency(&self) -> NSTimeInterval;

        #[method(deviceOutputLatency)]
        pub unsafe fn deviceOutputLatency(&self) -> NSTimeInterval;

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(startHardwareAndReturnError:_)]
        pub unsafe fn startHardwareAndReturnError(&self) -> Result<(), Retained<NSError>>;

        #[method(stopHardware)]
        pub unsafe fn stopHardware(&self);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudiounitbusarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AUAudioUnitBusArray;
);

unsafe impl NSFastEnumeration for AUAudioUnitBusArray {}

unsafe impl NSObjectProtocol for AUAudioUnitBusArray {}

extern_methods!(
    unsafe impl AUAudioUnitBusArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAudioUnit:busType:busses:)]
        pub unsafe fn initWithAudioUnit_busType_busses(
            this: Allocated<Self>,
            owner: &AUAudioUnit,
            bus_type: AUAudioUnitBusType,
            bus_array: &NSArray<AUAudioUnitBus>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAudioUnit:busType:)]
        pub unsafe fn initWithAudioUnit_busType(
            this: Allocated<Self>,
            owner: &AUAudioUnit,
            bus_type: AUAudioUnitBusType,
        ) -> Retained<Self>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Retained<AUAudioUnitBus>;

        #[method(isCountChangeable)]
        pub unsafe fn isCountChangeable(&self) -> bool;

        #[method(setBusCount:error:_)]
        pub unsafe fn setBusCount_error(&self, count: NSUInteger) -> Result<(), Retained<NSError>>;

        #[method(addObserverToAllBusses:forKeyPath:options:context:)]
        pub unsafe fn addObserverToAllBusses_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[method(removeObserverFromAllBusses:forKeyPath:context:)]
        pub unsafe fn removeObserverFromAllBusses_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[method_id(@__retain_semantics Other ownerAudioUnit)]
        pub unsafe fn ownerAudioUnit(&self) -> Retained<AUAudioUnit>;

        #[method(busType)]
        pub unsafe fn busType(&self) -> AUAudioUnitBusType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AUAudioUnitBusArray {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudiounitbus?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AUAudioUnitBus;
);

unsafe impl NSObjectProtocol for AUAudioUnitBus {}

extern_methods!(
    unsafe impl AUAudioUnitBus {
        #[method(shouldAllocateBuffer)]
        pub unsafe fn shouldAllocateBuffer(&self) -> bool;

        #[method(setShouldAllocateBuffer:)]
        pub unsafe fn setShouldAllocateBuffer(&self, should_allocate_buffer: bool);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(index)]
        pub unsafe fn index(&self) -> NSUInteger;

        #[method(busType)]
        pub unsafe fn busType(&self) -> AUAudioUnitBusType;

        #[method_id(@__retain_semantics Other ownerAudioUnit)]
        pub unsafe fn ownerAudioUnit(&self) -> Retained<AUAudioUnit>;

        #[method_id(@__retain_semantics Other supportedChannelLayoutTags)]
        pub unsafe fn supportedChannelLayoutTags(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[method(contextPresentationLatency)]
        pub unsafe fn contextPresentationLatency(&self) -> NSTimeInterval;

        #[method(setContextPresentationLatency:)]
        pub unsafe fn setContextPresentationLatency(
            &self,
            context_presentation_latency: NSTimeInterval,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AUAudioUnitBus {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auaudiounitpreset?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AUAudioUnitPreset;
);

unsafe impl NSCoding for AUAudioUnitPreset {}

unsafe impl NSObjectProtocol for AUAudioUnitPreset {}

unsafe impl NSSecureCoding for AUAudioUnitPreset {}

extern_methods!(
    unsafe impl AUAudioUnitPreset {
        #[method(number)]
        pub unsafe fn number(&self) -> NSInteger;

        #[method(setNumber:)]
        pub unsafe fn setNumber(&self, number: NSInteger);

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AUAudioUnitPreset {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/callhostblock?language=objc)
#[cfg(feature = "block2")]
pub type CallHostBlock = *mut block2::Block<dyn Fn(NonNull<NSDictionary>) -> NonNull<NSDictionary>>;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aumessagechannel?language=objc)
    pub unsafe trait AUMessageChannel {
        #[optional]
        #[method_id(@__retain_semantics Other callAudioUnit:)]
        unsafe fn callAudioUnit(&self, message: &NSDictionary) -> Retained<NSDictionary>;

        #[cfg(feature = "block2")]
        #[optional]
        #[method(callHostBlock)]
        unsafe fn callHostBlock(&self) -> CallHostBlock;

        #[cfg(feature = "block2")]
        #[optional]
        #[method(setCallHostBlock:)]
        unsafe fn setCallHostBlock(&self, call_host_block: CallHostBlock);
    }

    unsafe impl ProtocolType for dyn AUMessageChannel {}
);
