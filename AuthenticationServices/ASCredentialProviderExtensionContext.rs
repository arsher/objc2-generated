//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/ascredentialproviderextensioncontext?language=objc)
    #[unsafe(super(NSExtensionContext, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASCredentialProviderExtensionContext;
);

unsafe impl NSObjectProtocol for ASCredentialProviderExtensionContext {}

extern_methods!(
    unsafe impl ASCredentialProviderExtensionContext {
        #[cfg(all(feature = "ASPasswordCredential", feature = "block2"))]
        #[method(completeRequestWithSelectedCredential:completionHandler:)]
        pub unsafe fn completeRequestWithSelectedCredential_completionHandler(
            &self,
            credential: &ASPasswordCredential,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(all(feature = "ASPasskeyAssertionCredential", feature = "block2"))]
        #[method(completeAssertionRequestWithSelectedPasskeyCredential:completionHandler:)]
        pub unsafe fn completeAssertionRequestWithSelectedPasskeyCredential_completionHandler(
            &self,
            credential: &ASPasskeyAssertionCredential,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(all(feature = "ASPasskeyRegistrationCredential", feature = "block2"))]
        #[method(completeRegistrationRequestWithSelectedPasskeyCredential:completionHandler:)]
        pub unsafe fn completeRegistrationRequestWithSelectedPasskeyCredential_completionHandler(
            &self,
            credential: &ASPasskeyRegistrationCredential,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(all(feature = "ASOneTimeCodeCredential", feature = "block2"))]
        #[method(completeOneTimeCodeRequestWithSelectedCredential:completionHandler:)]
        pub unsafe fn completeOneTimeCodeRequestWithSelectedCredential_completionHandler(
            &self,
            credential: &ASOneTimeCodeCredential,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[method(completeExtensionConfigurationRequest)]
        pub unsafe fn completeExtensionConfigurationRequest(&self);

        #[cfg(feature = "block2")]
        #[method(completeRequestReturningItems:completionHandler:)]
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[method(cancelRequestWithError:)]
        pub unsafe fn cancelRequestWithError(&self, error: &NSError);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASCredentialProviderExtensionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
