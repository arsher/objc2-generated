//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEComposeSession;

    unsafe impl ClassType for MEComposeSession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MEComposeSession {}

unsafe impl NSObjectProtocol for MEComposeSession {}

unsafe impl NSSecureCoding for MEComposeSession {}

extern_methods!(
    unsafe impl MEComposeSession {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other sessionID)]
        pub unsafe fn sessionID(&self) -> Id<NSUUID>;

        #[cfg(feature = "MEMessage")]
        #[method_id(@__retain_semantics Other mailMessage)]
        pub unsafe fn mailMessage(&self) -> Id<MEMessage>;

        #[cfg(feature = "MEComposeContext")]
        #[method_id(@__retain_semantics Other composeContext)]
        pub unsafe fn composeContext(&self) -> Id<MEComposeContext>;

        #[method(reloadSession)]
        pub unsafe fn reloadSession(&self);
    }
);

extern "C" {
    pub static MEComposeSessionErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MEComposeSessionErrorCode(pub NSInteger);
impl MEComposeSessionErrorCode {
    #[doc(alias = "MEComposeSessionErrorCodeInvalidRecipients")]
    pub const InvalidRecipients: Self = Self(0);
    #[doc(alias = "MEComposeSessionErrorCodeInvalidHeaders")]
    pub const InvalidHeaders: Self = Self(1);
    #[doc(alias = "MEComposeSessionErrorCodeInvalidBody")]
    pub const InvalidBody: Self = Self(2);
}

unsafe impl Encode for MEComposeSessionErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MEComposeSessionErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait MEComposeSessionHandler: NSObjectProtocol {
        #[method(mailComposeSessionDidBegin:)]
        unsafe fn mailComposeSessionDidBegin(&self, session: &MEComposeSession);

        #[method(mailComposeSessionDidEnd:)]
        unsafe fn mailComposeSessionDidEnd(&self, session: &MEComposeSession);

        #[cfg(all(feature = "MEExtensionViewController", feature = "objc2-app-kit"))]
        #[method_id(@__retain_semantics Other viewControllerForSession:)]
        unsafe fn viewControllerForSession(
            &self,
            session: &MEComposeSession,
            mtm: MainThreadMarker,
        ) -> Id<MEExtensionViewController>;

        #[cfg(all(
            feature = "MEAddressAnnotation",
            feature = "MEEmailAddress",
            feature = "block2"
        ))]
        #[optional]
        #[method(session:annotateAddressesWithCompletionHandler:)]
        unsafe fn session_annotateAddressesWithCompletionHandler(
            &self,
            session: &MEComposeSession,
            completion_handler: &block2::Block<
                dyn Fn(NonNull<NSDictionary<MEEmailAddress, MEAddressAnnotation>>),
            >,
        );

        #[cfg(feature = "block2")]
        #[optional]
        #[method(session:canSendMessageWithCompletionHandler:)]
        unsafe fn session_canSendMessageWithCompletionHandler(
            &self,
            session: &MEComposeSession,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[optional]
        #[method_id(@__retain_semantics Other additionalHeadersForSession:)]
        unsafe fn additionalHeadersForSession(
            &self,
            session: &MEComposeSession,
        ) -> Id<NSDictionary<NSString, NSArray<NSString>>>;
    }

    unsafe impl ProtocolType for dyn MEComposeSessionHandler {}
);
