//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlclstmdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCLSTMDescriptor;
);

unsafe impl NSCopying for MLCLSTMDescriptor {}

unsafe impl CopyingHelper for MLCLSTMDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCLSTMDescriptor {}

extern_methods!(
    unsafe impl MLCLSTMDescriptor {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[method(inputSize)]
        pub unsafe fn inputSize(&self) -> NSUInteger;

        #[deprecated]
        #[method(hiddenSize)]
        pub unsafe fn hiddenSize(&self) -> NSUInteger;

        #[deprecated]
        #[method(layerCount)]
        pub unsafe fn layerCount(&self) -> NSUInteger;

        #[deprecated]
        #[method(usesBiases)]
        pub unsafe fn usesBiases(&self) -> bool;

        #[deprecated]
        #[method(batchFirst)]
        pub unsafe fn batchFirst(&self) -> bool;

        #[deprecated]
        #[method(isBidirectional)]
        pub unsafe fn isBidirectional(&self) -> bool;

        #[deprecated]
        #[method(returnsSequences)]
        pub unsafe fn returnsSequences(&self) -> bool;

        #[deprecated]
        #[method(dropout)]
        pub unsafe fn dropout(&self) -> c_float;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(resultMode)]
        pub unsafe fn resultMode(&self) -> MLCLSTMResultMode;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:usesBiases:isBidirectional:dropout:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_isBidirectional_dropout(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
            uses_biases: bool,
            is_bidirectional: bool,
            dropout: c_float,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:usesBiases:batchFirst:isBidirectional:dropout:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_dropout(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
            uses_biases: bool,
            batch_first: bool,
            is_bidirectional: bool,
            dropout: c_float,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:usesBiases:batchFirst:isBidirectional:returnsSequences:dropout:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_returnsSequences_dropout(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
            uses_biases: bool,
            batch_first: bool,
            is_bidirectional: bool,
            returns_sequences: bool,
            dropout: c_float,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithInputSize:hiddenSize:layerCount:usesBiases:batchFirst:isBidirectional:returnsSequences:dropout:resultMode:)]
        pub unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_returnsSequences_dropout_resultMode(
            input_size: NSUInteger,
            hidden_size: NSUInteger,
            layer_count: NSUInteger,
            uses_biases: bool,
            batch_first: bool,
            is_bidirectional: bool,
            returns_sequences: bool,
            dropout: c_float,
            result_mode: MLCLSTMResultMode,
        ) -> Retained<Self>;
    }
);
