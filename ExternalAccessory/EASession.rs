//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/externalaccessory/easession?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EASession;
);

unsafe impl NSObjectProtocol for EASession {}

extern_methods!(
    unsafe impl EASession {
        #[cfg(feature = "EAAccessory")]
        #[method_id(@__retain_semantics Init initWithAccessory:forProtocol:)]
        pub unsafe fn initWithAccessory_forProtocol(
            this: Allocated<Self>,
            accessory: &EAAccessory,
            protocol_string: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "EAAccessory")]
        #[method_id(@__retain_semantics Other accessory)]
        pub unsafe fn accessory(&self) -> Option<Retained<EAAccessory>>;

        #[method_id(@__retain_semantics Other protocolString)]
        pub unsafe fn protocolString(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other inputStream)]
        pub unsafe fn inputStream(&self) -> Option<Retained<NSInputStream>>;

        #[method_id(@__retain_semantics Other outputStream)]
        pub unsafe fn outputStream(&self) -> Option<Retained<NSOutputStream>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EASession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
