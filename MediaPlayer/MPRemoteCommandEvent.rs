//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpremotecommandevent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPRemoteCommandEvent;
);

unsafe impl NSObjectProtocol for MPRemoteCommandEvent {}

extern_methods!(
    unsafe impl MPRemoteCommandEvent {
        #[cfg(feature = "MPRemoteCommand")]
        #[method_id(@__retain_semantics Other command)]
        pub unsafe fn command(&self) -> Retained<MPRemoteCommand>;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPRemoteCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpskipintervalcommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSkipIntervalCommandEvent;
);

unsafe impl NSObjectProtocol for MPSkipIntervalCommandEvent {}

extern_methods!(
    unsafe impl MPSkipIntervalCommandEvent {
        #[method(interval)]
        pub unsafe fn interval(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSkipIntervalCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpseekcommandeventtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSeekCommandEventType(pub NSUInteger);
impl MPSeekCommandEventType {
    #[doc(alias = "MPSeekCommandEventTypeBeginSeeking")]
    pub const BeginSeeking: Self = Self(0);
    #[doc(alias = "MPSeekCommandEventTypeEndSeeking")]
    pub const EndSeeking: Self = Self(1);
}

unsafe impl Encode for MPSeekCommandEventType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSeekCommandEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpseekcommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSeekCommandEvent;
);

unsafe impl NSObjectProtocol for MPSeekCommandEvent {}

extern_methods!(
    unsafe impl MPSeekCommandEvent {
        #[method(type)]
        pub unsafe fn r#type(&self) -> MPSeekCommandEventType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSeekCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpratingcommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPRatingCommandEvent;
);

unsafe impl NSObjectProtocol for MPRatingCommandEvent {}

extern_methods!(
    unsafe impl MPRatingCommandEvent {
        #[method(rating)]
        pub unsafe fn rating(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPRatingCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpchangeplaybackratecommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangePlaybackRateCommandEvent;
);

unsafe impl NSObjectProtocol for MPChangePlaybackRateCommandEvent {}

extern_methods!(
    unsafe impl MPChangePlaybackRateCommandEvent {
        #[method(playbackRate)]
        pub unsafe fn playbackRate(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPChangePlaybackRateCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpfeedbackcommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPFeedbackCommandEvent;
);

unsafe impl NSObjectProtocol for MPFeedbackCommandEvent {}

extern_methods!(
    unsafe impl MPFeedbackCommandEvent {
        #[method(isNegative)]
        pub unsafe fn isNegative(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPFeedbackCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpchangelanguageoptioncommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangeLanguageOptionCommandEvent;
);

unsafe impl NSObjectProtocol for MPChangeLanguageOptionCommandEvent {}

extern_methods!(
    unsafe impl MPChangeLanguageOptionCommandEvent {
        #[cfg(feature = "MPNowPlayingInfoLanguageOption")]
        #[method_id(@__retain_semantics Other languageOption)]
        pub unsafe fn languageOption(&self) -> Retained<MPNowPlayingInfoLanguageOption>;

        #[cfg(feature = "MPRemoteControlTypes")]
        #[method(setting)]
        pub unsafe fn setting(&self) -> MPChangeLanguageOptionSetting;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPChangeLanguageOptionCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpchangeplaybackpositioncommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangePlaybackPositionCommandEvent;
);

unsafe impl NSObjectProtocol for MPChangePlaybackPositionCommandEvent {}

extern_methods!(
    unsafe impl MPChangePlaybackPositionCommandEvent {
        #[method(positionTime)]
        pub unsafe fn positionTime(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPChangePlaybackPositionCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpchangeshufflemodecommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangeShuffleModeCommandEvent;
);

unsafe impl NSObjectProtocol for MPChangeShuffleModeCommandEvent {}

extern_methods!(
    unsafe impl MPChangeShuffleModeCommandEvent {
        #[cfg(feature = "MPRemoteControlTypes")]
        #[method(shuffleType)]
        pub unsafe fn shuffleType(&self) -> MPShuffleType;

        #[method(preservesShuffleMode)]
        pub unsafe fn preservesShuffleMode(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPChangeShuffleModeCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpchangerepeatmodecommandevent?language=objc)
    #[unsafe(super(MPRemoteCommandEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangeRepeatModeCommandEvent;
);

unsafe impl NSObjectProtocol for MPChangeRepeatModeCommandEvent {}

extern_methods!(
    unsafe impl MPChangeRepeatModeCommandEvent {
        #[cfg(feature = "MPRemoteControlTypes")]
        #[method(repeatType)]
        pub unsafe fn repeatType(&self) -> MPRepeatType;

        #[method(preservesRepeatMode)]
        pub unsafe fn preservesRepeatMode(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPChangeRepeatModeCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
