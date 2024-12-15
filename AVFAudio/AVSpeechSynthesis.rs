//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechboundary?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechBoundary(pub NSInteger);
impl AVSpeechBoundary {
    #[doc(alias = "AVSpeechBoundaryImmediate")]
    pub const Immediate: Self = Self(0);
    #[doc(alias = "AVSpeechBoundaryWord")]
    pub const Word: Self = Self(1);
}

unsafe impl Encode for AVSpeechBoundary {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechBoundary {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoicequality?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisVoiceQuality(pub NSInteger);
impl AVSpeechSynthesisVoiceQuality {
    #[doc(alias = "AVSpeechSynthesisVoiceQualityDefault")]
    pub const Default: Self = Self(1);
    #[doc(alias = "AVSpeechSynthesisVoiceQualityEnhanced")]
    pub const Enhanced: Self = Self(2);
    #[doc(alias = "AVSpeechSynthesisVoiceQualityPremium")]
    pub const Premium: Self = Self(3);
}

unsafe impl Encode for AVSpeechSynthesisVoiceQuality {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisVoiceQuality {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoicegender?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisVoiceGender(pub NSInteger);
impl AVSpeechSynthesisVoiceGender {
    #[doc(alias = "AVSpeechSynthesisVoiceGenderUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "AVSpeechSynthesisVoiceGenderMale")]
    pub const Male: Self = Self(1);
    #[doc(alias = "AVSpeechSynthesisVoiceGenderFemale")]
    pub const Female: Self = Self(2);
}

unsafe impl Encode for AVSpeechSynthesisVoiceGender {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisVoiceGender {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesismarkermark?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisMarkerMark(pub NSInteger);
impl AVSpeechSynthesisMarkerMark {
    #[doc(alias = "AVSpeechSynthesisMarkerMarkPhoneme")]
    pub const Phoneme: Self = Self(0);
    #[doc(alias = "AVSpeechSynthesisMarkerMarkWord")]
    pub const Word: Self = Self(1);
    #[doc(alias = "AVSpeechSynthesisMarkerMarkSentence")]
    pub const Sentence: Self = Self(2);
    #[doc(alias = "AVSpeechSynthesisMarkerMarkParagraph")]
    pub const Paragraph: Self = Self(3);
    #[doc(alias = "AVSpeechSynthesisMarkerMarkBookmark")]
    pub const Bookmark: Self = Self(4);
}

unsafe impl Encode for AVSpeechSynthesisMarkerMark {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisMarkerMark {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechutteranceminimumspeechrate?language=objc)
    pub static AVSpeechUtteranceMinimumSpeechRate: c_float;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechutterancemaximumspeechrate?language=objc)
    pub static AVSpeechUtteranceMaximumSpeechRate: c_float;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechutterancedefaultspeechrate?language=objc)
    pub static AVSpeechUtteranceDefaultSpeechRate: c_float;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoiceidentifieralex?language=objc)
    pub static AVSpeechSynthesisVoiceIdentifierAlex: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisipanotationattribute?language=objc)
    pub static AVSpeechSynthesisIPANotationAttribute: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesizerbuffercallback?language=objc)
#[cfg(all(feature = "AVAudioBuffer", feature = "block2"))]
pub type AVSpeechSynthesizerBufferCallback = *mut block2::Block<dyn Fn(NonNull<AVAudioBuffer>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesizermarkercallback?language=objc)
#[cfg(feature = "block2")]
pub type AVSpeechSynthesizerMarkerCallback =
    *mut block2::Block<dyn Fn(NonNull<NSArray<AVSpeechSynthesisMarker>>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesispersonalvoiceauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisPersonalVoiceAuthorizationStatus(pub NSUInteger);
impl AVSpeechSynthesisPersonalVoiceAuthorizationStatus {
    #[doc(alias = "AVSpeechSynthesisPersonalVoiceAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "AVSpeechSynthesisPersonalVoiceAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    #[doc(alias = "AVSpeechSynthesisPersonalVoiceAuthorizationStatusUnsupported")]
    pub const Unsupported: Self = Self(2);
    #[doc(alias = "AVSpeechSynthesisPersonalVoiceAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for AVSpeechSynthesisPersonalVoiceAuthorizationStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisPersonalVoiceAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoicetraits?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisVoiceTraits(pub NSUInteger);
bitflags::bitflags! {
    impl AVSpeechSynthesisVoiceTraits: NSUInteger {
        const AVSpeechSynthesisVoiceTraitNone = 0;
        const AVSpeechSynthesisVoiceTraitIsNoveltyVoice = 1<<0;
        const AVSpeechSynthesisVoiceTraitIsPersonalVoice = 1<<1;
    }
}

unsafe impl Encode for AVSpeechSynthesisVoiceTraits {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisVoiceTraits {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisavailablevoicesdidchangenotification?language=objc)
    pub static AVSpeechSynthesisAvailableVoicesDidChangeNotification: &'static NSNotificationName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSpeechSynthesisVoice;
);

unsafe impl Send for AVSpeechSynthesisVoice {}

unsafe impl Sync for AVSpeechSynthesisVoice {}

unsafe impl NSCoding for AVSpeechSynthesisVoice {}

unsafe impl NSObjectProtocol for AVSpeechSynthesisVoice {}

unsafe impl NSSecureCoding for AVSpeechSynthesisVoice {}

extern_methods!(
    unsafe impl AVSpeechSynthesisVoice {
        #[method_id(@__retain_semantics Other speechVoices)]
        pub unsafe fn speechVoices() -> Retained<NSArray<AVSpeechSynthesisVoice>>;

        #[method_id(@__retain_semantics Other currentLanguageCode)]
        pub unsafe fn currentLanguageCode() -> Retained<NSString>;

        #[method_id(@__retain_semantics Other voiceWithLanguage:)]
        pub unsafe fn voiceWithLanguage(
            language_code: Option<&NSString>,
        ) -> Option<Retained<AVSpeechSynthesisVoice>>;

        #[method_id(@__retain_semantics Other voiceWithIdentifier:)]
        pub unsafe fn voiceWithIdentifier(
            identifier: &NSString,
        ) -> Option<Retained<AVSpeechSynthesisVoice>>;

        #[method_id(@__retain_semantics Other language)]
        pub unsafe fn language(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(quality)]
        pub unsafe fn quality(&self) -> AVSpeechSynthesisVoiceQuality;

        #[method(gender)]
        pub unsafe fn gender(&self) -> AVSpeechSynthesisVoiceGender;

        #[method_id(@__retain_semantics Other audioFileSettings)]
        pub unsafe fn audioFileSettings(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[method(voiceTraits)]
        pub unsafe fn voiceTraits(&self) -> AVSpeechSynthesisVoiceTraits;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSpeechSynthesisVoice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechutterance?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSpeechUtterance;
);

unsafe impl NSCoding for AVSpeechUtterance {}

unsafe impl NSCopying for AVSpeechUtterance {}

unsafe impl CopyingHelper for AVSpeechUtterance {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVSpeechUtterance {}

unsafe impl NSSecureCoding for AVSpeechUtterance {}

extern_methods!(
    unsafe impl AVSpeechUtterance {
        #[method_id(@__retain_semantics Other speechUtteranceWithString:)]
        pub unsafe fn speechUtteranceWithString(string: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Other speechUtteranceWithAttributedString:)]
        pub unsafe fn speechUtteranceWithAttributedString(
            string: &NSAttributedString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other speechUtteranceWithSSMLRepresentation:)]
        pub unsafe fn speechUtteranceWithSSMLRepresentation(
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            this: Allocated<Self>,
            string: &NSAttributedString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSSMLRepresentation:)]
        pub unsafe fn initWithSSMLRepresentation(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other voice)]
        pub unsafe fn voice(&self) -> Option<Retained<AVSpeechSynthesisVoice>>;

        #[method(setVoice:)]
        pub unsafe fn setVoice(&self, voice: Option<&AVSpeechSynthesisVoice>);

        #[method_id(@__retain_semantics Other speechString)]
        pub unsafe fn speechString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other attributedSpeechString)]
        pub unsafe fn attributedSpeechString(&self) -> Retained<NSAttributedString>;

        #[method(rate)]
        pub unsafe fn rate(&self) -> c_float;

        #[method(setRate:)]
        pub unsafe fn setRate(&self, rate: c_float);

        #[method(pitchMultiplier)]
        pub unsafe fn pitchMultiplier(&self) -> c_float;

        #[method(setPitchMultiplier:)]
        pub unsafe fn setPitchMultiplier(&self, pitch_multiplier: c_float);

        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[method(prefersAssistiveTechnologySettings)]
        pub unsafe fn prefersAssistiveTechnologySettings(&self) -> bool;

        #[method(setPrefersAssistiveTechnologySettings:)]
        pub unsafe fn setPrefersAssistiveTechnologySettings(
            &self,
            prefers_assistive_technology_settings: bool,
        );

        #[method(preUtteranceDelay)]
        pub unsafe fn preUtteranceDelay(&self) -> NSTimeInterval;

        #[method(setPreUtteranceDelay:)]
        pub unsafe fn setPreUtteranceDelay(&self, pre_utterance_delay: NSTimeInterval);

        #[method(postUtteranceDelay)]
        pub unsafe fn postUtteranceDelay(&self) -> NSTimeInterval;

        #[method(setPostUtteranceDelay:)]
        pub unsafe fn setPostUtteranceDelay(&self, post_utterance_delay: NSTimeInterval);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSpeechUtterance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSpeechSynthesizer;
);

unsafe impl NSObjectProtocol for AVSpeechSynthesizer {}

extern_methods!(
    unsafe impl AVSpeechSynthesizer {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVSpeechSynthesizerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AVSpeechSynthesizerDelegate>>,
        );

        #[method(isSpeaking)]
        pub unsafe fn isSpeaking(&self) -> bool;

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[method(speakUtterance:)]
        pub unsafe fn speakUtterance(&self, utterance: &AVSpeechUtterance);

        #[cfg(all(feature = "AVAudioBuffer", feature = "block2"))]
        #[method(writeUtterance:toBufferCallback:)]
        pub unsafe fn writeUtterance_toBufferCallback(
            &self,
            utterance: &AVSpeechUtterance,
            buffer_callback: AVSpeechSynthesizerBufferCallback,
        );

        #[cfg(all(feature = "AVAudioBuffer", feature = "block2"))]
        #[method(writeUtterance:toBufferCallback:toMarkerCallback:)]
        pub unsafe fn writeUtterance_toBufferCallback_toMarkerCallback(
            &self,
            utterance: &AVSpeechUtterance,
            buffer_callback: AVSpeechSynthesizerBufferCallback,
            marker_callback: AVSpeechSynthesizerMarkerCallback,
        );

        #[method(stopSpeakingAtBoundary:)]
        pub unsafe fn stopSpeakingAtBoundary(&self, boundary: AVSpeechBoundary) -> bool;

        #[method(pauseSpeakingAtBoundary:)]
        pub unsafe fn pauseSpeakingAtBoundary(&self, boundary: AVSpeechBoundary) -> bool;

        #[method(continueSpeaking)]
        pub unsafe fn continueSpeaking(&self) -> bool;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other outputChannels)]
        pub unsafe fn outputChannels(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionChannelDescription>>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method(setOutputChannels:)]
        pub unsafe fn setOutputChannels(
            &self,
            output_channels: Option<&NSArray<AVAudioSessionChannelDescription>>,
        );

