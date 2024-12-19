//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmeasurement?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMeasurement<UnitType: ?Sized = AnyObject>;
);

#[cfg(feature = "NSObject")]
unsafe impl<UnitType: ?Sized + NSCoding> NSCoding for NSMeasurement<UnitType> {}

#[cfg(feature = "NSObject")]
unsafe impl<UnitType: ?Sized> NSCopying for NSMeasurement<UnitType> {}

#[cfg(feature = "NSObject")]
unsafe impl<UnitType: ?Sized + Message> CopyingHelper for NSMeasurement<UnitType> {
    type Result = Self;
}

unsafe impl<UnitType: ?Sized> NSObjectProtocol for NSMeasurement<UnitType> {}

#[cfg(feature = "NSObject")]
unsafe impl<UnitType: ?Sized + NSSecureCoding> NSSecureCoding for NSMeasurement<UnitType> {}

extern_methods!(
    unsafe impl<UnitType: Message> NSMeasurement<UnitType> {
        #[method_id(@__retain_semantics Other unit)]
        pub unsafe fn unit(&self) -> Retained<UnitType>;

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDoubleValue:unit:)]
        pub unsafe fn initWithDoubleValue_unit(
            this: Allocated<Self>,
            double_value: c_double,
            unit: &UnitType,
        ) -> Retained<Self>;

        #[cfg(feature = "NSUnit")]
        #[method(canBeConvertedToUnit:)]
        pub unsafe fn canBeConvertedToUnit(&self, unit: &NSUnit) -> bool;

        #[cfg(feature = "NSUnit")]
        #[method_id(@__retain_semantics Other measurementByConvertingToUnit:)]
        pub unsafe fn measurementByConvertingToUnit(
            &self,
            unit: &NSUnit,
        ) -> Retained<NSMeasurement>;

        #[method_id(@__retain_semantics Other measurementByAddingMeasurement:)]
        pub unsafe fn measurementByAddingMeasurement(
            &self,
            measurement: &NSMeasurement<UnitType>,
        ) -> Retained<NSMeasurement<UnitType>>;

        #[method_id(@__retain_semantics Other measurementBySubtractingMeasurement:)]
        pub unsafe fn measurementBySubtractingMeasurement(
            &self,
            measurement: &NSMeasurement<UnitType>,
        ) -> Retained<NSMeasurement<UnitType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<UnitType: Message> NSMeasurement<UnitType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
