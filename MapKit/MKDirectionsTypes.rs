//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MKDirectionsTransportType {
        MKDirectionsTransportTypeAutomobile = 1 << 0,
        MKDirectionsTransportTypeWalking = 1 << 1,
        MKDirectionsTransportTypeTransit = 1 << 2,
        MKDirectionsTransportTypeAny = 0x0FFFFFFF,
    }
);