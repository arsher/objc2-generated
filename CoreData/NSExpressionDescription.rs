//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsexpressiondescription?language=objc)
    #[unsafe(super(NSPropertyDescription, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPropertyDescription")]
    pub struct NSExpressionDescription;
);

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSCoding for NSExpressionDescription {}

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSCopying for NSExpressionDescription {}

#[cfg(feature = "NSPropertyDescription")]
unsafe impl CopyingHelper for NSExpressionDescription {
    type Result = Self;
}

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSObjectProtocol for NSExpressionDescription {}

extern_methods!(
    #[cfg(feature = "NSPropertyDescription")]
    unsafe impl NSExpressionDescription {
        #[method_id(@__retain_semantics Other expression)]
        pub unsafe fn expression(&self) -> Option<Retained<NSExpression>>;

        /// Setter for [`expression`][Self::expression].
        #[method(setExpression:)]
        pub unsafe fn setExpression(&self, expression: Option<&NSExpression>);

        #[cfg(feature = "NSAttributeDescription")]
        #[method(expressionResultType)]
        pub unsafe fn expressionResultType(&self) -> NSAttributeType;

        #[cfg(feature = "NSAttributeDescription")]
        /// Setter for [`expressionResultType`][Self::expressionResultType].
        #[method(setExpressionResultType:)]
        pub unsafe fn setExpressionResultType(&self, expression_result_type: NSAttributeType);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPropertyDescription")]
    unsafe impl NSExpressionDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
