//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsynchronizedlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    pub struct AVSynchronizedLayer;
);

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl CAMediaTiming for AVSynchronizedLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSCoding for AVSynchronizedLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSObjectProtocol for AVSynchronizedLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSSecureCoding for AVSynchronizedLayer {}

extern_methods!(
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVSynchronizedLayer {
        #[cfg(feature = "AVPlayerItem")]
        #[method_id(@__retain_semantics Other synchronizedLayerWithPlayerItem:)]
        pub unsafe fn synchronizedLayerWithPlayerItem(
            player_item: &AVPlayerItem,
        ) -> Retained<AVSynchronizedLayer>;

        #[cfg(feature = "AVPlayerItem")]
        #[method_id(@__retain_semantics Other playerItem)]
        pub unsafe fn playerItem(&self, mtm: MainThreadMarker) -> Option<Retained<AVPlayerItem>>;

        #[cfg(feature = "AVPlayerItem")]
        #[method(setPlayerItem:)]
        pub unsafe fn setPlayerItem(
            &self,
            player_item: Option<&AVPlayerItem>,
            mtm: MainThreadMarker,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVSynchronizedLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVSynchronizedLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
