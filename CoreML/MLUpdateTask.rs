//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlupdatetask?language=objc)
    #[unsafe(super(MLTask, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLTask")]
    pub struct MLUpdateTask;
);

#[cfg(feature = "MLTask")]
unsafe impl NSObjectProtocol for MLUpdateTask {}

extern_methods!(
    #[cfg(feature = "MLTask")]
    unsafe impl MLUpdateTask {
        #[cfg(all(
            feature = "MLBatchProvider",
            feature = "MLModelConfiguration",
            feature = "MLUpdateContext",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other updateTaskForModelAtURL:trainingData:configuration:completionHandler:error:_)]
        pub unsafe fn updateTaskForModelAtURL_trainingData_configuration_completionHandler_error(
            model_url: &NSURL,
            training_data: &ProtocolObject<dyn MLBatchProvider>,
            configuration: Option<&MLModelConfiguration>,
            completion_handler: &block2::Block<dyn Fn(NonNull<MLUpdateContext>)>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(
            feature = "MLBatchProvider",
            feature = "MLUpdateContext",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other updateTaskForModelAtURL:trainingData:completionHandler:error:_)]
        pub unsafe fn updateTaskForModelAtURL_trainingData_completionHandler_error(
            model_url: &NSURL,
            training_data: &ProtocolObject<dyn MLBatchProvider>,
            completion_handler: &block2::Block<dyn Fn(NonNull<MLUpdateContext>)>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(
            feature = "MLBatchProvider",
            feature = "MLModelConfiguration",
            feature = "MLUpdateProgressHandlers"
        ))]
        #[method_id(@__retain_semantics Other updateTaskForModelAtURL:trainingData:configuration:progressHandlers:error:_)]
        pub unsafe fn updateTaskForModelAtURL_trainingData_configuration_progressHandlers_error(
            model_url: &NSURL,
            training_data: &ProtocolObject<dyn MLBatchProvider>,
            configuration: Option<&MLModelConfiguration>,
            progress_handlers: &MLUpdateProgressHandlers,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "MLBatchProvider", feature = "MLUpdateProgressHandlers"))]
        #[method_id(@__retain_semantics Other updateTaskForModelAtURL:trainingData:progressHandlers:error:_)]
        pub unsafe fn updateTaskForModelAtURL_trainingData_progressHandlers_error(
            model_url: &NSURL,
            training_data: &ProtocolObject<dyn MLBatchProvider>,
            progress_handlers: &MLUpdateProgressHandlers,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "MLKey", feature = "MLParameterKey"))]
        #[method(resumeWithParameters:)]
        pub unsafe fn resumeWithParameters(
            &self,
            update_parameters: &NSDictionary<MLParameterKey, AnyObject>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
