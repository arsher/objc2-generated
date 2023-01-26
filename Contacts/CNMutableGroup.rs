//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNMutableGroup")]
    pub struct CNMutableGroup;

    #[cfg(feature = "Contacts_CNMutableGroup")]
    unsafe impl ClassType for CNMutableGroup {
        #[inherits(NSObject)]
        type Super = CNGroup;
    }
);

#[cfg(feature = "Contacts_CNMutableGroup")]
unsafe impl NSCoding for CNMutableGroup {}

#[cfg(feature = "Contacts_CNMutableGroup")]
unsafe impl NSObjectProtocol for CNMutableGroup {}

#[cfg(feature = "Contacts_CNMutableGroup")]
unsafe impl NSSecureCoding for CNMutableGroup {}

extern_methods!(
    #[cfg(feature = "Contacts_CNMutableGroup")]
    unsafe impl CNMutableGroup {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);
