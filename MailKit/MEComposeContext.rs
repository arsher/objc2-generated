//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mailkit/mecomposeuseraction?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MEComposeUserAction(pub NSInteger);
impl MEComposeUserAction {
    #[doc(alias = "MEComposeUserActionNewMessage")]
    pub const NewMessage: Self = Self(1);
    #[doc(alias = "MEComposeUserActionReply")]
    pub const Reply: Self = Self(2);
    #[doc(alias = "MEComposeUserActionReplyAll")]
    pub const ReplyAll: Self = Self(3);
    #[doc(alias = "MEComposeUserActionForward")]
    pub const Forward: Self = Self(4);
}

unsafe impl Encode for MEComposeUserAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MEComposeUserAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mailkit/mecomposecontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEComposeContext;
);

unsafe impl NSObjectProtocol for MEComposeContext {}

extern_methods!(
    unsafe impl MEComposeContext {
        #[method_id(@__retain_semantics Other contextID)]
        pub unsafe fn contextID(&self) -> Retained<NSUUID>;

        #[cfg(feature = "MEMessage")]
        #[method_id(@__retain_semantics Other originalMessage)]
        pub unsafe fn originalMessage(&self) -> Option<Retained<MEMessage>>;

        #[method(action)]
        pub unsafe fn action(&self) -> MEComposeUserAction;

        #[method(isEncrypted)]
        pub unsafe fn isEncrypted(&self) -> bool;

        #[method(shouldEncrypt)]
        pub unsafe fn shouldEncrypt(&self) -> bool;

        #[method(isSigned)]
        pub unsafe fn isSigned(&self) -> bool;

        #[method(shouldSign)]
        pub unsafe fn shouldSign(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MEComposeContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
