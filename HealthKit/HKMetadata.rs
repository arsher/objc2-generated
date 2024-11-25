//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeydeviceserialnumber?language=objc)
    pub static HKMetadataKeyDeviceSerialNumber: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeybodytemperaturesensorlocation?language=objc)
    pub static HKMetadataKeyBodyTemperatureSensorLocation: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkbodytemperaturesensorlocation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKBodyTemperatureSensorLocation(pub NSInteger);
impl HKBodyTemperatureSensorLocation {
    #[doc(alias = "HKBodyTemperatureSensorLocationOther")]
    pub const Other: Self = Self(0);
    #[doc(alias = "HKBodyTemperatureSensorLocationArmpit")]
    pub const Armpit: Self = Self(1);
    #[doc(alias = "HKBodyTemperatureSensorLocationBody")]
    pub const Body: Self = Self(2);
    #[doc(alias = "HKBodyTemperatureSensorLocationEar")]
    pub const Ear: Self = Self(3);
    #[doc(alias = "HKBodyTemperatureSensorLocationFinger")]
    pub const Finger: Self = Self(4);
    #[doc(alias = "HKBodyTemperatureSensorLocationGastroIntestinal")]
    pub const GastroIntestinal: Self = Self(5);
    #[doc(alias = "HKBodyTemperatureSensorLocationMouth")]
    pub const Mouth: Self = Self(6);
    #[doc(alias = "HKBodyTemperatureSensorLocationRectum")]
    pub const Rectum: Self = Self(7);
    #[doc(alias = "HKBodyTemperatureSensorLocationToe")]
    pub const Toe: Self = Self(8);
    #[doc(alias = "HKBodyTemperatureSensorLocationEarDrum")]
    pub const EarDrum: Self = Self(9);
    #[doc(alias = "HKBodyTemperatureSensorLocationTemporalArtery")]
    pub const TemporalArtery: Self = Self(10);
    #[doc(alias = "HKBodyTemperatureSensorLocationForehead")]
    pub const Forehead: Self = Self(11);
}

unsafe impl Encode for HKBodyTemperatureSensorLocation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKBodyTemperatureSensorLocation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyheartratesensorlocation?language=objc)
    pub static HKMetadataKeyHeartRateSensorLocation: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkheartratesensorlocation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKHeartRateSensorLocation(pub NSInteger);
impl HKHeartRateSensorLocation {
    #[doc(alias = "HKHeartRateSensorLocationOther")]
    pub const Other: Self = Self(0);
    #[doc(alias = "HKHeartRateSensorLocationChest")]
    pub const Chest: Self = Self(1);
    #[doc(alias = "HKHeartRateSensorLocationWrist")]
    pub const Wrist: Self = Self(2);
    #[doc(alias = "HKHeartRateSensorLocationFinger")]
    pub const Finger: Self = Self(3);
    #[doc(alias = "HKHeartRateSensorLocationHand")]
    pub const Hand: Self = Self(4);
    #[doc(alias = "HKHeartRateSensorLocationEarLobe")]
    pub const EarLobe: Self = Self(5);
    #[doc(alias = "HKHeartRateSensorLocationFoot")]
    pub const Foot: Self = Self(6);
}

unsafe impl Encode for HKHeartRateSensorLocation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKHeartRateSensorLocation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyheartratemotioncontext?language=objc)
    pub static HKMetadataKeyHeartRateMotionContext: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkheartratemotioncontext?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKHeartRateMotionContext(pub NSInteger);
impl HKHeartRateMotionContext {
    #[doc(alias = "HKHeartRateMotionContextNotSet")]
    pub const NotSet: Self = Self(0);
    #[doc(alias = "HKHeartRateMotionContextSedentary")]
    pub const Sedentary: Self = Self(1);
    #[doc(alias = "HKHeartRateMotionContextActive")]
    pub const Active: Self = Self(2);
}

unsafe impl Encode for HKHeartRateMotionContext {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKHeartRateMotionContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyusermotioncontext?language=objc)
    pub static HKMetadataKeyUserMotionContext: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkusermotioncontext?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKUserMotionContext(pub NSInteger);
