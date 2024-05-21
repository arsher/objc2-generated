//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZBridgedNetworkInterface;

    unsafe impl ClassType for VZBridgedNetworkInterface {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for VZBridgedNetworkInterface {}

extern_methods!(
    unsafe impl VZBridgedNetworkInterface {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other networkInterfaces)]
        pub unsafe fn networkInterfaces() -> Retained<NSArray<VZBridgedNetworkInterface>>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedDisplayName)]
        pub unsafe fn localizedDisplayName(&self) -> Option<Retained<NSString>>;
    }
);
