//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// MLModelCollection
    ///
    /// A collection of models managed as part of Core ML Model Deployment.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelcollection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use Background Assets or NSURLSession instead."]
    pub struct MLModelCollection;
);

unsafe impl NSObjectProtocol for MLModelCollection {}

extern_methods!(
    unsafe impl MLModelCollection {
        /// The identifier of the model collection you want to access, as configured in the Core ML Model Deployment dashboard.
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[cfg(feature = "MLModelCollectionEntry")]
        /// Information about the models downloaded in the collection, or an empty dictionary if the collection has not been downloaded.
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other entries)]
        pub unsafe fn entries(&self) -> Retained<NSDictionary<NSString, MLModelCollectionEntry>>;

        /// The identifier for the currently downloaded deployment, corresponding to a recent deployment on the Core ML Model Deployment dashboard.
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other deploymentID)]
        pub unsafe fn deploymentID(&self) -> Retained<NSString>;

        #[cfg(feature = "block2")]
        /// Request access to a model collection. If the collection is not downloaded on the device, it is requested
        /// from Core ML Model Deployment.
        ///
        ///
        /// When called, this method downloads the model collection if it is not already on the device. Once
        /// all models are downloaded, an MLModelCollection instance is made available for use with the completion handler.
        ///
        ///
        /// Parameter `identifier`: The model collection identifier, as managed in Core ML Model Deployment.
        ///
        /// Parameter `completionHandler`: The completion handler, invoked with a valid MLModelCollection instance on success or NSError on failure.
        ///
        /// Returns: NSProgress for updates during setup and download of the model collection
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other beginAccessingModelCollectionWithIdentifier:completionHandler:)]
        pub unsafe fn beginAccessingModelCollectionWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut MLModelCollection, *mut NSError)>,
        ) -> Retained<NSProgress>;

        #[cfg(feature = "block2")]
        /// End access to a model collection. This informs the system you have finished accessing the models within the collection.
        ///
        ///
        /// Call this method as soon as you have finished using the models in this collection.
        ///
        ///
        /// Parameter `identifier`: The model collection identifier, as managed in Core ML Model Deployment.
        ///
        /// Parameter `completionHandler`: The completion handler, invoked with YES on success or NSError on failure.
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method(endAccessingModelCollectionWithIdentifier:completionHandler:)]
        pub unsafe fn endAccessingModelCollectionWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// Notification posted when the model collection has changed.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelcollectiondidchangenotification?language=objc)
    pub static MLModelCollectionDidChangeNotification: &'static NSNotificationName;
}
