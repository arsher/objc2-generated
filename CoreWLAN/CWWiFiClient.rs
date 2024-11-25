//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corewlan/cweventdelegate?language=objc)
    pub unsafe trait CWEventDelegate {
        #[optional]
        #[method(clientConnectionInterrupted)]
        unsafe fn clientConnectionInterrupted(&self);

        #[optional]
        #[method(clientConnectionInvalidated)]
        unsafe fn clientConnectionInvalidated(&self);

        #[optional]
        #[method(powerStateDidChangeForWiFiInterfaceWithName:)]
        unsafe fn powerStateDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[optional]
        #[method(ssidDidChangeForWiFiInterfaceWithName:)]
        unsafe fn ssidDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[optional]
        #[method(bssidDidChangeForWiFiInterfaceWithName:)]
        unsafe fn bssidDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[optional]
        #[method(countryCodeDidChangeForWiFiInterfaceWithName:)]
        unsafe fn countryCodeDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[optional]
        #[method(linkDidChangeForWiFiInterfaceWithName:)]
        unsafe fn linkDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[optional]
        #[method(linkQualityDidChangeForWiFiInterfaceWithName:rssi:transmitRate:)]
        unsafe fn linkQualityDidChangeForWiFiInterfaceWithName_rssi_transmitRate(
            &self,
            interface_name: &NSString,
            rssi: NSInteger,
            transmit_rate: c_double,
        );

        #[optional]
        #[method(modeDidChangeForWiFiInterfaceWithName:)]
        unsafe fn modeDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[optional]
        #[method(scanCacheUpdatedForWiFiInterfaceWithName:)]
        unsafe fn scanCacheUpdatedForWiFiInterfaceWithName(&self, interface_name: &NSString);
    }

    unsafe impl ProtocolType for dyn CWEventDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwwificlient?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWWiFiClient;
);

unsafe impl NSObjectProtocol for CWWiFiClient {}

extern_methods!(
    unsafe impl CWWiFiClient {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other sharedWiFiClient)]
        pub unsafe fn sharedWiFiClient() -> Retained<CWWiFiClient>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CWInterface")]
        #[method_id(@__retain_semantics Other interface)]
        pub unsafe fn interface(&self) -> Option<Retained<CWInterface>>;

        #[method_id(@__retain_semantics Other interfaceNames)]
        pub unsafe fn interfaceNames(&self) -> Option<Retained<NSArray<NSString>>>;

        #[deprecated = "Use -[CWWiFiClient interfaceNames] instead"]
        #[method_id(@__retain_semantics Other interfaceNames)]
        pub unsafe fn interfaceNames_class() -> Option<Retained<NSArray<NSString>>>;

        #[cfg(feature = "CWInterface")]
        #[method_id(@__retain_semantics Other interfaceWithName:)]
        pub unsafe fn interfaceWithName(
            &self,
            interface_name: Option<&NSString>,
        ) -> Option<Retained<CWInterface>>;

        #[cfg(feature = "CWInterface")]
        #[method_id(@__retain_semantics Other interfaces)]
        pub unsafe fn interfaces(&self) -> Option<Retained<NSArray<CWInterface>>>;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(startMonitoringEventWithType:error:_)]
        pub unsafe fn startMonitoringEventWithType_error(
            &self,
            r#type: CWEventType,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(stopMonitoringEventWithType:error:_)]
        pub unsafe fn stopMonitoringEventWithType_error(
            &self,
            r#type: CWEventType,
        ) -> Result<(), Retained<NSError>>;

        #[method(stopMonitoringAllEventsAndReturnError:_)]
        pub unsafe fn stopMonitoringAllEventsAndReturnError(&self)
            -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWWiFiClient {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
