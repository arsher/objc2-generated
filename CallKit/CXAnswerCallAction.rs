//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    pub struct CXAnswerCallAction;

    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    unsafe impl ClassType for CXAnswerCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
    }
);

extern_methods!(
    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    unsafe impl CXAnswerCallAction {
        #[cfg(feature = "Foundation_NSDate")]
        #[method(fulfillWithDateConnected:)]
        pub unsafe fn fulfillWithDateConnected(&self, date_connected: &NSDate);
    }
);

extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    unsafe impl CXAnswerCallAction {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
        ) -> Id<Self, Shared>;
    }
);
