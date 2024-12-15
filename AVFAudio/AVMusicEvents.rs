//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmusicevent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMusicEvent;
);

unsafe impl NSObjectProtocol for AVMusicEvent {}

extern_methods!(
    unsafe impl AVMusicEvent {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMusicEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidinoteevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDINoteEvent;
);

unsafe impl NSObjectProtocol for AVMIDINoteEvent {}

extern_methods!(
    unsafe impl AVMIDINoteEvent {
        #[cfg(feature = "AVAudioTypes")]
        #[method_id(@__retain_semantics Init initWithChannel:key:velocity:duration:)]
        pub unsafe fn initWithChannel_key_velocity_duration(
            this: Allocated<Self>,
            channel: u32,
            key_num: u32,
            velocity: u32,
            duration: AVMusicTimeStamp,
        ) -> Retained<Self>;

        #[method(channel)]
        pub unsafe fn channel(&self) -> u32;

        #[method(setChannel:)]
        pub unsafe fn setChannel(&self, channel: u32);

        #[method(key)]
        pub unsafe fn key(&self) -> u32;

        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: u32);

        #[method(velocity)]
        pub unsafe fn velocity(&self) -> u32;

        #[method(setVelocity:)]
        pub unsafe fn setVelocity(&self, velocity: u32);

        #[cfg(feature = "AVAudioTypes")]
        #[method(duration)]
        pub unsafe fn duration(&self) -> AVMusicTimeStamp;

        #[cfg(feature = "AVAudioTypes")]
        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: AVMusicTimeStamp);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDINoteEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidichannelevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDIChannelEvent;
);

unsafe impl NSObjectProtocol for AVMIDIChannelEvent {}

