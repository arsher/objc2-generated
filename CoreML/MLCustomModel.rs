//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlcustommodel?language=objc)
    pub unsafe trait MLCustomModel {
        #[cfg(feature = "MLModelDescription")]
        #[method_id(@__retain_semantics Init initWithModelDescription:parameterDictionary:error:_)]
        unsafe fn initWithModelDescription_parameterDictionary_error(
            this: Allocated<Self>,
            model_description: &MLModelDescription,
            parameters: &NSDictionary<NSString, AnyObject>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "MLFeatureProvider", feature = "MLPredictionOptions"))]
        #[method_id(@__retain_semantics Other predictionFromFeatures:options:error:_)]
        unsafe fn predictionFromFeatures_options_error(
            &self,
            input: &ProtocolObject<dyn MLFeatureProvider>,
            options: &MLPredictionOptions,
        ) -> Result<Retained<ProtocolObject<dyn MLFeatureProvider>>, Retained<NSError>>;

        #[cfg(all(feature = "MLBatchProvider", feature = "MLPredictionOptions"))]
        #[optional]
        #[method_id(@__retain_semantics Other predictionsFromBatch:options:error:_)]
        unsafe fn predictionsFromBatch_options_error(
            &self,
            input_batch: &ProtocolObject<dyn MLBatchProvider>,
            options: &MLPredictionOptions,
        ) -> Result<Retained<ProtocolObject<dyn MLBatchProvider>>, Retained<NSError>>;
    }

    unsafe impl ProtocolType for dyn MLCustomModel {}
);
