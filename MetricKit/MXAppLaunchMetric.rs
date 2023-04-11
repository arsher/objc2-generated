//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXAppLaunchMetric")]
    pub struct MXAppLaunchMetric;

    #[cfg(feature = "MetricKit_MXAppLaunchMetric")]
    unsafe impl ClassType for MXAppLaunchMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXAppLaunchMetric")]
unsafe impl NSCoding for MXAppLaunchMetric {}

#[cfg(feature = "MetricKit_MXAppLaunchMetric")]
unsafe impl NSObjectProtocol for MXAppLaunchMetric {}

#[cfg(feature = "MetricKit_MXAppLaunchMetric")]
unsafe impl NSSecureCoding for MXAppLaunchMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXAppLaunchMetric")]
    unsafe impl MXAppLaunchMetric {
        #[cfg(all(
            feature = "Foundation_NSUnitDuration",
            feature = "MetricKit_MXHistogram"
        ))]
        #[method_id(@__retain_semantics Other histogrammedTimeToFirstDraw)]
        pub unsafe fn histogrammedTimeToFirstDraw(&self) -> Id<MXHistogram<NSUnitDuration>>;

        #[cfg(all(
            feature = "Foundation_NSUnitDuration",
            feature = "MetricKit_MXHistogram"
        ))]
        #[method_id(@__retain_semantics Other histogrammedApplicationResumeTime)]
        pub unsafe fn histogrammedApplicationResumeTime(&self) -> Id<MXHistogram<NSUnitDuration>>;

        #[cfg(all(
            feature = "Foundation_NSUnitDuration",
            feature = "MetricKit_MXHistogram"
        ))]
        #[method_id(@__retain_semantics Other histogrammedOptimizedTimeToFirstDraw)]
        pub unsafe fn histogrammedOptimizedTimeToFirstDraw(
            &self,
        ) -> Id<MXHistogram<NSUnitDuration>>;

        #[cfg(all(
            feature = "Foundation_NSUnitDuration",
            feature = "MetricKit_MXHistogram"
        ))]
        #[method_id(@__retain_semantics Other histogrammedExtendedLaunch)]
        pub unsafe fn histogrammedExtendedLaunch(&self) -> Id<MXHistogram<NSUnitDuration>>;
    }
);