extern_methods!(
    unsafe impl AVMIDIChannelEvent {
        #[method(channel)]
        pub unsafe fn channel(&self) -> u32;

        #[method(setChannel:)]
        pub unsafe fn setChannel(&self, channel: u32);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDIChannelEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidicontrolchangemessagetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVMIDIControlChangeMessageType(pub NSInteger);
impl AVMIDIControlChangeMessageType {
    #[doc(alias = "AVMIDIControlChangeMessageTypeBankSelect")]
    pub const BankSelect: Self = Self(0);
    #[doc(alias = "AVMIDIControlChangeMessageTypeModWheel")]
    pub const ModWheel: Self = Self(1);
    #[doc(alias = "AVMIDIControlChangeMessageTypeBreath")]
    pub const Breath: Self = Self(2);
    #[doc(alias = "AVMIDIControlChangeMessageTypeFoot")]
    pub const Foot: Self = Self(4);
    #[doc(alias = "AVMIDIControlChangeMessageTypePortamentoTime")]
    pub const PortamentoTime: Self = Self(5);
    #[doc(alias = "AVMIDIControlChangeMessageTypeDataEntry")]
    pub const DataEntry: Self = Self(6);
    #[doc(alias = "AVMIDIControlChangeMessageTypeVolume")]
    pub const Volume: Self = Self(7);
    #[doc(alias = "AVMIDIControlChangeMessageTypeBalance")]
    pub const Balance: Self = Self(8);
    #[doc(alias = "AVMIDIControlChangeMessageTypePan")]
    pub const Pan: Self = Self(10);
    #[doc(alias = "AVMIDIControlChangeMessageTypeExpression")]
    pub const Expression: Self = Self(11);
    #[doc(alias = "AVMIDIControlChangeMessageTypeSustain")]
    pub const Sustain: Self = Self(64);
    #[doc(alias = "AVMIDIControlChangeMessageTypePortamento")]
    pub const Portamento: Self = Self(65);
    #[doc(alias = "AVMIDIControlChangeMessageTypeSostenuto")]
    pub const Sostenuto: Self = Self(66);
    #[doc(alias = "AVMIDIControlChangeMessageTypeSoft")]
    pub const Soft: Self = Self(67);
    #[doc(alias = "AVMIDIControlChangeMessageTypeLegatoPedal")]
    pub const LegatoPedal: Self = Self(68);
    #[doc(alias = "AVMIDIControlChangeMessageTypeHold2Pedal")]
    pub const Hold2Pedal: Self = Self(69);
    #[doc(alias = "AVMIDIControlChangeMessageTypeFilterResonance")]
    pub const FilterResonance: Self = Self(71);
    #[doc(alias = "AVMIDIControlChangeMessageTypeReleaseTime")]
    pub const ReleaseTime: Self = Self(72);
    #[doc(alias = "AVMIDIControlChangeMessageTypeAttackTime")]
    pub const AttackTime: Self = Self(73);
    #[doc(alias = "AVMIDIControlChangeMessageTypeBrightness")]
    pub const Brightness: Self = Self(74);
    #[doc(alias = "AVMIDIControlChangeMessageTypeDecayTime")]
    pub const DecayTime: Self = Self(75);
    #[doc(alias = "AVMIDIControlChangeMessageTypeVibratoRate")]
    pub const VibratoRate: Self = Self(76);
    #[doc(alias = "AVMIDIControlChangeMessageTypeVibratoDepth")]
    pub const VibratoDepth: Self = Self(77);
    #[doc(alias = "AVMIDIControlChangeMessageTypeVibratoDelay")]
    pub const VibratoDelay: Self = Self(78);
    #[doc(alias = "AVMIDIControlChangeMessageTypeReverbLevel")]
    pub const ReverbLevel: Self = Self(91);
    #[doc(alias = "AVMIDIControlChangeMessageTypeChorusLevel")]
    pub const ChorusLevel: Self = Self(93);
    #[doc(alias = "AVMIDIControlChangeMessageTypeRPN_LSB")]
    pub const RPN_LSB: Self = Self(100);
    #[doc(alias = "AVMIDIControlChangeMessageTypeRPN_MSB")]
    pub const RPN_MSB: Self = Self(101);
    #[doc(alias = "AVMIDIControlChangeMessageTypeAllSoundOff")]
    pub const AllSoundOff: Self = Self(120);
    #[doc(alias = "AVMIDIControlChangeMessageTypeResetAllControllers")]
    pub const ResetAllControllers: Self = Self(121);
    #[doc(alias = "AVMIDIControlChangeMessageTypeAllNotesOff")]
    pub const AllNotesOff: Self = Self(123);
    #[doc(alias = "AVMIDIControlChangeMessageTypeOmniModeOff")]
    pub const OmniModeOff: Self = Self(124);
    #[doc(alias = "AVMIDIControlChangeMessageTypeOmniModeOn")]
    pub const OmniModeOn: Self = Self(125);
    #[doc(alias = "AVMIDIControlChangeMessageTypeMonoModeOn")]
    pub const MonoModeOn: Self = Self(126);
    #[doc(alias = "AVMIDIControlChangeMessageTypeMonoModeOff")]
    pub const MonoModeOff: Self = Self(127);
}

unsafe impl Encode for AVMIDIControlChangeMessageType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVMIDIControlChangeMessageType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidicontrolchangeevent?language=objc)
    #[unsafe(super(AVMIDIChannelEvent, AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDIControlChangeEvent;
);

unsafe impl NSObjectProtocol for AVMIDIControlChangeEvent {}

extern_methods!(
    unsafe impl AVMIDIControlChangeEvent {
        #[method_id(@__retain_semantics Init initWithChannel:messageType:value:)]
        pub unsafe fn initWithChannel_messageType_value(
            this: Allocated<Self>,
            channel: u32,
            message_type: AVMIDIControlChangeMessageType,
            value: u32,
        ) -> Retained<Self>;

        #[method(messageType)]
        pub unsafe fn messageType(&self) -> AVMIDIControlChangeMessageType;

        #[method(value)]
        pub unsafe fn value(&self) -> u32;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDIControlChangeEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidipolypressureevent?language=objc)
    #[unsafe(super(AVMIDIChannelEvent, AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDIPolyPressureEvent;
);

