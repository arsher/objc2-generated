//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxplaydtmfcallactiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CXPlayDTMFCallActionType(pub NSInteger);
impl CXPlayDTMFCallActionType {
    #[doc(alias = "CXPlayDTMFCallActionTypeSingleTone")]
    pub const SingleTone: Self = Self(1);
    #[doc(alias = "CXPlayDTMFCallActionTypeSoftPause")]
    pub const SoftPause: Self = Self(2);
    #[doc(alias = "CXPlayDTMFCallActionTypeHardPause")]
    pub const HardPause: Self = Self(3);
}

unsafe impl Encode for CXPlayDTMFCallActionType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CXPlayDTMFCallActionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxplaydtmfcallaction?language=objc)
    #[unsafe(super(CXCallAction, CXAction, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    pub struct CXPlayDTMFCallAction;
);

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSCoding for CXPlayDTMFCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSCopying for CXPlayDTMFCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl CopyingHelper for CXPlayDTMFCallAction {
    type Result = Self;
}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSObjectProtocol for CXPlayDTMFCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSSecureCoding for CXPlayDTMFCallAction {}

extern_methods!(
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXPlayDTMFCallAction {
        #[method_id(@__retain_semantics Init initWithCallUUID:digits:type:)]
        pub unsafe fn initWithCallUUID_digits_type(
            this: Allocated<Self>,
            call_uuid: &NSUUID,
            digits: &NSString,
            r#type: CXPlayDTMFCallActionType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(this: Allocated<Self>, call_uuid: &NSUUID)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Other digits)]
        pub unsafe fn digits(&self) -> Retained<NSString>;

        #[method(setDigits:)]
        pub unsafe fn setDigits(&self, digits: &NSString);

        #[method(type)]
        pub unsafe fn r#type(&self) -> CXPlayDTMFCallActionType;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: CXPlayDTMFCallActionType);
    }
);

extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXPlayDTMFCallAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXPlayDTMFCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
