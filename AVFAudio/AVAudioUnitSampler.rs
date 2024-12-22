//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-audio-toolbox")]
#[cfg(not(target_os = "watchos"))]
use objc2_audio_toolbox::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Apple's sampler audio unit.
    ///
    /// An AVAudioUnit for Apple's Sampler Audio Unit. The sampler can be configured by loading
    /// instruments from different types of files such as an aupreset, a DLS or SF2 sound bank,
    /// an EXS24 instrument, a single audio file, or an array of audio files.
    ///
    /// The output is a single stereo bus.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounitsampler?language=objc)
    #[unsafe(super(AVAudioUnitMIDIInstrument, AVAudioUnit, AVAudioNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AVAudioNode",
        feature = "AVAudioUnit",
        feature = "AVAudioUnitMIDIInstrument"
    ))]
    pub struct AVAudioUnitSampler;
);

#[cfg(all(
    feature = "AVAudioMixing",
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitMIDIInstrument"
))]
unsafe impl AVAudio3DMixing for AVAudioUnitSampler {}

#[cfg(all(
    feature = "AVAudioMixing",
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitMIDIInstrument"
))]
unsafe impl AVAudioMixing for AVAudioUnitSampler {}

#[cfg(all(
    feature = "AVAudioMixing",
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitMIDIInstrument"
))]
unsafe impl AVAudioStereoMixing for AVAudioUnitSampler {}

#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitMIDIInstrument"
))]
unsafe impl NSObjectProtocol for AVAudioUnitSampler {}

extern_methods!(
    #[cfg(all(
        feature = "AVAudioNode",
        feature = "AVAudioUnit",
        feature = "AVAudioUnitMIDIInstrument"
    ))]
    unsafe impl AVAudioUnitSampler {
        /// loads a specific instrument from the specified sound bank
        ///
        /// Parameter `bankURL`: URL for a Soundbank file. The file can be either a DLS bank (.dls) or a SoundFont bank (.sf2).
        ///
        /// Parameter `program`: program number for the instrument to load
        ///
        /// Parameter `bankMSB`: MSB for the bank number for the instrument to load.  This is usually 0x79 for melodic
        /// instruments and 0x78 for percussion instruments.
        ///
        /// Parameter `bankLSB`: LSB for the bank number for the instrument to load.  This is often 0, and represents the "bank variation".
        ///
        /// Parameter `outError`: the status of the operation
        ///
        /// This method reads from file and allocates memory, so it should not be called on a real time thread.
        #[method(loadSoundBankInstrumentAtURL:program:bankMSB:bankLSB:error:_)]
        pub unsafe fn loadSoundBankInstrumentAtURL_program_bankMSB_bankLSB_error(
            &self,
            bank_url: &NSURL,
            program: u8,
            bank_msb: u8,
            bank_lsb: u8,
        ) -> Result<(), Retained<NSError>>;

        /// configures the sampler by loading the specified preset file.
        ///
        /// Parameter `instrumentURL`: URL to the preset file or audio file
        ///
        /// Parameter `outError`: the status of the operation
        ///
        /// The file can be of one of the following types: Logic/GarageBand EXS24 instrument,
        /// the Sampler AU's native aupreset, or an audio file (eg. .caf, .aiff, .wav, .mp3).
        ///
        /// If an audio file URL is loaded, it will become the sole sample in a new default instrument.
        /// Any information contained in the file regarding its keyboard placement (e.g. root key,
        /// key range) will be used.
        /// This method reads from file and allocates memory, so it should not be called on a real time thread.
        #[method(loadInstrumentAtURL:error:_)]
        pub unsafe fn loadInstrumentAtURL_error(
            &self,
            instrument_url: &NSURL,
        ) -> Result<(), Retained<NSError>>;

        /// configures the sampler by loading a set of audio files.
        ///
        /// Parameter `audioFiles`: array of URLs for audio files to be loaded
        ///
        /// Parameter `outError`: the status of the operation
        ///
        /// The audio files are loaded into a new default instrument with each audio file placed
        /// into its own sampler zone. Any information contained in the audio file regarding
        /// their placement on the keyboard (e.g. root key, key range) will be used.
        /// This method reads from file and allocates memory, so it should not be called on a real time thread.
        #[method(loadAudioFilesAtURLs:error:_)]
        pub unsafe fn loadAudioFilesAtURLs_error(
            &self,
            audio_files: &NSArray<NSURL>,
        ) -> Result<(), Retained<NSError>>;

        /// adjusts the pan for all the notes played.
        /// Range:     -100 -> +100
        /// Default:   0
        #[method(stereoPan)]
        pub unsafe fn stereoPan(&self) -> c_float;

        /// Setter for [`stereoPan`][Self::stereoPan].
        #[method(setStereoPan:)]
        pub unsafe fn setStereoPan(&self, stereo_pan: c_float);

        /// adjusts the gain of all the notes played
        /// Range:     -90.0 -> +12 db
        /// Default: 0 db
        #[method(overallGain)]
        pub unsafe fn overallGain(&self) -> c_float;

        /// Setter for [`overallGain`][Self::overallGain].
        #[method(setOverallGain:)]
        pub unsafe fn setOverallGain(&self, overall_gain: c_float);

        /// adjusts the gain of all the notes played
        /// Range:     -90.0 -> +12 db
        /// Default: 0 db
        #[deprecated]
        #[method(masterGain)]
        pub unsafe fn masterGain(&self) -> c_float;

        /// Setter for [`masterGain`][Self::masterGain].
        #[deprecated]
        #[method(setMasterGain:)]
        pub unsafe fn setMasterGain(&self, master_gain: c_float);

        /// adjusts the tuning of all the notes played.
        /// Range:     -2400 -> +2400 cents
        /// Default:   0
        #[method(globalTuning)]
        pub unsafe fn globalTuning(&self) -> c_float;

        /// Setter for [`globalTuning`][Self::globalTuning].
        #[method(setGlobalTuning:)]
        pub unsafe fn setGlobalTuning(&self, global_tuning: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAudioUnitMIDIInstrument`
    #[cfg(all(
        feature = "AVAudioNode",
        feature = "AVAudioUnit",
        feature = "AVAudioUnitMIDIInstrument"
    ))]
    unsafe impl AVAudioUnitSampler {
        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        /// initialize the node with the component description
        ///
        /// Parameter `description`: audio component description structure that describes the audio component of type kAudioUnitType_MusicDevice
        /// or kAudioUnitType_RemoteInstrument.
        #[method_id(@__retain_semantics Init initWithAudioComponentDescription:)]
        pub unsafe fn initWithAudioComponentDescription(
            this: Allocated<Self>,
            description: AudioComponentDescription,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AVAudioNode",
        feature = "AVAudioUnit",
        feature = "AVAudioUnitMIDIInstrument"
    ))]
    unsafe impl AVAudioUnitSampler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
