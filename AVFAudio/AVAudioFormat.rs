//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiocommonformat?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAudioCommonFormat(pub NSUInteger);
impl AVAudioCommonFormat {
    pub const AVAudioOtherFormat: Self = Self(0);
    pub const AVAudioPCMFormatFloat32: Self = Self(1);
    pub const AVAudioPCMFormatFloat64: Self = Self(2);
    pub const AVAudioPCMFormatInt16: Self = Self(3);
    pub const AVAudioPCMFormatInt32: Self = Self(4);
}

unsafe impl Encode for AVAudioCommonFormat {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVAudioCommonFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioformat?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioFormat;
);

unsafe impl NSCoding for AVAudioFormat {}

unsafe impl NSObjectProtocol for AVAudioFormat {}

unsafe impl NSSecureCoding for AVAudioFormat {}

extern_methods!(
    unsafe impl AVAudioFormat {
        #[cfg(feature = "objc2-core-audio-types")]
        #[method_id(@__retain_semantics Init initWithStreamDescription:)]
        pub unsafe fn initWithStreamDescription(
            this: Allocated<Self>,
            asbd: NonNull<AudioStreamBasicDescription>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "AVAudioChannelLayout", feature = "objc2-core-audio-types"))]
        #[method_id(@__retain_semantics Init initWithStreamDescription:channelLayout:)]
        pub unsafe fn initWithStreamDescription_channelLayout(
            this: Allocated<Self>,
            asbd: NonNull<AudioStreamBasicDescription>,
            layout: Option<&AVAudioChannelLayout>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "AVAudioTypes")]
        #[method_id(@__retain_semantics Init initStandardFormatWithSampleRate:channels:)]
        pub unsafe fn initStandardFormatWithSampleRate_channels(
            this: Allocated<Self>,
            sample_rate: c_double,
            channels: AVAudioChannelCount,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "AVAudioChannelLayout")]
        #[method_id(@__retain_semantics Init initStandardFormatWithSampleRate:channelLayout:)]
        pub unsafe fn initStandardFormatWithSampleRate_channelLayout(
            this: Allocated<Self>,
            sample_rate: c_double,
            layout: &AVAudioChannelLayout,
        ) -> Retained<Self>;

        #[cfg(feature = "AVAudioTypes")]
        #[method_id(@__retain_semantics Init initWithCommonFormat:sampleRate:channels:interleaved:)]
        pub unsafe fn initWithCommonFormat_sampleRate_channels_interleaved(
            this: Allocated<Self>,
            format: AVAudioCommonFormat,
            sample_rate: c_double,
            channels: AVAudioChannelCount,
            interleaved: bool,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "AVAudioChannelLayout")]
        #[method_id(@__retain_semantics Init initWithCommonFormat:sampleRate:interleaved:channelLayout:)]
        pub unsafe fn initWithCommonFormat_sampleRate_interleaved_channelLayout(
            this: Allocated<Self>,
            format: AVAudioCommonFormat,
            sample_rate: c_double,
            interleaved: bool,
            layout: &AVAudioChannelLayout,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSettings:)]
        pub unsafe fn initWithSettings(
            this: Allocated<Self>,
            settings: &NSDictionary<NSString, AnyObject>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-media")]
        #[method_id(@__retain_semantics Init initWithCMAudioFormatDescription:)]
        pub unsafe fn initWithCMAudioFormatDescription(
            this: Allocated<Self>,
            format_description: CMAudioFormatDescriptionRef,
        ) -> Retained<Self>;

        #[method(isEqual:)]
        pub unsafe fn isEqual(&self, object: &AnyObject) -> bool;

        #[method(isStandard)]
        pub unsafe fn isStandard(&self) -> bool;

        #[method(commonFormat)]
        pub unsafe fn commonFormat(&self) -> AVAudioCommonFormat;

        #[cfg(feature = "AVAudioTypes")]
        #[method(channelCount)]
        pub unsafe fn channelCount(&self) -> AVAudioChannelCount;

        #[method(sampleRate)]
        pub unsafe fn sampleRate(&self) -> c_double;

        #[method(isInterleaved)]
        pub unsafe fn isInterleaved(&self) -> bool;

        #[cfg(feature = "objc2-core-audio-types")]
        #[method(streamDescription)]
        pub unsafe fn streamDescription(&self) -> NonNull<AudioStreamBasicDescription>;

        #[cfg(feature = "AVAudioChannelLayout")]
        #[method_id(@__retain_semantics Other channelLayout)]
        pub unsafe fn channelLayout(&self) -> Option<Retained<AVAudioChannelLayout>>;

        #[method_id(@__retain_semantics Other magicCookie)]
        pub unsafe fn magicCookie(&self) -> Option<Retained<NSData>>;

        #[method(setMagicCookie:)]
        pub unsafe fn setMagicCookie(&self, magic_cookie: Option<&NSData>);

        #[method_id(@__retain_semantics Other settings)]
        pub unsafe fn settings(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(formatDescription)]
        pub unsafe fn formatDescription(&self) -> CMAudioFormatDescriptionRef;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioFormat {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);