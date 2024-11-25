//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkcorrelation?language=objc)
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

        #[method_id(@__retain_semantics Other objects)]
        pub unsafe fn objects(&self) -> Retained<NSSet<HKSample>>;

        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other correlationWithType:startDate:endDate:objects:)]
        pub unsafe fn correlationWithType_startDate_endDate_objects(
            correlation_type: &HKCorrelationType,
            start_date: &NSDate,
            end_date: &NSDate,
            objects: &NSSet<HKSample>,
        ) -> Retained<Self>;

        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other correlationWithType:startDate:endDate:objects:metadata:)]
        pub unsafe fn correlationWithType_startDate_endDate_objects_metadata(
            correlation_type: &HKCorrelationType,
            start_date: &NSDate,
            end_date: &NSDate,
            objects: &NSSet<HKSample>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKDevice", feature = "HKObjectType"))]
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
