//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

extern_methods!(
    /// CLVisitExtensions
    #[cfg(feature = "CLLocationManager")]
    unsafe impl CLLocationManager {
        #[method(startMonitoringVisits)]
        pub unsafe fn startMonitoringVisits(&self);

        #[method(stopMonitoringVisits)]
        pub unsafe fn stopMonitoringVisits(&self);
    }
);
