//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Contain information about an email address. This can include both valid and invalid email addresses.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mailkit/meemailaddress?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEEmailAddress;
);

unsafe impl NSCoding for MEEmailAddress {}

unsafe impl NSCopying for MEEmailAddress {}

unsafe impl CopyingHelper for MEEmailAddress {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MEEmailAddress {}

unsafe impl NSSecureCoding for MEEmailAddress {}

extern_methods!(
    unsafe impl MEEmailAddress {
        /// The raw string for the email address.
        #[method_id(@__retain_semantics Other rawString)]
        pub unsafe fn rawString(&self) -> Retained<NSString>;

        /// The simple address string portion of the raw string if it is valid. For example, the
        /// `addressString`of "John Appleseed <j.appleseed@example.com>" will be "j.appleseed@example.com".
        #[method_id(@__retain_semantics Other addressString)]
        pub unsafe fn addressString(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithRawString:)]
        pub unsafe fn initWithRawString(
            this: Allocated<Self>,
            raw_string: &NSString,
        ) -> Retained<Self>;
    }
);
