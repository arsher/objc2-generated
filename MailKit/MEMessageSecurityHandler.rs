//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static MEMessageSecurityErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MEMessageSecurityErrorCode(pub NSInteger);
impl MEMessageSecurityErrorCode {
    pub const MEMessageSecurityEncodingError: Self = Self(0);
    pub const MEMessageSecurityDecodingError: Self = Self(1);
}

unsafe impl Encode for MEMessageSecurityErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MEMessageSecurityErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    #[cfg(all(feature = "MEMessageDecoder", feature = "MEMessageEncoder"))]
    pub unsafe trait MEMessageSecurityHandler: MEMessageDecoder + MEMessageEncoder {
        #[cfg(all(
            feature = "MEExtensionViewController",
            feature = "MEMessageSigner",
            feature = "objc2-app-kit"
        ))]
        #[method_id(@__retain_semantics Other extensionViewControllerForMessageSigners:)]
        unsafe fn extensionViewControllerForMessageSigners(
            &self,
            message_signers: &NSArray<MEMessageSigner>,
            mtm: MainThreadMarker,
        ) -> Option<Retained<MEExtensionViewController>>;

        #[cfg(all(feature = "MEExtensionViewController", feature = "objc2-app-kit"))]
        #[method_id(@__retain_semantics Other extensionViewControllerForMessageContext:)]
        unsafe fn extensionViewControllerForMessageContext(
            &self,
            context: &NSData,
            mtm: MainThreadMarker,
        ) -> Option<Retained<MEExtensionViewController>>;

        #[cfg(all(
            feature = "MEExtensionViewController",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[method(primaryActionClickedForMessageContext:completionHandler:)]
        unsafe fn primaryActionClickedForMessageContext_completionHandler(
            &self,
            context: &NSData,
            completion_handler: &block2::Block<dyn Fn(*mut MEExtensionViewController)>,
        );
    }

    #[cfg(all(feature = "MEMessageDecoder", feature = "MEMessageEncoder"))]
    unsafe impl ProtocolType for dyn MEMessageSecurityHandler {}
);
