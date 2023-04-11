//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKQuantitySeriesSampleBuilder")]
    pub struct HKQuantitySeriesSampleBuilder;

    #[cfg(feature = "HealthKit_HKQuantitySeriesSampleBuilder")]
    unsafe impl ClassType for HKQuantitySeriesSampleBuilder {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKQuantitySeriesSampleBuilder")]
unsafe impl NSObjectProtocol for HKQuantitySeriesSampleBuilder {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKQuantitySeriesSampleBuilder")]
    unsafe impl HKQuantitySeriesSampleBuilder {
        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "HealthKit_HKDevice",
            feature = "HealthKit_HKHealthStore",
            feature = "HealthKit_HKQuantityType"
        ))]
        #[method_id(@__retain_semantics Init initWithHealthStore:quantityType:startDate:device:)]
        pub unsafe fn initWithHealthStore_quantityType_startDate_device(
            this: Option<Allocated<Self>>,
            health_store: &HKHealthStore,
            quantity_type: &HKQuantityType,
            start_date: &NSDate,
            device: Option<&HKDevice>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "HealthKit_HKQuantityType")]
        #[method_id(@__retain_semantics Other quantityType)]
        pub unsafe fn quantityType(&self) -> Id<HKQuantityType>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[cfg(feature = "HealthKit_HKDevice")]
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<HKDevice>>;

        #[cfg(all(
            feature = "Foundation_NSDateInterval",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKQuantity"
        ))]
        #[method(insertQuantity:dateInterval:error:_)]
        pub unsafe fn insertQuantity_dateInterval_error(
            &self,
            quantity: &HKQuantity,
            date_interval: &NSDateInterval,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKQuantity"
        ))]
        #[method(insertQuantity:date:error:_)]
        pub unsafe fn insertQuantity_date_error(
            &self,
            quantity: &HKQuantity,
            date: &NSDate,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKQuantitySample"
        ))]
        #[method(finishSeriesWithMetadata:endDate:completion:)]
        pub unsafe fn finishSeriesWithMetadata_endDate_completion(
            &self,
            metadata: Option<&NSDictionary<NSString, Object>>,
            end_date: Option<&NSDate>,
            completion: &Block<(*mut NSArray<HKQuantitySample>, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKQuantitySample"
        ))]
        #[method(finishSeriesWithMetadata:completion:)]
        pub unsafe fn finishSeriesWithMetadata_completion(
            &self,
            metadata: Option<&NSDictionary<NSString, Object>>,
            completion: &Block<(*mut NSArray<HKQuantitySample>, *mut NSError), ()>,
        );

        #[method(discard)]
        pub unsafe fn discard(&self);
    }
);
