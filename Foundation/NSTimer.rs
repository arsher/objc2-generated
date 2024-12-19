//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nstimer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTimer;
);

unsafe impl NSObjectProtocol for NSTimer {}

extern_methods!(
    unsafe impl NSTimer {
        #[cfg(all(feature = "NSDate", feature = "NSInvocation"))]
        #[method_id(@__retain_semantics Other timerWithTimeInterval:invocation:repeats:)]
        pub unsafe fn timerWithTimeInterval_invocation_repeats(
            ti: NSTimeInterval,
            invocation: &NSInvocation,
            yes_or_no: bool,
        ) -> Retained<NSTimer>;

        #[cfg(all(feature = "NSDate", feature = "NSInvocation"))]
        #[method_id(@__retain_semantics Other scheduledTimerWithTimeInterval:invocation:repeats:)]
        pub unsafe fn scheduledTimerWithTimeInterval_invocation_repeats(
            ti: NSTimeInterval,
            invocation: &NSInvocation,
            yes_or_no: bool,
        ) -> Retained<NSTimer>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other timerWithTimeInterval:target:selector:userInfo:repeats:)]
        pub unsafe fn timerWithTimeInterval_target_selector_userInfo_repeats(
            ti: NSTimeInterval,
            a_target: &AnyObject,
            a_selector: Sel,
            user_info: Option<&AnyObject>,
            yes_or_no: bool,
        ) -> Retained<NSTimer>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other scheduledTimerWithTimeInterval:target:selector:userInfo:repeats:)]
        pub unsafe fn scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
            ti: NSTimeInterval,
            a_target: &AnyObject,
            a_selector: Sel,
            user_info: Option<&AnyObject>,
            yes_or_no: bool,
        ) -> Retained<NSTimer>;

        #[cfg(all(feature = "NSDate", feature = "block2"))]
        #[method_id(@__retain_semantics Other timerWithTimeInterval:repeats:block:)]
        pub unsafe fn timerWithTimeInterval_repeats_block(
            interval: NSTimeInterval,
            repeats: bool,
            block: &block2::Block<dyn Fn(NonNull<NSTimer>)>,
        ) -> Retained<NSTimer>;

        #[cfg(all(feature = "NSDate", feature = "block2"))]
        #[method_id(@__retain_semantics Other scheduledTimerWithTimeInterval:repeats:block:)]
        pub unsafe fn scheduledTimerWithTimeInterval_repeats_block(
            interval: NSTimeInterval,
            repeats: bool,
            block: &block2::Block<dyn Fn(NonNull<NSTimer>)>,
        ) -> Retained<NSTimer>;

        #[cfg(all(feature = "NSDate", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithFireDate:interval:repeats:block:)]
        pub unsafe fn initWithFireDate_interval_repeats_block(
            this: Allocated<Self>,
            date: &NSDate,
            interval: NSTimeInterval,
            repeats: bool,
            block: &block2::Block<dyn Fn(NonNull<NSTimer>)>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Init initWithFireDate:interval:target:selector:userInfo:repeats:)]
        pub unsafe fn initWithFireDate_interval_target_selector_userInfo_repeats(
            this: Allocated<Self>,
            date: &NSDate,
            ti: NSTimeInterval,
            t: &AnyObject,
            s: Sel,
            ui: Option<&AnyObject>,
            rep: bool,
        ) -> Retained<Self>;

        #[method(fire)]
        pub unsafe fn fire(&self);

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other fireDate)]
        pub unsafe fn fireDate(&self) -> Retained<NSDate>;

        #[cfg(feature = "NSDate")]
        #[method(setFireDate:)]
        pub unsafe fn setFireDate(&self, fire_date: &NSDate);

        #[cfg(feature = "NSDate")]
        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[cfg(feature = "NSDate")]
        #[method(tolerance)]
        pub unsafe fn tolerance(&self) -> NSTimeInterval;

        #[cfg(feature = "NSDate")]
        #[method(setTolerance:)]
        pub unsafe fn setTolerance(&self, tolerance: NSTimeInterval);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTimer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
