//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcinstancenormalizationlayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCInstanceNormalizationLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCInstanceNormalizationLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCInstanceNormalizationLayer {
        #[deprecated]
        #[method(featureChannelCount)]
        pub unsafe fn featureChannelCount(&self) -> NSUInteger;

        #[cfg(feature = "MLCTensor")]
        #[method_id(@__retain_semantics Other mean)]
        pub unsafe fn mean(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[method_id(@__retain_semantics Other variance)]
        pub unsafe fn variance(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other beta)]
        pub unsafe fn beta(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other gamma)]
        pub unsafe fn gamma(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other betaParameter)]
        pub unsafe fn betaParameter(&self) -> Option<Retained<MLCTensorParameter>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other gammaParameter)]
        pub unsafe fn gammaParameter(&self) -> Option<Retained<MLCTensorParameter>>;

        #[deprecated]
        #[method(varianceEpsilon)]
        pub unsafe fn varianceEpsilon(&self) -> c_float;

        #[deprecated]
        #[method(momentum)]
        pub unsafe fn momentum(&self) -> c_float;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithFeatureChannelCount:beta:gamma:varianceEpsilon:)]
        pub unsafe fn layerWithFeatureChannelCount_beta_gamma_varianceEpsilon(
            feature_channel_count: NSUInteger,
            beta: Option<&MLCTensor>,
            gamma: Option<&MLCTensor>,
            variance_epsilon: c_float,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithFeatureChannelCount:beta:gamma:varianceEpsilon:momentum:)]
        pub unsafe fn layerWithFeatureChannelCount_beta_gamma_varianceEpsilon_momentum(
            feature_channel_count: NSUInteger,
            beta: Option<&MLCTensor>,
            gamma: Option<&MLCTensor>,
            variance_epsilon: c_float,
            momentum: c_float,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTensor")]
        #[method_id(@__retain_semantics Other layerWithFeatureChannelCount:mean:variance:beta:gamma:varianceEpsilon:momentum:)]
        pub unsafe fn layerWithFeatureChannelCount_mean_variance_beta_gamma_varianceEpsilon_momentum(
            feature_channel_count: NSUInteger,
            mean: &MLCTensor,
            variance: &MLCTensor,
            beta: Option<&MLCTensor>,
            gamma: Option<&MLCTensor>,
            variance_epsilon: c_float,
            momentum: c_float,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCInstanceNormalizationLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
