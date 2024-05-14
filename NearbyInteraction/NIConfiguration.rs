//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NIDiscoveryToken;

    unsafe impl ClassType for NIDiscoveryToken {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NIDiscoveryToken {}

unsafe impl NSCopying for NIDiscoveryToken {}

unsafe impl NSObjectProtocol for NIDiscoveryToken {}

unsafe impl NSSecureCoding for NIDiscoveryToken {}

extern_methods!(
    unsafe impl NIDiscoveryToken {
        #[cfg(feature = "NIDeviceCapability")]
        #[method_id(@__retain_semantics Other deviceCapabilities)]
        pub unsafe fn deviceCapabilities(&self) -> Id<ProtocolObject<dyn NIDeviceCapability>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NIConfiguration;

    unsafe impl ClassType for NIConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NIConfiguration {}

unsafe impl NSCopying for NIConfiguration {}

unsafe impl NSObjectProtocol for NIConfiguration {}

unsafe impl NSSecureCoding for NIConfiguration {}

extern_methods!(
    unsafe impl NIConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NINearbyPeerConfiguration;

    unsafe impl ClassType for NINearbyPeerConfiguration {
        #[inherits(NSObject)]
        type Super = NIConfiguration;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NINearbyPeerConfiguration {}

unsafe impl NSCopying for NINearbyPeerConfiguration {}

unsafe impl NSObjectProtocol for NINearbyPeerConfiguration {}

unsafe impl NSSecureCoding for NINearbyPeerConfiguration {}

extern_methods!(
    unsafe impl NINearbyPeerConfiguration {
        #[method_id(@__retain_semantics Other peerDiscoveryToken)]
        pub unsafe fn peerDiscoveryToken(&self) -> Id<NIDiscoveryToken>;

        #[method_id(@__retain_semantics Init initWithPeerToken:)]
        pub unsafe fn initWithPeerToken(
            this: Allocated<Self>,
            peer_token: &NIDiscoveryToken,
        ) -> Id<Self>;

        #[method(isCameraAssistanceEnabled)]
        pub unsafe fn isCameraAssistanceEnabled(&self) -> bool;

        #[method(setCameraAssistanceEnabled:)]
        pub unsafe fn setCameraAssistanceEnabled(&self, camera_assistance_enabled: bool);

        #[method(isExtendedDistanceMeasurementEnabled)]
        pub unsafe fn isExtendedDistanceMeasurementEnabled(&self) -> bool;

        #[method(setExtendedDistanceMeasurementEnabled:)]
        pub unsafe fn setExtendedDistanceMeasurementEnabled(
            &self,
            extended_distance_measurement_enabled: bool,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NINearbyAccessoryConfiguration;

    unsafe impl ClassType for NINearbyAccessoryConfiguration {
        #[inherits(NSObject)]
        type Super = NIConfiguration;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NINearbyAccessoryConfiguration {}

unsafe impl NSCopying for NINearbyAccessoryConfiguration {}

unsafe impl NSObjectProtocol for NINearbyAccessoryConfiguration {}

unsafe impl NSSecureCoding for NINearbyAccessoryConfiguration {}

extern_methods!(
    unsafe impl NINearbyAccessoryConfiguration {
        #[method_id(@__retain_semantics Other accessoryDiscoveryToken)]
        pub unsafe fn accessoryDiscoveryToken(&self) -> Id<NIDiscoveryToken>;

        #[method(isCameraAssistanceEnabled)]
        pub unsafe fn isCameraAssistanceEnabled(&self) -> bool;

        #[method(setCameraAssistanceEnabled:)]
        pub unsafe fn setCameraAssistanceEnabled(&self, camera_assistance_enabled: bool);

        #[method_id(@__retain_semantics Init initWithData:error:_)]
        pub unsafe fn initWithData_error(
            this: Allocated<Self>,
            data: &NSData,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init initWithAccessoryData:bluetoothPeerIdentifier:error:_)]
        pub unsafe fn initWithAccessoryData_bluetoothPeerIdentifier_error(
            this: Allocated<Self>,
            accessory_data: &NSData,
            identifier: &NSUUID,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);