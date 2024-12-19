//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutactivitytype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKWorkoutActivityType(pub NSUInteger);
impl HKWorkoutActivityType {
    #[doc(alias = "HKWorkoutActivityTypeAmericanFootball")]
    pub const AmericanFootball: Self = Self(1);
    #[doc(alias = "HKWorkoutActivityTypeArchery")]
    pub const Archery: Self = Self(2);
    #[doc(alias = "HKWorkoutActivityTypeAustralianFootball")]
    pub const AustralianFootball: Self = Self(3);
    #[doc(alias = "HKWorkoutActivityTypeBadminton")]
    pub const Badminton: Self = Self(4);
    #[doc(alias = "HKWorkoutActivityTypeBaseball")]
    pub const Baseball: Self = Self(5);
    #[doc(alias = "HKWorkoutActivityTypeBasketball")]
    pub const Basketball: Self = Self(6);
    #[doc(alias = "HKWorkoutActivityTypeBowling")]
    pub const Bowling: Self = Self(7);
    #[doc(alias = "HKWorkoutActivityTypeBoxing")]
    pub const Boxing: Self = Self(8);
    #[doc(alias = "HKWorkoutActivityTypeClimbing")]
    pub const Climbing: Self = Self(9);
    #[doc(alias = "HKWorkoutActivityTypeCricket")]
    pub const Cricket: Self = Self(10);
    #[doc(alias = "HKWorkoutActivityTypeCrossTraining")]
    pub const CrossTraining: Self = Self(11);
    #[doc(alias = "HKWorkoutActivityTypeCurling")]
    pub const Curling: Self = Self(12);
    #[doc(alias = "HKWorkoutActivityTypeCycling")]
    pub const Cycling: Self = Self(13);
    #[deprecated = "Use HKWorkoutActivityTypeSocialDance or HKWorkoutActivityTypeCardioDance"]
    #[doc(alias = "HKWorkoutActivityTypeDance")]
    pub const Dance: Self = Self(14);
    #[deprecated = "Use HKWorkoutActivityTypeSocialDance, HKWorkoutActivityTypeCardioDance, HKWorkoutActivityTypeBarre or HKWorkoutActivityTypePilates"]
    #[doc(alias = "HKWorkoutActivityTypeDanceInspiredTraining")]
    pub const DanceInspiredTraining: Self = Self(15);
    #[doc(alias = "HKWorkoutActivityTypeElliptical")]
    pub const Elliptical: Self = Self(16);
    #[doc(alias = "HKWorkoutActivityTypeEquestrianSports")]
    pub const EquestrianSports: Self = Self(17);
    #[doc(alias = "HKWorkoutActivityTypeFencing")]
    pub const Fencing: Self = Self(18);
    #[doc(alias = "HKWorkoutActivityTypeFishing")]
    pub const Fishing: Self = Self(19);
    #[doc(alias = "HKWorkoutActivityTypeFunctionalStrengthTraining")]
    pub const FunctionalStrengthTraining: Self = Self(20);
    #[doc(alias = "HKWorkoutActivityTypeGolf")]
    pub const Golf: Self = Self(21);
    #[doc(alias = "HKWorkoutActivityTypeGymnastics")]
    pub const Gymnastics: Self = Self(22);
    #[doc(alias = "HKWorkoutActivityTypeHandball")]
    pub const Handball: Self = Self(23);
    #[doc(alias = "HKWorkoutActivityTypeHiking")]
    pub const Hiking: Self = Self(24);
    #[doc(alias = "HKWorkoutActivityTypeHockey")]
    pub const Hockey: Self = Self(25);
    #[doc(alias = "HKWorkoutActivityTypeHunting")]
    pub const Hunting: Self = Self(26);
    #[doc(alias = "HKWorkoutActivityTypeLacrosse")]
    pub const Lacrosse: Self = Self(27);
    #[doc(alias = "HKWorkoutActivityTypeMartialArts")]
    pub const MartialArts: Self = Self(28);
    #[doc(alias = "HKWorkoutActivityTypeMindAndBody")]
    pub const MindAndBody: Self = Self(29);
    #[deprecated = "Use HKWorkoutActivityTypeMixedCardio or HKWorkoutActivityTypeHighIntensityIntervalTraining"]
    #[doc(alias = "HKWorkoutActivityTypeMixedMetabolicCardioTraining")]
    pub const MixedMetabolicCardioTraining: Self = Self(30);
    #[doc(alias = "HKWorkoutActivityTypePaddleSports")]
    pub const PaddleSports: Self = Self(31);
    #[doc(alias = "HKWorkoutActivityTypePlay")]
    pub const Play: Self = Self(32);
    #[doc(alias = "HKWorkoutActivityTypePreparationAndRecovery")]
    pub const PreparationAndRecovery: Self = Self(33);
    #[doc(alias = "HKWorkoutActivityTypeRacquetball")]
    pub const Racquetball: Self = Self(34);
    #[doc(alias = "HKWorkoutActivityTypeRowing")]
    pub const Rowing: Self = Self(35);
    #[doc(alias = "HKWorkoutActivityTypeRugby")]
    pub const Rugby: Self = Self(36);
    #[doc(alias = "HKWorkoutActivityTypeRunning")]
    pub const Running: Self = Self(37);
    #[doc(alias = "HKWorkoutActivityTypeSailing")]
    pub const Sailing: Self = Self(38);
    #[doc(alias = "HKWorkoutActivityTypeSkatingSports")]
    pub const SkatingSports: Self = Self(39);
    #[doc(alias = "HKWorkoutActivityTypeSnowSports")]
    pub const SnowSports: Self = Self(40);
    #[doc(alias = "HKWorkoutActivityTypeSoccer")]
    pub const Soccer: Self = Self(41);
    #[doc(alias = "HKWorkoutActivityTypeSoftball")]
    pub const Softball: Self = Self(42);
    #[doc(alias = "HKWorkoutActivityTypeSquash")]
    pub const Squash: Self = Self(43);
    #[doc(alias = "HKWorkoutActivityTypeStairClimbing")]
    pub const StairClimbing: Self = Self(44);
    #[doc(alias = "HKWorkoutActivityTypeSurfingSports")]
    pub const SurfingSports: Self = Self(45);
    #[doc(alias = "HKWorkoutActivityTypeSwimming")]
    pub const Swimming: Self = Self(46);
    #[doc(alias = "HKWorkoutActivityTypeTableTennis")]
    pub const TableTennis: Self = Self(47);
    #[doc(alias = "HKWorkoutActivityTypeTennis")]
    pub const Tennis: Self = Self(48);
    #[doc(alias = "HKWorkoutActivityTypeTrackAndField")]
    pub const TrackAndField: Self = Self(49);
    #[doc(alias = "HKWorkoutActivityTypeTraditionalStrengthTraining")]
    pub const TraditionalStrengthTraining: Self = Self(50);
    #[doc(alias = "HKWorkoutActivityTypeVolleyball")]
    pub const Volleyball: Self = Self(51);
    #[doc(alias = "HKWorkoutActivityTypeWalking")]
    pub const Walking: Self = Self(52);
    #[doc(alias = "HKWorkoutActivityTypeWaterFitness")]
    pub const WaterFitness: Self = Self(53);
    #[doc(alias = "HKWorkoutActivityTypeWaterPolo")]
    pub const WaterPolo: Self = Self(54);
    #[doc(alias = "HKWorkoutActivityTypeWaterSports")]
    pub const WaterSports: Self = Self(55);
    #[doc(alias = "HKWorkoutActivityTypeWrestling")]
    pub const Wrestling: Self = Self(56);
    #[doc(alias = "HKWorkoutActivityTypeYoga")]
    pub const Yoga: Self = Self(57);
    #[doc(alias = "HKWorkoutActivityTypeBarre")]
    pub const Barre: Self = Self(58);
    #[doc(alias = "HKWorkoutActivityTypeCoreTraining")]
    pub const CoreTraining: Self = Self(59);
    #[doc(alias = "HKWorkoutActivityTypeCrossCountrySkiing")]
    pub const CrossCountrySkiing: Self = Self(60);
    #[doc(alias = "HKWorkoutActivityTypeDownhillSkiing")]
    pub const DownhillSkiing: Self = Self(61);
    #[doc(alias = "HKWorkoutActivityTypeFlexibility")]
    pub const Flexibility: Self = Self(62);
    #[doc(alias = "HKWorkoutActivityTypeHighIntensityIntervalTraining")]
    pub const HighIntensityIntervalTraining: Self = Self(63);
    #[doc(alias = "HKWorkoutActivityTypeJumpRope")]
    pub const JumpRope: Self = Self(64);
    #[doc(alias = "HKWorkoutActivityTypeKickboxing")]
    pub const Kickboxing: Self = Self(65);
    #[doc(alias = "HKWorkoutActivityTypePilates")]
    pub const Pilates: Self = Self(66);
    #[doc(alias = "HKWorkoutActivityTypeSnowboarding")]
    pub const Snowboarding: Self = Self(67);
    #[doc(alias = "HKWorkoutActivityTypeStairs")]
    pub const Stairs: Self = Self(68);
    #[doc(alias = "HKWorkoutActivityTypeStepTraining")]
    pub const StepTraining: Self = Self(69);
    #[doc(alias = "HKWorkoutActivityTypeWheelchairWalkPace")]
    pub const WheelchairWalkPace: Self = Self(70);
    #[doc(alias = "HKWorkoutActivityTypeWheelchairRunPace")]
    pub const WheelchairRunPace: Self = Self(71);
    #[doc(alias = "HKWorkoutActivityTypeTaiChi")]
    pub const TaiChi: Self = Self(72);
    #[doc(alias = "HKWorkoutActivityTypeMixedCardio")]
    pub const MixedCardio: Self = Self(73);
    #[doc(alias = "HKWorkoutActivityTypeHandCycling")]
    pub const HandCycling: Self = Self(74);
    #[doc(alias = "HKWorkoutActivityTypeDiscSports")]
    pub const DiscSports: Self = Self(75);
    #[doc(alias = "HKWorkoutActivityTypeFitnessGaming")]
    pub const FitnessGaming: Self = Self(76);
    #[doc(alias = "HKWorkoutActivityTypeCardioDance")]
    pub const CardioDance: Self = Self(77);
    #[doc(alias = "HKWorkoutActivityTypeSocialDance")]
    pub const SocialDance: Self = Self(78);
    #[doc(alias = "HKWorkoutActivityTypePickleball")]
    pub const Pickleball: Self = Self(79);
    #[doc(alias = "HKWorkoutActivityTypeCooldown")]
    pub const Cooldown: Self = Self(80);
    #[doc(alias = "HKWorkoutActivityTypeSwimBikeRun")]
    pub const SwimBikeRun: Self = Self(82);
    #[doc(alias = "HKWorkoutActivityTypeTransition")]
    pub const Transition: Self = Self(83);
    #[doc(alias = "HKWorkoutActivityTypeUnderwaterDiving")]
    pub const UnderwaterDiving: Self = Self(84);
    #[doc(alias = "HKWorkoutActivityTypeOther")]
    pub const Other: Self = Self(3000);
}

