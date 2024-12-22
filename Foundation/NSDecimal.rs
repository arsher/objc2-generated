//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// *************    Type definitions        **********
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsroundingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRoundingMode(pub NSUInteger);
impl NSRoundingMode {
    pub const NSRoundPlain: Self = Self(0);
    pub const NSRoundDown: Self = Self(1);
    pub const NSRoundUp: Self = Self(2);
    pub const NSRoundBankers: Self = Self(3);
}

unsafe impl Encode for NSRoundingMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSRoundingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscalculationerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCalculationError(pub NSUInteger);
impl NSCalculationError {
    pub const NSCalculationNoError: Self = Self(0);
    pub const NSCalculationLossOfPrecision: Self = Self(1);
    pub const NSCalculationUnderflow: Self = Self(2);
    pub const NSCalculationOverflow: Self = Self(3);
    pub const NSCalculationDivideByZero: Self = Self(4);
}

unsafe impl Encode for NSCalculationError {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCalculationError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn NSDecimalIsNotANumber(dcm: NonNull<NSDecimal>,) -> Bool;

extern "C-unwind" {
    /// *************    Operations        **********
    pub fn NSDecimalCopy(destination: NonNull<NSDecimal>, source: NonNull<NSDecimal>);
}

extern "C-unwind" {
    pub fn NSDecimalCompact(number: NonNull<NSDecimal>);
}

extern "C-unwind" {
    #[cfg(feature = "NSObjCRuntime")]
    pub fn NSDecimalCompare(
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
    ) -> NSComparisonResult;
}

extern "C-unwind" {
    pub fn NSDecimalRound(
        result: NonNull<NSDecimal>,
        number: NonNull<NSDecimal>,
        scale: NSInteger,
        rounding_mode: NSRoundingMode,
    );
}

extern "C-unwind" {
    pub fn NSDecimalNormalize(
        number1: NonNull<NSDecimal>,
        number2: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C-unwind" {
    pub fn NSDecimalAdd(
        result: NonNull<NSDecimal>,
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C-unwind" {
    pub fn NSDecimalSubtract(
        result: NonNull<NSDecimal>,
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C-unwind" {
    pub fn NSDecimalMultiply(
        result: NonNull<NSDecimal>,
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C-unwind" {
    pub fn NSDecimalDivide(
        result: NonNull<NSDecimal>,
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C-unwind" {
    pub fn NSDecimalPower(
        result: NonNull<NSDecimal>,
        number: NonNull<NSDecimal>,
        power: NSUInteger,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C-unwind" {
    pub fn NSDecimalMultiplyByPowerOf10(
        result: NonNull<NSDecimal>,
        number: NonNull<NSDecimal>,
        power: c_short,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

#[cfg(feature = "NSString")]
#[inline]
pub unsafe extern "C-unwind" fn NSDecimalString(
    dcm: NonNull<NSDecimal>,
    locale: Option<&AnyObject>,
) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSDecimalString(
            dcm: NonNull<NSDecimal>,
            locale: Option<&AnyObject>,
        ) -> NonNull<NSString>;
    }
    let ret = unsafe { NSDecimalString(dcm, locale) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}
