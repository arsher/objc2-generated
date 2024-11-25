//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelstructureprogrambinding?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelStructureProgramBinding;
);

unsafe impl Send for MLModelStructureProgramBinding {}

unsafe impl Sync for MLModelStructureProgramBinding {}

unsafe impl NSObjectProtocol for MLModelStructureProgramBinding {}

extern_methods!(
    unsafe impl MLModelStructureProgramBinding {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "MLModelStructureProgramValue")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Retained<MLModelStructureProgramValue>>;
    }
);
