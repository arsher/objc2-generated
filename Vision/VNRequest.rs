//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-ml")]
use objc2_core_ml::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "block2")]
pub type VNRequestCompletionHandler = *mut block2::Block<dyn Fn(NonNull<VNRequest>, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNRequest;

    unsafe impl ClassType for VNRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for VNRequest {}

unsafe impl NSObjectProtocol for VNRequest {}

extern_methods!(
    unsafe impl VNRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Id<Self>;

        #[method(preferBackgroundProcessing)]
        pub unsafe fn preferBackgroundProcessing(&self) -> bool;

        #[method(setPreferBackgroundProcessing:)]
        pub unsafe fn setPreferBackgroundProcessing(&self, prefer_background_processing: bool);

        #[deprecated]
        #[method(usesCPUOnly)]
        pub unsafe fn usesCPUOnly(&self) -> bool;

        #[deprecated]
        #[method(setUsesCPUOnly:)]
        pub unsafe fn setUsesCPUOnly(&self, uses_cpu_only: bool);

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Id<NSArray<VNObservation>>>;

        #[cfg(feature = "block2")]
        #[method(completionHandler)]
        pub unsafe fn completionHandler(&self) -> VNRequestCompletionHandler;

        #[method(revision)]
        pub unsafe fn revision(&self) -> NSUInteger;

        #[method(setRevision:)]
        pub unsafe fn setRevision(&self, revision: NSUInteger);

        #[method_id(@__retain_semantics Other supportedRevisions)]
        pub unsafe fn supportedRevisions() -> Id<NSIndexSet>;

        #[method(defaultRevision)]
        pub unsafe fn defaultRevision() -> NSUInteger;

        #[method(currentRevision)]
        pub unsafe fn currentRevision() -> NSUInteger;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

pub static VNRequestRevisionUnspecified: NSUInteger = 0;

extern_methods!(
    unsafe impl VNRequest {
        #[cfg(all(feature = "VNTypes", feature = "objc2-core-ml"))]
        #[method_id(@__retain_semantics Other supportedComputeStageDevicesAndReturnError:_)]
        pub unsafe fn supportedComputeStageDevicesAndReturnError(
            &self,
        ) -> Result<
            Id<NSDictionary<VNComputeStage, NSArray<ProtocolObject<dyn MLComputeDeviceProtocol>>>>,
            Id<NSError>,
        >;

        #[cfg(all(feature = "VNTypes", feature = "objc2-core-ml"))]
        #[method_id(@__retain_semantics Other computeDeviceForComputeStage:)]
        pub unsafe fn computeDeviceForComputeStage(
            &self,
            compute_stage: &VNComputeStage,
        ) -> Option<Id<ProtocolObject<dyn MLComputeDeviceProtocol>>>;

        #[cfg(all(feature = "VNTypes", feature = "objc2-core-ml"))]
        #[method(setComputeDevice:forComputeStage:)]
        pub unsafe fn setComputeDevice_forComputeStage(
            &self,
            compute_device: Option<&ProtocolObject<dyn MLComputeDeviceProtocol>>,
            compute_stage: &VNComputeStage,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNImageBasedRequest;

    unsafe impl ClassType for VNImageBasedRequest {
        #[inherits(NSObject)]
        type Super = VNRequest;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for VNImageBasedRequest {}

unsafe impl NSObjectProtocol for VNImageBasedRequest {}

extern_methods!(
    unsafe impl VNImageBasedRequest {
        #[method(regionOfInterest)]
        pub unsafe fn regionOfInterest(&self) -> CGRect;

        #[method(setRegionOfInterest:)]
        pub unsafe fn setRegionOfInterest(&self, region_of_interest: CGRect);
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    unsafe impl VNImageBasedRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNImageBasedRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "block2")]
pub type VNRequestProgressHandler =
    *mut block2::Block<dyn Fn(NonNull<VNRequest>, c_double, *mut NSError)>;

extern_protocol!(
    pub unsafe trait VNRequestProgressProviding: NSObjectProtocol {
        #[cfg(feature = "block2")]
        #[method(progressHandler)]
        unsafe fn progressHandler(&self) -> VNRequestProgressHandler;

        #[cfg(feature = "block2")]
        #[method(setProgressHandler:)]
        unsafe fn setProgressHandler(&self, progress_handler: VNRequestProgressHandler);

        #[method(indeterminate)]
        unsafe fn indeterminate(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn VNRequestProgressProviding {}
);