        #[method(usesApplicationAudioSession)]
        pub unsafe fn usesApplicationAudioSession(&self) -> bool;

        #[method(setUsesApplicationAudioSession:)]
        pub unsafe fn setUsesApplicationAudioSession(&self, uses_application_audio_session: bool);

        #[method(mixToTelephonyUplink)]
        pub unsafe fn mixToTelephonyUplink(&self) -> bool;

        #[method(setMixToTelephonyUplink:)]
        pub unsafe fn setMixToTelephonyUplink(&self, mix_to_telephony_uplink: bool);

        #[cfg(feature = "block2")]
        #[method(requestPersonalVoiceAuthorizationWithCompletionHandler:)]
        pub unsafe fn requestPersonalVoiceAuthorizationWithCompletionHandler(
            handler: &block2::Block<dyn Fn(AVSpeechSynthesisPersonalVoiceAuthorizationStatus)>,
        );

        #[method(personalVoiceAuthorizationStatus)]
        pub unsafe fn personalVoiceAuthorizationStatus(
        ) -> AVSpeechSynthesisPersonalVoiceAuthorizationStatus;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSpeechSynthesizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesizerdelegate?language=objc)
    pub unsafe trait AVSpeechSynthesizerDelegate: NSObjectProtocol {
        #[optional]
        #[method(speechSynthesizer:didStartSpeechUtterance:)]
        unsafe fn speechSynthesizer_didStartSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:didFinishSpeechUtterance:)]
        unsafe fn speechSynthesizer_didFinishSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:didPauseSpeechUtterance:)]
        unsafe fn speechSynthesizer_didPauseSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:didContinueSpeechUtterance:)]
        unsafe fn speechSynthesizer_didContinueSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:didCancelSpeechUtterance:)]
        unsafe fn speechSynthesizer_didCancelSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:willSpeakRangeOfSpeechString:utterance:)]
        unsafe fn speechSynthesizer_willSpeakRangeOfSpeechString_utterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            character_range: NSRange,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:willSpeakMarker:utterance:)]
        unsafe fn speechSynthesizer_willSpeakMarker_utterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            marker: &AVSpeechSynthesisMarker,
            utterance: &AVSpeechUtterance,
        );
    }

    unsafe impl ProtocolType for dyn AVSpeechSynthesizerDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesismarker?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSpeechSynthesisMarker;
);