unsafe impl Encode for HKWorkoutActivityType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for HKWorkoutActivityType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkouteventtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKWorkoutEventType(pub NSInteger);
impl HKWorkoutEventType {
    #[doc(alias = "HKWorkoutEventTypePause")]
    pub const Pause: Self = Self(1);
    #[doc(alias = "HKWorkoutEventTypeResume")]
    pub const Resume: Self = Self(2);
    #[doc(alias = "HKWorkoutEventTypeLap")]
    pub const Lap: Self = Self(3);
    #[doc(alias = "HKWorkoutEventTypeMarker")]
    pub const Marker: Self = Self(4);
    #[doc(alias = "HKWorkoutEventTypeMotionPaused")]
    pub const MotionPaused: Self = Self(5);
    #[doc(alias = "HKWorkoutEventTypeMotionResumed")]
    pub const MotionResumed: Self = Self(6);
    #[doc(alias = "HKWorkoutEventTypeSegment")]
    pub const Segment: Self = Self(7);
    #[doc(alias = "HKWorkoutEventTypePauseOrResumeRequest")]
    pub const PauseOrResumeRequest: Self = Self(8);
}

unsafe impl Encode for HKWorkoutEventType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKWorkoutEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutevent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKWorkoutEvent;
);

unsafe impl Send for HKWorkoutEvent {}

