//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbperipheralstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CBPeripheralState(pub NSInteger);
impl CBPeripheralState {
    #[doc(alias = "CBPeripheralStateDisconnected")]
    pub const Disconnected: Self = Self(0);
    #[doc(alias = "CBPeripheralStateConnecting")]
    pub const Connecting: Self = Self(1);
    #[doc(alias = "CBPeripheralStateConnected")]
    pub const Connected: Self = Self(2);
    #[doc(alias = "CBPeripheralStateDisconnecting")]
    pub const Disconnecting: Self = Self(3);
}

unsafe impl Encode for CBPeripheralState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CBPeripheralState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbcharacteristicwritetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CBCharacteristicWriteType(pub NSInteger);
impl CBCharacteristicWriteType {
    pub const CBCharacteristicWriteWithResponse: Self = Self(0);
    pub const CBCharacteristicWriteWithoutResponse: Self = Self(1);
}

unsafe impl Encode for CBCharacteristicWriteType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CBCharacteristicWriteType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbperipheral?language=objc)
    #[unsafe(super(CBPeer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CBPeer")]
    pub struct CBPeripheral;
);

#[cfg(feature = "CBPeer")]
unsafe impl NSCopying for CBPeripheral {}

#[cfg(feature = "CBPeer")]
unsafe impl CopyingHelper for CBPeripheral {
    type Result = Self;
}

#[cfg(feature = "CBPeer")]
unsafe impl NSObjectProtocol for CBPeripheral {}

