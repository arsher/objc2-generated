//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderExtensionContext")]
    pub struct ASCredentialProviderExtensionContext;

    #[cfg(feature = "AuthenticationServices_ASCredentialProviderExtensionContext")]
    unsafe impl ClassType for ASCredentialProviderExtensionContext {
        #[inherits(NSObject)]
        type Super = NSExtensionContext;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASCredentialProviderExtensionContext")]
unsafe impl NSObjectProtocol for ASCredentialProviderExtensionContext {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderExtensionContext")]
    unsafe impl ASCredentialProviderExtensionContext {
        #[cfg(feature = "AuthenticationServices_ASPasswordCredential")]
        #[method(completeRequestWithSelectedCredential:completionHandler:)]
        pub unsafe fn completeRequestWithSelectedCredential_completionHandler(
            &self,
            credential: &ASPasswordCredential,
            completion_handler: Option<&Block<(Bool,), ()>>,
        );

        #[method(completeExtensionConfigurationRequest)]
        pub unsafe fn completeExtensionConfigurationRequest(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(completeRequestReturningItems:completionHandler:)]
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completion_handler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(cancelRequestWithError:)]
        pub unsafe fn cancelRequestWithError(&self, error: &NSError);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderExtensionContext")]
    unsafe impl ASCredentialProviderExtensionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
