//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mailkit/meencodedoutgoingmessage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEEncodedOutgoingMessage;
);

unsafe impl NSCoding for MEEncodedOutgoingMessage {}

unsafe impl NSObjectProtocol for MEEncodedOutgoingMessage {}

unsafe impl NSSecureCoding for MEEncodedOutgoingMessage {}

extern_methods!(
    unsafe impl MEEncodedOutgoingMessage {
        #[method_id(@__retain_semantics Init initWithRawData:isSigned:isEncrypted:)]
        pub unsafe fn initWithRawData_isSigned_isEncrypted(
            this: Allocated<Self>,
            raw_data: &NSData,
            is_signed: bool,
            is_encrypted: bool,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other rawData)]
        pub unsafe fn rawData(&self) -> Retained<NSData>;

        #[method(isSigned)]
        pub unsafe fn isSigned(&self) -> bool;

        #[method(isEncrypted)]
        pub unsafe fn isEncrypted(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MEEncodedOutgoingMessage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