extern_methods!(
    #[cfg(feature = "CBPeer")]
    unsafe impl CBPeripheral {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Retained<ProtocolObject<dyn CBPeripheralDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CBPeripheralDelegate>>,
        );

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other RSSI)]
        pub unsafe fn RSSI(&self) -> Option<Retained<NSNumber>>;

        #[method(state)]
        pub unsafe fn state(&self) -> CBPeripheralState;

        #[cfg(all(feature = "CBAttribute", feature = "CBService"))]
        #[method_id(@__retain_semantics Other services)]
        pub unsafe fn services(&self) -> Option<Retained<NSArray<CBService>>>;

        #[method(canSendWriteWithoutResponse)]
        pub unsafe fn canSendWriteWithoutResponse(&self) -> bool;

        #[method(ancsAuthorized)]
        pub unsafe fn ancsAuthorized(&self) -> bool;

        #[method(readRSSI)]
        pub unsafe fn readRSSI(&self);

        #[cfg(feature = "CBUUID")]
        #[method(discoverServices:)]
        pub unsafe fn discoverServices(&self, service_uui_ds: Option<&NSArray<CBUUID>>);

        #[cfg(all(feature = "CBAttribute", feature = "CBService", feature = "CBUUID"))]
        #[method(discoverIncludedServices:forService:)]
        pub unsafe fn discoverIncludedServices_forService(
            &self,
            included_service_uui_ds: Option<&NSArray<CBUUID>>,
            service: &CBService,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBService", feature = "CBUUID"))]
        #[method(discoverCharacteristics:forService:)]
        pub unsafe fn discoverCharacteristics_forService(
            &self,
            characteristic_uui_ds: Option<&NSArray<CBUUID>>,
            service: &CBService,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBCharacteristic"))]
        #[method(readValueForCharacteristic:)]
        pub unsafe fn readValueForCharacteristic(&self, characteristic: &CBCharacteristic);

        #[method(maximumWriteValueLengthForType:)]
        pub unsafe fn maximumWriteValueLengthForType(
            &self,
            r#type: CBCharacteristicWriteType,
        ) -> NSUInteger;

        #[cfg(all(feature = "CBAttribute", feature = "CBCharacteristic"))]
        #[method(writeValue:forCharacteristic:type:)]
        pub unsafe fn writeValue_forCharacteristic_type(
            &self,
            data: &NSData,
            characteristic: &CBCharacteristic,
            r#type: CBCharacteristicWriteType,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBCharacteristic"))]
        #[method(setNotifyValue:forCharacteristic:)]
        pub unsafe fn setNotifyValue_forCharacteristic(
            &self,
            enabled: bool,
            characteristic: &CBCharacteristic,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBCharacteristic"))]
        #[method(discoverDescriptorsForCharacteristic:)]
        pub unsafe fn discoverDescriptorsForCharacteristic(
            &self,
            characteristic: &CBCharacteristic,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBDescriptor"))]
        #[method(readValueForDescriptor:)]
        pub unsafe fn readValueForDescriptor(&self, descriptor: &CBDescriptor);

        #[cfg(all(feature = "CBAttribute", feature = "CBDescriptor"))]
        #[method(writeValue:forDescriptor:)]
        pub unsafe fn writeValue_forDescriptor(&self, data: &NSData, descriptor: &CBDescriptor);

        #[cfg(feature = "CBL2CAPChannel")]
        #[method(openL2CAPChannel:)]
        pub unsafe fn openL2CAPChannel(&self, psm: CBL2CAPPSM);
    }
);

extern_methods!(
    /// Methods declared on superclass `CBPeer`
    #[cfg(feature = "CBPeer")]
    unsafe impl CBPeripheral {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CBPeer")]
    unsafe impl CBPeripheral {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbperipheraldelegate?language=objc)
    pub unsafe trait CBPeripheralDelegate: NSObjectProtocol {
        #[cfg(feature = "CBPeer")]
        #[optional]
        #[method(peripheralDidUpdateName:)]
        unsafe fn peripheralDidUpdateName(&self, peripheral: &CBPeripheral);

        #[cfg(all(feature = "CBAttribute", feature = "CBPeer", feature = "CBService"))]
        #[optional]
        #[method(peripheral:didModifyServices:)]
        unsafe fn peripheral_didModifyServices(
            &self,
            peripheral: &CBPeripheral,
            invalidated_services: &NSArray<CBService>,
        );

        #[cfg(feature = "CBPeer")]
        #[deprecated]
        #[optional]
        #[method(peripheralDidUpdateRSSI:error:)]
        unsafe fn peripheralDidUpdateRSSI_error(
            &self,
            peripheral: &CBPeripheral,
            error: Option<&NSError>,
        );

        #[cfg(feature = "CBPeer")]
        #[optional]
        #[method(peripheral:didReadRSSI:error:)]
        unsafe fn peripheral_didReadRSSI_error(
            &self,
            peripheral: &CBPeripheral,
            rssi: &NSNumber,
            error: Option<&NSError>,
        );

        #[cfg(feature = "CBPeer")]
        #[optional]
        #[method(peripheral:didDiscoverServices:)]
        unsafe fn peripheral_didDiscoverServices(
            &self,
            peripheral: &CBPeripheral,
            error: Option<&NSError>,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBPeer", feature = "CBService"))]
        #[optional]
        #[method(peripheral:didDiscoverIncludedServicesForService:error:)]
        unsafe fn peripheral_didDiscoverIncludedServicesForService_error(
            &self,
            peripheral: &CBPeripheral,
            service: &CBService,
            error: Option<&NSError>,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBPeer", feature = "CBService"))]
        #[optional]
        #[method(peripheral:didDiscoverCharacteristicsForService:error:)]
        unsafe fn peripheral_didDiscoverCharacteristicsForService_error(
            &self,
            peripheral: &CBPeripheral,
            service: &CBService,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "CBAttribute",
            feature = "CBCharacteristic",
            feature = "CBPeer"
        ))]
        #[optional]
        #[method(peripheral:didUpdateValueForCharacteristic:error:)]
        unsafe fn peripheral_didUpdateValueForCharacteristic_error(
            &self,
            peripheral: &CBPeripheral,
            characteristic: &CBCharacteristic,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "CBAttribute",
            feature = "CBCharacteristic",
            feature = "CBPeer"
        ))]
        #[optional]
        #[method(peripheral:didWriteValueForCharacteristic:error:)]
        unsafe fn peripheral_didWriteValueForCharacteristic_error(
            &self,
            peripheral: &CBPeripheral,
            characteristic: &CBCharacteristic,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "CBAttribute",
            feature = "CBCharacteristic",
            feature = "CBPeer"
        ))]
        #[optional]
        #[method(peripheral:didUpdateNotificationStateForCharacteristic:error:)]
        unsafe fn peripheral_didUpdateNotificationStateForCharacteristic_error(
            &self,
            peripheral: &CBPeripheral,
            characteristic: &CBCharacteristic,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "CBAttribute",
            feature = "CBCharacteristic",
            feature = "CBPeer"
        ))]
        #[optional]
        #[method(peripheral:didDiscoverDescriptorsForCharacteristic:error:)]
        unsafe fn peripheral_didDiscoverDescriptorsForCharacteristic_error(
            &self,
            peripheral: &CBPeripheral,
            characteristic: &CBCharacteristic,
            error: Option<&NSError>,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBDescriptor", feature = "CBPeer"))]
        #[optional]
        #[method(peripheral:didUpdateValueForDescriptor:error:)]
        unsafe fn peripheral_didUpdateValueForDescriptor_error(
            &self,
            peripheral: &CBPeripheral,
            descriptor: &CBDescriptor,
            error: Option<&NSError>,
        );

        #[cfg(all(feature = "CBAttribute", feature = "CBDescriptor", feature = "CBPeer"))]
        #[optional]
        #[method(peripheral:didWriteValueForDescriptor:error:)]
        unsafe fn peripheral_didWriteValueForDescriptor_error(
            &self,
            peripheral: &CBPeripheral,
            descriptor: &CBDescriptor,
            error: Option<&NSError>,
        );

        #[cfg(feature = "CBPeer")]
        #[optional]
        #[method(peripheralIsReadyToSendWriteWithoutResponse:)]
        unsafe fn peripheralIsReadyToSendWriteWithoutResponse(&self, peripheral: &CBPeripheral);

        #[cfg(all(feature = "CBL2CAPChannel", feature = "CBPeer"))]
        #[optional]
        #[method(peripheral:didOpenL2CAPChannel:error:)]
        unsafe fn peripheral_didOpenL2CAPChannel_error(
            &self,
            peripheral: &CBPeripheral,
            channel: Option<&CBL2CAPChannel>,
            error: Option<&NSError>,
        );
    }

    unsafe impl ProtocolType for dyn CBPeripheralDelegate {}
);
