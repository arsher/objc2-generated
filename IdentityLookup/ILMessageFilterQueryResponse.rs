//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilmessagefilterqueryresponse?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILMessageFilterQueryResponse;
);

unsafe impl NSCoding for ILMessageFilterQueryResponse {}

unsafe impl NSObjectProtocol for ILMessageFilterQueryResponse {}

unsafe impl NSSecureCoding for ILMessageFilterQueryResponse {}

extern_methods!(
    unsafe impl ILMessageFilterQueryResponse {
        #[cfg(feature = "ILMessageFilterAction")]
        #[method(action)]
        pub unsafe fn action(&self) -> ILMessageFilterAction;

        #[cfg(feature = "ILMessageFilterAction")]
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: ILMessageFilterAction);

        #[cfg(feature = "ILMessageFilterAction")]
        #[method(subAction)]
        pub unsafe fn subAction(&self) -> ILMessageFilterSubAction;

        #[cfg(feature = "ILMessageFilterAction")]
        #[method(setSubAction:)]
        pub unsafe fn setSubAction(&self, sub_action: ILMessageFilterSubAction);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ILMessageFilterQueryResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
