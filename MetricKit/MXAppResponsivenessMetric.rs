//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxappresponsivenessmetric?language=objc)
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXAppResponsivenessMetric;
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXAppResponsivenessMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXAppResponsivenessMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXAppResponsivenessMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXAppResponsivenessMetric {
        #[cfg(feature = "MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedApplicationHangTime)]
        pub unsafe fn histogrammedApplicationHangTime(
            &self,
        ) -> Retained<MXHistogram<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXAppResponsivenessMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