unsafe impl Sync for HKWorkoutEvent {}

unsafe impl NSCoding for HKWorkoutEvent {}

unsafe impl NSCopying for HKWorkoutEvent {}

unsafe impl CopyingHelper for HKWorkoutEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKWorkoutEvent {}

unsafe impl NSSecureCoding for HKWorkoutEvent {}

extern_methods!(
    unsafe impl HKWorkoutEvent {
        #[method(type)]
        pub unsafe fn r#type(&self) -> HKWorkoutEventType;

        #[deprecated]
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other dateInterval)]
        pub unsafe fn dateInterval(&self) -> Retained<NSDateInterval>;

        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other workoutEventWithType:date:)]
        pub unsafe fn workoutEventWithType_date(
            r#type: HKWorkoutEventType,
            date: &NSDate,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other workoutEventWithType:date:metadata:)]
        pub unsafe fn workoutEventWithType_date_metadata(
            r#type: HKWorkoutEventType,
            date: &NSDate,
            metadata: &NSDictionary<NSString, AnyObject>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other workoutEventWithType:dateInterval:metadata:)]
        pub unsafe fn workoutEventWithType_dateInterval_metadata(
            r#type: HKWorkoutEventType,
            date_interval: &NSDateInterval,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKWorkoutEvent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkout?language=objc)
    #[unsafe(super(HKSample, HKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    pub struct HKWorkout;
);

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Send for HKWorkout {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Sync for HKWorkout {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCoding for HKWorkout {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSObjectProtocol for HKWorkout {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSSecureCoding for HKWorkout {}

extern_methods!(
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKWorkout {
        #[method(workoutActivityType)]
        pub unsafe fn workoutActivityType(&self) -> HKWorkoutActivityType;

        #[method_id(@__retain_semantics Other workoutEvents)]
        pub unsafe fn workoutEvents(&self) -> Option<Retained<NSArray<HKWorkoutEvent>>>;

        #[cfg(feature = "HKWorkoutActivity")]
        #[method_id(@__retain_semantics Other workoutActivities)]
        pub unsafe fn workoutActivities(&self) -> Retained<NSArray<HKWorkoutActivity>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use statisticsForType: passing the HKQuantityType for HKQuantityTypeIdentifierActiveEnergyBurned"]
        #[method_id(@__retain_semantics Other totalEnergyBurned)]
        pub unsafe fn totalEnergyBurned(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use statisticsForType: passing the HKQuantityType for the desired distance type"]
        #[method_id(@__retain_semantics Other totalDistance)]
        pub unsafe fn totalDistance(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use statisticsForType: passing the HKQuantityType for HKQuantityTypeIdentifierSwimmingStrokeCount"]
        #[method_id(@__retain_semantics Other totalSwimmingStrokeCount)]
        pub unsafe fn totalSwimmingStrokeCount(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use statisticsForType: passing the HKQuantityType for HKQuantityTypeIdentifierFlightClimbed"]
        #[method_id(@__retain_semantics Other totalFlightsClimbed)]
        pub unsafe fn totalFlightsClimbed(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(all(feature = "HKObjectType", feature = "HKStatistics"))]
        #[method_id(@__retain_semantics Other allStatistics)]
        pub unsafe fn allStatistics(&self) -> Retained<NSDictionary<HKQuantityType, HKStatistics>>;

        #[cfg(all(feature = "HKObjectType", feature = "HKStatistics"))]
        #[method_id(@__retain_semantics Other statisticsForType:)]
        pub unsafe fn statisticsForType(
            &self,
            quantity_type: &HKQuantityType,
        ) -> Option<Retained<HKStatistics>>;

        #[deprecated = "Use HKWorkoutBuilder"]
        #[method_id(@__retain_semantics Other workoutWithActivityType:startDate:endDate:)]
        pub unsafe fn workoutWithActivityType_startDate_endDate(
            workout_activity_type: HKWorkoutActivityType,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Retained<Self>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use HKWorkoutBuilder"]
        #[method_id(@__retain_semantics Other workoutWithActivityType:startDate:endDate:workoutEvents:totalEnergyBurned:totalDistance:metadata:)]
        pub unsafe fn workoutWithActivityType_startDate_endDate_workoutEvents_totalEnergyBurned_totalDistance_metadata(
            workout_activity_type: HKWorkoutActivityType,
            start_date: &NSDate,
            end_date: &NSDate,
            workout_events: Option<&NSArray<HKWorkoutEvent>>,
            total_energy_burned: Option<&HKQuantity>,
            total_distance: Option<&HKQuantity>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKDevice", feature = "HKQuantity"))]
        #[deprecated = "Use HKWorkoutBuilder"]
        #[method_id(@__retain_semantics Other workoutWithActivityType:startDate:endDate:workoutEvents:totalEnergyBurned:totalDistance:device:metadata:)]
        pub unsafe fn workoutWithActivityType_startDate_endDate_workoutEvents_totalEnergyBurned_totalDistance_device_metadata(
            workout_activity_type: HKWorkoutActivityType,
            start_date: &NSDate,
            end_date: &NSDate,
            workout_events: Option<&NSArray<HKWorkoutEvent>>,
            total_energy_burned: Option<&HKQuantity>,
            total_distance: Option<&HKQuantity>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "HKQuantity")]
        #[deprecated = "Use HKWorkoutBuilder"]
        #[method_id(@__retain_semantics Other workoutWithActivityType:startDate:endDate:duration:totalEnergyBurned:totalDistance:metadata:)]
        pub unsafe fn workoutWithActivityType_startDate_endDate_duration_totalEnergyBurned_totalDistance_metadata(
            workout_activity_type: HKWorkoutActivityType,
            start_date: &NSDate,
            end_date: &NSDate,
            duration: NSTimeInterval,
            total_energy_burned: Option<&HKQuantity>,
            total_distance: Option<&HKQuantity>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKDevice", feature = "HKQuantity"))]
        #[deprecated = "Use HKWorkoutBuilder"]
        #[method_id(@__retain_semantics Other workoutWithActivityType:startDate:endDate:duration:totalEnergyBurned:totalDistance:device:metadata:)]
        pub unsafe fn workoutWithActivityType_startDate_endDate_duration_totalEnergyBurned_totalDistance_device_metadata(
            workout_activity_type: HKWorkoutActivityType,
            start_date: &NSDate,
            end_date: &NSDate,
            duration: NSTimeInterval,
            total_energy_burned: Option<&HKQuantity>,
            total_distance: Option<&HKQuantity>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKDevice", feature = "HKQuantity"))]
        #[deprecated = "Use HKWorkoutBuilder"]
        #[method_id(@__retain_semantics Other workoutWithActivityType:startDate:endDate:workoutEvents:totalEnergyBurned:totalDistance:totalSwimmingStrokeCount:device:metadata:)]
        pub unsafe fn workoutWithActivityType_startDate_endDate_workoutEvents_totalEnergyBurned_totalDistance_totalSwimmingStrokeCount_device_metadata(
            workout_activity_type: HKWorkoutActivityType,
            start_date: &NSDate,
            end_date: &NSDate,
            workout_events: Option<&NSArray<HKWorkoutEvent>>,
            total_energy_burned: Option<&HKQuantity>,
            total_distance: Option<&HKQuantity>,
            total_swimming_stroke_count: Option<&HKQuantity>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKDevice", feature = "HKQuantity"))]
        #[deprecated = "Use HKWorkoutBuilder"]
        #[method_id(@__retain_semantics Other workoutWithActivityType:startDate:endDate:workoutEvents:totalEnergyBurned:totalDistance:totalFlightsClimbed:device:metadata:)]
        pub unsafe fn workoutWithActivityType_startDate_endDate_workoutEvents_totalEnergyBurned_totalDistance_totalFlightsClimbed_device_metadata(
            workout_activity_type: HKWorkoutActivityType,
            start_date: &NSDate,
            end_date: &NSDate,
            workout_events: Option<&NSArray<HKWorkoutEvent>>,
            total_energy_burned: Option<&HKQuantity>,
            total_distance: Option<&HKQuantity>,
            total_flights_climbed: Option<&HKQuantity>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKWorkout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKWorkout {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkoutduration?language=objc)
    pub static HKPredicateKeyPathWorkoutDuration: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkouttotaldistance?language=objc)
    pub static HKPredicateKeyPathWorkoutTotalDistance: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkouttotalenergyburned?language=objc)
    pub static HKPredicateKeyPathWorkoutTotalEnergyBurned: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkouttype?language=objc)
    pub static HKPredicateKeyPathWorkoutType: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkouttotalswimmingstrokecount?language=objc)
    pub static HKPredicateKeyPathWorkoutTotalSwimmingStrokeCount: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkouttotalflightsclimbed?language=objc)
    pub static HKPredicateKeyPathWorkoutTotalFlightsClimbed: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkoutsumquantity?language=objc)
    pub static HKPredicateKeyPathWorkoutSumQuantity: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkoutminimumquantity?language=objc)
    pub static HKPredicateKeyPathWorkoutMinimumQuantity: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkoutmaximumquantity?language=objc)
    pub static HKPredicateKeyPathWorkoutMaximumQuantity: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkoutaveragequantity?language=objc)
    pub static HKPredicateKeyPathWorkoutAverageQuantity: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkoutactivity?language=objc)
    pub static HKPredicateKeyPathWorkoutActivity: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutsortidentifierduration?language=objc)
    pub static HKWorkoutSortIdentifierDuration: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutsortidentifiertotaldistance?language=objc)
    pub static HKWorkoutSortIdentifierTotalDistance: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutsortidentifiertotalenergyburned?language=objc)
    pub static HKWorkoutSortIdentifierTotalEnergyBurned: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutsortidentifiertotalswimmingstrokecount?language=objc)
    pub static HKWorkoutSortIdentifierTotalSwimmingStrokeCount: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutsortidentifiertotalflightsclimbed?language=objc)
    pub static HKWorkoutSortIdentifierTotalFlightsClimbed: &'static NSString;
}
