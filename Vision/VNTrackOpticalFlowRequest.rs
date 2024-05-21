//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNTrackOpticalFlowRequestComputationAccuracy(pub NSUInteger);
impl VNTrackOpticalFlowRequestComputationAccuracy {
    #[doc(alias = "VNTrackOpticalFlowRequestComputationAccuracyLow")]
    pub const Low: Self = Self(0);
    #[doc(alias = "VNTrackOpticalFlowRequestComputationAccuracyMedium")]
    pub const Medium: Self = Self(1);
    #[doc(alias = "VNTrackOpticalFlowRequestComputationAccuracyHigh")]
    pub const High: Self = Self(2);
    #[doc(alias = "VNTrackOpticalFlowRequestComputationAccuracyVeryHigh")]
    pub const VeryHigh: Self = Self(3);
}

unsafe impl Encode for VNTrackOpticalFlowRequestComputationAccuracy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VNTrackOpticalFlowRequestComputationAccuracy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    pub struct VNTrackOpticalFlowRequest;

    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl ClassType for VNTrackOpticalFlowRequest {
        #[inherits(VNImageBasedRequest, VNRequest, NSObject)]
        type Super = VNStatefulRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSCopying for VNTrackOpticalFlowRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSObjectProtocol for VNTrackOpticalFlowRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNTrackOpticalFlowRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[method(computationAccuracy)]
        pub unsafe fn computationAccuracy(&self) -> VNTrackOpticalFlowRequestComputationAccuracy;

        #[method(setComputationAccuracy:)]
        pub unsafe fn setComputationAccuracy(
            &self,
            computation_accuracy: VNTrackOpticalFlowRequestComputationAccuracy,
        );

        #[method(outputPixelFormat)]
        pub unsafe fn outputPixelFormat(&self) -> OSType;

        #[method(setOutputPixelFormat:)]
        pub unsafe fn setOutputPixelFormat(&self, output_pixel_format: OSType);

        #[method(keepNetworkOutput)]
        pub unsafe fn keepNetworkOutput(&self) -> bool;

        #[method(setKeepNetworkOutput:)]
        pub unsafe fn setKeepNetworkOutput(&self, keep_network_output: bool);

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNPixelBufferObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNStatefulRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNTrackOpticalFlowRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

pub static VNTrackOpticalFlowRequestRevision1: NSUInteger = 1;
