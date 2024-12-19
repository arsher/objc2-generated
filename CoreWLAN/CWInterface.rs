//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwinterface?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWInterface;
);

unsafe impl NSObjectProtocol for CWInterface {}

extern_methods!(
    unsafe impl CWInterface {
        #[method_id(@__retain_semantics Other interfaceName)]
        pub unsafe fn interfaceName(&self) -> Option<Retained<NSString>>;

        #[method(powerOn)]
        pub unsafe fn powerOn(&self) -> bool;

        #[cfg(feature = "CWChannel")]
        #[method_id(@__retain_semantics Other supportedWLANChannels)]
        pub unsafe fn supportedWLANChannels(&self) -> Option<Retained<NSSet<CWChannel>>>;

        #[cfg(feature = "CWChannel")]
        #[method_id(@__retain_semantics Other wlanChannel)]
        pub unsafe fn wlanChannel(&self) -> Option<Retained<CWChannel>>;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(activePHYMode)]
        pub unsafe fn activePHYMode(&self) -> CWPHYMode;

        #[method_id(@__retain_semantics Other ssid)]
        pub unsafe fn ssid(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other bssid)]
        pub unsafe fn bssid(&self) -> Option<Retained<NSString>>;

        #[method(rssiValue)]
        pub unsafe fn rssiValue(&self) -> NSInteger;

        #[method(noiseMeasurement)]
        pub unsafe fn noiseMeasurement(&self) -> NSInteger;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(security)]
        pub unsafe fn security(&self) -> CWSecurity;

        #[method(transmitRate)]
        pub unsafe fn transmitRate(&self) -> c_double;

        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(interfaceMode)]
        pub unsafe fn interfaceMode(&self) -> CWInterfaceMode;

        #[method(transmitPower)]
        pub unsafe fn transmitPower(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other hardwareAddress)]
        pub unsafe fn hardwareAddress(&self) -> Option<Retained<NSString>>;

        #[method(serviceActive)]
        pub unsafe fn serviceActive(&self) -> bool;

        #[cfg(feature = "CWNetwork")]
        #[method_id(@__retain_semantics Other cachedScanResults)]
        pub unsafe fn cachedScanResults(&self) -> Option<Retained<NSSet<CWNetwork>>>;

        #[cfg(feature = "CWConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Option<Retained<CWConfiguration>>;

        #[deprecated = "Use -[CWWiFiClient interfaceNames] instead"]
        #[method_id(@__retain_semantics Other interfaceNames)]
        pub unsafe fn interfaceNames() -> Option<Retained<NSSet<NSString>>>;

        #[deprecated = "Use -[CWWiFiClient interface] instead"]
        #[method_id(@__retain_semantics Other interface)]
        pub unsafe fn interface() -> Retained<Self>;

        #[deprecated = "Use -[CWWiFiClient interfaceWithName:] instead"]
        #[method_id(@__retain_semantics Other interfaceWithName:)]
        pub unsafe fn interfaceWithName(name: Option<&NSString>) -> Retained<Self>;

        #[deprecated = "Use -[CWWiFiClient interfaceWithName:] instead"]
        #[method_id(@__retain_semantics Init initWithInterfaceName:)]
        pub unsafe fn initWithInterfaceName(
            this: Allocated<Self>,
            name: Option<&NSString>,
        ) -> Retained<Self>;

        #[method(setPower:error:_)]
        pub unsafe fn setPower_error(&self, power: bool) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "CWChannel")]
        #[method(setWLANChannel:error:_)]
        pub unsafe fn setWLANChannel_error(
            &self,
            channel: &CWChannel,
        ) -> Result<(), Retained<NSError>>;

        #[method(setPairwiseMasterKey:error:_)]
        pub unsafe fn setPairwiseMasterKey_error(
            &self,
            key: Option<&NSData>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(setWEPKey:flags:index:error:_)]
        pub unsafe fn setWEPKey_flags_index_error(
            &self,
            key: Option<&NSData>,
            flags: CWCipherKeyFlags,
            index: NSInteger,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "CWNetwork")]
        #[method_id(@__retain_semantics Other scanForNetworksWithSSID:error:_)]
        pub unsafe fn scanForNetworksWithSSID_error(
            &self,
            ssid: Option<&NSData>,
        ) -> Result<Retained<NSSet<CWNetwork>>, Retained<NSError>>;

        #[cfg(feature = "CWNetwork")]
        #[method_id(@__retain_semantics Other scanForNetworksWithSSID:includeHidden:error:_)]
        pub unsafe fn scanForNetworksWithSSID_includeHidden_error(
            &self,
            ssid: Option<&NSData>,
            include_hidden: bool,
        ) -> Result<Retained<NSSet<CWNetwork>>, Retained<NSError>>;

        #[cfg(feature = "CWNetwork")]
        #[method_id(@__retain_semantics Other scanForNetworksWithName:error:_)]
        pub unsafe fn scanForNetworksWithName_error(
            &self,
            network_name: Option<&NSString>,
        ) -> Result<Retained<NSSet<CWNetwork>>, Retained<NSError>>;

        #[cfg(feature = "CWNetwork")]
        #[method_id(@__retain_semantics Other scanForNetworksWithName:includeHidden:error:_)]
        pub unsafe fn scanForNetworksWithName_includeHidden_error(
            &self,
            network_name: Option<&NSString>,
            include_hidden: bool,
        ) -> Result<Retained<NSSet<CWNetwork>>, Retained<NSError>>;

        #[cfg(feature = "CWNetwork")]
        #[method(associateToNetwork:password:error:_)]
        pub unsafe fn associateToNetwork_password_error(
            &self,
            network: &CWNetwork,
            password: Option<&NSString>,
        ) -> Result<(), Retained<NSError>>;

        #[method(disassociate)]
        pub unsafe fn disassociate(&self);

        #[cfg(feature = "CoreWLANTypes")]
        #[deprecated]
        #[method(startIBSSModeWithSSID:security:channel:password:error:_)]
        pub unsafe fn startIBSSModeWithSSID_security_channel_password_error(
            &self,
            ssid_data: &NSData,
            security: CWIBSSModeSecurity,
            channel: NSUInteger,
            password: Option<&NSString>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWInterface {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
