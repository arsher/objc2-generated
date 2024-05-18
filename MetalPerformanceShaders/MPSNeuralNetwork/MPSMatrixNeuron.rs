//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixneuron?language=objc)
    #[unsafe(super(MPSMatrixUnaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
    pub struct MPSMatrixNeuron;
);

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl NSCoding for MPSMatrixNeuron {}

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl NSCopying for MPSMatrixNeuron {}

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl CopyingHelper for MPSMatrixNeuron {
    type Result = Self;
}

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl NSObjectProtocol for MPSMatrixNeuron {}

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl NSSecureCoding for MPSMatrixNeuron {}

extern_methods!(
    #[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
    unsafe impl MPSMatrixNeuron {
        #[method(sourceNumberOfFeatureVectors)]
        pub unsafe fn sourceNumberOfFeatureVectors(&self) -> NSUInteger;

        #[method(setSourceNumberOfFeatureVectors:)]
        pub unsafe fn setSourceNumberOfFeatureVectors(
            &self,
            source_number_of_feature_vectors: NSUInteger,
        );

        #[method(sourceInputFeatureChannels)]
        pub unsafe fn sourceInputFeatureChannels(&self) -> NSUInteger;

        #[method(setSourceInputFeatureChannels:)]
        pub unsafe fn setSourceInputFeatureChannels(
            &self,
            source_input_feature_channels: NSUInteger,
        );

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> c_double;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: c_double);

        #[cfg(feature = "MPSCNNNeuronType")]
        #[method(setNeuronType:parameterA:parameterB:parameterC:)]
        pub unsafe fn setNeuronType_parameterA_parameterB_parameterC(
            &self,
            neuron_type: MPSCNNNeuronType,
            parameter_a: c_float,
            parameter_b: c_float,
            parameter_c: c_float,
        );

        #[cfg(feature = "MPSCNNNeuronType")]
        #[method(neuronType)]
        pub unsafe fn neuronType(&self) -> MPSCNNNeuronType;

        #[method(neuronParameterA)]
        pub unsafe fn neuronParameterA(&self) -> c_float;

        #[method(neuronParameterB)]
        pub unsafe fn neuronParameterB(&self) -> c_float;

        #[method(neuronParameterC)]
        pub unsafe fn neuronParameterC(&self) -> c_float;

        #[method(setNeuronToPReLUWithParametersA:)]
        pub unsafe fn setNeuronToPReLUWithParametersA(&self, a: &NSData);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSMatrix")]
        #[method(encodeToCommandBuffer:inputMatrix:biasVector:resultMatrix:)]
        pub unsafe fn encodeToCommandBuffer_inputMatrix_biasVector_resultMatrix(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            input_matrix: &MPSMatrix,
            bias_vector: Option<&MPSVector>,
            result_matrix: &MPSMatrix,
        );

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Copy copyWithZone:device:)]
        pub unsafe fn copyWithZone_device(
            &self,
            zone: *mut NSZone,
            device: Option<&ProtocolObject<dyn MTLDevice>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
    unsafe impl MPSMatrixNeuron {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
    unsafe impl MPSMatrixNeuron {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixneurongradient?language=objc)
    #[unsafe(super(MPSMatrixBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
    pub struct MPSMatrixNeuronGradient;
);

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl NSCoding for MPSMatrixNeuronGradient {}

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl NSCopying for MPSMatrixNeuronGradient {}

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl CopyingHelper for MPSMatrixNeuronGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl NSObjectProtocol for MPSMatrixNeuronGradient {}

#[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
unsafe impl NSSecureCoding for MPSMatrixNeuronGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
    unsafe impl MPSMatrixNeuronGradient {
        #[method(sourceNumberOfFeatureVectors)]
        pub unsafe fn sourceNumberOfFeatureVectors(&self) -> NSUInteger;

        #[method(setSourceNumberOfFeatureVectors:)]
        pub unsafe fn setSourceNumberOfFeatureVectors(
            &self,
            source_number_of_feature_vectors: NSUInteger,
        );

        #[method(sourceInputFeatureChannels)]
        pub unsafe fn sourceInputFeatureChannels(&self) -> NSUInteger;

        #[method(setSourceInputFeatureChannels:)]
        pub unsafe fn setSourceInputFeatureChannels(
            &self,
            source_input_feature_channels: NSUInteger,
        );

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> c_double;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: c_double);

        #[cfg(feature = "MPSCNNNeuronType")]
        #[method(setNeuronType:parameterA:parameterB:parameterC:)]
        pub unsafe fn setNeuronType_parameterA_parameterB_parameterC(
            &self,
            neuron_type: MPSCNNNeuronType,
            parameter_a: c_float,
            parameter_b: c_float,
            parameter_c: c_float,
        );

        #[cfg(feature = "MPSCNNNeuronType")]
        #[method(neuronType)]
        pub unsafe fn neuronType(&self) -> MPSCNNNeuronType;

        #[method(neuronParameterA)]
        pub unsafe fn neuronParameterA(&self) -> c_float;

        #[method(neuronParameterB)]
        pub unsafe fn neuronParameterB(&self) -> c_float;

        #[method(neuronParameterC)]
        pub unsafe fn neuronParameterC(&self) -> c_float;

        #[method(setNeuronToPReLUWithParametersA:)]
        pub unsafe fn setNeuronToPReLUWithParametersA(&self, a: &NSData);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSMatrix")]
        #[method(encodeToCommandBuffer:gradientMatrix:inputMatrix:biasVector:resultGradientForDataMatrix:resultGradientForBiasVector:)]
        pub unsafe fn encodeToCommandBuffer_gradientMatrix_inputMatrix_biasVector_resultGradientForDataMatrix_resultGradientForBiasVector(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            gradient_matrix: &MPSMatrix,
            input_matrix: &MPSMatrix,
            bias_vector: Option<&MPSVector>,
            result_gradient_for_data_matrix: &MPSMatrix,
            result_gradient_for_bias_vector: Option<&MPSVector>,
        );

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Copy copyWithZone:device:)]
        pub unsafe fn copyWithZone_device(
            &self,
            zone: *mut NSZone,
            device: Option<&ProtocolObject<dyn MTLDevice>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
    unsafe impl MPSMatrixNeuronGradient {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSKernel", feature = "MPSMatrixTypes"))]
    unsafe impl MPSMatrixNeuronGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
