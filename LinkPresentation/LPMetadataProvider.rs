//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LPMetadataProvider;

    unsafe impl ClassType for LPMetadataProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LPMetadataProvider {}

extern_methods!(
    unsafe impl LPMetadataProvider {
        #[cfg(all(feature = "LPLinkMetadata", feature = "block2"))]
        #[method(startFetchingMetadataForURL:completionHandler:)]
        pub unsafe fn startFetchingMetadataForURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &block2::Block<dyn Fn(*mut LPLinkMetadata, *mut NSError)>,
        );

        #[cfg(all(feature = "LPLinkMetadata", feature = "block2"))]
        #[method(startFetchingMetadataForRequest:completionHandler:)]
        pub unsafe fn startFetchingMetadataForRequest_completionHandler(
            &self,
            request: &NSURLRequest,
            completion_handler: &block2::Block<dyn Fn(*mut LPLinkMetadata, *mut NSError)>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(shouldFetchSubresources)]
        pub unsafe fn shouldFetchSubresources(&self) -> bool;

        #[method(setShouldFetchSubresources:)]
        pub unsafe fn setShouldFetchSubresources(&self, should_fetch_subresources: bool);

        #[method(timeout)]
        pub unsafe fn timeout(&self) -> NSTimeInterval;

        #[method(setTimeout:)]
        pub unsafe fn setTimeout(&self, timeout: NSTimeInterval);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LPMetadataProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
