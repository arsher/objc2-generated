//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalidentifier?language=objc)
// NS_TYPED_ENUM
pub type VNAnimalIdentifier = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalidentifierdog?language=objc)
    pub static VNAnimalIdentifierDog: &'static VNAnimalIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalidentifiercat?language=objc)
    pub static VNAnimalIdentifierCat: &'static VNAnimalIdentifier;
}

extern_class!(
    /// A request that will recognize various animals in an image. The list of animals supported by the recognition algorithm can be queried by  -supportedIdentifiersAndReturnError:
    ///
    ///
    /// This request will generate VNRecognizedObjectObservation objects with a defined boundingBox, label and confidence level.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vnrecognizeanimalsrequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNRecognizeAnimalsRequest;
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNRecognizeAnimalsRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNRecognizeAnimalsRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNRecognizeAnimalsRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNRecognizeAnimalsRequest {
        /// This class method returns a list of all animals supported by the recognition algorithm
        ///
        ///
        /// This request will generate a collection of names for supported animals by current recognition algorithm.
        #[deprecated]
        #[method_id(@__retain_semantics Other knownAnimalIdentifiersForRevision:error:_)]
        pub unsafe fn knownAnimalIdentifiersForRevision_error(
            request_revision: NSUInteger,
        ) -> Result<Retained<NSArray<VNAnimalIdentifier>>, Retained<NSError>>;

        /// Obtain the collection of identifiers supported by the target request.
        ///
        /// This method will return the collection of all possible classification identifiers that are produced by the target request based on its current state of configuration at the time of the call.
        ///
        ///
        /// Parameter `error`: The address of the variable that will be populated with the error if the call fails.
        ///
        ///
        /// Returns: The collection of classification identifiers, or nil if a failure occurs.
        #[method_id(@__retain_semantics Other supportedIdentifiersAndReturnError:_)]
        pub unsafe fn supportedIdentifiersAndReturnError(
            &self,
        ) -> Result<Retained<NSArray<VNAnimalIdentifier>>, Retained<NSError>>;

        #[cfg(feature = "VNObservation")]
        /// VNRecognizedObjectObservation results.
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNRecognizedObjectObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNRecognizeAnimalsRequest {
        /// Creates a new VNRequest with no completion handler.
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Creates a new VNRequest with an optional completion handler.
        ///
        ///
        /// Parameter `completionHandler`: The block to be invoked after the request has completed its processing. The completion handler gets executed on the same dispatch queue as the request being executed.
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNRecognizeAnimalsRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnrecognizeanimalsrequestrevision1?language=objc)
pub static VNRecognizeAnimalsRequestRevision1: NSUInteger = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnrecognizeanimalsrequestrevision2?language=objc)
pub static VNRecognizeAnimalsRequestRevision2: NSUInteger = 2;
