//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXSignpostIntervalData")]
    pub struct MXSignpostIntervalData;

    #[cfg(feature = "MetricKit_MXSignpostIntervalData")]
    unsafe impl ClassType for MXSignpostIntervalData {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXSignpostIntervalData")]
unsafe impl NSCoding for MXSignpostIntervalData {}

#[cfg(feature = "MetricKit_MXSignpostIntervalData")]
unsafe impl NSObjectProtocol for MXSignpostIntervalData {}

#[cfg(feature = "MetricKit_MXSignpostIntervalData")]
unsafe impl NSSecureCoding for MXSignpostIntervalData {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXSignpostIntervalData")]
    unsafe impl MXSignpostIntervalData {
        #[cfg(all(
            feature = "Foundation_NSUnitDuration",
            feature = "MetricKit_MXHistogram"
        ))]
        #[method_id(@__retain_semantics Other histogrammedSignpostDuration)]
        pub unsafe fn histogrammedSignpostDuration(&self) -> Id<MXHistogram<NSUnitDuration>>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitDuration"
        ))]
        #[method_id(@__retain_semantics Other cumulativeCPUTime)]
        pub unsafe fn cumulativeCPUTime(&self) -> Option<Id<NSMeasurement<NSUnitDuration>>>;

        #[cfg(all(
            feature = "Foundation_NSUnitInformationStorage",
            feature = "MetricKit_MXAverage"
        ))]
        #[method_id(@__retain_semantics Other averageMemory)]
        pub unsafe fn averageMemory(&self) -> Option<Id<MXAverage<NSUnitInformationStorage>>>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        #[method_id(@__retain_semantics Other cumulativeLogicalWrites)]
        pub unsafe fn cumulativeLogicalWrites(
            &self,
        ) -> Option<Id<NSMeasurement<NSUnitInformationStorage>>>;

        #[cfg(all(feature = "Foundation_NSMeasurement", feature = "Foundation_NSUnit"))]
        #[method_id(@__retain_semantics Other cumulativeHitchTimeRatio)]
        pub unsafe fn cumulativeHitchTimeRatio(&self) -> Option<Id<NSMeasurement<NSUnit>>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXSignpostMetric")]
    pub struct MXSignpostMetric;

    #[cfg(feature = "MetricKit_MXSignpostMetric")]
    unsafe impl ClassType for MXSignpostMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXSignpostMetric")]
unsafe impl NSCoding for MXSignpostMetric {}

#[cfg(feature = "MetricKit_MXSignpostMetric")]
unsafe impl NSObjectProtocol for MXSignpostMetric {}

#[cfg(feature = "MetricKit_MXSignpostMetric")]
unsafe impl NSSecureCoding for MXSignpostMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXSignpostMetric")]
    unsafe impl MXSignpostMetric {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other signpostName)]
        pub unsafe fn signpostName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other signpostCategory)]
        pub unsafe fn signpostCategory(&self) -> Id<NSString>;

        #[cfg(feature = "MetricKit_MXSignpostIntervalData")]
        #[method_id(@__retain_semantics Other signpostIntervalData)]
        pub unsafe fn signpostIntervalData(&self) -> Option<Id<MXSignpostIntervalData>>;

        #[method(totalCount)]
        pub unsafe fn totalCount(&self) -> NSUInteger;
    }
);
