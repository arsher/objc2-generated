//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXMetricPayload;

    unsafe impl ClassType for MXMetricPayload {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MXMetricPayload {}

unsafe impl NSObjectProtocol for MXMetricPayload {}

unsafe impl NSSecureCoding for MXMetricPayload {}

extern_methods!(
    unsafe impl MXMetricPayload {
        #[method_id(@__retain_semantics Other latestApplicationVersion)]
        pub unsafe fn latestApplicationVersion(&self) -> Retained<NSString>;

        #[method(includesMultipleApplicationVersions)]
        pub unsafe fn includesMultipleApplicationVersions(&self) -> bool;

        #[method_id(@__retain_semantics Other timeStampBegin)]
        pub unsafe fn timeStampBegin(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other timeStampEnd)]
        pub unsafe fn timeStampEnd(&self) -> Retained<NSDate>;

        #[cfg(all(feature = "MXCPUMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other cpuMetrics)]
        pub unsafe fn cpuMetrics(&self) -> Option<Retained<MXCPUMetric>>;

        #[cfg(all(feature = "MXGPUMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other gpuMetrics)]
        pub unsafe fn gpuMetrics(&self) -> Option<Retained<MXGPUMetric>>;

        #[cfg(all(feature = "MXCellularConditionMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other cellularConditionMetrics)]
        pub unsafe fn cellularConditionMetrics(
            &self,
        ) -> Option<Retained<MXCellularConditionMetric>>;

        #[cfg(all(feature = "MXAppRunTimeMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other applicationTimeMetrics)]
        pub unsafe fn applicationTimeMetrics(&self) -> Option<Retained<MXAppRunTimeMetric>>;

        #[cfg(all(feature = "MXLocationActivityMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other locationActivityMetrics)]
        pub unsafe fn locationActivityMetrics(&self) -> Option<Retained<MXLocationActivityMetric>>;

        #[cfg(all(feature = "MXMetric", feature = "MXNetworkTransferMetric"))]
        #[method_id(@__retain_semantics Other networkTransferMetrics)]
        pub unsafe fn networkTransferMetrics(&self) -> Option<Retained<MXNetworkTransferMetric>>;

        #[cfg(all(feature = "MXAppLaunchMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other applicationLaunchMetrics)]
        pub unsafe fn applicationLaunchMetrics(&self) -> Option<Retained<MXAppLaunchMetric>>;

        #[cfg(all(feature = "MXAppResponsivenessMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other applicationResponsivenessMetrics)]
        pub unsafe fn applicationResponsivenessMetrics(
            &self,
        ) -> Option<Retained<MXAppResponsivenessMetric>>;

        #[cfg(all(feature = "MXDiskIOMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other diskIOMetrics)]
        pub unsafe fn diskIOMetrics(&self) -> Option<Retained<MXDiskIOMetric>>;

        #[cfg(all(feature = "MXMemoryMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other memoryMetrics)]
        pub unsafe fn memoryMetrics(&self) -> Option<Retained<MXMemoryMetric>>;

        #[cfg(all(feature = "MXDisplayMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other displayMetrics)]
        pub unsafe fn displayMetrics(&self) -> Option<Retained<MXDisplayMetric>>;

        #[cfg(all(feature = "MXAnimationMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other animationMetrics)]
        pub unsafe fn animationMetrics(&self) -> Option<Retained<MXAnimationMetric>>;

        #[cfg(all(feature = "MXAppExitMetric", feature = "MXMetric"))]
        #[method_id(@__retain_semantics Other applicationExitMetrics)]
        pub unsafe fn applicationExitMetrics(&self) -> Option<Retained<MXAppExitMetric>>;

        #[cfg(all(feature = "MXMetric", feature = "MXSignpostMetric"))]
        #[method_id(@__retain_semantics Other signpostMetrics)]
        pub unsafe fn signpostMetrics(&self) -> Option<Retained<NSArray<MXSignpostMetric>>>;

        #[cfg(feature = "MXMetaData")]
        #[method_id(@__retain_semantics Other metaData)]
        pub unsafe fn metaData(&self) -> Option<Retained<MXMetaData>>;

        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Retained<NSData>;

        #[deprecated]
        #[method_id(@__retain_semantics Other DictionaryRepresentation)]
        pub unsafe fn DictionaryRepresentation(&self) -> Retained<NSDictionary>;

        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Retained<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXMetricPayload {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
