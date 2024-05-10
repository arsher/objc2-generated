//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static ASWebAuthenticationSessionErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASWebAuthenticationSessionErrorCode(pub NSInteger);
impl ASWebAuthenticationSessionErrorCode {
    #[doc(alias = "ASWebAuthenticationSessionErrorCodeCanceledLogin")]
    pub const CanceledLogin: Self = Self(1);
    #[doc(alias = "ASWebAuthenticationSessionErrorCodePresentationContextNotProvided")]
    pub const PresentationContextNotProvided: Self = Self(2);
    #[doc(alias = "ASWebAuthenticationSessionErrorCodePresentationContextInvalid")]
    pub const PresentationContextInvalid: Self = Self(3);
}

unsafe impl Encode for ASWebAuthenticationSessionErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASWebAuthenticationSessionErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "block2")]
pub type ASWebAuthenticationSessionCompletionHandler =
    *mut block2::Block<dyn Fn(*mut NSURL, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASWebAuthenticationSession;

    unsafe impl ClassType for ASWebAuthenticationSession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for ASWebAuthenticationSession {}

extern_methods!(
    unsafe impl ASWebAuthenticationSession {
        #[cfg(feature = "block2")]
        #[deprecated = "Use initWithURL:callback:completionHandler: instead"]
        #[method_id(@__retain_semantics Init initWithURL:callbackURLScheme:completionHandler:)]
        pub unsafe fn initWithURL_callbackURLScheme_completionHandler(
            this: Allocated<Self>,
            url: &NSURL,
            callback_url_scheme: Option<&NSString>,
            completion_handler: ASWebAuthenticationSessionCompletionHandler,
        ) -> Id<Self>;

        #[cfg(all(feature = "ASWebAuthenticationSessionCallback", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithURL:callback:completionHandler:)]
        pub unsafe fn initWithURL_callback_completionHandler(
            this: Allocated<Self>,
            url: &NSURL,
            callback: &ASWebAuthenticationSessionCallback,
            completion_handler: ASWebAuthenticationSessionCompletionHandler,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
        ) -> Option<Id<ProtocolObject<dyn ASWebAuthenticationPresentationContextProviding>>>;

        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentation_context_provider: Option<
                &ProtocolObject<dyn ASWebAuthenticationPresentationContextProviding>,
            >,
        );

        #[method(prefersEphemeralWebBrowserSession)]
        pub unsafe fn prefersEphemeralWebBrowserSession(&self) -> bool;

        #[method(setPrefersEphemeralWebBrowserSession:)]
        pub unsafe fn setPrefersEphemeralWebBrowserSession(
            &self,
            prefers_ephemeral_web_browser_session: bool,
        );

        #[method_id(@__retain_semantics Other additionalHeaderFields)]
        pub unsafe fn additionalHeaderFields(&self)
            -> Option<Id<NSDictionary<NSString, NSString>>>;

        #[method(setAdditionalHeaderFields:)]
        pub unsafe fn setAdditionalHeaderFields(
            &self,
            additional_header_fields: Option<&NSDictionary<NSString, NSString>>,
        );

        #[method(canStart)]
        pub unsafe fn canStart(&self) -> bool;

        #[method(start)]
        pub unsafe fn start(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait ASWebAuthenticationPresentationContextProviding:
        NSObjectProtocol
    {
        #[cfg(feature = "ASFoundation")]
        #[method_id(@__retain_semantics Other presentationAnchorForWebAuthenticationSession:)]
        unsafe fn presentationAnchorForWebAuthenticationSession(
            &self,
            session: &ASWebAuthenticationSession,
            mtm: MainThreadMarker,
        ) -> Id<ASPresentationAnchor>;
    }

    unsafe impl ProtocolType for dyn ASWebAuthenticationPresentationContextProviding {}
);
