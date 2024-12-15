//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-audio-toolbox")]
#[cfg(not(target_os = "watchos"))]
use objc2_audio_toolbox::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittimeeffect?language=objc)
    #[unsafe(super(AVAudioUnit, AVAudioNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
    pub struct AVAudioUnitTimeEffect;
);

#[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
unsafe impl NSObjectProtocol for AVAudioUnitTimeEffect {}

extern_methods!(
    #[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
    unsafe impl AVAudioUnitTimeEffect {
        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Init initWithAudioComponentDescription:)]
        pub unsafe fn initWithAudioComponentDescription(
            this: Allocated<Self>,
            audio_component_description: AudioComponentDescription,
        ) -> Retained<Self>;

        #[method(bypass)]
        pub unsafe fn bypass(&self) -> bool;

        #[method(setBypass:)]
        pub unsafe fn setBypass(&self, bypass: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
    unsafe impl AVAudioUnitTimeEffect {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
