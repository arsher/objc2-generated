//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cametaldisplaylinkupdate?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAMetalDisplayLinkUpdate;
);

unsafe impl NSObjectProtocol for CAMetalDisplayLinkUpdate {}

extern_methods!(
    unsafe impl CAMetalDisplayLinkUpdate {
        #[cfg(all(feature = "CAMetalLayer", feature = "objc2-metal"))]
        #[method_id(@__retain_semantics Other drawable)]
        pub unsafe fn drawable(&self) -> Retained<ProtocolObject<dyn CAMetalDrawable>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(targetTimestamp)]
        pub unsafe fn targetTimestamp(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(targetPresentationTimestamp)]
        pub unsafe fn targetPresentationTimestamp(&self) -> CFTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAMetalDisplayLinkUpdate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cametaldisplaylinkdelegate?language=objc)
    pub unsafe trait CAMetalDisplayLinkDelegate {
        #[method(metalDisplayLink:needsUpdate:)]
        unsafe fn metalDisplayLink_needsUpdate(
            &self,
            link: &CAMetalDisplayLink,
            update: &CAMetalDisplayLinkUpdate,
        );
    }

    unsafe impl ProtocolType for dyn CAMetalDisplayLinkDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cametaldisplaylink?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAMetalDisplayLink;
);

unsafe impl NSObjectProtocol for CAMetalDisplayLink {}

extern_methods!(
    unsafe impl CAMetalDisplayLink {
        #[cfg(all(feature = "CALayer", feature = "CAMetalLayer"))]
        #[method_id(@__retain_semantics Init initWithMetalLayer:)]
        pub unsafe fn initWithMetalLayer(
            this: Allocated<Self>,
            layer: &CAMetalLayer,
        ) -> Retained<Self>;

        #[method(addToRunLoop:forMode:)]
        pub unsafe fn addToRunLoop_forMode(&self, runloop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, runloop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn CAMetalDisplayLinkDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CAMetalDisplayLinkDelegate>>,
        );

        #[method(preferredFrameLatency)]
        pub unsafe fn preferredFrameLatency(&self) -> c_float;

        #[method(setPreferredFrameLatency:)]
        pub unsafe fn setPreferredFrameLatency(&self, preferred_frame_latency: c_float);

        #[cfg(feature = "CAFrameRateRange")]
        #[method(preferredFrameRateRange)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        #[cfg(feature = "CAFrameRateRange")]
        #[method(setPreferredFrameRateRange:)]
        pub unsafe fn setPreferredFrameRateRange(
            &self,
            preferred_frame_rate_range: CAFrameRateRange,
        );

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[method(setPaused:)]
        pub unsafe fn setPaused(&self, paused: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAMetalDisplayLink {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
