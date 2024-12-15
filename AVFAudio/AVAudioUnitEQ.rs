//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-audio-toolbox")]
#[cfg(not(target_os = "watchos"))]
use objc2_audio_toolbox::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiouniteqfiltertype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAudioUnitEQFilterType(pub NSInteger);
impl AVAudioUnitEQFilterType {
    #[doc(alias = "AVAudioUnitEQFilterTypeParametric")]
    pub const Parametric: Self = Self(0);
    #[doc(alias = "AVAudioUnitEQFilterTypeLowPass")]
    pub const LowPass: Self = Self(1);
    #[doc(alias = "AVAudioUnitEQFilterTypeHighPass")]
    pub const HighPass: Self = Self(2);
    #[doc(alias = "AVAudioUnitEQFilterTypeResonantLowPass")]
    pub const ResonantLowPass: Self = Self(3);
    #[doc(alias = "AVAudioUnitEQFilterTypeResonantHighPass")]
    pub const ResonantHighPass: Self = Self(4);
    #[doc(alias = "AVAudioUnitEQFilterTypeBandPass")]
    pub const BandPass: Self = Self(5);
    #[doc(alias = "AVAudioUnitEQFilterTypeBandStop")]
    pub const BandStop: Self = Self(6);
    #[doc(alias = "AVAudioUnitEQFilterTypeLowShelf")]
    pub const LowShelf: Self = Self(7);
    #[doc(alias = "AVAudioUnitEQFilterTypeHighShelf")]
    pub const HighShelf: Self = Self(8);
    #[doc(alias = "AVAudioUnitEQFilterTypeResonantLowShelf")]
    pub const ResonantLowShelf: Self = Self(9);
    #[doc(alias = "AVAudioUnitEQFilterTypeResonantHighShelf")]
    pub const ResonantHighShelf: Self = Self(10);
}

unsafe impl Encode for AVAudioUnitEQFilterType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVAudioUnitEQFilterType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiouniteqfilterparameters?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioUnitEQFilterParameters;
);

unsafe impl NSObjectProtocol for AVAudioUnitEQFilterParameters {}

extern_methods!(
    unsafe impl AVAudioUnitEQFilterParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(filterType)]
        pub unsafe fn filterType(&self) -> AVAudioUnitEQFilterType;

        #[method(setFilterType:)]
        pub unsafe fn setFilterType(&self, filter_type: AVAudioUnitEQFilterType);

        #[method(frequency)]
        pub unsafe fn frequency(&self) -> c_float;

        #[method(setFrequency:)]
        pub unsafe fn setFrequency(&self, frequency: c_float);

        #[method(bandwidth)]
        pub unsafe fn bandwidth(&self) -> c_float;

        #[method(setBandwidth:)]
        pub unsafe fn setBandwidth(&self, bandwidth: c_float);

        #[method(gain)]
        pub unsafe fn gain(&self) -> c_float;

        #[method(setGain:)]
        pub unsafe fn setGain(&self, gain: c_float);

        #[method(bypass)]
        pub unsafe fn bypass(&self) -> bool;

        #[method(setBypass:)]
        pub unsafe fn setBypass(&self, bypass: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioUnitEQFilterParameters {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiouniteq?language=objc)
    #[unsafe(super(AVAudioUnitEffect, AVAudioUnit, AVAudioNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AVAudioNode",
        feature = "AVAudioUnit",
        feature = "AVAudioUnitEffect"
    ))]
    pub struct AVAudioUnitEQ;
);

#[cfg(all(
    feature = "AVAudioNode",
    feature = "AVAudioUnit",
    feature = "AVAudioUnitEffect"
))]
unsafe impl NSObjectProtocol for AVAudioUnitEQ {}

extern_methods!(
    #[cfg(all(
        feature = "AVAudioNode",
        feature = "AVAudioUnit",
        feature = "AVAudioUnitEffect"
    ))]
    unsafe impl AVAudioUnitEQ {
        #[method_id(@__retain_semantics Init initWithNumberOfBands:)]
        pub unsafe fn initWithNumberOfBands(
            this: Allocated<Self>,
            number_of_bands: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other bands)]
        pub unsafe fn bands(&self) -> Retained<NSArray<AVAudioUnitEQFilterParameters>>;

        #[method(globalGain)]
        pub unsafe fn globalGain(&self) -> c_float;

        #[method(setGlobalGain:)]
        pub unsafe fn setGlobalGain(&self, global_gain: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAudioUnitEffect`
    #[cfg(all(
        feature = "AVAudioNode",
        feature = "AVAudioUnit",
        feature = "AVAudioUnitEffect"
    ))]
    unsafe impl AVAudioUnitEQ {
        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Init initWithAudioComponentDescription:)]
        pub unsafe fn initWithAudioComponentDescription(
            this: Allocated<Self>,
            audio_component_description: AudioComponentDescription,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AVAudioNode",
        feature = "AVAudioUnit",
        feature = "AVAudioUnitEffect"
    ))]
    unsafe impl AVAudioUnitEQ {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
