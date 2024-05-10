//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DCDevice;

    unsafe impl ClassType for DCDevice {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for DCDevice {}

extern_methods!(
    unsafe impl DCDevice {
        #[method_id(@__retain_semantics Other currentDevice)]
        pub unsafe fn currentDevice() -> Id<DCDevice>;

        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;

        #[cfg(feature = "block2")]
        #[method(generateTokenWithCompletionHandler:)]
        pub unsafe fn generateTokenWithCompletionHandler(
            &self,
            completion: &block2::Block<dyn Fn(*mut NSData, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DCDevice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
