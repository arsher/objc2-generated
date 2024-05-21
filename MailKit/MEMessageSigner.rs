//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEMessageSigner;

    unsafe impl ClassType for MEMessageSigner {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MEMessageSigner {}

unsafe impl NSObjectProtocol for MEMessageSigner {}

unsafe impl NSSecureCoding for MEMessageSigner {}

extern_methods!(
    unsafe impl MEMessageSigner {
        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Other emailAddresses)]
        pub unsafe fn emailAddresses(&self) -> Retained<NSArray<MEEmailAddress>>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Init initWithEmailAddresses:signatureLabel:context:)]
        pub unsafe fn initWithEmailAddresses_signatureLabel_context(
            this: Allocated<Self>,
            email_addresses: &NSArray<MEEmailAddress>,
            label: &NSString,
            context: Option<&NSData>,
        ) -> Retained<Self>;
    }
);
