//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKQuery;

    unsafe impl ClassType for HKQuery {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for HKQuery {}

extern_methods!(
    unsafe impl HKQuery {
        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other objectType)]
        pub unsafe fn objectType(&self) -> Option<Retained<HKObjectType>>;

        #[cfg(feature = "HKObjectType")]
        #[deprecated]
        #[method_id(@__retain_semantics Other sampleType)]
        pub unsafe fn sampleType(&self) -> Option<Retained<HKSampleType>>;

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKQueryOptions(pub NSUInteger);
bitflags::bitflags! {
    impl HKQueryOptions: NSUInteger {
        const HKQueryOptionNone = 0;
        const HKQueryOptionStrictStartDate = 1<<0;
        const HKQueryOptionStrictEndDate = 1<<1;
    }
}

unsafe impl Encode for HKQueryOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for HKQueryOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// HKObjectPredicates
    unsafe impl HKQuery {
        #[method_id(@__retain_semantics Other predicateForObjectsWithMetadataKey:)]
        pub unsafe fn predicateForObjectsWithMetadataKey(key: &NSString) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForObjectsWithMetadataKey:allowedValues:)]
        pub unsafe fn predicateForObjectsWithMetadataKey_allowedValues(
            key: &NSString,
            allowed_values: &NSArray,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForObjectsWithMetadataKey:operatorType:value:)]
        pub unsafe fn predicateForObjectsWithMetadataKey_operatorType_value(
            key: &NSString,
            operator_type: NSPredicateOperatorType,
            value: &AnyObject,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKSource")]
        #[method_id(@__retain_semantics Other predicateForObjectsFromSource:)]
        pub unsafe fn predicateForObjectsFromSource(source: &HKSource) -> Retained<NSPredicate>;

        #[cfg(feature = "HKSource")]
        #[method_id(@__retain_semantics Other predicateForObjectsFromSources:)]
        pub unsafe fn predicateForObjectsFromSources(
            sources: &NSSet<HKSource>,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKSourceRevision")]
        #[method_id(@__retain_semantics Other predicateForObjectsFromSourceRevisions:)]
        pub unsafe fn predicateForObjectsFromSourceRevisions(
            source_revisions: &NSSet<HKSourceRevision>,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKDevice")]
        #[method_id(@__retain_semantics Other predicateForObjectsFromDevices:)]
        pub unsafe fn predicateForObjectsFromDevices(
            devices: &NSSet<HKDevice>,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForObjectsWithDeviceProperty:allowedValues:)]
        pub unsafe fn predicateForObjectsWithDeviceProperty_allowedValues(
            key: &NSString,
            allowed_values: &NSSet<NSString>,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForObjectWithUUID:)]
        pub unsafe fn predicateForObjectWithUUID(uuid: &NSUUID) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForObjectsWithUUIDs:)]
        pub unsafe fn predicateForObjectsWithUUIDs(uui_ds: &NSSet<NSUUID>)
            -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForObjectsWithNoCorrelation)]
        pub unsafe fn predicateForObjectsWithNoCorrelation() -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObject", feature = "HKSample", feature = "HKWorkout"))]
        #[method_id(@__retain_semantics Other predicateForObjectsFromWorkout:)]
        pub unsafe fn predicateForObjectsFromWorkout(workout: &HKWorkout) -> Retained<NSPredicate>;

        #[cfg(all(
            feature = "HKElectrocardiogram",
            feature = "HKObject",
            feature = "HKSample"
        ))]
        #[method_id(@__retain_semantics Other predicateForObjectsAssociatedWithElectrocardiogram:)]
        pub unsafe fn predicateForObjectsAssociatedWithElectrocardiogram(
            electrocardiogram: &HKElectrocardiogram,
        ) -> Retained<NSPredicate>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKSample",
            feature = "HKWorkout",
            feature = "HKWorkoutActivity"
        ))]
        #[method_id(@__retain_semantics Other predicateForWorkoutEffortSamplesRelatedToWorkout:activity:)]
        pub unsafe fn predicateForWorkoutEffortSamplesRelatedToWorkout_activity(
            workout: &HKWorkout,
            activity: Option<&HKWorkoutActivity>,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKSamplePredicates
    unsafe impl HKQuery {
        #[method_id(@__retain_semantics Other predicateForSamplesWithStartDate:endDate:options:)]
        pub unsafe fn predicateForSamplesWithStartDate_endDate_options(
            start_date: Option<&NSDate>,
            end_date: Option<&NSDate>,
            options: HKQueryOptions,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKQuantitySamplePredicates
    unsafe impl HKQuery {
        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other predicateForQuantitySamplesWithOperatorType:quantity:)]
        pub unsafe fn predicateForQuantitySamplesWithOperatorType_quantity(
            operator_type: NSPredicateOperatorType,
            quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKCategorySamplePredicates
    unsafe impl HKQuery {
        #[method_id(@__retain_semantics Other predicateForCategorySamplesWithOperatorType:value:)]
        pub unsafe fn predicateForCategorySamplesWithOperatorType_value(
            operator_type: NSPredicateOperatorType,
            value: NSInteger,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForCategorySamplesEqualToValues:)]
        pub unsafe fn predicateForCategorySamplesEqualToValues(
            values: &NSSet<NSNumber>,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKWorkoutPredicates
    unsafe impl HKQuery {
        #[cfg(feature = "HKWorkout")]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithWorkoutActivityType:)]
        pub unsafe fn predicateForWorkoutsWithWorkoutActivityType(
            workout_activity_type: HKWorkoutActivityType,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:duration:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_duration(
            operator_type: NSPredicateOperatorType,
            duration: NSTimeInterval,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use predicateForWorkoutActivitiesWithOperatorType:quantityType:sumQuantity: passing the HKQuantityType for HKQuantityTypeIdentifierActiveEnergyBurned"]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:totalEnergyBurned:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_totalEnergyBurned(
            operator_type: NSPredicateOperatorType,
            total_energy_burned: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use predicateForWorkoutActivitiesWithOperatorType:quantityType:sumQuantity: passing the HKQuantityType for the desired distance type"]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:totalDistance:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_totalDistance(
            operator_type: NSPredicateOperatorType,
            total_distance: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use predicateForWorkoutActivitiesWithOperatorType:quantityType:sumQuantity: passing the HKQuantityType for HKQuantityTypeIdentifierSwimmingStrokeCount"]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:totalSwimmingStrokeCount:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_totalSwimmingStrokeCount(
            operator_type: NSPredicateOperatorType,
            total_swimming_stroke_count: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use predicateForWorkoutActivitiesWithOperatorType:quantityType:sumQuantity: passing the HKQuantityType for HKQuantityTypeIdentifierFlightsClimbed"]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:totalFlightsClimbed:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_totalFlightsClimbed(
            operator_type: NSPredicateOperatorType,
            total_flights_climbed: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:quantityType:sumQuantity:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_quantityType_sumQuantity(
            operator_type: NSPredicateOperatorType,
            quantity_type: &HKQuantityType,
            sum_quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:quantityType:minimumQuantity:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_quantityType_minimumQuantity(
            operator_type: NSPredicateOperatorType,
            quantity_type: &HKQuantityType,
            minimum_quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:quantityType:maximumQuantity:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_quantityType_maximumQuantity(
            operator_type: NSPredicateOperatorType,
            quantity_type: &HKQuantityType,
            maximum_quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other predicateForWorkoutsWithOperatorType:quantityType:averageQuantity:)]
        pub unsafe fn predicateForWorkoutsWithOperatorType_quantityType_averageQuantity(
            operator_type: NSPredicateOperatorType,
            quantity_type: &HKQuantityType,
            average_quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKWorkoutActivityPredicates
    unsafe impl HKQuery {
        #[cfg(feature = "HKWorkout")]
        #[method_id(@__retain_semantics Other predicateForWorkoutActivitiesWithWorkoutActivityType:)]
        pub unsafe fn predicateForWorkoutActivitiesWithWorkoutActivityType(
            workout_activity_type: HKWorkoutActivityType,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForWorkoutActivitiesWithOperatorType:duration:)]
        pub unsafe fn predicateForWorkoutActivitiesWithOperatorType_duration(
            operator_type: NSPredicateOperatorType,
            duration: NSTimeInterval,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForWorkoutActivitiesWithStartDate:endDate:options:)]
        pub unsafe fn predicateForWorkoutActivitiesWithStartDate_endDate_options(
            start_date: Option<&NSDate>,
            end_date: Option<&NSDate>,
            options: HKQueryOptions,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other predicateForWorkoutActivitiesWithOperatorType:quantityType:sumQuantity:)]
        pub unsafe fn predicateForWorkoutActivitiesWithOperatorType_quantityType_sumQuantity(
            operator_type: NSPredicateOperatorType,
            quantity_type: &HKQuantityType,
            sum_quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other predicateForWorkoutActivitiesWithOperatorType:quantityType:minimumQuantity:)]
        pub unsafe fn predicateForWorkoutActivitiesWithOperatorType_quantityType_minimumQuantity(
            operator_type: NSPredicateOperatorType,
            quantity_type: &HKQuantityType,
            minimum_quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other predicateForWorkoutActivitiesWithOperatorType:quantityType:maximumQuantity:)]
        pub unsafe fn predicateForWorkoutActivitiesWithOperatorType_quantityType_maximumQuantity(
            operator_type: NSPredicateOperatorType,
            quantity_type: &HKQuantityType,
            maximum_quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other predicateForWorkoutActivitiesWithOperatorType:quantityType:averageQuantity:)]
        pub unsafe fn predicateForWorkoutActivitiesWithOperatorType_quantityType_averageQuantity(
            operator_type: NSPredicateOperatorType,
            quantity_type: &HKQuantityType,
            average_quantity: &HKQuantity,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForWorkoutsWithActivityPredicate:)]
        pub unsafe fn predicateForWorkoutsWithActivityPredicate(
            activity_predicate: &NSPredicate,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKActivitySummaryPredicates
    unsafe impl HKQuery {
        #[method_id(@__retain_semantics Other predicateForActivitySummaryWithDateComponents:)]
        pub unsafe fn predicateForActivitySummaryWithDateComponents(
            date_components: &NSDateComponents,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForActivitySummariesBetweenStartDateComponents:endDateComponents:)]
        pub unsafe fn predicateForActivitySummariesBetweenStartDateComponents_endDateComponents(
            start_date_components: &NSDateComponents,
            end_date_components: &NSDateComponents,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKClinicalRecordPredicates
    unsafe impl HKQuery {
        #[cfg(feature = "HKFHIRResource")]
        #[method_id(@__retain_semantics Other predicateForClinicalRecordsWithFHIRResourceType:)]
        pub unsafe fn predicateForClinicalRecordsWithFHIRResourceType(
            resource_type: &HKFHIRResourceType,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HKFHIRResource", feature = "HKSource"))]
        #[method_id(@__retain_semantics Other predicateForClinicalRecordsFromSource:FHIRResourceType:identifier:)]
        pub unsafe fn predicateForClinicalRecordsFromSource_FHIRResourceType_identifier(
            source: &HKSource,
            resource_type: &HKFHIRResourceType,
            identifier: &NSString,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKElectrocardiogramPredicates
    unsafe impl HKQuery {
        #[cfg(feature = "HKElectrocardiogram")]
        #[method_id(@__retain_semantics Other predicateForElectrocardiogramsWithClassification:)]
        pub unsafe fn predicateForElectrocardiogramsWithClassification(
            classification: HKElectrocardiogramClassification,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKElectrocardiogram")]
        #[method_id(@__retain_semantics Other predicateForElectrocardiogramsWithSymptomsStatus:)]
        pub unsafe fn predicateForElectrocardiogramsWithSymptomsStatus(
            symptoms_status: HKElectrocardiogramSymptomsStatus,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKVerifiableClinicalRecordPredicates
    unsafe impl HKQuery {
        #[method_id(@__retain_semantics Other predicateForVerifiableClinicalRecordsWithRelevantDateWithinDateInterval:)]
        pub unsafe fn predicateForVerifiableClinicalRecordsWithRelevantDateWithinDateInterval(
            date_interval: &NSDateInterval,
        ) -> Retained<NSPredicate>;
    }
);

extern_methods!(
    /// HKStateOfMind
    unsafe impl HKQuery {
        #[method_id(@__retain_semantics Other predicateForStatesOfMindWithValence:operatorType:)]
        pub unsafe fn predicateForStatesOfMindWithValence_operatorType(
            valence: c_double,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKStateOfMind")]
        #[method_id(@__retain_semantics Other predicateForStatesOfMindWithKind:)]
        pub unsafe fn predicateForStatesOfMindWithKind(
            kind: HKStateOfMindKind,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKStateOfMind")]
        #[method_id(@__retain_semantics Other predicateForStatesOfMindWithLabel:)]
        pub unsafe fn predicateForStatesOfMindWithLabel(
            label: HKStateOfMindLabel,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HKStateOfMind")]
        #[method_id(@__retain_semantics Other predicateForStatesOfMindWithAssociation:)]
        pub unsafe fn predicateForStatesOfMindWithAssociation(
            association: HKStateOfMindAssociation,
        ) -> Retained<NSPredicate>;
    }
);
