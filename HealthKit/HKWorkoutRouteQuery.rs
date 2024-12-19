//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutroutequery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKWorkoutRouteQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKWorkoutRouteQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKWorkoutRouteQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKWorkoutRouteQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKWorkoutRouteQuery {
        #[cfg(all(
            feature = "HKObject",
            feature = "HKSample",
            feature = "HKSeriesSample",
            feature = "HKWorkoutRoute",
            feature = "block2",
            feature = "objc2-core-location"
        ))]
        #[method_id(@__retain_semantics Init initWithRoute:dataHandler:)]
        pub unsafe fn initWithRoute_dataHandler(
            this: Allocated<Self>,
            workout_route: &HKWorkoutRoute,
            data_handler: &block2::Block<
                dyn Fn(NonNull<HKWorkoutRouteQuery>, *mut NSArray<CLLocation>, Bool, *mut NSError),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKSample",
            feature = "HKSeriesSample",
            feature = "HKWorkoutRoute",
            feature = "block2",
            feature = "objc2-core-location"
        ))]
        #[method_id(@__retain_semantics Init initWithRoute:dateInterval:dataHandler:)]
        pub unsafe fn initWithRoute_dateInterval_dataHandler(
            this: Allocated<Self>,
            workout_route: &HKWorkoutRoute,
            date_interval: &NSDateInterval,
            data_handler: &block2::Block<
                dyn Fn(NonNull<HKWorkoutRouteQuery>, *mut NSArray<CLLocation>, Bool, *mut NSError),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKWorkoutRouteQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKWorkoutRouteQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
