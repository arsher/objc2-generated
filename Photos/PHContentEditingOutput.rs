//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-uniform-type-identifiers")]
use objc2_uniform_type_identifiers::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phcontenteditingoutput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHContentEditingOutput;
);

unsafe impl NSObjectProtocol for PHContentEditingOutput {}

extern_methods!(
    unsafe impl PHContentEditingOutput {
        #[cfg(feature = "PHContentEditingInput")]
        #[method_id(@__retain_semantics Init initWithContentEditingInput:)]
        pub unsafe fn initWithContentEditingInput(
            this: Allocated<Self>,
            content_editing_input: &PHContentEditingInput,
        ) -> Retained<Self>;

        #[cfg(feature = "PHAdjustmentData")]
        #[method_id(@__retain_semantics Other adjustmentData)]
        pub unsafe fn adjustmentData(&self) -> Option<Retained<PHAdjustmentData>>;

        #[cfg(feature = "PHAdjustmentData")]
        #[method(setAdjustmentData:)]
        pub unsafe fn setAdjustmentData(&self, adjustment_data: Option<&PHAdjustmentData>);

        #[method_id(@__retain_semantics Other renderedContentURL)]
        pub unsafe fn renderedContentURL(&self) -> Retained<NSURL>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        #[method_id(@__retain_semantics Other defaultRenderedContentType)]
        pub unsafe fn defaultRenderedContentType(&self) -> Option<Retained<UTType>>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        #[method_id(@__retain_semantics Other supportedRenderedContentTypes)]
        pub unsafe fn supportedRenderedContentTypes(&self) -> Retained<NSArray<UTType>>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        #[method_id(@__retain_semantics Other renderedContentURLForType:error:_)]
        pub unsafe fn renderedContentURLForType_error(
            &self,
            r#type: &UTType,
        ) -> Result<Retained<NSURL>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHContentEditingOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
