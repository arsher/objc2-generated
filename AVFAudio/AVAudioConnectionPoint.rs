//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioconnectionpoint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioConnectionPoint;
);

unsafe impl NSObjectProtocol for AVAudioConnectionPoint {}

extern_methods!(
    unsafe impl AVAudioConnectionPoint {
        #[cfg(all(feature = "AVAudioNode", feature = "AVAudioTypes"))]
        #[method_id(@__retain_semantics Init initWithNode:bus:)]
        pub unsafe fn initWithNode_bus(
            this: Allocated<Self>,
            node: &AVAudioNode,
            bus: AVAudioNodeBus,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "AVAudioNode")]
        #[method_id(@__retain_semantics Other node)]
        pub unsafe fn node(&self) -> Option<Retained<AVAudioNode>>;

        #[cfg(feature = "AVAudioTypes")]
        #[method(bus)]
        pub unsafe fn bus(&self) -> AVAudioNodeBus;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioConnectionPoint {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
