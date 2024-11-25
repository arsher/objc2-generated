//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxaction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXAction;
);

unsafe impl NSCoding for CXAction {}

unsafe impl NSCopying for CXAction {}

unsafe impl CopyingHelper for CXAction {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CXAction {}

unsafe impl NSSecureCoding for CXAction {}

extern_methods!(
    unsafe impl CXAction {
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[method(isComplete)]
        pub unsafe fn isComplete(&self) -> bool;

        #[method_id(@__retain_semantics Other timeoutDate)]
        pub unsafe fn timeoutDate(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(fulfill)]
        pub unsafe fn fulfill(&self);

        #[method(fail)]
        pub unsafe fn fail(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
