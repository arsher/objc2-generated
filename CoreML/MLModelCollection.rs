//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelcollection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use Background Assets or NSURLSession instead."]
    pub struct MLModelCollection;
);

unsafe impl NSObjectProtocol for MLModelCollection {}

extern_methods!(
    unsafe impl MLModelCollection {
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[cfg(feature = "MLModelCollectionEntry")]
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other entries)]
        pub unsafe fn entries(&self) -> Retained<NSDictionary<NSString, MLModelCollectionEntry>>;

        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other deploymentID)]
        pub unsafe fn deploymentID(&self) -> Retained<NSString>;

        #[cfg(feature = "block2")]
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other beginAccessingModelCollectionWithIdentifier:completionHandler:)]
        pub unsafe fn beginAccessingModelCollectionWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut MLModelCollection, *mut NSError)>,
        ) -> Retained<NSProgress>;

        #[cfg(feature = "block2")]
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
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelcollectiondidchangenotification?language=objc)
    pub static MLModelCollectionDidChangeNotification: &'static NSNotificationName;
}
