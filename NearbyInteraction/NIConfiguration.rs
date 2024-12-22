//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A type used to uniquely discover and identify a device in a nearby interaction session.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/nearbyinteraction/nidiscoverytoken?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NIDiscoveryToken;
);

unsafe impl NSCoding for NIDiscoveryToken {}

unsafe impl NSCopying for NIDiscoveryToken {}

unsafe impl CopyingHelper for NIDiscoveryToken {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NIDiscoveryToken {}

unsafe impl NSSecureCoding for NIDiscoveryToken {}

extern_methods!(
    unsafe impl NIDiscoveryToken {
        #[cfg(feature = "NIDeviceCapability")]
        /// Get the protocol that describes nearby interaction capabilities of the device that generated this token.
        ///
        /// Detailed description on the capability protocol is in NIDeviceCapability.h.
        #[method_id(@__retain_semantics Other deviceCapabilities)]
        pub unsafe fn deviceCapabilities(&self)
            -> Retained<ProtocolObject<dyn NIDeviceCapability>>;

        /// Unavailable
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object to describe and configure parameters to be used in a nearby interaction session.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/nearbyinteraction/niconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NIConfiguration;
);

unsafe impl NSCoding for NIConfiguration {}

unsafe impl NSCopying for NIConfiguration {}

unsafe impl CopyingHelper for NIConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NIConfiguration {}

unsafe impl NSSecureCoding for NIConfiguration {}

extern_methods!(
    unsafe impl NIConfiguration {
        /// Unavailable
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object to describe and configure parameters to be used in a nearby interaction session for mutual relative positional measurements.
    ///
    /// Devices engaged in a session run with an NINearbyPeerConfiguration are able to continuously generate positional measurements relative to one another.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/nearbyinteraction/ninearbypeerconfiguration?language=objc)
    #[unsafe(super(NIConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NINearbyPeerConfiguration;
);

unsafe impl NSCoding for NINearbyPeerConfiguration {}

unsafe impl NSCopying for NINearbyPeerConfiguration {}

unsafe impl CopyingHelper for NINearbyPeerConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NINearbyPeerConfiguration {}

unsafe impl NSSecureCoding for NINearbyPeerConfiguration {}

extern_methods!(
    unsafe impl NINearbyPeerConfiguration {
        /// The discovery token identifying the peer device for this session configuration.
        #[method_id(@__retain_semantics Other peerDiscoveryToken)]
        pub unsafe fn peerDiscoveryToken(&self) -> Retained<NIDiscoveryToken>;

        /// Initializes a new configuration with the provided peer token.
        ///
        /// Parameter `peerToken`: A discovery token received from the peer for this session.
        #[method_id(@__retain_semantics Init initWithPeerToken:)]
        pub unsafe fn initWithPeerToken(
            this: Allocated<Self>,
            peer_token: &NIDiscoveryToken,
        ) -> Retained<Self>;

        /// Enables camera assistance during the NISession run with this configuration
        ///
        /// :
        /// If true, optionally call setARSession: on the NISession before calling runWithConfiguration:
        /// If true and setARSession: is not called, an ARSession will automatically be created
        /// If true and the platform does not support camera assistance, the NISession will generate an error when runWithConfiguration: is called
        ///
        /// Note: : Check supportsCameraAssistance property in NIDeviceCapability returned from deviceCapabilities properties on NISession
        #[method(isCameraAssistanceEnabled)]
        pub unsafe fn isCameraAssistanceEnabled(&self) -> bool;

        /// Setter for [`isCameraAssistanceEnabled`][Self::isCameraAssistanceEnabled].
        #[method(setCameraAssistanceEnabled:)]
        pub unsafe fn setCameraAssistanceEnabled(&self, camera_assistance_enabled: bool);

        /// If both peers are capable, enables extended distance measurement for the NISession that runs with this configuration
        ///
        /// :
        /// If true, the NISession will use extended distance measurement capabilities while ranging with a peer that is also capable of extended distance measurement
        /// This property is compatible with the cameraAssistanceEnabled property
        ///
        /// Note: : Check supportsExtendedDistanceMeasurement property from deviceCapabilities properties on NISession and the deviceCapabilities property on the NIDiscoveryToken generated by the peer device to understand mutual capabilities
        #[method(isExtendedDistanceMeasurementEnabled)]
        pub unsafe fn isExtendedDistanceMeasurementEnabled(&self) -> bool;

        /// Setter for [`isExtendedDistanceMeasurementEnabled`][Self::isExtendedDistanceMeasurementEnabled].
        #[method(setExtendedDistanceMeasurementEnabled:)]
        pub unsafe fn setExtendedDistanceMeasurementEnabled(
            &self,
            extended_distance_measurement_enabled: bool,
        );

        /// Unavailable
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A session configuration that enables interaction with supported accessories.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/nearbyinteraction/ninearbyaccessoryconfiguration?language=objc)
    #[unsafe(super(NIConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NINearbyAccessoryConfiguration;
);

unsafe impl NSCoding for NINearbyAccessoryConfiguration {}

unsafe impl NSCopying for NINearbyAccessoryConfiguration {}

unsafe impl CopyingHelper for NINearbyAccessoryConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NINearbyAccessoryConfiguration {}

unsafe impl NSSecureCoding for NINearbyAccessoryConfiguration {}

extern_methods!(
    unsafe impl NINearbyAccessoryConfiguration {
        /// The discovery token identifying the accessory device for this session configuration.
        ///
        /// NINearbyObject updates for this accessory will contain this discovery token.
        #[method_id(@__retain_semantics Other accessoryDiscoveryToken)]
        pub unsafe fn accessoryDiscoveryToken(&self) -> Retained<NIDiscoveryToken>;

        /// Enables camera assistance during the NISession run with this configuration
        ///
        /// :
        /// If YES, optionally call -setARSession: on the NISession before calling -runWithConfiguration:
        /// If YES and setARSession: is not called, an ARSession will automatically be created
        /// If YES  and the platform does not support camera assistance, the NISession will generate an error when runWithConfiguration: is called
        ///
        /// Note: : Check supportsCameraAssistance property in NIDeviceCapability returned from deviceCapabilities properties on NISession
        #[method(isCameraAssistanceEnabled)]
        pub unsafe fn isCameraAssistanceEnabled(&self) -> bool;

        /// Setter for [`isCameraAssistanceEnabled`][Self::isCameraAssistanceEnabled].
        #[method(setCameraAssistanceEnabled:)]
        pub unsafe fn setCameraAssistanceEnabled(&self, camera_assistance_enabled: bool);

        /// Create a new nearby accessory configuration using data received from the accessory.
        ///
        /// Parameter `data`: Configuration data received from the accessory.
        ///
        /// Parameter `error`: An optional out error parameter that will be populated with an error if the provided data is invalid or unsupported.
        #[method_id(@__retain_semantics Init initWithData:error:_)]
        pub unsafe fn initWithData_error(
            this: Allocated<Self>,
            data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        /// Create a new nearby accessory configuration for an accessory that is also a paired Bluetooth device
        ///
        ///
        /// Parameter `accessoryData`: Configuration data received from the accessory
        ///
        /// Parameter `bluetoothPeerIdentifier`: The accessory's Bluetooth identifier
        ///
        /// Parameter `error`: An optional out error parameter that will be populated with an error if the provided inputs are invalid or unsupported.
        ///
        ///
        /// The accessory must be a Bluetooth LE peripheral that is paired, actively connected, and implements the Nearby Interaction Service and Accessory Configuration Characteristic.
        #[method_id(@__retain_semantics Init initWithAccessoryData:bluetoothPeerIdentifier:error:_)]
        pub unsafe fn initWithAccessoryData_bluetoothPeerIdentifier_error(
            this: Allocated<Self>,
            accessory_data: &NSData,
            identifier: &NSUUID,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        /// Unavailable
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
