//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioConsolePortConfigurationArray;

    unsafe impl ClassType for VZVirtioConsolePortConfigurationArray {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for VZVirtioConsolePortConfigurationArray {}

unsafe impl NSObjectProtocol for VZVirtioConsolePortConfigurationArray {}

extern_methods!(
    unsafe impl VZVirtioConsolePortConfigurationArray {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(maximumPortCount)]
        pub unsafe fn maximumPortCount(&self) -> u32;

        #[method(setMaximumPortCount:)]
        pub unsafe fn setMaximumPortCount(&self, maximum_port_count: u32);

        #[cfg(all(
            feature = "VZConsolePortConfiguration",
            feature = "VZVirtioConsolePortConfiguration"
        ))]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            port_index: NSUInteger,
        ) -> Option<Retained<VZVirtioConsolePortConfiguration>>;

        #[cfg(all(
            feature = "VZConsolePortConfiguration",
            feature = "VZVirtioConsolePortConfiguration"
        ))]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            configuration: Option<&VZVirtioConsolePortConfiguration>,
            port_index: NSUInteger,
        );
    }
);
