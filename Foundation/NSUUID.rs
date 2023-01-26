//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUUID")]
    pub struct NSUUID;

    #[cfg(feature = "Foundation_NSUUID")]
    unsafe impl ClassType for NSUUID {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSUUID")]
unsafe impl NSCoding for NSUUID {}

#[cfg(feature = "Foundation_NSUUID")]
unsafe impl NSObjectProtocol for NSUUID {}

#[cfg(feature = "Foundation_NSUUID")]
unsafe impl NSSecureCoding for NSUUID {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUUID")]
    unsafe impl NSUUID {
        #[method_id(@__retain_semantics Other UUID)]
        pub fn UUID() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithUUIDString:)]
        pub fn initWithUUIDString(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method(compare:)]
        pub unsafe fn compare(&self, other_uuid: &NSUUID) -> NSComparisonResult;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other UUIDString)]
        pub fn UUIDString(&self) -> Id<NSString, Shared>;
    }
);
