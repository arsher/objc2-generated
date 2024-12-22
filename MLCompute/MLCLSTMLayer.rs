//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A LSTM layer
    ///
    /// The hidden and cell state for inputs and outputs have a layout of [numberOfLayers, numberOfDirections, batchSize, hiddenSize].
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlclstmlayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCLSTMLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCLSTMLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCLSTMLayer {
        #[cfg(feature = "MLCLSTMDescriptor")]
        /// The LSTM descriptor
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Retained<MLCLSTMDescriptor>;

        #[cfg(feature = "MLCActivationDescriptor")]
        /// The array of gate activations for input, hidden, cell and output gates
        ///
        /// The default gate activations are: sigmoid, sigmoid, tanh, sigmoid
        #[deprecated]
        #[method_id(@__retain_semantics Other gateActivations)]
        pub unsafe fn gateActivations(&self) -> Retained<NSArray<MLCActivationDescriptor>>;

        #[cfg(feature = "MLCActivationDescriptor")]
        /// The output activation descriptor
        #[deprecated]
        #[method_id(@__retain_semantics Other outputResultActivation)]
        pub unsafe fn outputResultActivation(&self) -> Retained<MLCActivationDescriptor>;

        #[cfg(feature = "MLCTensor")]
        /// The array of tensors describing the input weights for the input, hidden, cell and output gates
        #[deprecated]
        #[method_id(@__retain_semantics Other inputWeights)]
        pub unsafe fn inputWeights(&self) -> Retained<NSArray<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        /// The array of tensors describing the hidden weights for the input, hidden, cell and output gates
        #[deprecated]
        #[method_id(@__retain_semantics Other hiddenWeights)]
        pub unsafe fn hiddenWeights(&self) -> Retained<NSArray<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        /// The array of tensors describing the peephole weights for the input, hidden, cell and output gates
        #[deprecated]
        #[method_id(@__retain_semantics Other peepholeWeights)]
        pub unsafe fn peepholeWeights(&self) -> Option<Retained<NSArray<MLCTensor>>>;

        #[cfg(feature = "MLCTensor")]
        /// The array of tensors describing the bias terms for the input, hidden, cell and output gates
        #[deprecated]
        #[method_id(@__retain_semantics Other biases)]
        pub unsafe fn biases(&self) -> Option<Retained<NSArray<MLCTensor>>>;

        #[cfg(feature = "MLCTensorParameter")]
        /// The input weights tensor parameters used for optimizer update
        #[deprecated]
        #[method_id(@__retain_semantics Other inputWeightsParameters)]
        pub unsafe fn inputWeightsParameters(&self) -> Retained<NSArray<MLCTensorParameter>>;

        #[cfg(feature = "MLCTensorParameter")]
        /// The hidden weights tensor parameters used for optimizer update
        #[deprecated]
        #[method_id(@__retain_semantics Other hiddenWeightsParameters)]
        pub unsafe fn hiddenWeightsParameters(&self) -> Retained<NSArray<MLCTensorParameter>>;

        #[cfg(feature = "MLCTensorParameter")]
        /// The peephole weights tensor parameters used for optimizer update
        #[deprecated]
        #[method_id(@__retain_semantics Other peepholeWeightsParameters)]
        pub unsafe fn peepholeWeightsParameters(
            &self,
        ) -> Option<Retained<NSArray<MLCTensorParameter>>>;

        #[cfg(feature = "MLCTensorParameter")]
        /// The bias tensor parameter used for optimizer update
        #[deprecated]
        #[method_id(@__retain_semantics Other biasesParameters)]
        pub unsafe fn biasesParameters(&self) -> Option<Retained<NSArray<MLCTensorParameter>>>;

        #[cfg(all(feature = "MLCLSTMDescriptor", feature = "MLCTensor"))]
        /// Create a LSTM layer
        ///
        /// Parameter `descriptor`: The LSTM descriptor
        ///
        /// Parameter `inputWeights`: An array of (layerCount * 4) tensors describing the input weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        ///
        /// Parameter `hiddenWeights`: An array of (layerCount * 4) tensors describing the hidden weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        ///
        /// Returns: A new LSTM layer.
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:inputWeights:hiddenWeights:biases:)]
        pub unsafe fn layerWithDescriptor_inputWeights_hiddenWeights_biases(
            descriptor: &MLCLSTMDescriptor,
            input_weights: &NSArray<MLCTensor>,
            hidden_weights: &NSArray<MLCTensor>,
            biases: Option<&NSArray<MLCTensor>>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "MLCLSTMDescriptor", feature = "MLCTensor"))]
        /// Create a LSTM layer
        ///
        /// Parameter `descriptor`: The LSTM descriptor
        ///
        /// Parameter `inputWeights`: An array of (layerCount * 4) tensors describing the input weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        ///
        /// Parameter `hiddenWeights`: An array of (layerCount * 4) tensors describing the hidden weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        ///
        /// Parameter `peepholeWeights`: An array of (layerCount * 4) tensors describing the peephole weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        ///
        /// Returns: A new LSTM layer.
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:inputWeights:hiddenWeights:peepholeWeights:biases:)]
        pub unsafe fn layerWithDescriptor_inputWeights_hiddenWeights_peepholeWeights_biases(
            descriptor: &MLCLSTMDescriptor,
            input_weights: &NSArray<MLCTensor>,
            hidden_weights: &NSArray<MLCTensor>,
            peephole_weights: Option<&NSArray<MLCTensor>>,
            biases: Option<&NSArray<MLCTensor>>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "MLCActivationDescriptor",
            feature = "MLCLSTMDescriptor",
            feature = "MLCTensor"
        ))]
        /// Create a LSTM layer
        ///
        /// Parameter `descriptor`: The LSTM descriptor
        ///
        /// Parameter `inputWeights`: An array of (layerCount * 4) tensors describing the input weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        /// For bidirectional LSTM, the forward time weights for all stacked layers will come first followed by backward time weights
        ///
        /// Parameter `hiddenWeights`: An array of (layerCount * 4) tensors describing the hidden weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        /// For bidirectional LSTM, the forward time weights for all stacked layers will come first followed by backward time weights
        ///
        /// Parameter `peepholeWeights`: An array of (layerCount * 4) tensors describing the peephole weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        ///
        /// Parameter `biases`: An array of (layerCount * 4) tensors describing the input weights for the
        /// input, hidden, cell and output gates for layer0, layer1.. layer(n-1) for layerCount=n.
        /// For bidirectional LSTM, the forward time bias terms for all stacked layers will come first followed by backward time bias terms
        ///
        /// Parameter `gateActivations`: An array of 4 neuron descriptors for the input, hidden, cell and output gate activations.
        ///
        /// Parameter `outputResultActivation`: The neuron descriptor used for the activation function applied to output result.  Default is tanh.
        ///
        /// Returns: A new  LSTM layer.
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
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCLSTMLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
