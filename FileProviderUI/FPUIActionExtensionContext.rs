//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_file_provider::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileproviderui/fpuierrordomain?language=objc)
    pub static FPUIErrorDomain: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/fileproviderui/fpuiactionidentifier?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type FPUIActionIdentifier = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/fileproviderui/fpuiextensionerrorcode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FPUIExtensionErrorCode(pub NSUInteger);
impl FPUIExtensionErrorCode {
    #[doc(alias = "FPUIExtensionErrorCodeUserCancelled")]
    pub const UserCancelled: Self = Self(0);
    #[doc(alias = "FPUIExtensionErrorCodeFailed")]
    pub const Failed: Self = Self(1);
}

unsafe impl Encode for FPUIExtensionErrorCode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for FPUIExtensionErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/fileproviderui/fpuiactionextensioncontext?language=objc)
    #[unsafe(super(NSExtensionContext, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FPUIActionExtensionContext;
);

unsafe impl NSObjectProtocol for FPUIActionExtensionContext {}

extern_methods!(
    unsafe impl FPUIActionExtensionContext {
        #[method_id(@__retain_semantics Other domainIdentifier)]
        pub unsafe fn domainIdentifier(&self) -> Option<Retained<NSFileProviderDomainIdentifier>>;

        #[method(completeRequest)]
        pub unsafe fn completeRequest(&self);

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
    unsafe impl FPUIActionExtensionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
