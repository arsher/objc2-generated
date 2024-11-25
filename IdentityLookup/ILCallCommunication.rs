//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilcallcommunication?language=objc)
    #[unsafe(super(ILCommunication, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ILCommunication")]
    pub struct ILCallCommunication;
);

#[cfg(feature = "ILCommunication")]
unsafe impl NSCoding for ILCallCommunication {}

#[cfg(feature = "ILCommunication")]
unsafe impl NSObjectProtocol for ILCallCommunication {}

#[cfg(feature = "ILCommunication")]
unsafe impl NSSecureCoding for ILCallCommunication {}

extern_methods!(
    #[cfg(feature = "ILCommunication")]
    unsafe impl ILCallCommunication {
        #[method(isEqualToCallCommunication:)]
        pub unsafe fn isEqualToCallCommunication(
            &self,
            communication: &ILCallCommunication,
        ) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "ILCommunication")]
    unsafe impl ILCallCommunication {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
