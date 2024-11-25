//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearchcompletionhandler?language=objc)
#[cfg(all(feature = "MKLocalSearchResponse", feature = "block2"))]
pub type MKLocalSearchCompletionHandler =
    *mut block2::Block<dyn Fn(*mut MKLocalSearchResponse, *mut NSError)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearch?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalSearch;
);

unsafe impl NSObjectProtocol for MKLocalSearch {}

extern_methods!(
    unsafe impl MKLocalSearch {
        #[cfg(feature = "MKLocalSearchRequest")]
        #[method_id(@__retain_semantics Init initWithRequest:)]
        pub unsafe fn initWithRequest(
            this: Allocated<Self>,
            request: &MKLocalSearchRequest,
        ) -> Retained<Self>;

        #[cfg(feature = "MKLocalPointsOfInterestRequest")]
        #[method_id(@__retain_semantics Init initWithPointsOfInterestRequest:)]
        pub unsafe fn initWithPointsOfInterestRequest(
            this: Allocated<Self>,
            request: &MKLocalPointsOfInterestRequest,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKLocalSearchResponse", feature = "block2"))]
        #[method(startWithCompletionHandler:)]
        pub unsafe fn startWithCompletionHandler(
            &self,
            completion_handler: MKLocalSearchCompletionHandler,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isSearching)]
        pub unsafe fn isSearching(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalSearch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