impl HKUserMotionContext {
    #[doc(alias = "HKUserMotionContextNotSet")]
    pub const NotSet: Self = Self(0);
    #[doc(alias = "HKUserMotionContextStationary")]
    pub const Stationary: Self = Self(1);
    #[doc(alias = "HKUserMotionContextActive")]
    pub const Active: Self = Self(2);
}

unsafe impl Encode for HKUserMotionContext {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKUserMotionContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeysessionestimate?language=objc)
    pub static HKMetadataKeySessionEstimate: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkheartraterecoverytesttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKHeartRateRecoveryTestType(pub NSInteger);
impl HKHeartRateRecoveryTestType {
    #[doc(alias = "HKHeartRateRecoveryTestTypeMaxExercise")]
    pub const MaxExercise: Self = Self(1);
    #[doc(alias = "HKHeartRateRecoveryTestTypePredictionSubMaxExercise")]
    pub const PredictionSubMaxExercise: Self = Self(2);
    #[doc(alias = "HKHeartRateRecoveryTestTypePredictionNonExercise")]
    pub const PredictionNonExercise: Self = Self(3);
}

unsafe impl Encode for HKHeartRateRecoveryTestType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKHeartRateRecoveryTestType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyheartraterecoverytesttype?language=objc)
    pub static HKMetadataKeyHeartRateRecoveryTestType: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyheartraterecoveryactivitytype?language=objc)
    pub static HKMetadataKeyHeartRateRecoveryActivityType: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyheartraterecoveryactivityduration?language=objc)
    pub static HKMetadataKeyHeartRateRecoveryActivityDuration: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyheartraterecoverymaxobservedrecoveryheartrate?language=objc)
    pub static HKMetadataKeyHeartRateRecoveryMaxObservedRecoveryHeartRate: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyfoodtype?language=objc)
    pub static HKMetadataKeyFoodType: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyudideviceidentifier?language=objc)
    pub static HKMetadataKeyUDIDeviceIdentifier: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyudiproductionidentifier?language=objc)
    pub static HKMetadataKeyUDIProductionIdentifier: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeydigitalsignature?language=objc)
    pub static HKMetadataKeyDigitalSignature: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyexternaluuid?language=objc)
    pub static HKMetadataKeyExternalUUID: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeysyncidentifier?language=objc)
    pub static HKMetadataKeySyncIdentifier: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeysyncversion?language=objc)
    pub static HKMetadataKeySyncVersion: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeytimezone?language=objc)
    pub static HKMetadataKeyTimeZone: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeydevicename?language=objc)
    pub static HKMetadataKeyDeviceName: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeydevicemanufacturername?language=objc)
    pub static HKMetadataKeyDeviceManufacturerName: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeywastakeninlab?language=objc)
    pub static HKMetadataKeyWasTakenInLab: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyreferencerangelowerlimit?language=objc)
    pub static HKMetadataKeyReferenceRangeLowerLimit: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyreferencerangeupperlimit?language=objc)
    pub static HKMetadataKeyReferenceRangeUpperLimit: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeywasuserentered?language=objc)
    pub static HKMetadataKeyWasUserEntered: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyworkoutbrandname?language=objc)
    pub static HKMetadataKeyWorkoutBrandName: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeygroupfitness?language=objc)
    pub static HKMetadataKeyGroupFitness: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyapplefitnessplussession?language=objc)
    pub static HKMetadataKeyAppleFitnessPlusSession: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyindoorworkout?language=objc)
    pub static HKMetadataKeyIndoorWorkout: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeycoachedworkout?language=objc)
    pub static HKMetadataKeyCoachedWorkout: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkweathercondition?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKWeatherCondition(pub NSInteger);
