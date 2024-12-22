//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// MLPredictionOptions
    ///
    /// An object to hold options / controls / parameters of how
    /// model prediction is performed
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlpredictionoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLPredictionOptions;
);

unsafe impl NSObjectProtocol for MLPredictionOptions {}

extern_methods!(
    unsafe impl MLPredictionOptions {
        /// Set to YES to force computation to be on the CPU only
        #[deprecated]
        #[method(usesCPUOnly)]
        pub unsafe fn usesCPUOnly(&self) -> bool;

        /// Setter for [`usesCPUOnly`][Self::usesCPUOnly].
        #[deprecated]
        #[method(setUsesCPUOnly:)]
        pub unsafe fn setUsesCPUOnly(&self, uses_cpu_only: bool);

        /// Propose the model to use the specified backing objects for the
        /// output feature values.
        ///
        ///
        /// Use the property to get the inference result directly into the
        /// client allocated buffer when possible for efficient memory management.
        ///
        /// The property is a dictionary of the feature name and the output backing
        /// object.
        ///
        /// The framework may not use the specified backing object and instead allocates
        /// one by itself if the outputBacking dictionary doesn't contain the entry for
        /// the feature name, the model doesn't support the user allocated buffers, or in
        /// the batch prediction mode. To check if the backing object was used, compare
        /// the output prediction and the backing object by object identity.
        ///
        ///
        /// ```text
        ///   CVPixelBufferRef outputBacking = ...;
        ///   [options setOutputBackings:@{@"outputImage" : (__bridge id)outputBacking}];
        ///   id<MLFeatureProvider> prediction = [model predictionFromFeatures:inputFeatures options:options error:&error];
        ///   if ([prediction featureValueForName:@"outputImage"].imageBufferValue == outputBacking) {
        ///     // backing was used.
        ///   }
        ///   else {
        ///     // backing was NOT used.
        ///   }
        /// ```
        ///
        /// The backing object must be either CVPixelBuffer or MLMultiArray depending on
        /// the feature value type.
        ///
        /// Do not lock the base address of the CVPixelBuffer. In the case of a MLMultiArray
        /// backed by a pixel buffer, make sure not to lock the underlying pixel buffer by not
        /// calling any data methods such as `.dataPointer` and subscript methods before the
        /// prediction.
        ///
        /// The framework ignores a backing object with an unknown feature name.
        ///
        /// For the best performance, use page-aligned address in MLMultiArray.
        ///
        ///
        /// ```text
        ///   #import <mach/vm_page_size.h>
        ///   :
        ///   void *backingBuffer = aligned_alloc(vm_page_size, round_page(backingBufferSize));
        ///   if (backingBuffer == NULL) { ... error handling ... }
        ///   MLMultiArray *outputBacking = [[MLMultiArray alloc] initWithDataPointer:(char *)backingBuffer
        ///                                                                         ...
        ///                                                               deallocator:^(void *) { free(backingBuffer); }
        ///                                                                         ... ];
        /// ```
        ///
        /// For CVPixelBuffer backing, consider to use IOSurface-backed CVPixelBuffer
        /// created by CVPixelBufferPool because it is often the most efficient choice for
        /// memory footprint and performance, especially when the pixel buffers are
        /// subsequently used for playback or export. (See also AVSampleBufferDisplayLayer
        /// and AVAssetWriter.)
        ///
        /// The output backing object must satisfy the output feature description's
        /// `-isAllowedValue:` test, or the framework reporets an error at the prediction
        /// time. The exception is FP16 MLMultiArray backed by CVPixelBuffer, which may be
        /// accepted in Double or Float32 multi array output feature depending on the
        /// underlying inference engine.
        #[method_id(@__retain_semantics Other outputBackings)]
        pub unsafe fn outputBackings(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        /// Setter for [`outputBackings`][Self::outputBackings].
        #[method(setOutputBackings:)]
        pub unsafe fn setOutputBackings(&self, output_backings: &NSDictionary<NSString, AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLPredictionOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
