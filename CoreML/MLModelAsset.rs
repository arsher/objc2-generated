//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelasset?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelAsset;
);

unsafe impl NSObjectProtocol for MLModelAsset {}

extern_methods!(
    unsafe impl MLModelAsset {
        #[method_id(@__retain_semantics Other modelAssetWithSpecificationData:error:_)]
        pub unsafe fn modelAssetWithSpecificationData_error(
            specification_data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other modelAssetWithSpecificationData:blobMapping:error:_)]
        pub unsafe fn modelAssetWithSpecificationData_blobMapping_error(
            specification_data: &NSData,
            blob_mapping: &NSDictionary<NSURL, NSData>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other modelAssetWithURL:error:_)]
        pub unsafe fn modelAssetWithURL_error(
            compiled_model_url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "MLModelDescription", feature = "block2"))]
        #[method(modelDescriptionWithCompletionHandler:)]
        pub unsafe fn modelDescriptionWithCompletionHandler(
            &self,
            handler: &block2::Block<dyn Fn(*mut MLModelDescription, *mut NSError)>,
        );

        #[cfg(all(feature = "MLModelDescription", feature = "block2"))]
        #[method(modelDescriptionOfFunctionNamed:completionHandler:)]
        pub unsafe fn modelDescriptionOfFunctionNamed_completionHandler(
            &self,
            function_name: &NSString,
            handler: &block2::Block<dyn Fn(*mut MLModelDescription, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(functionNamesWithCompletionHandler:)]
        pub unsafe fn functionNamesWithCompletionHandler(
            &self,
            handler: &block2::Block<dyn Fn(*mut NSArray<NSString>, *mut NSError)>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
