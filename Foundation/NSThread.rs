//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsthread?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSThread;
);

unsafe impl NSObjectProtocol for NSThread {}

extern_methods!(
    unsafe impl NSThread {
        #[method_id(@__retain_semantics Other currentThread)]
        pub fn currentThread() -> Retained<NSThread>;

        #[cfg(feature = "block2")]
        #[method(detachNewThreadWithBlock:)]
        pub unsafe fn detachNewThreadWithBlock(block: &block2::Block<dyn Fn()>);

        #[method(detachNewThreadSelector:toTarget:withObject:)]
        pub unsafe fn detachNewThreadSelector_toTarget_withObject(
            selector: Sel,
            target: &AnyObject,
            argument: Option<&AnyObject>,
        );

        #[method(isMultiThreaded)]
        pub fn isMultiThreaded() -> bool;

        #[cfg(feature = "NSDictionary")]
        #[method_id(@__retain_semantics Other threadDictionary)]
        pub unsafe fn threadDictionary(&self) -> Retained<NSMutableDictionary>;

        #[cfg(feature = "NSDate")]
        #[method(sleepUntilDate:)]
        pub unsafe fn sleepUntilDate(date: &NSDate);

        #[cfg(feature = "NSDate")]
        #[method(sleepForTimeInterval:)]
        pub unsafe fn sleepForTimeInterval(ti: NSTimeInterval);

        #[method(exit)]
        pub unsafe fn exit();

        #[method(threadPriority)]
        pub unsafe fn threadPriority_class() -> c_double;

        #[method(setThreadPriority:)]
        pub unsafe fn setThreadPriority_class(p: c_double) -> bool;

        #[method(threadPriority)]
        pub unsafe fn threadPriority(&self) -> c_double;

        #[method(setThreadPriority:)]
        pub unsafe fn setThreadPriority(&self, thread_priority: c_double);

        #[cfg(feature = "NSObjCRuntime")]
        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[cfg(feature = "NSObjCRuntime")]
        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, quality_of_service: NSQualityOfService);

        #[cfg(all(feature = "NSArray", feature = "NSValue"))]
        #[method_id(@__retain_semantics Other callStackReturnAddresses)]
        pub unsafe fn callStackReturnAddresses() -> Retained<NSArray<NSNumber>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other callStackSymbols)]
        pub unsafe fn callStackSymbols() -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(stackSize)]
        pub unsafe fn stackSize(&self) -> NSUInteger;

        #[method(setStackSize:)]
        pub unsafe fn setStackSize(&self, stack_size: NSUInteger);

        #[method(isMainThread)]
        pub fn isMainThread(&self) -> bool;

        #[method(isMainThread)]
        pub fn isMainThread_class() -> bool;

        #[method_id(@__retain_semantics Other mainThread)]
        pub fn mainThread() -> Retained<NSThread>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithTarget:selector:object:)]
        pub unsafe fn initWithTarget_selector_object(
            this: Allocated<Self>,
            target: &AnyObject,
            selector: Sel,
            argument: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithBlock:)]
        pub unsafe fn initWithBlock(
            this: Allocated<Self>,
            block: &block2::Block<dyn Fn()>,
        ) -> Retained<Self>;

        #[method(isExecuting)]
        pub unsafe fn isExecuting(&self) -> bool;

        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;

        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(start)]
        pub unsafe fn start(&self);

        #[method(main)]
        pub unsafe fn main(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSThread {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for NSThread {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nswillbecomemultithreadednotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSWillBecomeMultiThreadedNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdidbecomesinglethreadednotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSDidBecomeSingleThreadedNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsthreadwillexitnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSThreadWillExitNotification: &'static NSNotificationName;
}

extern_category!(
    /// Category "NSThreadPerformAdditions" on [`NSObject`].
    #[doc(alias = "NSThreadPerformAdditions")]
    pub unsafe trait NSObjectNSThreadPerformAdditions {
        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(performSelectorOnMainThread:withObject:waitUntilDone:modes:)]
        unsafe fn performSelectorOnMainThread_withObject_waitUntilDone_modes(
            &self,
            a_selector: Sel,
            arg: Option<&AnyObject>,
            wait: bool,
            array: Option<&NSArray<NSString>>,
        );

        #[method(performSelectorOnMainThread:withObject:waitUntilDone:)]
        unsafe fn performSelectorOnMainThread_withObject_waitUntilDone(
            &self,
            a_selector: Sel,
            arg: Option<&AnyObject>,
            wait: bool,
        );

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(performSelector:onThread:withObject:waitUntilDone:modes:)]
        unsafe fn performSelector_onThread_withObject_waitUntilDone_modes(
            &self,
            a_selector: Sel,
            thr: &NSThread,
            arg: Option<&AnyObject>,
            wait: bool,
            array: Option<&NSArray<NSString>>,
        );

        #[method(performSelector:onThread:withObject:waitUntilDone:)]
        unsafe fn performSelector_onThread_withObject_waitUntilDone(
            &self,
            a_selector: Sel,
            thr: &NSThread,
            arg: Option<&AnyObject>,
            wait: bool,
        );

        #[method(performSelectorInBackground:withObject:)]
        unsafe fn performSelectorInBackground_withObject(
            &self,
            a_selector: Sel,
            arg: Option<&AnyObject>,
        );
    }

    unsafe impl NSObjectNSThreadPerformAdditions for NSObject {}
);
