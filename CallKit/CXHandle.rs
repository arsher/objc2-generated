//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CXHandleType {
        CXHandleTypeGeneric = 1,
        CXHandleTypePhoneNumber = 2,
        CXHandleTypeEmailAddress = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXHandle")]
    pub struct CXHandle;

    #[cfg(feature = "CallKit_CXHandle")]
    unsafe impl ClassType for CXHandle {
        type Super = NSObject;
    }
);

#[cfg(feature = "CallKit_CXHandle")]
unsafe impl NSCoding for CXHandle {}

#[cfg(feature = "CallKit_CXHandle")]
unsafe impl NSObjectProtocol for CXHandle {}

#[cfg(feature = "CallKit_CXHandle")]
unsafe impl NSSecureCoding for CXHandle {}

extern_methods!(
    #[cfg(feature = "CallKit_CXHandle")]
    unsafe impl CXHandle {
        #[method(type)]
        pub unsafe fn r#type(&self) -> CXHandleType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithType:value:)]
        pub unsafe fn initWithType_value(
            this: Option<Allocated<Self>>,
            r#type: CXHandleType,
            value: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(isEqualToHandle:)]
        pub unsafe fn isEqualToHandle(&self, handle: &CXHandle) -> bool;
    }
);
