//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxdisplaymetric?language=objc)
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXDisplayMetric;
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXDisplayMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXDisplayMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXDisplayMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXDisplayMetric {
        #[cfg(all(feature = "MXAverage", feature = "MXUnit"))]
        #[method_id(@__retain_semantics Other averagePixelLuminance)]
        pub unsafe fn averagePixelLuminance(
            &self,
        ) -> Option<Retained<MXAverage<MXUnitAveragePixelLuminance>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXDisplayMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
