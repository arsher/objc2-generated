//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cadisplaylink?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CADisplayLink;
);

unsafe impl NSObjectProtocol for CADisplayLink {}

extern_methods!(
    unsafe impl CADisplayLink {
        #[method_id(@__retain_semantics Other displayLinkWithTarget:selector:)]
        pub unsafe fn displayLinkWithTarget_selector(
            target: &AnyObject,
            sel: Sel,
        ) -> Retained<CADisplayLink>;

        #[method(addToRunLoop:forMode:)]
        pub unsafe fn addToRunLoop_forMode(&self, runloop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, runloop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> CFTimeInterval;

        #[method(duration)]
        pub unsafe fn duration(&self) -> CFTimeInterval;

        #[method(targetTimestamp)]
        pub unsafe fn targetTimestamp(&self) -> CFTimeInterval;

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[method(setPaused:)]
        pub unsafe fn setPaused(&self, paused: bool);

        #[deprecated = "preferredFramesPerSecond"]
        #[method(frameInterval)]
        pub unsafe fn frameInterval(&self) -> NSInteger;

        #[deprecated = "preferredFramesPerSecond"]
        #[method(setFrameInterval:)]
        pub unsafe fn setFrameInterval(&self, frame_interval: NSInteger);

        #[deprecated]
        #[method(preferredFramesPerSecond)]
        pub unsafe fn preferredFramesPerSecond(&self) -> NSInteger;

        #[deprecated]
        #[method(setPreferredFramesPerSecond:)]
        pub unsafe fn setPreferredFramesPerSecond(&self, preferred_frames_per_second: NSInteger);

        #[cfg(feature = "CAFrameRateRange")]
        #[method(preferredFrameRateRange)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        #[cfg(feature = "CAFrameRateRange")]
        #[method(setPreferredFrameRateRange:)]
        pub unsafe fn setPreferredFrameRateRange(
            &self,
            preferred_frame_rate_range: CAFrameRateRange,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CADisplayLink {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
