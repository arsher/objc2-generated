//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    pub struct VNTrackHomographicImageRegistrationRequest;

    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl ClassType for VNTrackHomographicImageRegistrationRequest {
        #[inherits(VNImageBasedRequest, VNRequest, NSObject)]
        type Super = VNStatefulRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSCopying for VNTrackHomographicImageRegistrationRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSObjectProtocol for VNTrackHomographicImageRegistrationRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNTrackHomographicImageRegistrationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(
            &self,
        ) -> Option<Retained<NSArray<VNImageHomographicAlignmentObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNStatefulRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNTrackHomographicImageRegistrationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

pub static VNTrackHomographicImageRegistrationRequestRevision1: NSUInteger = 1;
