//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcembeddingdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCEmbeddingDescriptor;
);

unsafe impl NSCopying for MLCEmbeddingDescriptor {}

unsafe impl CopyingHelper for MLCEmbeddingDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCEmbeddingDescriptor {}

extern_methods!(
    unsafe impl MLCEmbeddingDescriptor {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other embeddingCount)]
        pub unsafe fn embeddingCount(&self) -> Retained<NSNumber>;

        #[deprecated]
        #[method_id(@__retain_semantics Other embeddingDimension)]
        pub unsafe fn embeddingDimension(&self) -> Retained<NSNumber>;

        #[deprecated]
        #[method_id(@__retain_semantics Other paddingIndex)]
        pub unsafe fn paddingIndex(&self) -> Option<Retained<NSNumber>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other maximumNorm)]
        pub unsafe fn maximumNorm(&self) -> Option<Retained<NSNumber>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other pNorm)]
        pub unsafe fn pNorm(&self) -> Option<Retained<NSNumber>>;

        #[deprecated]
        #[method(scalesGradientByFrequency)]
        pub unsafe fn scalesGradientByFrequency(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithEmbeddingCount:embeddingDimension:)]
        pub unsafe fn descriptorWithEmbeddingCount_embeddingDimension(
            embedding_count: &NSNumber,
            embedding_dimension: &NSNumber,
        ) -> Option<Retained<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithEmbeddingCount:embeddingDimension:paddingIndex:maximumNorm:pNorm:scalesGradientByFrequency:)]
        pub unsafe fn descriptorWithEmbeddingCount_embeddingDimension_paddingIndex_maximumNorm_pNorm_scalesGradientByFrequency(
            embedding_count: &NSNumber,
            embedding_dimension: &NSNumber,
            padding_index: Option<&NSNumber>,
            maximum_norm: Option<&NSNumber>,
            p_norm: Option<&NSNumber>,
            scales_gradient_by_frequency: bool,
        ) -> Option<Retained<Self>>;
    }
);