impl HKWeatherCondition {
    #[doc(alias = "HKWeatherConditionNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "HKWeatherConditionClear")]
    pub const Clear: Self = Self(1);
    #[doc(alias = "HKWeatherConditionFair")]
    pub const Fair: Self = Self(2);
    #[doc(alias = "HKWeatherConditionPartlyCloudy")]
    pub const PartlyCloudy: Self = Self(3);
    #[doc(alias = "HKWeatherConditionMostlyCloudy")]
    pub const MostlyCloudy: Self = Self(4);
    #[doc(alias = "HKWeatherConditionCloudy")]
    pub const Cloudy: Self = Self(5);
    #[doc(alias = "HKWeatherConditionFoggy")]
    pub const Foggy: Self = Self(6);
    #[doc(alias = "HKWeatherConditionHaze")]
    pub const Haze: Self = Self(7);
    #[doc(alias = "HKWeatherConditionWindy")]
    pub const Windy: Self = Self(8);
    #[doc(alias = "HKWeatherConditionBlustery")]
    pub const Blustery: Self = Self(9);
    #[doc(alias = "HKWeatherConditionSmoky")]
    pub const Smoky: Self = Self(10);
    #[doc(alias = "HKWeatherConditionDust")]
    pub const Dust: Self = Self(11);
    #[doc(alias = "HKWeatherConditionSnow")]
    pub const Snow: Self = Self(12);
    #[doc(alias = "HKWeatherConditionHail")]
    pub const Hail: Self = Self(13);
    #[doc(alias = "HKWeatherConditionSleet")]
    pub const Sleet: Self = Self(14);
    #[doc(alias = "HKWeatherConditionFreezingDrizzle")]
    pub const FreezingDrizzle: Self = Self(15);
    #[doc(alias = "HKWeatherConditionFreezingRain")]
    pub const FreezingRain: Self = Self(16);
    #[doc(alias = "HKWeatherConditionMixedRainAndHail")]
    pub const MixedRainAndHail: Self = Self(17);
    #[doc(alias = "HKWeatherConditionMixedRainAndSnow")]
    pub const MixedRainAndSnow: Self = Self(18);
    #[doc(alias = "HKWeatherConditionMixedRainAndSleet")]
    pub const MixedRainAndSleet: Self = Self(19);
    #[doc(alias = "HKWeatherConditionMixedSnowAndSleet")]
    pub const MixedSnowAndSleet: Self = Self(20);
    #[doc(alias = "HKWeatherConditionDrizzle")]
    pub const Drizzle: Self = Self(21);
    #[doc(alias = "HKWeatherConditionScatteredShowers")]
    pub const ScatteredShowers: Self = Self(22);
    #[doc(alias = "HKWeatherConditionShowers")]
    pub const Showers: Self = Self(23);
    #[doc(alias = "HKWeatherConditionThunderstorms")]
    pub const Thunderstorms: Self = Self(24);
    #[doc(alias = "HKWeatherConditionTropicalStorm")]
    pub const TropicalStorm: Self = Self(25);
    #[doc(alias = "HKWeatherConditionHurricane")]
    pub const Hurricane: Self = Self(26);
    #[doc(alias = "HKWeatherConditionTornado")]
    pub const Tornado: Self = Self(27);
}

unsafe impl Encode for HKWeatherCondition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKWeatherCondition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyweathercondition?language=objc)
    pub static HKMetadataKeyWeatherCondition: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyweathertemperature?language=objc)
    pub static HKMetadataKeyWeatherTemperature: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyweatherhumidity?language=objc)
    pub static HKMetadataKeyWeatherHumidity: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeysexualactivityprotectionused?language=objc)
    pub static HKMetadataKeySexualActivityProtectionUsed: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeymenstrualcyclestart?language=objc)
    pub static HKMetadataKeyMenstrualCycleStart: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeylaplength?language=objc)
    pub static HKMetadataKeyLapLength: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkoutswimminglocationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKWorkoutSwimmingLocationType(pub NSInteger);
impl HKWorkoutSwimmingLocationType {
    #[doc(alias = "HKWorkoutSwimmingLocationTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "HKWorkoutSwimmingLocationTypePool")]
    pub const Pool: Self = Self(1);
    #[doc(alias = "HKWorkoutSwimmingLocationTypeOpenWater")]
    pub const OpenWater: Self = Self(2);
}

unsafe impl Encode for HKWorkoutSwimmingLocationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKWorkoutSwimmingLocationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyswimminglocationtype?language=objc)
    pub static HKMetadataKeySwimmingLocationType: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkswimmingstrokestyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKSwimmingStrokeStyle(pub NSInteger);
impl HKSwimmingStrokeStyle {
    #[doc(alias = "HKSwimmingStrokeStyleUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "HKSwimmingStrokeStyleMixed")]
    pub const Mixed: Self = Self(1);
    #[doc(alias = "HKSwimmingStrokeStyleFreestyle")]
    pub const Freestyle: Self = Self(2);
    #[doc(alias = "HKSwimmingStrokeStyleBackstroke")]
    pub const Backstroke: Self = Self(3);
    #[doc(alias = "HKSwimmingStrokeStyleBreaststroke")]
    pub const Breaststroke: Self = Self(4);
    #[doc(alias = "HKSwimmingStrokeStyleButterfly")]
    pub const Butterfly: Self = Self(5);
    #[doc(alias = "HKSwimmingStrokeStyleKickboard")]
    pub const Kickboard: Self = Self(6);
}

