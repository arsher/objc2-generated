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

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudionodetapblock?language=objc)
#[cfg(all(feature = "AVAudioBuffer", feature = "AVAudioTime", feature = "block2"))]
pub type AVAudioNodeTapBlock =
    *mut block2::Block<dyn Fn(NonNull<AVAudioPCMBuffer>, NonNull<AVAudioTime>)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudionode?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioNode;
);

unsafe impl NSObjectProtocol for AVAudioNode {}

extern_methods!(
    unsafe impl AVAudioNode {
        #[method(reset)]
        pub unsafe fn reset(&self);

        #[cfg(all(feature = "AVAudioFormat", feature = "AVAudioTypes"))]
        #[method_id(@__retain_semantics Other inputFormatForBus:)]
        pub unsafe fn inputFormatForBus(&self, bus: AVAudioNodeBus) -> Retained<AVAudioFormat>;

        #[cfg(all(feature = "AVAudioFormat", feature = "AVAudioTypes"))]
        #[method_id(@__retain_semantics Other outputFormatForBus:)]
        pub unsafe fn outputFormatForBus(&self, bus: AVAudioNodeBus) -> Retained<AVAudioFormat>;

        #[cfg(feature = "AVAudioTypes")]
        #[method_id(@__retain_semantics Other nameForInputBus:)]
        pub unsafe fn nameForInputBus(&self, bus: AVAudioNodeBus) -> Option<Retained<NSString>>;

        #[cfg(feature = "AVAudioTypes")]
        #[method_id(@__retain_semantics Other nameForOutputBus:)]
        pub unsafe fn nameForOutputBus(&self, bus: AVAudioNodeBus) -> Option<Retained<NSString>>;

        #[cfg(all(
            feature = "AVAudioBuffer",
            feature = "AVAudioFormat",
            feature = "AVAudioTime",
            feature = "AVAudioTypes",
            feature = "block2"
        ))]
        #[method(installTapOnBus:bufferSize:format:block:)]
        pub unsafe fn installTapOnBus_bufferSize_format_block(
            &self,
            bus: AVAudioNodeBus,
            buffer_size: AVAudioFrameCount,
            format: Option<&AVAudioFormat>,
            tap_block: AVAudioNodeTapBlock,
        );

        #[cfg(feature = "AVAudioTypes")]
        #[method(removeTapOnBus:)]
        pub unsafe fn removeTapOnBus(&self, bus: AVAudioNodeBus);

        #[cfg(feature = "AVAudioEngine")]
        #[method_id(@__retain_semantics Other engine)]
        pub unsafe fn engine(&self) -> Option<Retained<AVAudioEngine>>;

        #[method(numberOfInputs)]
        pub unsafe fn numberOfInputs(&self) -> NSUInteger;

        #[method(numberOfOutputs)]
        pub unsafe fn numberOfOutputs(&self) -> NSUInteger;

        #[cfg(feature = "AVAudioTime")]
        #[method_id(@__retain_semantics Other lastRenderTime)]
        pub unsafe fn lastRenderTime(&self) -> Option<Retained<AVAudioTime>>;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other AUAudioUnit)]
        pub unsafe fn AUAudioUnit(&self) -> Retained<AUAudioUnit>;

        #[method(latency)]
        pub unsafe fn latency(&self) -> NSTimeInterval;

        #[method(outputPresentationLatency)]
        pub unsafe fn outputPresentationLatency(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
