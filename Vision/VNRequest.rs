//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-ml")]
use objc2_core_ml::*;
use objc2_foundation::*;

use crate::*;

/// A block that is executed at the completion of a request.
///
/// The completion handler is called for each request when it is finished processing, before the performRequests call returns. When an array of multiple requests is executed with one performRequests call, each request's completion handler is invoked when that request has finished its processing. This invocation may therefore occur while other requests in the array are either still executing or waiting for execution. This allows, for example, UI to be updated while the first tasks are complete instead of having to wait that all requests have to finish. Note, however, that performRequests is not an asynchronous method, for which completion handlers are most typically used
///
/// Parameter `request`: The VNRequest that has been completed. The results of the request if no error was encountered are populated in the results array of the request.
///
/// Parameter `error`: The error that caused the request to fail, or nil if completed successfully.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vnrequestcompletionhandler?language=objc)
#[cfg(feature = "block2")]
pub type VNRequestCompletionHandler = *mut block2::Block<dyn Fn(NonNull<VNRequest>, *mut NSError)>;

extern_class!(
    /// VNRequest objects describe the operation to be performed as well as act as the recipient of the operation's resultant observations.
    ///
    /// VNRequest objects are instantiated in a pre-configured nominal state. Prior to sending a VNRequest to a request handler to perform a desired operation, the default configuration can be changed by modifying the values of VNRequest properties. The VNRequest class itself acts as a base class and is not meant to be directly instantiated.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vnrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNRequest;
);

unsafe impl NSCopying for VNRequest {}

unsafe impl CopyingHelper for VNRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VNRequest {}

