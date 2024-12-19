//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cncontactsuserdefaults?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactsUserDefaults;
);

unsafe impl NSObjectProtocol for CNContactsUserDefaults {}

extern_methods!(
    unsafe impl CNContactsUserDefaults {
        #[method_id(@__retain_semantics Other sharedDefaults)]
        pub unsafe fn sharedDefaults() -> Retained<Self>;

        #[cfg(feature = "CNContact")]
        #[method(sortOrder)]
        pub unsafe fn sortOrder(&self) -> CNContactSortOrder;

        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactsUserDefaults {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
