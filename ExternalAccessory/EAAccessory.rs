//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/externalaccessory/eaconnectionidnone?language=objc)
pub const EAConnectionIDNone: c_uint = 0;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/externalaccessory/eaaccessory?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EAAccessory;
);

unsafe impl NSObjectProtocol for EAAccessory {}

extern_methods!(
    unsafe impl EAAccessory {
        #[method(isConnected)]
        pub unsafe fn isConnected(&self) -> bool;

        #[method(connectionID)]
        pub unsafe fn connectionID(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other manufacturer)]
        pub unsafe fn manufacturer(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other modelNumber)]
        pub unsafe fn modelNumber(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other serialNumber)]
        pub unsafe fn serialNumber(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other firmwareRevision)]
        pub unsafe fn firmwareRevision(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other hardwareRevision)]
        pub unsafe fn hardwareRevision(&self) -> Retained<NSString>;

        #[deprecated = "Not supported"]
        #[method_id(@__retain_semantics Other dockType)]
        pub unsafe fn dockType(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other protocolStrings)]
        pub unsafe fn protocolStrings(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn EAAccessoryDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EAAccessoryDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EAAccessory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/externalaccessory/eaaccessorydelegate?language=objc)
    pub unsafe trait EAAccessoryDelegate: NSObjectProtocol {
        #[optional]
        #[method(accessoryDidDisconnect:)]
        unsafe fn accessoryDidDisconnect(&self, accessory: &EAAccessory);
    }

    unsafe impl ProtocolType for dyn EAAccessoryDelegate {}
);
