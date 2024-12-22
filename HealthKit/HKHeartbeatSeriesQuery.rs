//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkheartbeatseriesquery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKHeartbeatSeriesQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKHeartbeatSeriesQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKHeartbeatSeriesQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKHeartbeatSeriesQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKHeartbeatSeriesQuery {
        #[cfg(all(
            feature = "HKHeartbeatSeriesSample",
            feature = "HKObject",
            feature = "HKSample",
            feature = "HKSeriesSample",
            feature = "block2"
        ))]
        /// Returns a query that will retrieve heartbeat timestamps for the specified HKHeartbeatSeriesSample.
        ///
        ///
        /// Parameter `heartbeatSeries`: The HKHeartbeatSeriesSample for which the heartbeat data will be returned.
        ///
        /// Parameter `dataHandler`: The block to invoke with results from the query. It is called repeatedly for each
        /// heartbeat in the series. timeSinceSeriesStart is the time elapsed in seconds after the
        /// series startDate that represents when the heartbeat occured. precededByGap indicates if
        /// there was a gap in data collection before the current heartbeat, meaning that one or more
        /// heartbeats may have occured since the previous heartbeat in the series. Once done is YES,
        /// or stopQuery called, the query is complete and no more calls to the handler will be made.
        #[method_id(@__retain_semantics Init initWithHeartbeatSeries:dataHandler:)]
        pub unsafe fn initWithHeartbeatSeries_dataHandler(
            this: Allocated<Self>,
            heartbeat_series: &HKHeartbeatSeriesSample,
            data_handler: &block2::Block<
                dyn Fn(NonNull<HKHeartbeatSeriesQuery>, NSTimeInterval, Bool, Bool, *mut NSError),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKHeartbeatSeriesQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKHeartbeatSeriesQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
