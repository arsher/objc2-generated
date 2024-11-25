//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelstructureprogramvalue?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelStructureProgramValue;
);

unsafe impl Send for MLModelStructureProgramValue {}

unsafe impl Sync for MLModelStructureProgramValue {}

unsafe impl NSObjectProtocol for MLModelStructureProgramValue {}

extern_methods!(
    unsafe impl MLModelStructureProgramValue {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
