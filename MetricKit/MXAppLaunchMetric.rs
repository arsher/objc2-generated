//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxapplaunchmetric?language=objc)
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXAppLaunchMetric;
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXAppLaunchMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXAppLaunchMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXAppLaunchMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXAppLaunchMetric {
        #[cfg(feature = "MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedTimeToFirstDraw)]
        pub unsafe fn histogrammedTimeToFirstDraw(&self) -> Retained<MXHistogram<NSUnitDuration>>;

        #[cfg(feature = "MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedApplicationResumeTime)]
        pub unsafe fn histogrammedApplicationResumeTime(
            &self,
        ) -> Retained<MXHistogram<NSUnitDuration>>;

        #[cfg(feature = "MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedOptimizedTimeToFirstDraw)]
        pub unsafe fn histogrammedOptimizedTimeToFirstDraw(
            &self,
        ) -> Retained<MXHistogram<NSUnitDuration>>;

        #[cfg(feature = "MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedExtendedLaunch)]
        pub unsafe fn histogrammedExtendedLaunch(&self) -> Retained<MXHistogram<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXAppLaunchMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
