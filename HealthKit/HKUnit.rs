//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKUnit;

    unsafe impl ClassType for HKUnit {
        type Super = NSObject;
    }
);

unsafe impl Send for HKUnit {}

unsafe impl Sync for HKUnit {}

unsafe impl NSCoding for HKUnit {}

unsafe impl NSCopying for HKUnit {}

unsafe impl CopyingHelper for HKUnit {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKUnit {}

unsafe impl NSSecureCoding for HKUnit {}

extern_methods!(
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other unitString)]
        pub unsafe fn unitString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other unitFromString:)]
        pub unsafe fn unitFromString(string: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Other unitFromMassFormatterUnit:)]
        pub unsafe fn unitFromMassFormatterUnit(
            mass_formatter_unit: NSMassFormatterUnit,
        ) -> Retained<Self>;

        #[method(massFormatterUnitFromUnit:)]
        pub unsafe fn massFormatterUnitFromUnit(unit: &HKUnit) -> NSMassFormatterUnit;

        #[method_id(@__retain_semantics Other unitFromLengthFormatterUnit:)]
        pub unsafe fn unitFromLengthFormatterUnit(
            length_formatter_unit: NSLengthFormatterUnit,
        ) -> Retained<Self>;

        #[method(lengthFormatterUnitFromUnit:)]
        pub unsafe fn lengthFormatterUnitFromUnit(unit: &HKUnit) -> NSLengthFormatterUnit;

        #[method_id(@__retain_semantics Other unitFromEnergyFormatterUnit:)]
        pub unsafe fn unitFromEnergyFormatterUnit(
            energy_formatter_unit: NSEnergyFormatterUnit,
        ) -> Retained<Self>;

        #[method(energyFormatterUnitFromUnit:)]
        pub unsafe fn energyFormatterUnitFromUnit(unit: &HKUnit) -> NSEnergyFormatterUnit;

        #[method(isNull)]
        pub unsafe fn isNull(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKMetricPrefix(pub NSInteger);
impl HKMetricPrefix {
    #[doc(alias = "HKMetricPrefixNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "HKMetricPrefixFemto")]
    pub const Femto: Self = Self(13);
    #[doc(alias = "HKMetricPrefixPico")]
    pub const Pico: Self = Self(1);
    #[doc(alias = "HKMetricPrefixNano")]
    pub const Nano: Self = Self(2);
    #[doc(alias = "HKMetricPrefixMicro")]
    pub const Micro: Self = Self(3);
    #[doc(alias = "HKMetricPrefixMilli")]
    pub const Milli: Self = Self(4);
    #[doc(alias = "HKMetricPrefixCenti")]
    pub const Centi: Self = Self(5);
    #[doc(alias = "HKMetricPrefixDeci")]
    pub const Deci: Self = Self(6);
    #[doc(alias = "HKMetricPrefixDeca")]
    pub const Deca: Self = Self(7);
    #[doc(alias = "HKMetricPrefixHecto")]
    pub const Hecto: Self = Self(8);
    #[doc(alias = "HKMetricPrefixKilo")]
    pub const Kilo: Self = Self(9);
    #[doc(alias = "HKMetricPrefixMega")]
    pub const Mega: Self = Self(10);
    #[doc(alias = "HKMetricPrefixGiga")]
    pub const Giga: Self = Self(11);
    #[doc(alias = "HKMetricPrefixTera")]
    pub const Tera: Self = Self(12);
}

unsafe impl Encode for HKMetricPrefix {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKMetricPrefix {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// Mass
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other gramUnitWithMetricPrefix:)]
        pub unsafe fn gramUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other gramUnit)]
        pub unsafe fn gramUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other ounceUnit)]
        pub unsafe fn ounceUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other poundUnit)]
        pub unsafe fn poundUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other stoneUnit)]
        pub unsafe fn stoneUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other moleUnitWithMetricPrefix:molarMass:)]
        pub unsafe fn moleUnitWithMetricPrefix_molarMass(
            prefix: HKMetricPrefix,
            grams_per_mole: c_double,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other moleUnitWithMolarMass:)]
        pub unsafe fn moleUnitWithMolarMass(grams_per_mole: c_double) -> Retained<Self>;
    }
);

