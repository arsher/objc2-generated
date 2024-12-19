//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// CallOp
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other callSymbolName:inputTensors:outputTypes:name:)]
        pub unsafe fn callSymbolName_inputTensors_outputTypes_name(
            &self,
            symbol_name: &NSString,
            input_tensors: &NSArray<MPSGraphTensor>,
            output_types: &NSArray<MPSGraphType>,
            name: Option<&NSString>,
        ) -> Retained<NSArray<MPSGraphTensor>>;
    }
);
