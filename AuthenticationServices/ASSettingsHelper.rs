//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/assettingshelper?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASSettingsHelper;
);

unsafe impl NSObjectProtocol for ASSettingsHelper {}

extern_methods!(
    unsafe impl ASSettingsHelper {
        #[cfg(feature = "block2")]
        #[method(openCredentialProviderAppSettingsWithCompletionHandler:)]
        pub unsafe fn openCredentialProviderAppSettingsWithCompletionHandler(
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(openVerificationCodeAppSettingsWithCompletionHandler:)]
        pub unsafe fn openVerificationCodeAppSettingsWithCompletionHandler(
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(requestToTurnOnCredentialProviderExtensionWithCompletionHandler:)]
        pub unsafe fn requestToTurnOnCredentialProviderExtensionWithCompletionHandler(
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
