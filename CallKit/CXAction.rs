//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXAction")]
    pub struct CXAction;

    #[cfg(feature = "CallKit_CXAction")]
    unsafe impl ClassType for CXAction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXAction")]
unsafe impl NSCoding for CXAction {}

#[cfg(feature = "CallKit_CXAction")]
unsafe impl NSCopying for CXAction {}

#[cfg(feature = "CallKit_CXAction")]
unsafe impl NSObjectProtocol for CXAction {}

#[cfg(feature = "CallKit_CXAction")]
unsafe impl NSSecureCoding for CXAction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXAction")]
    unsafe impl CXAction {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[method(isComplete)]
        pub unsafe fn isComplete(&self) -> bool;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timeoutDate)]
        pub unsafe fn timeoutDate(&self) -> Id<NSDate>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method(fulfill)]
        pub unsafe fn fulfill(&self);

        #[method(fail)]
        pub unsafe fn fail(&self);
    }
);