unsafe impl NSObjectProtocol for AVMIDIPolyPressureEvent {}

extern_methods!(
    unsafe impl AVMIDIPolyPressureEvent {
        #[method_id(@__retain_semantics Init initWithChannel:key:pressure:)]
        pub unsafe fn initWithChannel_key_pressure(
            this: Allocated<Self>,
            channel: u32,
            key: u32,
            pressure: u32,
        ) -> Retained<Self>;

        #[method(key)]
        pub unsafe fn key(&self) -> u32;

        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: u32);

        #[method(pressure)]
        pub unsafe fn pressure(&self) -> u32;

        #[method(setPressure:)]
        pub unsafe fn setPressure(&self, pressure: u32);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDIPolyPressureEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidiprogramchangeevent?language=objc)
    #[unsafe(super(AVMIDIChannelEvent, AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDIProgramChangeEvent;
);

unsafe impl NSObjectProtocol for AVMIDIProgramChangeEvent {}

extern_methods!(
    unsafe impl AVMIDIProgramChangeEvent {
        #[method_id(@__retain_semantics Init initWithChannel:programNumber:)]
        pub unsafe fn initWithChannel_programNumber(
            this: Allocated<Self>,
            channel: u32,
            program_number: u32,
        ) -> Retained<Self>;

        #[method(programNumber)]
        pub unsafe fn programNumber(&self) -> u32;

        #[method(setProgramNumber:)]
        pub unsafe fn setProgramNumber(&self, program_number: u32);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDIProgramChangeEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidichannelpressureevent?language=objc)
    #[unsafe(super(AVMIDIChannelEvent, AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDIChannelPressureEvent;
);

unsafe impl NSObjectProtocol for AVMIDIChannelPressureEvent {}

extern_methods!(
    unsafe impl AVMIDIChannelPressureEvent {
        #[method_id(@__retain_semantics Init initWithChannel:pressure:)]
        pub unsafe fn initWithChannel_pressure(
            this: Allocated<Self>,
            channel: u32,
            pressure: u32,
        ) -> Retained<Self>;

        #[method(pressure)]
        pub unsafe fn pressure(&self) -> u32;

        #[method(setPressure:)]
        pub unsafe fn setPressure(&self, pressure: u32);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDIChannelPressureEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidipitchbendevent?language=objc)
    #[unsafe(super(AVMIDIChannelEvent, AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDIPitchBendEvent;
);

unsafe impl NSObjectProtocol for AVMIDIPitchBendEvent {}

extern_methods!(
    unsafe impl AVMIDIPitchBendEvent {
        #[method_id(@__retain_semantics Init initWithChannel:value:)]
        pub unsafe fn initWithChannel_value(
            this: Allocated<Self>,
            channel: u32,
            value: u32,
        ) -> Retained<Self>;

        #[method(value)]
        pub unsafe fn value(&self) -> u32;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: u32);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDIPitchBendEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidisysexevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDISysexEvent;
);

unsafe impl NSObjectProtocol for AVMIDISysexEvent {}

