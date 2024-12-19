//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmusicsequenceloadoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVMusicSequenceLoadOptions(pub NSUInteger);
bitflags::bitflags! {
    impl AVMusicSequenceLoadOptions: NSUInteger {
        const AVMusicSequenceLoadSMF_PreserveTracks = 0;
        const AVMusicSequenceLoadSMF_ChannelsToTracks = 1<<0;
    }
}

unsafe impl Encode for AVMusicSequenceLoadOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVMusicSequenceLoadOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/_avbeatrange?language=objc)
#[cfg(feature = "AVAudioTypes")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _AVBeatRange {
    pub start: AVMusicTimeStamp,
    pub length: AVMusicTimeStamp,
}

#[cfg(feature = "AVAudioTypes")]
unsafe impl Encode for _AVBeatRange {
    const ENCODING: Encoding = Encoding::Struct(
        "_AVBeatRange",
        &[<AVMusicTimeStamp>::ENCODING, <AVMusicTimeStamp>::ENCODING],
    );
}

#[cfg(feature = "AVAudioTypes")]
unsafe impl RefEncode for _AVBeatRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avbeatrange?language=objc)
#[cfg(feature = "AVAudioTypes")]
pub type AVBeatRange = _AVBeatRange;

// TODO: pub fn AVMakeBeatRange(start_beat: AVMusicTimeStamp,length_in_beats: AVMusicTimeStamp,) -> AVBeatRange;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykey?language=objc)
// NS_TYPED_ENUM
pub type AVAudioSequencerInfoDictionaryKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeyalbum?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyAlbum: &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeyapproximatedurationinseconds?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyApproximateDurationInSeconds:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeyartist?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyArtist: &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeychannellayout?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyChannelLayout:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeycomments?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyComments:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeycomposer?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyComposer:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeycopyright?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyCopyright:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeyencodingapplication?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyEncodingApplication:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeygenre?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyGenre: &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeyisrc?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyISRC: &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeykeysignature?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyKeySignature:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeylyricist?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyLyricist:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeynominalbitrate?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyNominalBitRate:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeyrecordeddate?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyRecordedDate:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeysourcebitdepth?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeySourceBitDepth:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeysourceencoder?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeySourceEncoder:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeysubtitle?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeySubTitle:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeytempo?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyTempo: &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeytimesignature?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyTimeSignature:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeytitle?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyTitle: &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeytracknumber?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyTrackNumber:
        &'static AVAudioSequencerInfoDictionaryKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerinfodictionarykeyyear?language=objc)
    pub static AVAudioSequencerInfoDictionaryKeyYear: &'static AVAudioSequencerInfoDictionaryKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencerusercallback?language=objc)
#[cfg(all(feature = "AVAudioTypes", feature = "block2"))]
pub type AVAudioSequencerUserCallback =
    *mut block2::Block<dyn Fn(NonNull<AVMusicTrack>, NonNull<NSData>, AVMusicTimeStamp)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosequencer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSequencer;
);

unsafe impl NSObjectProtocol for AVAudioSequencer {}

