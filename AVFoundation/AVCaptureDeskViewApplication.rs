//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturedeskviewapplication?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeskViewApplication;
);

unsafe impl NSObjectProtocol for AVCaptureDeskViewApplication {}

extern_methods!(
    unsafe impl AVCaptureDeskViewApplication {
        #[cfg(feature = "block2")]
        #[method(presentWithCompletionHandler:)]
        pub unsafe fn presentWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(presentWithLaunchConfiguration:completionHandler:)]
        pub unsafe fn presentWithLaunchConfiguration_completionHandler(
            &self,
            launch_configuration: &AVCaptureDeskViewApplicationLaunchConfiguration,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCaptureDeskViewApplication {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturedeskviewapplicationlaunchconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeskViewApplicationLaunchConfiguration;
);

unsafe impl NSObjectProtocol for AVCaptureDeskViewApplicationLaunchConfiguration {}

extern_methods!(
    unsafe impl AVCaptureDeskViewApplicationLaunchConfiguration {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(mainWindowFrame)]
        pub unsafe fn mainWindowFrame(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMainWindowFrame:)]
        pub unsafe fn setMainWindowFrame(&self, main_window_frame: CGRect);

        #[method(requiresSetUpModeCompletion)]
        pub unsafe fn requiresSetUpModeCompletion(&self) -> bool;

        #[method(setRequiresSetUpModeCompletion:)]
        pub unsafe fn setRequiresSetUpModeCompletion(&self, requires_set_up_mode_completion: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCaptureDeskViewApplicationLaunchConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