extern_methods!(
    unsafe impl AVMIDISysexEvent {
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Retained<Self>;

        #[method(sizeInBytes)]
        pub unsafe fn sizeInBytes(&self) -> u32;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDISysexEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidimetaeventtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVMIDIMetaEventType(pub NSInteger);
impl AVMIDIMetaEventType {
    #[doc(alias = "AVMIDIMetaEventTypeSequenceNumber")]
    pub const SequenceNumber: Self = Self(0x00);
    #[doc(alias = "AVMIDIMetaEventTypeText")]
    pub const Text: Self = Self(0x01);
    #[doc(alias = "AVMIDIMetaEventTypeCopyright")]
    pub const Copyright: Self = Self(0x02);
    #[doc(alias = "AVMIDIMetaEventTypeTrackName")]
    pub const TrackName: Self = Self(0x03);
    #[doc(alias = "AVMIDIMetaEventTypeInstrument")]
    pub const Instrument: Self = Self(0x04);
    #[doc(alias = "AVMIDIMetaEventTypeLyric")]
    pub const Lyric: Self = Self(0x05);
    #[doc(alias = "AVMIDIMetaEventTypeMarker")]
    pub const Marker: Self = Self(0x06);
    #[doc(alias = "AVMIDIMetaEventTypeCuePoint")]
    pub const CuePoint: Self = Self(0x07);
    #[doc(alias = "AVMIDIMetaEventTypeMidiChannel")]
    pub const MidiChannel: Self = Self(0x20);
    #[doc(alias = "AVMIDIMetaEventTypeMidiPort")]
    pub const MidiPort: Self = Self(0x21);
    #[doc(alias = "AVMIDIMetaEventTypeEndOfTrack")]
    pub const EndOfTrack: Self = Self(0x2f);
    #[doc(alias = "AVMIDIMetaEventTypeTempo")]
    pub const Tempo: Self = Self(0x51);
    #[doc(alias = "AVMIDIMetaEventTypeSmpteOffset")]
    pub const SmpteOffset: Self = Self(0x54);
    #[doc(alias = "AVMIDIMetaEventTypeTimeSignature")]
    pub const TimeSignature: Self = Self(0x58);
    #[doc(alias = "AVMIDIMetaEventTypeKeySignature")]
    pub const KeySignature: Self = Self(0x59);
    #[doc(alias = "AVMIDIMetaEventTypeProprietaryEvent")]
    pub const ProprietaryEvent: Self = Self(0x7f);
}

unsafe impl Encode for AVMIDIMetaEventType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVMIDIMetaEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmidimetaevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMIDIMetaEvent;
);

unsafe impl NSObjectProtocol for AVMIDIMetaEvent {}

extern_methods!(
    unsafe impl AVMIDIMetaEvent {
        #[method_id(@__retain_semantics Init initWithType:data:)]
        pub unsafe fn initWithType_data(
            this: Allocated<Self>,
            r#type: AVMIDIMetaEventType,
            data: &NSData,
        ) -> Retained<Self>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> AVMIDIMetaEventType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMIDIMetaEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmusicuserevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMusicUserEvent;
);

unsafe impl NSObjectProtocol for AVMusicUserEvent {}

extern_methods!(
    unsafe impl AVMusicUserEvent {
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Retained<Self>;

        #[method(sizeInBytes)]
        pub unsafe fn sizeInBytes(&self) -> u32;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMusicUserEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avextendednoteoneventdefaultinstrument?language=objc)
    pub static AVExtendedNoteOnEventDefaultInstrument: u32;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avextendednoteonevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVExtendedNoteOnEvent;
);

unsafe impl NSObjectProtocol for AVExtendedNoteOnEvent {}

extern_methods!(
    unsafe impl AVExtendedNoteOnEvent {
        #[cfg(feature = "AVAudioTypes")]
        #[method_id(@__retain_semantics Init initWithMIDINote:velocity:groupID:duration:)]
        pub unsafe fn initWithMIDINote_velocity_groupID_duration(
            this: Allocated<Self>,
            midi_note: c_float,
            velocity: c_float,
            group_id: u32,
            duration: AVMusicTimeStamp,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAudioTypes")]
        #[method_id(@__retain_semantics Init initWithMIDINote:velocity:instrumentID:groupID:duration:)]
        pub unsafe fn initWithMIDINote_velocity_instrumentID_groupID_duration(
            this: Allocated<Self>,
            midi_note: c_float,
            velocity: c_float,
            instrument_id: u32,
            group_id: u32,
            duration: AVMusicTimeStamp,
        ) -> Retained<Self>;

        #[method(midiNote)]
        pub unsafe fn midiNote(&self) -> c_float;

        #[method(setMidiNote:)]
        pub unsafe fn setMidiNote(&self, midi_note: c_float);

        #[method(velocity)]
        pub unsafe fn velocity(&self) -> c_float;

        #[method(setVelocity:)]
        pub unsafe fn setVelocity(&self, velocity: c_float);

        #[method(instrumentID)]
        pub unsafe fn instrumentID(&self) -> u32;

        #[method(setInstrumentID:)]
        pub unsafe fn setInstrumentID(&self, instrument_id: u32);

        #[method(groupID)]
        pub unsafe fn groupID(&self) -> u32;

        #[method(setGroupID:)]
        pub unsafe fn setGroupID(&self, group_id: u32);

        #[cfg(feature = "AVAudioTypes")]
        #[method(duration)]
        pub unsafe fn duration(&self) -> AVMusicTimeStamp;

        #[cfg(feature = "AVAudioTypes")]
        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: AVMusicTimeStamp);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVExtendedNoteOnEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avparameterevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVParameterEvent;
);

unsafe impl NSObjectProtocol for AVParameterEvent {}

extern_methods!(
    unsafe impl AVParameterEvent {
        #[method_id(@__retain_semantics Init initWithParameterID:scope:element:value:)]
        pub unsafe fn initWithParameterID_scope_element_value(
            this: Allocated<Self>,
            parameter_id: u32,
            scope: u32,
            element: u32,
            value: c_float,
        ) -> Retained<Self>;

        #[method(parameterID)]
        pub unsafe fn parameterID(&self) -> u32;

        #[method(setParameterID:)]
        pub unsafe fn setParameterID(&self, parameter_id: u32);

        #[method(scope)]
        pub unsafe fn scope(&self) -> u32;

        #[method(setScope:)]
        pub unsafe fn setScope(&self, scope: u32);

        #[method(element)]
        pub unsafe fn element(&self) -> u32;

        #[method(setElement:)]
        pub unsafe fn setElement(&self, element: u32);

        #[method(value)]
        pub unsafe fn value(&self) -> c_float;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVParameterEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaupresetevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAUPresetEvent;
);

unsafe impl NSObjectProtocol for AVAUPresetEvent {}

extern_methods!(
    unsafe impl AVAUPresetEvent {
        #[method_id(@__retain_semantics Init initWithScope:element:dictionary:)]
        pub unsafe fn initWithScope_element_dictionary(
            this: Allocated<Self>,
            scope: u32,
            element: u32,
            preset_dictionary: &NSDictionary,
        ) -> Retained<Self>;

        #[method(scope)]
        pub unsafe fn scope(&self) -> u32;

        #[method(setScope:)]
        pub unsafe fn setScope(&self, scope: u32);

        #[method(element)]
        pub unsafe fn element(&self) -> u32;

        #[method(setElement:)]
        pub unsafe fn setElement(&self, element: u32);

        #[method_id(@__retain_semantics Other presetDictionary)]
        pub unsafe fn presetDictionary(&self) -> Retained<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAUPresetEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avextendedtempoevent?language=objc)
    #[unsafe(super(AVMusicEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVExtendedTempoEvent;
);

unsafe impl NSObjectProtocol for AVExtendedTempoEvent {}

extern_methods!(
    unsafe impl AVExtendedTempoEvent {
        #[method_id(@__retain_semantics Init initWithTempo:)]
        pub unsafe fn initWithTempo(this: Allocated<Self>, tempo: c_double) -> Retained<Self>;

        #[method(tempo)]
        pub unsafe fn tempo(&self) -> c_double;

        #[method(setTempo:)]
        pub unsafe fn setTempo(&self, tempo: c_double);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVExtendedTempoEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);