//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXSetGroupCallAction")]
    pub struct CXSetGroupCallAction;

    #[cfg(feature = "CallKit_CXSetGroupCallAction")]
    unsafe impl ClassType for CXSetGroupCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXSetGroupCallAction")]
unsafe impl NSCoding for CXSetGroupCallAction {}

#[cfg(feature = "CallKit_CXSetGroupCallAction")]
unsafe impl NSCopying for CXSetGroupCallAction {}

#[cfg(feature = "CallKit_CXSetGroupCallAction")]
unsafe impl NSObjectProtocol for CXSetGroupCallAction {}

#[cfg(feature = "CallKit_CXSetGroupCallAction")]
unsafe impl NSSecureCoding for CXSetGroupCallAction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXSetGroupCallAction")]
    unsafe impl CXSetGroupCallAction {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:callUUIDToGroupWith:)]
        pub unsafe fn initWithCallUUID_callUUIDToGroupWith(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
            call_uuid_to_group_with: Option<&NSUUID>,
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

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other callUUIDToGroupWith)]
        pub unsafe fn callUUIDToGroupWith(&self) -> Option<Id<NSUUID>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method(setCallUUIDToGroupWith:)]
        pub unsafe fn setCallUUIDToGroupWith(&self, call_uuid_to_group_with: Option<&NSUUID>);
    }
);
