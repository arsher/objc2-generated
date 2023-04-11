//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKActivitySummary")]
    pub struct HKActivitySummary;

    #[cfg(feature = "HealthKit_HKActivitySummary")]
    unsafe impl ClassType for HKActivitySummary {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKActivitySummary")]
unsafe impl NSCoding for HKActivitySummary {}

#[cfg(feature = "HealthKit_HKActivitySummary")]
unsafe impl NSCopying for HKActivitySummary {}

#[cfg(feature = "HealthKit_HKActivitySummary")]
unsafe impl NSObjectProtocol for HKActivitySummary {}

#[cfg(feature = "HealthKit_HKActivitySummary")]
unsafe impl NSSecureCoding for HKActivitySummary {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKActivitySummary")]
    unsafe impl HKActivitySummary {
        #[cfg(all(
            feature = "Foundation_NSCalendar",
            feature = "Foundation_NSDateComponents"
        ))]
        #[method_id(@__retain_semantics Other dateComponentsForCalendar:)]
        pub unsafe fn dateComponentsForCalendar(
            &self,
            calendar: &NSCalendar,
        ) -> Id<NSDateComponents>;

        #[method(activityMoveMode)]
        pub unsafe fn activityMoveMode(&self) -> HKActivityMoveMode;

        #[method(setActivityMoveMode:)]
        pub unsafe fn setActivityMoveMode(&self, activity_move_mode: HKActivityMoveMode);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other activeEnergyBurned)]
        pub unsafe fn activeEnergyBurned(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setActiveEnergyBurned:)]
        pub unsafe fn setActiveEnergyBurned(&self, active_energy_burned: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other appleMoveTime)]
        pub unsafe fn appleMoveTime(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setAppleMoveTime:)]
        pub unsafe fn setAppleMoveTime(&self, apple_move_time: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other appleExerciseTime)]
        pub unsafe fn appleExerciseTime(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setAppleExerciseTime:)]
        pub unsafe fn setAppleExerciseTime(&self, apple_exercise_time: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other appleStandHours)]
        pub unsafe fn appleStandHours(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setAppleStandHours:)]
        pub unsafe fn setAppleStandHours(&self, apple_stand_hours: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other activeEnergyBurnedGoal)]
        pub unsafe fn activeEnergyBurnedGoal(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setActiveEnergyBurnedGoal:)]
        pub unsafe fn setActiveEnergyBurnedGoal(&self, active_energy_burned_goal: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other appleMoveTimeGoal)]
        pub unsafe fn appleMoveTimeGoal(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setAppleMoveTimeGoal:)]
        pub unsafe fn setAppleMoveTimeGoal(&self, apple_move_time_goal: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[deprecated]
        #[method_id(@__retain_semantics Other appleExerciseTimeGoal)]
        pub unsafe fn appleExerciseTimeGoal(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[deprecated]
        #[method(setAppleExerciseTimeGoal:)]
        pub unsafe fn setAppleExerciseTimeGoal(&self, apple_exercise_time_goal: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other exerciseTimeGoal)]
        pub unsafe fn exerciseTimeGoal(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setExerciseTimeGoal:)]
        pub unsafe fn setExerciseTimeGoal(&self, exercise_time_goal: Option<&HKQuantity>);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[deprecated]
        #[method_id(@__retain_semantics Other appleStandHoursGoal)]
        pub unsafe fn appleStandHoursGoal(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[deprecated]
        #[method(setAppleStandHoursGoal:)]
        pub unsafe fn setAppleStandHoursGoal(&self, apple_stand_hours_goal: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other standHoursGoal)]
        pub unsafe fn standHoursGoal(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setStandHoursGoal:)]
        pub unsafe fn setStandHoursGoal(&self, stand_hours_goal: Option<&HKQuantity>);
    }
);

extern_static!(HKPredicateKeyPathDateComponents: &'static NSString);
