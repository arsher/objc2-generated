//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXCellularConditionMetric;

    #[cfg(feature = "MXMetric")]
    unsafe impl ClassType for MXCellularConditionMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXCellularConditionMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXCellularConditionMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXCellularConditionMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXCellularConditionMetric {
        #[cfg(all(feature = "MXHistogram", feature = "MXUnit"))]
        #[method_id(@__retain_semantics Other histogrammedCellularConditionTime)]
        pub unsafe fn histogrammedCellularConditionTime(
            &self,
        ) -> Retained<MXHistogram<MXUnitSignalBars>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXCellularConditionMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
