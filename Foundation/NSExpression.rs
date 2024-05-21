//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSExpressionType(pub NSUInteger);
impl NSExpressionType {
    pub const NSConstantValueExpressionType: Self = Self(0);
    pub const NSEvaluatedObjectExpressionType: Self = Self(1);
    pub const NSVariableExpressionType: Self = Self(2);
    pub const NSKeyPathExpressionType: Self = Self(3);
    pub const NSFunctionExpressionType: Self = Self(4);
    pub const NSUnionSetExpressionType: Self = Self(5);
    pub const NSIntersectSetExpressionType: Self = Self(6);
    pub const NSMinusSetExpressionType: Self = Self(7);
    pub const NSSubqueryExpressionType: Self = Self(13);
    pub const NSAggregateExpressionType: Self = Self(14);
    pub const NSAnyKeyExpressionType: Self = Self(15);
    pub const NSBlockExpressionType: Self = Self(19);
    pub const NSConditionalExpressionType: Self = Self(20);
}

unsafe impl Encode for NSExpressionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSExpressionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSExpression;

    unsafe impl ClassType for NSExpression {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSExpression {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSExpression {}

unsafe impl NSObjectProtocol for NSExpression {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSExpression {}

extern_methods!(
    unsafe impl NSExpression {
        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other expressionWithFormat:argumentArray:)]
        pub unsafe fn expressionWithFormat_argumentArray(
            expression_format: &NSString,
            arguments: &NSArray,
        ) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForConstantValue:)]
        pub unsafe fn expressionForConstantValue(obj: Option<&AnyObject>)
            -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForEvaluatedObject)]
        pub unsafe fn expressionForEvaluatedObject() -> Retained<NSExpression>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other expressionForVariable:)]
        pub unsafe fn expressionForVariable(string: &NSString) -> Retained<NSExpression>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other expressionForKeyPath:)]
        pub unsafe fn expressionForKeyPath(key_path: &NSString) -> Retained<NSExpression>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other expressionForFunction:arguments:)]
        pub unsafe fn expressionForFunction_arguments(
            name: &NSString,
            parameters: &NSArray,
        ) -> Retained<NSExpression>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other expressionForAggregate:)]
        pub unsafe fn expressionForAggregate(
            subexpressions: &NSArray<NSExpression>,
        ) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForUnionSet:with:)]
        pub unsafe fn expressionForUnionSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForIntersectSet:with:)]
        pub unsafe fn expressionForIntersectSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForMinusSet:with:)]
        pub unsafe fn expressionForMinusSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Retained<NSExpression>;

        #[cfg(all(feature = "NSPredicate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other expressionForSubquery:usingIteratorVariable:predicate:)]
        pub unsafe fn expressionForSubquery_usingIteratorVariable_predicate(
            expression: &NSExpression,
            variable: &NSString,
            predicate: &NSPredicate,
        ) -> Retained<NSExpression>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other expressionForFunction:selectorName:arguments:)]
        pub unsafe fn expressionForFunction_selectorName_arguments(
            target: &NSExpression,
            name: &NSString,
            parameters: Option<&NSArray>,
        ) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForAnyKey)]
        pub unsafe fn expressionForAnyKey() -> Retained<NSExpression>;

        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "block2"))]
        #[method_id(@__retain_semantics Other expressionForBlock:arguments:)]
        pub unsafe fn expressionForBlock_arguments(
            block: &block2::Block<
                dyn Fn(
                    *mut AnyObject,
                    NonNull<NSArray<NSExpression>>,
                    *mut NSMutableDictionary,
                ) -> NonNull<AnyObject>,
            >,
            arguments: Option<&NSArray<NSExpression>>,
        ) -> Retained<NSExpression>;

        #[cfg(feature = "NSPredicate")]
        #[method_id(@__retain_semantics Other expressionForConditional:trueExpression:falseExpression:)]
        pub unsafe fn expressionForConditional_trueExpression_falseExpression(
            predicate: &NSPredicate,
            true_expression: &NSExpression,
            false_expression: &NSExpression,
        ) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Init initWithExpressionType:)]
        pub unsafe fn initWithExpressionType(
            this: Allocated<Self>,
            r#type: NSExpressionType,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(expressionType)]
        pub unsafe fn expressionType(&self) -> NSExpressionType;

        #[method_id(@__retain_semantics Other constantValue)]
        pub unsafe fn constantValue(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other function)]
        pub unsafe fn function(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other variable)]
        pub unsafe fn variable(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other operand)]
        pub unsafe fn operand(&self) -> Retained<NSExpression>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Retained<NSArray<NSExpression>>>;

        #[method_id(@__retain_semantics Other collection)]
        pub unsafe fn collection(&self) -> Retained<AnyObject>;

        #[cfg(feature = "NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other leftExpression)]
        pub unsafe fn leftExpression(&self) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other rightExpression)]
        pub unsafe fn rightExpression(&self) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other trueExpression)]
        pub unsafe fn trueExpression(&self) -> Retained<NSExpression>;

        #[method_id(@__retain_semantics Other falseExpression)]
        pub unsafe fn falseExpression(&self) -> Retained<NSExpression>;

        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "block2"))]
        #[method(expressionBlock)]
        pub unsafe fn expressionBlock(
            &self,
        ) -> NonNull<
            block2::Block<
                dyn Fn(
                    *mut AnyObject,
                    NonNull<NSArray<NSExpression>>,
                    *mut NSMutableDictionary,
                ) -> NonNull<AnyObject>,
            >,
        >;

        #[cfg(feature = "NSDictionary")]
        #[method_id(@__retain_semantics Other expressionValueWithObject:context:)]
        pub unsafe fn expressionValueWithObject_context(
            &self,
            object: Option<&AnyObject>,
            context: Option<&NSMutableDictionary>,
        ) -> Option<Retained<AnyObject>>;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSExpression {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
