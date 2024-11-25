//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckreferenceaction?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKReferenceAction(pub NSUInteger);
impl CKReferenceAction {
    #[doc(alias = "CKReferenceActionNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "CKReferenceActionDeleteSelf")]
    pub const DeleteSelf: Self = Self(1);
}

unsafe impl Encode for CKReferenceAction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CKReferenceAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckreference?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKReference;
);

unsafe impl Send for CKReference {}

unsafe impl Sync for CKReference {}

unsafe impl NSCoding for CKReference {}

unsafe impl NSCopying for CKReference {}

unsafe impl CopyingHelper for CKReference {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKReference {}

unsafe impl NSSecureCoding for CKReference {}

extern_methods!(
    unsafe impl CKReference {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Init initWithRecordID:action:)]
        pub unsafe fn initWithRecordID_action(
            this: Allocated<Self>,
            record_id: &CKRecordID,
            action: CKReferenceAction,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Init initWithRecord:action:)]
        pub unsafe fn initWithRecord_action(
            this: Allocated<Self>,
            record: &CKRecord,
            action: CKReferenceAction,
        ) -> Retained<Self>;

        #[method(referenceAction)]
        pub unsafe fn referenceAction(&self) -> CKReferenceAction;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other recordID)]
        pub unsafe fn recordID(&self) -> Retained<CKRecordID>;
    }
);