unsafe impl Encode for HKSwimmingStrokeStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKSwimmingStrokeStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyswimmingstrokestyle?language=objc)
    pub static HKMetadataKeySwimmingStrokeStyle: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkinsulindeliveryreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKInsulinDeliveryReason(pub NSInteger);
impl HKInsulinDeliveryReason {
    #[doc(alias = "HKInsulinDeliveryReasonBasal")]
    pub const Basal: Self = Self(1);
    #[doc(alias = "HKInsulinDeliveryReasonBolus")]
    pub const Bolus: Self = Self(2);
}

unsafe impl Encode for HKInsulinDeliveryReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKInsulinDeliveryReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyinsulindeliveryreason?language=objc)
    pub static HKMetadataKeyInsulinDeliveryReason: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkbloodglucosemealtime?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKBloodGlucoseMealTime(pub NSInteger);
impl HKBloodGlucoseMealTime {
    #[doc(alias = "HKBloodGlucoseMealTimePreprandial")]
    pub const Preprandial: Self = Self(1);
    #[doc(alias = "HKBloodGlucoseMealTimePostprandial")]
    pub const Postprandial: Self = Self(2);
}

unsafe impl Encode for HKBloodGlucoseMealTime {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKBloodGlucoseMealTime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeybloodglucosemealtime?language=objc)
    pub static HKMetadataKeyBloodGlucoseMealTime: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkvo2maxtesttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKVO2MaxTestType(pub NSInteger);
impl HKVO2MaxTestType {
    #[doc(alias = "HKVO2MaxTestTypeMaxExercise")]
    pub const MaxExercise: Self = Self(1);
    #[doc(alias = "HKVO2MaxTestTypePredictionSubMaxExercise")]
    pub const PredictionSubMaxExercise: Self = Self(2);
    #[doc(alias = "HKVO2MaxTestTypePredictionNonExercise")]
    pub const PredictionNonExercise: Self = Self(3);
}

unsafe impl Encode for HKVO2MaxTestType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKVO2MaxTestType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyvo2maxtesttype?language=objc)
    pub static HKMetadataKeyVO2MaxTestType: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyaveragespeed?language=objc)
    pub static HKMetadataKeyAverageSpeed: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeymaximumspeed?language=objc)
    pub static HKMetadataKeyMaximumSpeed: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyalpineslopegrade?language=objc)
    pub static HKMetadataKeyAlpineSlopeGrade: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyelevationascended?language=objc)
    pub static HKMetadataKeyElevationAscended: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyelevationdescended?language=objc)
    pub static HKMetadataKeyElevationDescended: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyfitnessmachineduration?language=objc)
    pub static HKMetadataKeyFitnessMachineDuration: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyindoorbikedistance?language=objc)
    pub static HKMetadataKeyIndoorBikeDistance: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeycrosstrainerdistance?language=objc)
    pub static HKMetadataKeyCrossTrainerDistance: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyheartrateeventthreshold?language=objc)
    pub static HKMetadataKeyHeartRateEventThreshold: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyaveragemets?language=objc)
    pub static HKMetadataKeyAverageMETs: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyaudioexposurelevel?language=objc)
    pub static HKMetadataKeyAudioExposureLevel: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyaudioexposureduration?language=objc)
    pub static HKMetadataKeyAudioExposureDuration: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkappleecgalgorithmversion?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKAppleECGAlgorithmVersion(pub NSInteger);
impl HKAppleECGAlgorithmVersion {
    pub const HKAppleECGAlgorithmVersion1: Self = Self(1);
    pub const HKAppleECGAlgorithmVersion2: Self = Self(2);
}

unsafe impl Encode for HKAppleECGAlgorithmVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKAppleECGAlgorithmVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyappleecgalgorithmversion?language=objc)
    pub static HKMetadataKeyAppleECGAlgorithmVersion: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdeviceplacementside?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKDevicePlacementSide(pub NSInteger);