extern_methods!(
    unsafe impl VNRequest {
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

        /// A hint used to minimize the resource burden of the request. Memory footprint, processing footprint and/or CPU/GPU contention will be reduced (depending on the request), at the potential cost of longer execution time. This can help, for example, with ensuring UI updates and rendering are not getting blocked by Vision processing.
        #[method(preferBackgroundProcessing)]
        pub unsafe fn preferBackgroundProcessing(&self) -> bool;

        /// Setter for [`preferBackgroundProcessing`][Self::preferBackgroundProcessing].
        #[method(setPreferBackgroundProcessing:)]
        pub unsafe fn setPreferBackgroundProcessing(&self, prefer_background_processing: bool);

        /// This property, if set to YES, signifies that the request should be performed exclusively on the CPU and not on the GPU. The default value is NO, which signifies that the request is free to leverage the GPU to accelerate any work the request may require.
        #[deprecated]
        #[method(usesCPUOnly)]
        pub unsafe fn usesCPUOnly(&self) -> bool;

        /// Setter for [`usesCPUOnly`][Self::usesCPUOnly].
        #[deprecated]
        #[method(setUsesCPUOnly:)]
        pub unsafe fn setUsesCPUOnly(&self, uses_cpu_only: bool);

        #[cfg(feature = "VNObservation")]
        /// The collection of VNObservations generated by the processing of the request.
        ///
        /// The only valid time to access this property is after the request has been processed by a request handler.  If the request failed, this property will be nil; otherwise, it will be an array of zero or more VNObservation subclasses specific to the VNRequest subclass.
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNObservation>>>;

        #[cfg(feature = "block2")]
        /// The completion handler block that will be invoked after the request has completed processing.
        #[method(completionHandler)]
        pub unsafe fn completionHandler(&self) -> VNRequestCompletionHandler;

        /// The specific algorithm or implementation revision that is to be used to perform the request.
        #[method(revision)]
        pub unsafe fn revision(&self) -> NSUInteger;

        /// Setter for [`revision`][Self::revision].
        #[method(setRevision:)]
        pub unsafe fn setRevision(&self, revision: NSUInteger);

        /// Provides the collection of currently-supported algorithm or implementation versions for the class of request.
        ///
        /// This method allows clients to introspect at runtime what capabilities are available for each class of VNRequest in the Vision framework.
        #[method_id(@__retain_semantics Other supportedRevisions)]
        pub unsafe fn supportedRevisions() -> Retained<NSIndexSet>;

        /// Provides the revision of the request that was latest for the particular SDK that was linked with the client application.
        #[method(defaultRevision)]
        pub unsafe fn defaultRevision() -> NSUInteger;

        /// Provides the current revision supported by the request.
        #[method(currentRevision)]
        pub unsafe fn currentRevision() -> NSUInteger;

        /// Tries to abort the request as soon as possible. Results will be nil. The completionHandler (if present) will be called with an error of VNErrorRequestCancelled.
        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// A value that indicates that the request revision is either unknown or not applicable.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vnrequestrevisionunspecified?language=objc)
pub static VNRequestRevisionUnspecified: NSUInteger = 0;

extern_methods!(
    unsafe impl VNRequest {
        #[cfg(all(feature = "VNTypes", feature = "objc2-core-ml"))]
        /// Obtain the collection of compute device per stage that are supported by the request.
        ///
        /// This method's result is based on the current state of configuration of the target request at the time of the call.
        ///
        ///
        /// Parameter `error`: The address of a variable that will be populated with the error that describes the failure.  If the caller does not require this information, NULL can be passed.
        ///
        ///
        /// Returns: A dictionary of per-stage supported compute devices, or `nil` if an error occurs.
        #[method_id(@__retain_semantics Other supportedComputeStageDevicesAndReturnError:_)]
        pub unsafe fn supportedComputeStageDevicesAndReturnError(
            &self,
        ) -> Result<
            Retained<
                NSDictionary<VNComputeStage, NSArray<ProtocolObject<dyn MLComputeDeviceProtocol>>>,
            >,
            Retained<NSError>,
        >;

        #[cfg(all(feature = "VNTypes", feature = "objc2-core-ml"))]
        /// Determine what the currently configured compute device is for a specific compute stage.
        ///
        ///
        /// Parameter `computeStage`: The compute stage to be introspected.
        ///
        ///
        /// Returns: The currently assigned compute device, or `nil` if there is no explicit assignment.
        #[method_id(@__retain_semantics Other computeDeviceForComputeStage:)]
        pub unsafe fn computeDeviceForComputeStage(
            &self,
            compute_stage: &VNComputeStage,
        ) -> Option<Retained<ProtocolObject<dyn MLComputeDeviceProtocol>>>;

        #[cfg(all(feature = "VNTypes", feature = "objc2-core-ml"))]
        /// Assign a specific compute device for a compute stage.
        ///
        /// It is important to note that any compute device can be configured for a given compute stage.  Only when the request is performed is the validity of the (compute device / compute stage) assignments checked.  Valid compute devices for a request's compute stages can be obtained via `-supportedComputeStageDevicesAndReturnError:`.
        ///
        ///
        /// Parameter `computeDevice`: The compute device to assign to the compute stage.  Passing nil for this parameter will remove any explicit compute device assignment, allowing Vision to select which device to use.
        ///
        /// Parameter `computeStage`: The compute stage being configured.
        #[method(setComputeDevice:forComputeStage:)]
        pub unsafe fn setComputeDevice_forComputeStage(
            &self,
            compute_device: Option<&ProtocolObject<dyn MLComputeDeviceProtocol>>,
            compute_stage: &VNComputeStage,
        );
    }
);

extern_class!(
    /// A request that will process the contents of a reference image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vnimagebasedrequest?language=objc)
    #[unsafe(super(VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNImageBasedRequest;
);

unsafe impl NSCopying for VNImageBasedRequest {}

unsafe impl CopyingHelper for VNImageBasedRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VNImageBasedRequest {}

extern_methods!(
    unsafe impl VNImageBasedRequest {
        #[cfg(feature = "objc2-core-foundation")]
        /// The region of the image in which the request will be performed.  The rectangle is normalized to the dimensions of the image being processed and has its origin specified relative to the image's lower-left corner.
        ///
        ///
        /// The default value for this property is { { 0, 0 }, { 1, 1 } }.  Setting this property to a rectangle that is outside of the normalized coordinate space will be accepted but result in the request failing to be performed.
        #[method(regionOfInterest)]
        pub unsafe fn regionOfInterest(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`regionOfInterest`][Self::regionOfInterest].
        #[method(setRegionOfInterest:)]
        pub unsafe fn setRegionOfInterest(&self, region_of_interest: CGRect);
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    unsafe impl VNImageBasedRequest {
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
    unsafe impl VNImageBasedRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// A block that is executed at intervals during the processing of a request.
///
/// Parameter `request`: The VNRequest that has been completed. The results of the request, if no error was encountered, are populated in the results array of the request.
///
/// Parameter `fractionCompleted`: When possible the request will report its progress between 0.0 and 1.0. If the requests indeterminate property is set, then this value is undefined.
///
/// Parameter `error`: The error that caused the request to fail, or nil if completed successfully.
///
/// The results in the request can be populated with partial results. The progressHandler can be called on a different dispatch queue than what the request was initiated from.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vnrequestprogresshandler?language=objc)
#[cfg(feature = "block2")]
pub type VNRequestProgressHandler =
    *mut block2::Block<dyn Fn(NonNull<VNRequest>, c_double, *mut NSError)>;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnrequestprogressproviding?language=objc)
    pub unsafe trait VNRequestProgressProviding: NSObjectProtocol {
        #[cfg(feature = "block2")]
        /// Requests that support the VNRequestProgressProviding protocol would periodically call the progressHandler to report progress on longer running tasks.
        ///
        ///
        /// The progessHandler is optional allowing clients of the request to report progress to the user and/or display or process partial results when they become available. Note that the progressHandler can be called on a different dispatch queue than what the request was initiated from.
        #[method(progressHandler)]
        unsafe fn progressHandler(&self) -> VNRequestProgressHandler;

        #[cfg(feature = "block2")]
        /// Setter for [`progressHandler`][Self::progressHandler].
        #[method(setProgressHandler:)]
        unsafe fn setProgressHandler(&self, progress_handler: VNRequestProgressHandler);

        /// If a request cannot determine its progress in fractions completed, this property will be set.
        ///
        /// If this is set, it doesn't mean that the request will run forever just that the nature of the request is not broken down into identifiable fractions on which progress can be reported in increments. The progressHandler will nonetheless be called at suitable intervals.
        #[method(indeterminate)]
        unsafe fn indeterminate(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn VNRequestProgressProviding {}
);
