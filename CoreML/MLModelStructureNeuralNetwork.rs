//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelStructureNeuralNetwork;

    unsafe impl ClassType for MLModelStructureNeuralNetwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for MLModelStructureNeuralNetwork {}

unsafe impl Sync for MLModelStructureNeuralNetwork {}

unsafe impl NSObjectProtocol for MLModelStructureNeuralNetwork {}

extern_methods!(
    unsafe impl MLModelStructureNeuralNetwork {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "MLModelStructureNeuralNetworkLayer")]
        #[method_id(@__retain_semantics Other layers)]
        pub unsafe fn layers(&self) -> Retained<NSArray<MLModelStructureNeuralNetworkLayer>>;
    }
);
