//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkobjectquerynolimit?language=objc)
pub static HKObjectQueryNoLimit: NSUInteger = 0;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hksamplequery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKSampleQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKSampleQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKSampleQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKSampleQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKSampleQuery {
        /// The maximum number of results the receiver will return upon completion.
        #[method(limit)]
        pub unsafe fn limit(&self) -> NSUInteger;

        /// An array of NSSortDescriptors.
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Retained<NSArray<NSSortDescriptor>>>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKObjectType",
            feature = "HKSample",
            feature = "block2"
        ))]
        /// Returns a query that will retrieve HKSamples matching the given predicate.
        ///
        ///
        /// Parameter `sampleType`: The type of sample to retrieve.
        ///
        /// Parameter `predicate`: The predicate which samples should match.
        ///
        /// Parameter `limit`: The maximum number of samples to return.  Pass HKObjectQueryNoLimit for no limit.
        ///
        /// Parameter `sortDescriptors`: The sort descriptors to use to order the resulting samples.
        ///
        /// Parameter `resultsHandler`: The block to invoke with results when the query has finished executing.
        #[method_id(@__retain_semantics Init initWithSampleType:predicate:limit:sortDescriptors:resultsHandler:)]
        pub unsafe fn initWithSampleType_predicate_limit_sortDescriptors_resultsHandler(
            this: Allocated<Self>,
            sample_type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            limit: NSUInteger,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
            results_handler: &block2::Block<
                dyn Fn(NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKQueryDescriptor",
            feature = "HKSample",
            feature = "block2"
        ))]
        /// Returns a query that will retrieve HKSamples matching any of the given queryDescriptors.
        ///
        ///
        /// Parameter `queryDescriptors`: An array of query descriptors that describes the sample types and predicates
        /// used for querying.
        ///
        /// Parameter `limit`: The maximum number of samples to return. Pass HKObjectQueryNoLimit
        /// for no limit.
        ///
        /// Parameter `resultsHandler`: The block to invoke with results when the query has finished executing. This
        /// block is invoked once with results, an array of HKSamples matching the
        /// queryDescriptors passed in, or nil if an error occurred.
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:limit:resultsHandler:)]
        pub unsafe fn initWithQueryDescriptors_limit_resultsHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            limit: NSInteger,
            results_handler: &block2::Block<
                dyn Fn(NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKQueryDescriptor",
            feature = "HKSample",
            feature = "block2"
        ))]
        /// Returns a query that will retrieve HKSamples matching any of the given queryDescriptors.
        ///
        ///
        /// Parameter `queryDescriptors`: An array of query descriptors that describes the sample types and predicates
        /// used for querying.
        ///
        /// Parameter `limit`: The maximum number of samples to return. Pass HKObjectQueryNoLimit
        /// for no limit.
        ///
        /// Parameter `sortDescriptors`: The sort descriptors to use to order the resulting samples.
        ///
        /// Parameter `resultsHandler`: The block to invoke with results when the query has finished executing. This
        /// block is invoked once with results, an array of HKSamples matching the
        /// queryDescriptors passed in, or nil if an error occurred. The HKSamples in the
        /// array are sorted by the specified sortDescriptors.
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:limit:sortDescriptors:resultsHandler:)]
        pub unsafe fn initWithQueryDescriptors_limit_sortDescriptors_resultsHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            limit: NSInteger,
            sort_descriptors: &NSArray<NSSortDescriptor>,
            results_handler: &block2::Block<
                dyn Fn(NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKSampleQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKSampleQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
