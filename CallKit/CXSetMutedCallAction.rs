//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    pub struct CXSetMutedCallAction;

    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    unsafe impl ClassType for CXSetMutedCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
    }
);

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
unsafe impl NSCoding for CXSetMutedCallAction {}

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
unsafe impl NSObjectProtocol for CXSetMutedCallAction {}

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
unsafe impl NSSecureCoding for CXSetMutedCallAction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    unsafe impl CXSetMutedCallAction {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:muted:)]
        pub unsafe fn initWithCallUUID_muted(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
            muted: bool,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
        ) -> Id<Self, Shared>;

        #[method(isMuted)]
        pub unsafe fn isMuted(&self) -> bool;

        #[method(setMuted:)]
        pub unsafe fn setMuted(&self, muted: bool);
    }
);
