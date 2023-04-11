//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKCategorySample")]
    pub struct HKCategorySample;

    #[cfg(feature = "HealthKit_HKCategorySample")]
    unsafe impl ClassType for HKCategorySample {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKCategorySample")]
unsafe impl NSCoding for HKCategorySample {}

#[cfg(feature = "HealthKit_HKCategorySample")]
unsafe impl NSObjectProtocol for HKCategorySample {}

#[cfg(feature = "HealthKit_HKCategorySample")]
unsafe impl NSSecureCoding for HKCategorySample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKCategorySample")]
    unsafe impl HKCategorySample {
        #[cfg(feature = "HealthKit_HKCategoryType")]
        #[method_id(@__retain_semantics Other categoryType)]
        pub unsafe fn categoryType(&self) -> Id<HKCategoryType>;

        #[method(value)]
        pub unsafe fn value(&self) -> NSInteger;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKCategoryType"
        ))]
        #[method_id(@__retain_semantics Other categorySampleWithType:value:startDate:endDate:metadata:)]
        pub unsafe fn categorySampleWithType_value_startDate_endDate_metadata(
            r#type: &HKCategoryType,
            value: NSInteger,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "HealthKit_HKCategoryType"))]
        #[method_id(@__retain_semantics Other categorySampleWithType:value:startDate:endDate:)]
        pub unsafe fn categorySampleWithType_value_startDate_endDate(
            r#type: &HKCategoryType,
            value: NSInteger,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKCategoryType",
            feature = "HealthKit_HKDevice"
        ))]
        #[method_id(@__retain_semantics Other categorySampleWithType:value:startDate:endDate:device:metadata:)]
        pub unsafe fn categorySampleWithType_value_startDate_endDate_device_metadata(
            r#type: &HKCategoryType,
            value: NSInteger,
            start_date: &NSDate,
            end_date: &NSDate,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self>;
    }
);

extern_static!(HKPredicateKeyPathCategoryValue: &'static NSString);
