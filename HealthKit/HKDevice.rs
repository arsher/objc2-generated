//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeyname?language=objc)
    pub static HKDevicePropertyKeyName: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeymanufacturer?language=objc)
    pub static HKDevicePropertyKeyManufacturer: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeymodel?language=objc)
    pub static HKDevicePropertyKeyModel: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeyhardwareversion?language=objc)
    pub static HKDevicePropertyKeyHardwareVersion: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeyfirmwareversion?language=objc)
    pub static HKDevicePropertyKeyFirmwareVersion: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeysoftwareversion?language=objc)
    pub static HKDevicePropertyKeySoftwareVersion: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeylocalidentifier?language=objc)
    pub static HKDevicePropertyKeyLocalIdentifier: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeyudideviceidentifier?language=objc)
    pub static HKDevicePropertyKeyUDIDeviceIdentifier: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKDevice;
);

unsafe impl Send for HKDevice {}

unsafe impl Sync for HKDevice {}

unsafe impl NSCoding for HKDevice {}

unsafe impl NSCopying for HKDevice {}

unsafe impl CopyingHelper for HKDevice {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKDevice {}

unsafe impl NSSecureCoding for HKDevice {}

extern_methods!(
    unsafe impl HKDevice {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other manufacturer)]
        pub unsafe fn manufacturer(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other model)]
        pub unsafe fn model(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other hardwareVersion)]
        pub unsafe fn hardwareVersion(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other firmwareVersion)]
        pub unsafe fn firmwareVersion(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other softwareVersion)]
        pub unsafe fn softwareVersion(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other localIdentifier)]
        pub unsafe fn localIdentifier(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other UDIDeviceIdentifier)]
        pub unsafe fn UDIDeviceIdentifier(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Init initWithName:manufacturer:model:hardwareVersion:firmwareVersion:softwareVersion:localIdentifier:UDIDeviceIdentifier:)]
        pub unsafe fn initWithName_manufacturer_model_hardwareVersion_firmwareVersion_softwareVersion_localIdentifier_UDIDeviceIdentifier(
            this: Allocated<Self>,
            name: Option<&NSString>,
            manufacturer: Option<&NSString>,
            model: Option<&NSString>,
            hardware_version: Option<&NSString>,
            firmware_version: Option<&NSString>,
            software_version: Option<&NSString>,
            local_identifier: Option<&NSString>,
            udi_device_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other localDevice)]
        pub unsafe fn localDevice() -> Retained<HKDevice>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
