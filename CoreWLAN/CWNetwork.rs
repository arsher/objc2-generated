//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Represents a device participating in a Wi-Fi network, providing accessors to various network attributes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwnetwork?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWNetwork;
);

unsafe impl NSCoding for CWNetwork {}

unsafe impl NSCopying for CWNetwork {}

unsafe impl CopyingHelper for CWNetwork {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CWNetwork {}

unsafe impl NSSecureCoding for CWNetwork {}

extern_methods!(
    unsafe impl CWNetwork {
        /// Returns the service set identifier (SSID) for the Wi-Fi network device, encoded as a string.
        ///
        ///
        /// Returns nil if the SSID can not be encoded as a valid UTF-8 or WinLatin1 string.
        ///
        ///
        /// Note: SSID information is not available unless Location Services is enabled and the user has authorized the calling app to use location services.
        ///
        ///
        /// See also: CLLocationManager
        #[method_id(@__retain_semantics Other ssid)]
        pub unsafe fn ssid(&self) -> Option<Retained<NSString>>;

        /// Returns the service set identifier (SSID) for the Wi-Fi network device, encapsulated in an NSData object.
        ///
        ///
        /// The SSID is defined as 1-32 octets.
        ///
        ///
        /// Note: SSID information is not available unless Location Services is enabled and the user has authorized the calling app to use location services.
        ///
        ///
        /// See also: CLLocationManager
        #[method_id(@__retain_semantics Other ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Retained<NSData>>;

        /// Returns the basic service set identifier (BSSID) for the Wi-Fi network device, returned as UTF-8 string.
        ///
        ///
        /// Returns a UTF-8 string using hexadecimal characters formatted as XX:XX:XX:XX:XX:XX.
        ///
        ///
        /// Note: BSSID information is not available unless Location Services is enabled and the user has authorized the calling app to use location services.
        ///
        ///
        /// See also: CLLocationManager
        #[method_id(@__retain_semantics Other bssid)]
        pub unsafe fn bssid(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "CWChannel")]
        /// The operating channel of the Wi-Fi device.
        #[method_id(@__retain_semantics Other wlanChannel)]
        pub unsafe fn wlanChannel(&self) -> Option<Retained<CWChannel>>;

        /// Returns the received signal strength indication (RSSI) measurement (dBm) for the Wi-Fi device.
        #[method(rssiValue)]
        pub unsafe fn rssiValue(&self) -> NSInteger;

        /// Returns the noise measurement (dBm) for the Wi-Fi device.
        #[method(noiseMeasurement)]
        pub unsafe fn noiseMeasurement(&self) -> NSInteger;

        /// Returns information element data included in beacon or probe response frames.
        #[method_id(@__retain_semantics Other informationElementData)]
        pub unsafe fn informationElementData(&self) -> Option<Retained<NSData>>;

        /// Returns the advertised country code (ISO/IEC 3166-1:1997) for the Wi-Fi device.
        ///
        ///
        /// Note: Country code information is not available unless Location Services is enabled and the user has authorized the calling app to use location services.
        ///
        ///
        /// See also: CLLocationManager
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Retained<NSString>>;

        /// Returns the beacon interval (ms) for the Wi-Fi device.
        #[method(beaconInterval)]
        pub unsafe fn beaconInterval(&self) -> NSInteger;

        /// Returns: YES if the Wi-Fi device is part of an IBSS network, NO otherwise.
        ///
        ///
        /// Indicates whether or not the Wi-Fi device is participating in an independent basic service set (IBSS), or ad-hoc Wi-Fi network.
        #[method(ibss)]
        pub unsafe fn ibss(&self) -> bool;

        /// Parameter `network`: A CWNetwork object.
        ///
        ///
        /// Returns: YES if the objects are equal, NO otherwise.
        ///
        ///
        /// Determine CWNetwork equality.
        ///
        ///
        /// CWNetwork objects are considered equal if their corresponding
        /// <i>
        /// ssidData
        /// </i>
        /// and
        /// <i>
        /// bssid
        /// </i>
        /// properties are equal.
        #[method(isEqualToNetwork:)]
        pub unsafe fn isEqualToNetwork(&self, network: &CWNetwork) -> bool;

        #[cfg(feature = "CoreWLANTypes")]
        /// Parameter `security`: A CWSecurity type value.
        ///
        ///
        /// Returns: <i>
        /// YES
        /// </i>
        /// if the Wi-Fi device supports the specified security type,
        /// <i>
        /// NO
        /// </i>
        /// otherwise.
        ///
        ///
        /// Determine which security types a Wi-Fi device supports.
        #[method(supportsSecurity:)]
        pub unsafe fn supportsSecurity(&self, security: CWSecurity) -> bool;

        #[cfg(feature = "CoreWLANTypes")]
        /// Parameter `phyMode`: A CWPHYMode type value.
        ///
        ///
        /// Returns: YES if the Wi-Fi device supports the specified PHY mode, NO otherwise.
        ///
        ///
        /// Determine which PHY modes a Wi-Fi device supports.
        #[method(supportsPHYMode:)]
        pub unsafe fn supportsPHYMode(&self, phy_mode: CWPHYMode) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWNetwork {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
