//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An MXMetric subclass that encapsulates disk IO metrics.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxdiskiometric?language=objc)
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXDiskIOMetric;
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXDiskIOMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXDiskIOMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXDiskIOMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXDiskIOMetric {
        /// Cumulative amount of logical writes.
        ///
        /// Dimensioned as NSUnitInformationStorage.
        #[method_id(@__retain_semantics Other cumulativeLogicalWrites)]
        pub unsafe fn cumulativeLogicalWrites(
            &self,
        ) -> Retained<NSMeasurement<NSUnitInformationStorage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXDiskIOMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
