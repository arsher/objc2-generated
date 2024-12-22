//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An HKCorrelation is a collection of correlated objects.
    ///
    /// When multiple readings are taken together, it may be beneficial to correlate them so that they can be
    /// displayed together and share common metadata about how they were created.
    ///
    /// For example, systolic and diastolic blood pressure readings are typically presented together so these
    /// readings should be saved with a correlation of type blood pressure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkcorrelation?language=objc)
    #[unsafe(super(HKSample, HKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    pub struct HKCorrelation;
);

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Send for HKCorrelation {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Sync for HKCorrelation {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCoding for HKCorrelation {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSObjectProtocol for HKCorrelation {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSSecureCoding for HKCorrelation {}

extern_methods!(
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKCorrelation {
        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other correlationType)]
        pub unsafe fn correlationType(&self) -> Retained<HKCorrelationType>;

        /// A set of HKSamples containing all of the objects that were saved with the receiver.
        #[method_id(@__retain_semantics Other objects)]
        pub unsafe fn objects(&self) -> Retained<NSSet<HKSample>>;

        #[cfg(feature = "HKObjectType")]
        /// Creates a new HKCorrelation with the given type, start date, end date, and objects.
        ///
        /// objects must be a set of HKQuantitySamples and HKCategorySamples
        #[method_id(@__retain_semantics Other correlationWithType:startDate:endDate:objects:)]
        pub unsafe fn correlationWithType_startDate_endDate_objects(
            correlation_type: &HKCorrelationType,
            start_date: &NSDate,
            end_date: &NSDate,
            objects: &NSSet<HKSample>,
        ) -> Retained<Self>;

        #[cfg(feature = "HKObjectType")]
        /// Creates a new HKCorrelation with the given type, start date, end date, objects, and metadata.
        ///
        /// objects must be a set of HKQuantitySamples and HKCategorySamples
        #[method_id(@__retain_semantics Other correlationWithType:startDate:endDate:objects:metadata:)]
        pub unsafe fn correlationWithType_startDate_endDate_objects_metadata(
            correlation_type: &HKCorrelationType,
            start_date: &NSDate,
            end_date: &NSDate,
            objects: &NSSet<HKSample>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKDevice", feature = "HKObjectType"))]
        /// Creates a new HKCorrelation with the given type, start date, end date, objects, and metadata.
        ///
        /// Parameter `correlationType`: The correlation type of the objects set.
        ///
        /// Parameter `startDate`: The start date of the correlation.
        ///
        /// Parameter `endDate`: The end date of the correlation.
        ///
        /// Parameter `device`: The HKDevice that generated the samples (optional).
        ///
        /// Parameter `metadata`: Metadata for the correlation (optional).
        ///
        /// objects must be a set of HKQuantitySamples and HKCategorySamples
        #[method_id(@__retain_semantics Other correlationWithType:startDate:endDate:objects:device:metadata:)]
        pub unsafe fn correlationWithType_startDate_endDate_objects_device_metadata(
            correlation_type: &HKCorrelationType,
            start_date: &NSDate,
            end_date: &NSDate,
            objects: &NSSet<HKSample>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "HKObjectType")]
        /// Returns the set of correlated objects with the specified type.
        #[method_id(@__retain_semantics Other objectsForType:)]
        pub unsafe fn objectsForType(
            &self,
            object_type: &HKObjectType,
        ) -> Retained<NSSet<HKSample>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKCorrelation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKCorrelation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
