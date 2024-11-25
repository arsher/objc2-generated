//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidcharacteristicextendedpropertiesstring?language=objc)
    pub static CBUUIDCharacteristicExtendedPropertiesString: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidcharacteristicuserdescriptionstring?language=objc)
    pub static CBUUIDCharacteristicUserDescriptionString: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidclientcharacteristicconfigurationstring?language=objc)
    pub static CBUUIDClientCharacteristicConfigurationString: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidservercharacteristicconfigurationstring?language=objc)
    pub static CBUUIDServerCharacteristicConfigurationString: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidcharacteristicformatstring?language=objc)
    pub static CBUUIDCharacteristicFormatString: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidcharacteristicaggregateformatstring?language=objc)
    pub static CBUUIDCharacteristicAggregateFormatString: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidcharacteristicvalidrangestring?language=objc)
    pub static CBUUIDCharacteristicValidRangeString: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidcharacteristicobservationschedulestring?language=objc)
    pub static CBUUIDCharacteristicObservationScheduleString: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuidl2cappsmcharacteristicstring?language=objc)
    pub static CBUUIDL2CAPPSMCharacteristicString: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbuuid?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CBUUID;
);

unsafe impl NSCopying for CBUUID {}

unsafe impl CopyingHelper for CBUUID {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CBUUID {}

extern_methods!(
    unsafe impl CBUUID {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other UUIDString)]
        pub unsafe fn UUIDString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other UUIDWithString:)]
        pub unsafe fn UUIDWithString(the_string: &NSString) -> Retained<CBUUID>;

        #[method_id(@__retain_semantics Other UUIDWithData:)]
        pub unsafe fn UUIDWithData(the_data: &NSData) -> Retained<CBUUID>;

        #[method_id(@__retain_semantics Other UUIDWithNSUUID:)]
        pub unsafe fn UUIDWithNSUUID(the_uuid: &NSUUID) -> Retained<CBUUID>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CBUUID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
