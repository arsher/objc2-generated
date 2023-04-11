//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CXPlayDTMFCallActionType {
        CXPlayDTMFCallActionTypeSingleTone = 1,
        CXPlayDTMFCallActionTypeSoftPause = 2,
        CXPlayDTMFCallActionTypeHardPause = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
    pub struct CXPlayDTMFCallAction;

    #[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
    unsafe impl ClassType for CXPlayDTMFCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
unsafe impl NSCoding for CXPlayDTMFCallAction {}

#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
unsafe impl NSCopying for CXPlayDTMFCallAction {}

#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
unsafe impl NSObjectProtocol for CXPlayDTMFCallAction {}

#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
unsafe impl NSSecureCoding for CXPlayDTMFCallAction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
    unsafe impl CXPlayDTMFCallAction {
        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUUID"))]
        #[method_id(@__retain_semantics Init initWithCallUUID:digits:type:)]
        pub unsafe fn initWithCallUUID_digits_type(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
            digits: &NSString,
            r#type: CXPlayDTMFCallActionType,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other digits)]
        pub unsafe fn digits(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDigits:)]
        pub unsafe fn setDigits(&self, digits: &NSString);

        #[method(type)]
        pub unsafe fn r#type(&self) -> CXPlayDTMFCallActionType;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: CXPlayDTMFCallActionType);
    }
);