extern_methods!(
    unsafe impl AVAudioSequencer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "AVAudioEngine")]
        #[method_id(@__retain_semantics Init initWithAudioEngine:)]
        pub unsafe fn initWithAudioEngine(
            this: Allocated<Self>,
            engine: &AVAudioEngine,
        ) -> Retained<Self>;

        #[method(loadFromURL:options:error:_)]
        pub unsafe fn loadFromURL_options_error(
            &self,
            file_url: &NSURL,
            options: AVMusicSequenceLoadOptions,
        ) -> Result<(), Retained<NSError>>;

        #[method(loadFromData:options:error:_)]
        pub unsafe fn loadFromData_options_error(
            &self,
            data: &NSData,
            options: AVMusicSequenceLoadOptions,
        ) -> Result<(), Retained<NSError>>;

        #[method(writeToURL:SMPTEResolution:replaceExisting:error:_)]
        pub unsafe fn writeToURL_SMPTEResolution_replaceExisting_error(
            &self,
            file_url: &NSURL,
            resolution: NSInteger,
            replace: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioTypes")]
        #[method(secondsForBeats:)]
        pub unsafe fn secondsForBeats(&self, beats: AVMusicTimeStamp) -> NSTimeInterval;

        #[cfg(feature = "AVAudioTypes")]
        #[method(beatsForSeconds:)]
        pub unsafe fn beatsForSeconds(&self, seconds: NSTimeInterval) -> AVMusicTimeStamp;

        #[method(reverseEvents)]
        pub unsafe fn reverseEvents(&self);

        #[method_id(@__retain_semantics Other createAndAppendTrack)]
        pub unsafe fn createAndAppendTrack(&self) -> Retained<AVMusicTrack>;

        #[method(removeTrack:)]
        pub unsafe fn removeTrack(&self, track: &AVMusicTrack) -> bool;

        #[cfg(all(feature = "AVAudioTypes", feature = "block2"))]
        #[method(setUserCallback:)]
        pub unsafe fn setUserCallback(&self, user_callback: AVAudioSequencerUserCallback);

        #[method_id(@__retain_semantics Other tracks)]
        pub unsafe fn tracks(&self) -> Retained<NSArray<AVMusicTrack>>;

        #[method_id(@__retain_semantics Other tempoTrack)]
        pub unsafe fn tempoTrack(&self) -> Retained<AVMusicTrack>;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Retained<NSDictionary<NSString, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSequencer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVAudioSequencer_Player
    unsafe impl AVAudioSequencer {
        #[method(currentPositionInSeconds)]
        pub unsafe fn currentPositionInSeconds(&self) -> NSTimeInterval;

        #[method(setCurrentPositionInSeconds:)]
        pub unsafe fn setCurrentPositionInSeconds(
            &self,
            current_position_in_seconds: NSTimeInterval,
        );

        #[method(currentPositionInBeats)]
        pub unsafe fn currentPositionInBeats(&self) -> NSTimeInterval;

        #[method(setCurrentPositionInBeats:)]
        pub unsafe fn setCurrentPositionInBeats(&self, current_position_in_beats: NSTimeInterval);

        #[method(isPlaying)]
        pub unsafe fn isPlaying(&self) -> bool;

        #[method(rate)]
        pub unsafe fn rate(&self) -> c_float;

        #[method(setRate:)]
        pub unsafe fn setRate(&self, rate: c_float);

        #[method(prepareToPlay)]
        pub unsafe fn prepareToPlay(&self);

        #[method(startAndReturnError:_)]
        pub unsafe fn startAndReturnError(&self) -> Result<(), Retained<NSError>>;

        #[method(stop)]
        pub unsafe fn stop(&self);
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmusictrackloopcount?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVMusicTrackLoopCount(pub NSInteger);
impl AVMusicTrackLoopCount {
    #[doc(alias = "AVMusicTrackLoopCountForever")]
    pub const Forever: Self = Self(-1);
}

unsafe impl Encode for AVMusicTrackLoopCount {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVMusicTrackLoopCount {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmusictrack?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMusicTrack;
);

unsafe impl NSObjectProtocol for AVMusicTrack {}

extern_methods!(
    unsafe impl AVMusicTrack {
        #[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
        #[method_id(@__retain_semantics Other destinationAudioUnit)]
        pub unsafe fn destinationAudioUnit(&self) -> Option<Retained<AVAudioUnit>>;

        #[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
        #[method(setDestinationAudioUnit:)]
        pub unsafe fn setDestinationAudioUnit(&self, destination_audio_unit: Option<&AVAudioUnit>);

        #[cfg(feature = "AVAudioTypes")]
        #[method(loopRange)]
        pub unsafe fn loopRange(&self) -> AVBeatRange;

        #[cfg(feature = "AVAudioTypes")]
        #[method(setLoopRange:)]
        pub unsafe fn setLoopRange(&self, loop_range: AVBeatRange);

        #[method(isLoopingEnabled)]
        pub unsafe fn isLoopingEnabled(&self) -> bool;

        #[method(setLoopingEnabled:)]
        pub unsafe fn setLoopingEnabled(&self, looping_enabled: bool);

        #[method(numberOfLoops)]
        pub unsafe fn numberOfLoops(&self) -> NSInteger;

        #[method(setNumberOfLoops:)]
        pub unsafe fn setNumberOfLoops(&self, number_of_loops: NSInteger);

        #[cfg(feature = "AVAudioTypes")]
        #[method(offsetTime)]
        pub unsafe fn offsetTime(&self) -> AVMusicTimeStamp;

        #[cfg(feature = "AVAudioTypes")]
        #[method(setOffsetTime:)]
        pub unsafe fn setOffsetTime(&self, offset_time: AVMusicTimeStamp);

        #[method(isMuted)]
        pub unsafe fn isMuted(&self) -> bool;

        #[method(setMuted:)]
        pub unsafe fn setMuted(&self, muted: bool);

        #[method(isSoloed)]
        pub unsafe fn isSoloed(&self) -> bool;

        #[method(setSoloed:)]
        pub unsafe fn setSoloed(&self, soloed: bool);

        #[cfg(feature = "AVAudioTypes")]
        #[method(lengthInBeats)]
        pub unsafe fn lengthInBeats(&self) -> AVMusicTimeStamp;

        #[cfg(feature = "AVAudioTypes")]
        #[method(setLengthInBeats:)]
        pub unsafe fn setLengthInBeats(&self, length_in_beats: AVMusicTimeStamp);

        #[method(lengthInSeconds)]
        pub unsafe fn lengthInSeconds(&self) -> NSTimeInterval;

        #[method(setLengthInSeconds:)]
        pub unsafe fn setLengthInSeconds(&self, length_in_seconds: NSTimeInterval);

        #[method(timeResolution)]
        pub unsafe fn timeResolution(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMusicTrack {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avmusiceventenumerationblock?language=objc)
#[cfg(all(
    feature = "AVAudioTypes",
    feature = "AVMusicEvents",
    feature = "block2"
))]
pub type AVMusicEventEnumerationBlock =
    *mut block2::Block<dyn Fn(NonNull<AVMusicEvent>, NonNull<AVMusicTimeStamp>, NonNull<Bool>)>;

extern_methods!(
    /// AVMusicTrackEditor
    unsafe impl AVMusicTrack {
        #[method(usesAutomatedParameters)]
        pub unsafe fn usesAutomatedParameters(&self) -> bool;

        #[method(setUsesAutomatedParameters:)]
        pub unsafe fn setUsesAutomatedParameters(&self, uses_automated_parameters: bool);

        #[cfg(all(feature = "AVAudioTypes", feature = "AVMusicEvents"))]
        #[method(addEvent:atBeat:)]
        pub unsafe fn addEvent_atBeat(&self, event: &AVMusicEvent, beat: AVMusicTimeStamp);

        #[cfg(feature = "AVAudioTypes")]
        #[method(moveEventsInRange:byAmount:)]
        pub unsafe fn moveEventsInRange_byAmount(
            &self,
            range: AVBeatRange,
            beat_amount: AVMusicTimeStamp,
        );

        #[cfg(feature = "AVAudioTypes")]
        #[method(clearEventsInRange:)]
        pub unsafe fn clearEventsInRange(&self, range: AVBeatRange);

        #[cfg(feature = "AVAudioTypes")]
        #[method(cutEventsInRange:)]
        pub unsafe fn cutEventsInRange(&self, range: AVBeatRange);

        #[cfg(feature = "AVAudioTypes")]
        #[method(copyEventsInRange:fromTrack:insertAtBeat:)]
        pub unsafe fn copyEventsInRange_fromTrack_insertAtBeat(
            &self,
            range: AVBeatRange,
            source_track: &AVMusicTrack,
            insert_start_beat: AVMusicTimeStamp,
        );

        #[cfg(feature = "AVAudioTypes")]
        #[method(copyAndMergeEventsInRange:fromTrack:mergeAtBeat:)]
        pub unsafe fn copyAndMergeEventsInRange_fromTrack_mergeAtBeat(
            &self,
            range: AVBeatRange,
            source_track: &AVMusicTrack,
            merge_start_beat: AVMusicTimeStamp,
        );

        #[cfg(all(
            feature = "AVAudioTypes",
            feature = "AVMusicEvents",
            feature = "block2"
        ))]
        #[method(enumerateEventsInRange:usingBlock:)]
        pub unsafe fn enumerateEventsInRange_usingBlock(
            &self,
            range: AVBeatRange,
            block: AVMusicEventEnumerationBlock,
        );
    }
);
