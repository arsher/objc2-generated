//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscomparisonpredicateoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSComparisonPredicateOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSComparisonPredicateOptions: NSUInteger {
        const NSCaseInsensitivePredicateOption = 0x01;
        const NSDiacriticInsensitivePredicateOption = 0x02;
        const NSNormalizedPredicateOption = 0x04;
    }
}

unsafe impl Encode for NSComparisonPredicateOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSComparisonPredicateOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscomparisonpredicatemodifier?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSComparisonPredicateModifier(pub NSUInteger);
impl NSComparisonPredicateModifier {
    pub const NSDirectPredicateModifier: Self = Self(0);
    pub const NSAllPredicateModifier: Self = Self(1);
    pub const NSAnyPredicateModifier: Self = Self(2);
}

unsafe impl Encode for NSComparisonPredicateModifier {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSComparisonPredicateModifier {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspredicateoperatortype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPredicateOperatorType(pub NSUInteger);
impl NSPredicateOperatorType {
    pub const NSLessThanPredicateOperatorType: Self = Self(0);
    pub const NSLessThanOrEqualToPredicateOperatorType: Self = Self(1);
    pub const NSGreaterThanPredicateOperatorType: Self = Self(2);
    pub const NSGreaterThanOrEqualToPredicateOperatorType: Self = Self(3);
    pub const NSEqualToPredicateOperatorType: Self = Self(4);
    pub const NSNotEqualToPredicateOperatorType: Self = Self(5);
    pub const NSMatchesPredicateOperatorType: Self = Self(6);
    pub const NSLikePredicateOperatorType: Self = Self(7);
    pub const NSBeginsWithPredicateOperatorType: Self = Self(8);
    pub const NSEndsWithPredicateOperatorType: Self = Self(9);
    pub const NSInPredicateOperatorType: Self = Self(10);
    pub const NSCustomSelectorPredicateOperatorType: Self = Self(11);
    pub const NSContainsPredicateOperatorType: Self = Self(99);
    pub const NSBetweenPredicateOperatorType: Self = Self(100);
}

unsafe impl Encode for NSPredicateOperatorType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPredicateOperatorType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscomparisonpredicate?language=objc)
    #[unsafe(super(NSPredicate, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPredicate")]
    pub struct NSComparisonPredicate;
);

#[cfg(all(feature = "NSObject", feature = "NSPredicate"))]
unsafe impl NSCoding for NSComparisonPredicate {}

#[cfg(all(feature = "NSObject", feature = "NSPredicate"))]
unsafe impl NSCopying for NSComparisonPredicate {}

#[cfg(all(feature = "NSObject", feature = "NSPredicate"))]
unsafe impl CopyingHelper for NSComparisonPredicate {
    type Result = Self;
}

#[cfg(feature = "NSPredicate")]
unsafe impl NSObjectProtocol for NSComparisonPredicate {}

#[cfg(all(feature = "NSObject", feature = "NSPredicate"))]
unsafe impl NSSecureCoding for NSComparisonPredicate {}

extern_methods!(
    #[cfg(feature = "NSPredicate")]
    unsafe impl NSComparisonPredicate {
        #[cfg(feature = "NSExpression")]
        #[method_id(@__retain_semantics Other predicateWithLeftExpression:rightExpression:modifier:type:options:)]
        pub unsafe fn predicateWithLeftExpression_rightExpression_modifier_type_options(
            lhs: &NSExpression,
            rhs: &NSExpression,
            modifier: NSComparisonPredicateModifier,
            r#type: NSPredicateOperatorType,
            options: NSComparisonPredicateOptions,
        ) -> Retained<NSComparisonPredicate>;

        #[cfg(feature = "NSExpression")]
        #[method_id(@__retain_semantics Other predicateWithLeftExpression:rightExpression:customSelector:)]
        pub unsafe fn predicateWithLeftExpression_rightExpression_customSelector(
            lhs: &NSExpression,
            rhs: &NSExpression,
            selector: Sel,
        ) -> Retained<NSComparisonPredicate>;

        #[cfg(feature = "NSExpression")]
        #[method_id(@__retain_semantics Init initWithLeftExpression:rightExpression:modifier:type:options:)]
        pub unsafe fn initWithLeftExpression_rightExpression_modifier_type_options(
            this: Allocated<Self>,
            lhs: &NSExpression,
            rhs: &NSExpression,
            modifier: NSComparisonPredicateModifier,
            r#type: NSPredicateOperatorType,
            options: NSComparisonPredicateOptions,
        ) -> Retained<Self>;

        #[cfg(feature = "NSExpression")]
        #[method_id(@__retain_semantics Init initWithLeftExpression:rightExpression:customSelector:)]
        pub unsafe fn initWithLeftExpression_rightExpression_customSelector(
            this: Allocated<Self>,
            lhs: &NSExpression,
            rhs: &NSExpression,
            selector: Sel,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(predicateOperatorType)]
        pub unsafe fn predicateOperatorType(&self) -> NSPredicateOperatorType;

        #[method(comparisonPredicateModifier)]
        pub unsafe fn comparisonPredicateModifier(&self) -> NSComparisonPredicateModifier;

        #[cfg(feature = "NSExpression")]
        #[method_id(@__retain_semantics Other leftExpression)]
        pub unsafe fn leftExpression(&self) -> Retained<NSExpression>;

        #[cfg(feature = "NSExpression")]
        #[method_id(@__retain_semantics Other rightExpression)]
        pub unsafe fn rightExpression(&self) -> Retained<NSExpression>;

        #[method(customSelector)]
        pub unsafe fn customSelector(&self) -> Option<Sel>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSComparisonPredicateOptions;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPredicate")]
    unsafe impl NSComparisonPredicate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
