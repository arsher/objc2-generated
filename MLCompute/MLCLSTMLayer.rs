//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCLSTMLayer;

    #[cfg(feature = "MLCLayer")]
    unsafe impl ClassType for MLCLSTMLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCLSTMLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCLSTMLayer {
        #[cfg(feature = "MLCLSTMDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Id<MLCLSTMDescriptor>;

        #[cfg(feature = "MLCActivationDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other gateActivations)]
        pub unsafe fn gateActivations(&self) -> Id<NSArray<MLCActivationDescriptor>>;

        #[cfg(feature = "MLCActivationDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other outputResultActivation)]
        pub unsafe fn outputResultActivation(&self) -> Id<MLCActivationDescriptor>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other inputWeights)]
        pub unsafe fn inputWeights(&self) -> Id<NSArray<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other hiddenWeights)]
        pub unsafe fn hiddenWeights(&self) -> Id<NSArray<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other peepholeWeights)]
        pub unsafe fn peepholeWeights(&self) -> Option<Id<NSArray<MLCTensor>>>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other biases)]
        pub unsafe fn biases(&self) -> Option<Id<NSArray<MLCTensor>>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other inputWeightsParameters)]
        pub unsafe fn inputWeightsParameters(&self) -> Id<NSArray<MLCTensorParameter>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other hiddenWeightsParameters)]
        pub unsafe fn hiddenWeightsParameters(&self) -> Id<NSArray<MLCTensorParameter>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other peepholeWeightsParameters)]
        pub unsafe fn peepholeWeightsParameters(&self) -> Option<Id<NSArray<MLCTensorParameter>>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other biasesParameters)]
        pub unsafe fn biasesParameters(&self) -> Option<Id<NSArray<MLCTensorParameter>>>;

        #[cfg(all(feature = "MLCLSTMDescriptor", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:inputWeights:hiddenWeights:biases:)]
        pub unsafe fn layerWithDescriptor_inputWeights_hiddenWeights_biases(
            descriptor: &MLCLSTMDescriptor,
            input_weights: &NSArray<MLCTensor>,
            hidden_weights: &NSArray<MLCTensor>,
            biases: Option<&NSArray<MLCTensor>>,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "MLCLSTMDescriptor", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:inputWeights:hiddenWeights:peepholeWeights:biases:)]
        pub unsafe fn layerWithDescriptor_inputWeights_hiddenWeights_peepholeWeights_biases(
            descriptor: &MLCLSTMDescriptor,
            input_weights: &NSArray<MLCTensor>,
            hidden_weights: &NSArray<MLCTensor>,
            peephole_weights: Option<&NSArray<MLCTensor>>,
            biases: Option<&NSArray<MLCTensor>>,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "MLCActivationDescriptor",
            feature = "MLCLSTMDescriptor",
            feature = "MLCTensor"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:inputWeights:hiddenWeights:peepholeWeights:biases:gateActivations:outputResultActivation:)]
        pub unsafe fn layerWithDescriptor_inputWeights_hiddenWeights_peepholeWeights_biases_gateActivations_outputResultActivation(
            descriptor: &MLCLSTMDescriptor,
            input_weights: &NSArray<MLCTensor>,
            hidden_weights: &NSArray<MLCTensor>,
            peephole_weights: Option<&NSArray<MLCTensor>>,
            biases: Option<&NSArray<MLCTensor>>,
            gate_activations: &NSArray<MLCActivationDescriptor>,
            output_result_activation: &MLCActivationDescriptor,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCLSTMLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