unsafe impl Send for AVSpeechSynthesisMarker {}

unsafe impl Sync for AVSpeechSynthesisMarker {}

unsafe impl NSCoding for AVSpeechSynthesisMarker {}

unsafe impl NSCopying for AVSpeechSynthesisMarker {}

unsafe impl CopyingHelper for AVSpeechSynthesisMarker {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVSpeechSynthesisMarker {}

unsafe impl NSSecureCoding for AVSpeechSynthesisMarker {}

extern_methods!(
    unsafe impl AVSpeechSynthesisMarker {
        #[method(mark)]
        pub unsafe fn mark(&self) -> AVSpeechSynthesisMarkerMark;

        #[method(setMark:)]
        pub unsafe fn setMark(&self, mark: AVSpeechSynthesisMarkerMark);

        #[method(byteSampleOffset)]
        pub unsafe fn byteSampleOffset(&self) -> NSUInteger;

        #[method(setByteSampleOffset:)]
        pub unsafe fn setByteSampleOffset(&self, byte_sample_offset: NSUInteger);

        #[method(textRange)]
        pub unsafe fn textRange(&self) -> NSRange;

        #[method(setTextRange:)]
        pub unsafe fn setTextRange(&self, text_range: NSRange);

        #[method_id(@__retain_semantics Other bookmarkName)]
        pub unsafe fn bookmarkName(&self) -> Retained<NSString>;

        #[method(setBookmarkName:)]
        pub unsafe fn setBookmarkName(&self, bookmark_name: &NSString);

        #[method_id(@__retain_semantics Other phoneme)]
        pub unsafe fn phoneme(&self) -> Retained<NSString>;

        #[method(setPhoneme:)]
        pub unsafe fn setPhoneme(&self, phoneme: &NSString);

        #[method_id(@__retain_semantics Init initWithMarkerType:forTextRange:atByteSampleOffset:)]
        pub unsafe fn initWithMarkerType_forTextRange_atByteSampleOffset(
            this: Allocated<Self>,
            r#type: AVSpeechSynthesisMarkerMark,
            range: NSRange,
            byte_sample_offset: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithWordRange:atByteSampleOffset:)]
        pub unsafe fn initWithWordRange_atByteSampleOffset(
            this: Allocated<Self>,
            range: NSRange,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSentenceRange:atByteSampleOffset:)]
        pub unsafe fn initWithSentenceRange_atByteSampleOffset(
            this: Allocated<Self>,
            range: NSRange,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithParagraphRange:atByteSampleOffset:)]
        pub unsafe fn initWithParagraphRange_atByteSampleOffset(
            this: Allocated<Self>,
            range: NSRange,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithPhonemeString:atByteSampleOffset:)]
        pub unsafe fn initWithPhonemeString_atByteSampleOffset(
            this: Allocated<Self>,
            phoneme: &NSString,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBookmarkName:atByteSampleOffset:)]
        pub unsafe fn initWithBookmarkName_atByteSampleOffset(
            this: Allocated<Self>,
            mark: &NSString,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSpeechSynthesisMarker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);