//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxanswercallaction?language=objc)
    #[unsafe(super(CXCallAction, CXAction, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    pub struct CXAnswerCallAction;
);

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSCoding for CXAnswerCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSCopying for CXAnswerCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl CopyingHelper for CXAnswerCallAction {
    type Result = Self;
}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSObjectProtocol for CXAnswerCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSSecureCoding for CXAnswerCallAction {}

extern_methods!(
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXAnswerCallAction {
        #[method(fulfillWithDateConnected:)]
        pub unsafe fn fulfillWithDateConnected(&self, date_connected: &NSDate);
    }
);

extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXAnswerCallAction {
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(this: Allocated<Self>, call_uuid: &NSUUID)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXAnswerCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
