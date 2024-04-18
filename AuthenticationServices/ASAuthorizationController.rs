//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "ASAuthorization")]
        #[optional]
        #[method(authorizationController:didCompleteWithAuthorization:)]
        unsafe fn authorizationController_didCompleteWithAuthorization(
            &self,
            controller: &ASAuthorizationController,
            authorization: &ASAuthorization,
        );

        #[optional]
        #[method(authorizationController:didCompleteWithError:)]
        unsafe fn authorizationController_didCompleteWithError(
            &self,
            controller: &ASAuthorizationController,
            error: &NSError,
        );

        #[cfg(feature = "ASAuthorizationCustomMethod")]
        #[optional]
        #[method(authorizationController:didCompleteWithCustomMethod:)]
        unsafe fn authorizationController_didCompleteWithCustomMethod(
            &self,
            controller: &ASAuthorizationController,
            method: &ASAuthorizationCustomMethod,
        );
    }

    unsafe impl ProtocolType for dyn ASAuthorizationControllerDelegate {}
);

extern_protocol!(
    pub unsafe trait ASAuthorizationControllerPresentationContextProviding:
        NSObjectProtocol
    {
        #[cfg(feature = "ASFoundation")]
        #[method_id(@__retain_semantics Other presentationAnchorForAuthorizationController:)]
        unsafe fn presentationAnchorForAuthorizationController(
            &self,
            controller: &ASAuthorizationController,
            mtm: MainThreadMarker,
        ) -> Id<ASPresentationAnchor>;
    }

    unsafe impl ProtocolType for dyn ASAuthorizationControllerPresentationContextProviding {}
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationControllerRequestOptions(pub NSUInteger);
bitflags::bitflags! {
    impl ASAuthorizationControllerRequestOptions: NSUInteger {
        const ASAuthorizationControllerRequestOptionPreferImmediatelyAvailableCredentials = 1<<0;
    }
}

unsafe impl Encode for ASAuthorizationControllerRequestOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationControllerRequestOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationController;

    unsafe impl ClassType for ASAuthorizationController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for ASAuthorizationController {}

extern_methods!(
    unsafe impl ASAuthorizationController {
        #[cfg(feature = "ASAuthorizationRequest")]
        #[method_id(@__retain_semantics Other authorizationRequests)]
        pub unsafe fn authorizationRequests(&self) -> Id<NSArray<ASAuthorizationRequest>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn ASAuthorizationControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn ASAuthorizationControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
        ) -> Option<Id<ProtocolObject<dyn ASAuthorizationControllerPresentationContextProviding>>>;

        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentation_context_provider: Option<
                &ProtocolObject<dyn ASAuthorizationControllerPresentationContextProviding>,
            >,
        );

        #[cfg(feature = "ASAuthorizationCustomMethod")]
        #[method_id(@__retain_semantics Other customAuthorizationMethods)]
        pub unsafe fn customAuthorizationMethods(&self)
            -> Id<NSArray<ASAuthorizationCustomMethod>>;

        #[cfg(feature = "ASAuthorizationCustomMethod")]
        #[method(setCustomAuthorizationMethods:)]
        pub unsafe fn setCustomAuthorizationMethods(
            &self,
            custom_authorization_methods: &NSArray<ASAuthorizationCustomMethod>,
        );

        #[cfg(feature = "ASAuthorizationRequest")]
        #[method_id(@__retain_semantics Init initWithAuthorizationRequests:)]
        pub unsafe fn initWithAuthorizationRequests(
            this: Allocated<Self>,
            authorization_requests: &NSArray<ASAuthorizationRequest>,
        ) -> Id<Self>;

        #[method(performRequests)]
        pub unsafe fn performRequests(&self);

        #[method(performAutoFillAssistedRequests)]
        pub unsafe fn performAutoFillAssistedRequests(&self);

        #[method(performRequestsWithOptions:)]
        pub unsafe fn performRequestsWithOptions(
            &self,
            options: ASAuthorizationControllerRequestOptions,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
