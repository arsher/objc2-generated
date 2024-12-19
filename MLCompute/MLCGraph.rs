//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcgraph?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCGraph;
);

unsafe impl NSObjectProtocol for MLCGraph {}

extern_methods!(
    unsafe impl MLCGraph {
        #[cfg(feature = "MLCDevice")]
        #[deprecated]
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Retained<MLCDevice>>;

        #[cfg(feature = "MLCLayer")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layers)]
        pub unsafe fn layers(&self) -> Retained<NSArray<MLCLayer>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other graph)]
        pub unsafe fn graph() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other summarizedDOTDescription)]
        pub unsafe fn summarizedDOTDescription(&self) -> Retained<NSString>;

        #[cfg(all(feature = "MLCLayer", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other nodeWithLayer:source:)]
        pub unsafe fn nodeWithLayer_source(
            &self,
            layer: &MLCLayer,
            source: &MLCTensor,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(all(feature = "MLCLayer", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other nodeWithLayer:sources:)]
        pub unsafe fn nodeWithLayer_sources(
            &self,
            layer: &MLCLayer,
            sources: &NSArray<MLCTensor>,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(all(feature = "MLCLayer", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other nodeWithLayer:sources:disableUpdate:)]
        pub unsafe fn nodeWithLayer_sources_disableUpdate(
            &self,
            layer: &MLCLayer,
            sources: &NSArray<MLCTensor>,
            disable_update: bool,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(all(feature = "MLCLayer", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other nodeWithLayer:sources:lossLabels:)]
        pub unsafe fn nodeWithLayer_sources_lossLabels(
            &self,
            layer: &MLCLayer,
            sources: &NSArray<MLCTensor>,
            loss_labels: &NSArray<MLCTensor>,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other splitWithSource:splitCount:dimension:)]
        pub unsafe fn splitWithSource_splitCount_dimension(
            &self,
            source: &MLCTensor,
            split_count: NSUInteger,
            dimension: NSUInteger,
        ) -> Option<Retained<NSArray<MLCTensor>>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other splitWithSource:splitSectionLengths:dimension:)]
        pub unsafe fn splitWithSource_splitSectionLengths_dimension(
            &self,
            source: &MLCTensor,
            split_section_lengths: &NSArray<NSNumber>,
            dimension: NSUInteger,
        ) -> Option<Retained<NSArray<MLCTensor>>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other concatenateWithSources:dimension:)]
        pub unsafe fn concatenateWithSources_dimension(
            &self,
            sources: &NSArray<MLCTensor>,
            dimension: NSUInteger,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other reshapeWithShape:source:)]
        pub unsafe fn reshapeWithShape_source(
            &self,
            shape: &NSArray<NSNumber>,
            source: &MLCTensor,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other transposeWithDimensions:source:)]
        pub unsafe fn transposeWithDimensions_source(
            &self,
            dimensions: &NSArray<NSNumber>,
            source: &MLCTensor,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[method_id(@__retain_semantics Other selectWithSources:condition:)]
        pub unsafe fn selectWithSources_condition(
            &self,
            sources: &NSArray<MLCTensor>,
            condition: &MLCTensor,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[method_id(@__retain_semantics Other scatterWithDimension:source:indices:copyFrom:reductionType:)]
        pub unsafe fn scatterWithDimension_source_indices_copyFrom_reductionType(
            &self,
            dimension: NSUInteger,
            source: &MLCTensor,
            indices: &MLCTensor,
            copy_from: &MLCTensor,
            reduction_type: MLCReductionType,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[method_id(@__retain_semantics Other gatherWithDimension:source:indices:)]
        pub unsafe fn gatherWithDimension_source_indices(
            &self,
            dimension: NSUInteger,
            source: &MLCTensor,
            indices: &MLCTensor,
        ) -> Option<Retained<MLCTensor>>;

        #[cfg(all(
            feature = "MLCDevice",
            feature = "MLCTensor",
            feature = "MLCTensorData"
        ))]
        #[deprecated]
        #[method(bindAndWriteData:forInputs:toDevice:batchSize:synchronous:)]
        pub unsafe fn bindAndWriteData_forInputs_toDevice_batchSize_synchronous(
            &self,
            inputs_data: &NSDictionary<NSString, MLCTensorData>,
            input_tensors: &NSDictionary<NSString, MLCTensor>,
            device: &MLCDevice,
            batch_size: NSUInteger,
            synchronous: bool,
        ) -> bool;

        #[cfg(all(
            feature = "MLCDevice",
            feature = "MLCTensor",
            feature = "MLCTensorData"
        ))]
        #[deprecated]
        #[method(bindAndWriteData:forInputs:toDevice:synchronous:)]
        pub unsafe fn bindAndWriteData_forInputs_toDevice_synchronous(
            &self,
            inputs_data: &NSDictionary<NSString, MLCTensorData>,
            input_tensors: &NSDictionary<NSString, MLCTensor>,
            device: &MLCDevice,
            synchronous: bool,
        ) -> bool;

        #[cfg(all(feature = "MLCLayer", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other sourceTensorsForLayer:)]
        pub unsafe fn sourceTensorsForLayer(
            &self,
            layer: &MLCLayer,
        ) -> Retained<NSArray<MLCTensor>>;

        #[cfg(all(feature = "MLCLayer", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other resultTensorsForLayer:)]
        pub unsafe fn resultTensorsForLayer(
            &self,
            layer: &MLCLayer,
        ) -> Retained<NSArray<MLCTensor>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLCGraph {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
