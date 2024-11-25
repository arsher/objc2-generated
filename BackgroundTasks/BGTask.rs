//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/backgroundtasks/bgtask?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGTask;
);

unsafe impl NSObjectProtocol for BGTask {}

extern_methods!(
    unsafe impl BGTask {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[cfg(feature = "block2")]
        #[method(expirationHandler)]
        pub unsafe fn expirationHandler(&self) -> *mut block2::Block<dyn Fn()>;

        #[cfg(feature = "block2")]
        #[method(setExpirationHandler:)]
        pub unsafe fn setExpirationHandler(
            &self,
            expiration_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(setTaskCompletedWithSuccess:)]
        pub unsafe fn setTaskCompletedWithSuccess(&self, success: bool);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/backgroundtasks/bgprocessingtask?language=objc)
    #[unsafe(super(BGTask, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGProcessingTask;
);

unsafe impl NSObjectProtocol for BGProcessingTask {}

extern_methods!(
    unsafe impl BGProcessingTask {}
);

extern_methods!(
    /// Methods declared on superclass `BGTask`
    unsafe impl BGProcessingTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/backgroundtasks/bghealthresearchtask?language=objc)
    #[unsafe(super(BGProcessingTask, BGTask, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGHealthResearchTask;
);

unsafe impl NSObjectProtocol for BGHealthResearchTask {}

extern_methods!(
    unsafe impl BGHealthResearchTask {}
);

extern_methods!(
    /// Methods declared on superclass `BGTask`
    unsafe impl BGHealthResearchTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/backgroundtasks/bgapprefreshtask?language=objc)
    #[unsafe(super(BGTask, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGAppRefreshTask;
);

unsafe impl NSObjectProtocol for BGAppRefreshTask {}

extern_methods!(
    unsafe impl BGAppRefreshTask {}
);

extern_methods!(
    /// Methods declared on superclass `BGTask`
    unsafe impl BGAppRefreshTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
