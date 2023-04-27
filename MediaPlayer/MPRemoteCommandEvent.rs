//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
    pub struct MPRemoteCommandEvent;

    #[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
    unsafe impl ClassType for MPRemoteCommandEvent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
unsafe impl NSObjectProtocol for MPRemoteCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
    unsafe impl MPRemoteCommandEvent {
        #[cfg(feature = "MediaPlayer_MPRemoteCommand")]
        #[method_id(@__retain_semantics Other command)]
        pub unsafe fn command(&self) -> Id<MPRemoteCommand>;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
    unsafe impl MPRemoteCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPSkipIntervalCommandEvent")]
    pub struct MPSkipIntervalCommandEvent;

    #[cfg(feature = "MediaPlayer_MPSkipIntervalCommandEvent")]
    unsafe impl ClassType for MPSkipIntervalCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPSkipIntervalCommandEvent")]
unsafe impl NSObjectProtocol for MPSkipIntervalCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPSkipIntervalCommandEvent")]
    unsafe impl MPSkipIntervalCommandEvent {
        #[method(interval)]
        pub unsafe fn interval(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPSkipIntervalCommandEvent")]
    unsafe impl MPSkipIntervalCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MPSeekCommandEventType {
        MPSeekCommandEventTypeBeginSeeking = 0,
        MPSeekCommandEventTypeEndSeeking = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPSeekCommandEvent")]
    pub struct MPSeekCommandEvent;

    #[cfg(feature = "MediaPlayer_MPSeekCommandEvent")]
    unsafe impl ClassType for MPSeekCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPSeekCommandEvent")]
unsafe impl NSObjectProtocol for MPSeekCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPSeekCommandEvent")]
    unsafe impl MPSeekCommandEvent {
        #[method(type)]
        pub unsafe fn r#type(&self) -> MPSeekCommandEventType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPSeekCommandEvent")]
    unsafe impl MPSeekCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPRatingCommandEvent")]
    pub struct MPRatingCommandEvent;

    #[cfg(feature = "MediaPlayer_MPRatingCommandEvent")]
    unsafe impl ClassType for MPRatingCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPRatingCommandEvent")]
unsafe impl NSObjectProtocol for MPRatingCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPRatingCommandEvent")]
    unsafe impl MPRatingCommandEvent {
        #[method(rating)]
        pub unsafe fn rating(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPRatingCommandEvent")]
    unsafe impl MPRatingCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPChangePlaybackRateCommandEvent")]
    pub struct MPChangePlaybackRateCommandEvent;

    #[cfg(feature = "MediaPlayer_MPChangePlaybackRateCommandEvent")]
    unsafe impl ClassType for MPChangePlaybackRateCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPChangePlaybackRateCommandEvent")]
unsafe impl NSObjectProtocol for MPChangePlaybackRateCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPChangePlaybackRateCommandEvent")]
    unsafe impl MPChangePlaybackRateCommandEvent {
        #[method(playbackRate)]
        pub unsafe fn playbackRate(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPChangePlaybackRateCommandEvent")]
    unsafe impl MPChangePlaybackRateCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPFeedbackCommandEvent")]
    pub struct MPFeedbackCommandEvent;

    #[cfg(feature = "MediaPlayer_MPFeedbackCommandEvent")]
    unsafe impl ClassType for MPFeedbackCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPFeedbackCommandEvent")]
unsafe impl NSObjectProtocol for MPFeedbackCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPFeedbackCommandEvent")]
    unsafe impl MPFeedbackCommandEvent {
        #[method(isNegative)]
        pub unsafe fn isNegative(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPFeedbackCommandEvent")]
    unsafe impl MPFeedbackCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPChangeLanguageOptionCommandEvent")]
    pub struct MPChangeLanguageOptionCommandEvent;

    #[cfg(feature = "MediaPlayer_MPChangeLanguageOptionCommandEvent")]
    unsafe impl ClassType for MPChangeLanguageOptionCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPChangeLanguageOptionCommandEvent")]
unsafe impl NSObjectProtocol for MPChangeLanguageOptionCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPChangeLanguageOptionCommandEvent")]
    unsafe impl MPChangeLanguageOptionCommandEvent {
        #[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
        #[method_id(@__retain_semantics Other languageOption)]
        pub unsafe fn languageOption(&self) -> Id<MPNowPlayingInfoLanguageOption>;

        #[method(setting)]
        pub unsafe fn setting(&self) -> MPChangeLanguageOptionSetting;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPChangeLanguageOptionCommandEvent")]
    unsafe impl MPChangeLanguageOptionCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPChangePlaybackPositionCommandEvent")]
    pub struct MPChangePlaybackPositionCommandEvent;

    #[cfg(feature = "MediaPlayer_MPChangePlaybackPositionCommandEvent")]
    unsafe impl ClassType for MPChangePlaybackPositionCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPChangePlaybackPositionCommandEvent")]
unsafe impl NSObjectProtocol for MPChangePlaybackPositionCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPChangePlaybackPositionCommandEvent")]
    unsafe impl MPChangePlaybackPositionCommandEvent {
        #[method(positionTime)]
        pub unsafe fn positionTime(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPChangePlaybackPositionCommandEvent")]
    unsafe impl MPChangePlaybackPositionCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPChangeShuffleModeCommandEvent")]
    pub struct MPChangeShuffleModeCommandEvent;

    #[cfg(feature = "MediaPlayer_MPChangeShuffleModeCommandEvent")]
    unsafe impl ClassType for MPChangeShuffleModeCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPChangeShuffleModeCommandEvent")]
unsafe impl NSObjectProtocol for MPChangeShuffleModeCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPChangeShuffleModeCommandEvent")]
    unsafe impl MPChangeShuffleModeCommandEvent {
        #[method(shuffleType)]
        pub unsafe fn shuffleType(&self) -> MPShuffleType;

        #[method(preservesShuffleMode)]
        pub unsafe fn preservesShuffleMode(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPChangeShuffleModeCommandEvent")]
    unsafe impl MPChangeShuffleModeCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPChangeRepeatModeCommandEvent")]
    pub struct MPChangeRepeatModeCommandEvent;

    #[cfg(feature = "MediaPlayer_MPChangeRepeatModeCommandEvent")]
    unsafe impl ClassType for MPChangeRepeatModeCommandEvent {
        #[inherits(NSObject)]
        type Super = MPRemoteCommandEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPChangeRepeatModeCommandEvent")]
unsafe impl NSObjectProtocol for MPChangeRepeatModeCommandEvent {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPChangeRepeatModeCommandEvent")]
    unsafe impl MPChangeRepeatModeCommandEvent {
        #[method(repeatType)]
        pub unsafe fn repeatType(&self) -> MPRepeatType;

        #[method(preservesRepeatMode)]
        pub unsafe fn preservesRepeatMode(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPChangeRepeatModeCommandEvent")]
    unsafe impl MPChangeRepeatModeCommandEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);