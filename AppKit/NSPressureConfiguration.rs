//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspressureconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPressureConfiguration;
);

unsafe impl NSObjectProtocol for NSPressureConfiguration {}

extern_methods!(
    unsafe impl NSPressureConfiguration {
        #[cfg(feature = "NSEvent")]
        #[method(pressureBehavior)]
        pub unsafe fn pressureBehavior(&self) -> NSPressureBehavior;

        #[cfg(feature = "NSEvent")]
        #[method_id(@__retain_semantics Init initWithPressureBehavior:)]
        pub unsafe fn initWithPressureBehavior(
            this: Allocated<Self>,
            pressure_behavior: NSPressureBehavior,
        ) -> Retained<Self>;

        #[method(set)]
        pub unsafe fn set(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPressureConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSPressureConfiguration
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method_id(@__retain_semantics Other pressureConfiguration)]
        pub unsafe fn pressureConfiguration(&self) -> Option<Retained<NSPressureConfiguration>>;

        #[method(setPressureConfiguration:)]
        pub unsafe fn setPressureConfiguration(
            &self,
            pressure_configuration: Option<&NSPressureConfiguration>,
        );
    }
);