impl HKDevicePlacementSide {
    #[doc(alias = "HKDevicePlacementSideUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "HKDevicePlacementSideLeft")]
    pub const Left: Self = Self(1);
    #[doc(alias = "HKDevicePlacementSideRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "HKDevicePlacementSideCentral")]
    pub const Central: Self = Self(3);
}

unsafe impl Encode for HKDevicePlacementSide {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKDevicePlacementSide {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeydeviceplacementside?language=objc)
    pub static HKMetadataKeyDevicePlacementSide: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeybarometricpressure?language=objc)
    pub static HKMetadataKeyBarometricPressure: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyappledevicecalibrated?language=objc)
    pub static HKMetadataKeyAppleDeviceCalibrated: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyvo2maxvalue?language=objc)
    pub static HKMetadataKeyVO2MaxValue: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeylowcardiofitnesseventthreshold?language=objc)
    pub static HKMetadataKeyLowCardioFitnessEventThreshold: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeydateofearliestdatausedforestimate?language=objc)
    pub static HKMetadataKeyDateOfEarliestDataUsedForEstimate: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyalgorithmversion?language=objc)
    pub static HKMetadataKeyAlgorithmVersion: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyswolfscore?language=objc)
    pub static HKMetadataKeySWOLFScore: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyquantityclampedtolowerbound?language=objc)
    pub static HKMetadataKeyQuantityClampedToLowerBound: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyquantityclampedtoupperbound?language=objc)
    pub static HKMetadataKeyQuantityClampedToUpperBound: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyglassesprescriptiondescription?language=objc)
    pub static HKMetadataKeyGlassesPrescriptionDescription: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkwatersalinity?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKWaterSalinity(pub NSInteger);
impl HKWaterSalinity {
    #[doc(alias = "HKWaterSalinityFreshWater")]
    pub const FreshWater: Self = Self(1);
    #[doc(alias = "HKWaterSalinitySaltWater")]
    pub const SaltWater: Self = Self(2);
}

unsafe impl Encode for HKWaterSalinity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKWaterSalinity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeywatersalinity?language=objc)
    pub static HKMetadataKeyWaterSalinity: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyheadphonegain?language=objc)
    pub static HKMetadataKeyHeadphoneGain: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeycyclingfunctionalthresholdpowertesttype?language=objc)
    pub static HKMetadataKeyCyclingFunctionalThresholdPowerTestType: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkcyclingfunctionalthresholdpowertesttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKCyclingFunctionalThresholdPowerTestType(pub NSInteger);
impl HKCyclingFunctionalThresholdPowerTestType {
    #[doc(alias = "HKCyclingFunctionalThresholdPowerTestTypeMaxExercise60Minute")]
    pub const MaxExercise60Minute: Self = Self(1);
    #[doc(alias = "HKCyclingFunctionalThresholdPowerTestTypeMaxExercise20Minute")]
    pub const MaxExercise20Minute: Self = Self(2);
    #[doc(alias = "HKCyclingFunctionalThresholdPowerTestTypeRampTest")]
    pub const RampTest: Self = Self(3);
    #[doc(alias = "HKCyclingFunctionalThresholdPowerTestTypePredictionExercise")]
    pub const PredictionExercise: Self = Self(4);
}

unsafe impl Encode for HKCyclingFunctionalThresholdPowerTestType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKCyclingFunctionalThresholdPowerTestType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyactivitytype?language=objc)
    pub static HKMetadataKeyActivityType: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeyphysicaleffortestimationtype?language=objc)
    pub static HKMetadataKeyPhysicalEffortEstimationType: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkphysicaleffortestimationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKPhysicalEffortEstimationType(pub NSInteger);
impl HKPhysicalEffortEstimationType {
    #[doc(alias = "HKPhysicalEffortEstimationTypeActivityLookup")]
    pub const ActivityLookup: Self = Self(1);
    #[doc(alias = "HKPhysicalEffortEstimationTypeDeviceSensed")]
    pub const DeviceSensed: Self = Self(2);
}

unsafe impl Encode for HKPhysicalEffortEstimationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKPhysicalEffortEstimationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkmetadatakeymaximumlightintensity?language=objc)
    pub static HKMetadataKeyMaximumLightIntensity: &'static NSString;
}