extern_methods!(
    /// Length
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other meterUnitWithMetricPrefix:)]
        pub unsafe fn meterUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other meterUnit)]
        pub unsafe fn meterUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other inchUnit)]
        pub unsafe fn inchUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other footUnit)]
        pub unsafe fn footUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other yardUnit)]
        pub unsafe fn yardUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other mileUnit)]
        pub unsafe fn mileUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Volume
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other literUnitWithMetricPrefix:)]
        pub unsafe fn literUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other literUnit)]
        pub unsafe fn literUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other fluidOunceUSUnit)]
        pub unsafe fn fluidOunceUSUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other fluidOunceImperialUnit)]
        pub unsafe fn fluidOunceImperialUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other pintUSUnit)]
        pub unsafe fn pintUSUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other pintImperialUnit)]
        pub unsafe fn pintImperialUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other cupUSUnit)]
        pub unsafe fn cupUSUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other cupImperialUnit)]
        pub unsafe fn cupImperialUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Pressure
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other pascalUnitWithMetricPrefix:)]
        pub unsafe fn pascalUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other pascalUnit)]
        pub unsafe fn pascalUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other millimeterOfMercuryUnit)]
        pub unsafe fn millimeterOfMercuryUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other centimeterOfWaterUnit)]
        pub unsafe fn centimeterOfWaterUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other atmosphereUnit)]
        pub unsafe fn atmosphereUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other decibelAWeightedSoundPressureLevelUnit)]
        pub unsafe fn decibelAWeightedSoundPressureLevelUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other inchesOfMercuryUnit)]
        pub unsafe fn inchesOfMercuryUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Time
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other secondUnitWithMetricPrefix:)]
        pub unsafe fn secondUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other secondUnit)]
        pub unsafe fn secondUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other minuteUnit)]
        pub unsafe fn minuteUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other hourUnit)]
        pub unsafe fn hourUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other dayUnit)]
        pub unsafe fn dayUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Energy
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other jouleUnitWithMetricPrefix:)]
        pub unsafe fn jouleUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other jouleUnit)]
        pub unsafe fn jouleUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other kilocalorieUnit)]
        pub unsafe fn kilocalorieUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other smallCalorieUnit)]
        pub unsafe fn smallCalorieUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other largeCalorieUnit)]
        pub unsafe fn largeCalorieUnit() -> Retained<Self>;

        #[deprecated = "Use smallCalorieUnit or largeCalorieUnit, depending on which you mean"]
        #[method_id(@__retain_semantics Other calorieUnit)]
        pub unsafe fn calorieUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Temperature
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other degreeCelsiusUnit)]
        pub unsafe fn degreeCelsiusUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other degreeFahrenheitUnit)]
        pub unsafe fn degreeFahrenheitUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other kelvinUnit)]
        pub unsafe fn kelvinUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Conductance
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other siemenUnitWithMetricPrefix:)]
        pub unsafe fn siemenUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other siemenUnit)]
        pub unsafe fn siemenUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Pharmacology
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other internationalUnit)]
        pub unsafe fn internationalUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Scalar
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other countUnit)]
        pub unsafe fn countUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other percentUnit)]
        pub unsafe fn percentUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// HearingSensitivity
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other decibelHearingLevelUnit)]
        pub unsafe fn decibelHearingLevelUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Math
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other unitMultipliedByUnit:)]
        pub unsafe fn unitMultipliedByUnit(&self, unit: &HKUnit) -> Retained<HKUnit>;

        #[method_id(@__retain_semantics Other unitDividedByUnit:)]
        pub unsafe fn unitDividedByUnit(&self, unit: &HKUnit) -> Retained<HKUnit>;

        #[method_id(@__retain_semantics Other unitRaisedToPower:)]
        pub unsafe fn unitRaisedToPower(&self, power: NSInteger) -> Retained<HKUnit>;

        #[method_id(@__retain_semantics Other reciprocalUnit)]
        pub unsafe fn reciprocalUnit(&self) -> Retained<HKUnit>;
    }
);

extern_methods!(
    /// Frequency
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other hertzUnitWithMetricPrefix:)]
        pub unsafe fn hertzUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other hertzUnit)]
        pub unsafe fn hertzUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// ElectricPotentialDifference
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other voltUnitWithMetricPrefix:)]
        pub unsafe fn voltUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other voltUnit)]
        pub unsafe fn voltUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Power
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other wattUnitWithMetricPrefix:)]
        pub unsafe fn wattUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other wattUnit)]
        pub unsafe fn wattUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// OpticalPower
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other diopterUnit)]
        pub unsafe fn diopterUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other prismDiopterUnit)]
        pub unsafe fn prismDiopterUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Angle
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other radianAngleUnitWithMetricPrefix:)]
        pub unsafe fn radianAngleUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other radianAngleUnit)]
        pub unsafe fn radianAngleUnit() -> Retained<Self>;

        #[method_id(@__retain_semantics Other degreeAngleUnit)]
        pub unsafe fn degreeAngleUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Illuminance
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other luxUnitWithMetricPrefix:)]
        pub unsafe fn luxUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Retained<Self>;

        #[method_id(@__retain_semantics Other luxUnit)]
        pub unsafe fn luxUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// UnitLess
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other appleEffortScoreUnit)]
        pub unsafe fn appleEffortScoreUnit() -> Retained<Self>;
    }
);
