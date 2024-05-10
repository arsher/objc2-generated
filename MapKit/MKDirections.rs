//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "MKDirectionsResponse", feature = "block2"))]
pub type MKDirectionsHandler = *mut block2::Block<dyn Fn(*mut MKDirectionsResponse, *mut NSError)>;

#[cfg(all(feature = "MKDirectionsResponse", feature = "block2"))]
pub type MKETAHandler = *mut block2::Block<dyn Fn(*mut MKETAResponse, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKDirections;

    unsafe impl ClassType for MKDirections {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MKDirections {}

extern_methods!(
    unsafe impl MKDirections {
        #[cfg(feature = "MKDirectionsRequest")]
        #[method_id(@__retain_semantics Init initWithRequest:)]
        pub unsafe fn initWithRequest(
            this: Allocated<Self>,
            request: &MKDirectionsRequest,
        ) -> Id<Self>;

        #[cfg(all(feature = "MKDirectionsResponse", feature = "block2"))]
        #[method(calculateDirectionsWithCompletionHandler:)]
        pub unsafe fn calculateDirectionsWithCompletionHandler(
            &self,
            completion_handler: MKDirectionsHandler,
        );

        #[cfg(all(feature = "MKDirectionsResponse", feature = "block2"))]
        #[method(calculateETAWithCompletionHandler:)]
        pub unsafe fn calculateETAWithCompletionHandler(&self, completion_handler: MKETAHandler);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isCalculating)]
        pub unsafe fn isCalculating(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKDirections {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
