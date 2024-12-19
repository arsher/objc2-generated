//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphvariableop?language=objc)
    #[unsafe(super(MPSGraphOperation, MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphOperation"))]
    pub struct MPSGraphVariableOp;
);

#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphOperation"))]
unsafe impl NSCopying for MPSGraphVariableOp {}

#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphOperation"))]
unsafe impl CopyingHelper for MPSGraphVariableOp {
    type Result = Self;
}

#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphOperation"))]
unsafe impl NSObjectProtocol for MPSGraphVariableOp {}

extern_methods!(
    #[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphOperation"))]
    unsafe impl MPSGraphVariableOp {
        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Retained<MPSShape>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MPSDataType;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSGraphOperation`
    #[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphOperation"))]
    unsafe impl MPSGraphVariableOp {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphOperation"))]
    unsafe impl MPSGraphVariableOp {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// MemoryOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other placeholderWithShape:dataType:name:)]
        pub unsafe fn placeholderWithShape_dataType_name(
            &self,
            shape: Option<&MPSShape>,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other placeholderWithShape:name:)]
        pub unsafe fn placeholderWithShape_name(
            &self,
            shape: Option<&MPSShape>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other constantWithData:shape:dataType:)]
        pub unsafe fn constantWithData_shape_dataType(
            &self,
            data: &NSData,
            shape: &MPSShape,
            data_type: MPSDataType,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other constantWithScalar:dataType:)]
        pub unsafe fn constantWithScalar_dataType(
            &self,
            scalar: c_double,
            data_type: MPSDataType,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other constantWithScalar:shape:dataType:)]
        pub unsafe fn constantWithScalar_shape_dataType(
            &self,
            scalar: c_double,
            shape: &MPSShape,
            data_type: MPSDataType,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other constantWithRealPart:imaginaryPart:)]
        pub unsafe fn constantWithRealPart_imaginaryPart(
            &self,
            real_part: c_double,
            imaginary_part: c_double,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other constantWithRealPart:imaginaryPart:dataType:)]
        pub unsafe fn constantWithRealPart_imaginaryPart_dataType(
            &self,
            real_part: c_double,
            imaginary_part: c_double,
            data_type: MPSDataType,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other constantWithRealPart:imaginaryPart:shape:dataType:)]
        pub unsafe fn constantWithRealPart_imaginaryPart_shape_dataType(
            &self,
            real_part: c_double,
            imaginary_part: c_double,
            shape: &MPSShape,
            data_type: MPSDataType,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other variableWithData:shape:dataType:name:)]
        pub unsafe fn variableWithData_shape_dataType_name(
            &self,
            data: &NSData,
            shape: &MPSShape,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other variableFromTensorWithTensor:name:)]
        pub unsafe fn variableFromTensorWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other readVariable:name:)]
        pub unsafe fn readVariable_name(
            &self,
            variable: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(feature = "MPSGraphOperation", feature = "MPSGraphTensor"))]
        #[method_id(@__retain_semantics Other assignVariable:withValueOfTensor:name:)]
        pub unsafe fn assignVariable_withValueOfTensor_name(
            &self,
            variable: &MPSGraphTensor,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphOperation>;
    }
);
