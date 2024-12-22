//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A query to find HKCorrelations
    ///
    /// Correlations are HKSamples that contain a set of correlated samples. HKCorrelationQuery
    /// accepts a predicate to filter HKCorrelations and a dictionary of predicates to filter the
    /// correlated samples.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkcorrelationquery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKCorrelationQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKCorrelationQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKCorrelationQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKCorrelationQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKCorrelationQuery {
        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other correlationType)]
        pub unsafe fn correlationType(&self) -> Retained<HKCorrelationType>;

        #[cfg(feature = "HKObjectType")]
        /// A dictionary of predicates for the HKCorrelation's objects
        ///
        /// samplePredicates maps HKSampleTypes to NSPredicates. The predicate value will apply
        /// to objects of the key type.
        #[method_id(@__retain_semantics Other samplePredicates)]
        pub unsafe fn samplePredicates(
            &self,
        ) -> Option<Retained<NSDictionary<HKSampleType, NSPredicate>>>;

        #[cfg(all(
            feature = "HKCorrelation",
            feature = "HKObject",
            feature = "HKObjectType",
            feature = "HKSample",
            feature = "block2"
        ))]
        /// The designated initializer for HKCorrelationQuery.
        ///
        ///
        /// Parameter `correlationType`: The type of correlation that is being queried for
        ///
        ///
        /// Parameter `predicate`: The predicate for scoping which HKCorrelations are returned
        ///
        ///
        /// Parameter `samplePredicates`: A dictionary mapping HKSampleTypes to NSPredicates. If no predicate for a particular type
        /// is provided, it is assumed to be a nil predicate and objects of that type will not be
        /// filtered.
        #[method_id(@__retain_semantics Init initWithType:predicate:samplePredicates:completion:)]
        pub unsafe fn initWithType_predicate_samplePredicates_completion(
            this: Allocated<Self>,
            correlation_type: &HKCorrelationType,
            predicate: Option<&NSPredicate>,
            sample_predicates: Option<&NSDictionary<HKSampleType, NSPredicate>>,
            completion: &block2::Block<
                dyn Fn(NonNull<HKCorrelationQuery>, *mut NSArray<HKCorrelation>, *mut NSError),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKCorrelationQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKCorrelationQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
