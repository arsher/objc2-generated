//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSExpressionDescription")]
    pub struct NSExpressionDescription;

    #[cfg(feature = "CoreData_NSExpressionDescription")]
    unsafe impl ClassType for NSExpressionDescription {
        #[inherits(NSObject)]
        type Super = NSPropertyDescription;
    }
);

#[cfg(feature = "CoreData_NSExpressionDescription")]
unsafe impl NSCoding for NSExpressionDescription {}

#[cfg(feature = "CoreData_NSExpressionDescription")]
unsafe impl NSObjectProtocol for NSExpressionDescription {}

extern_methods!(
    #[cfg(feature = "CoreData_NSExpressionDescription")]
    unsafe impl NSExpressionDescription {
        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other expression)]
        pub unsafe fn expression(&self) -> Option<Id<NSExpression, Shared>>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method(setExpression:)]
        pub unsafe fn setExpression(&self, expression: Option<&NSExpression>);

        #[method(expressionResultType)]
        pub unsafe fn expressionResultType(&self) -> NSAttributeType;

        #[method(setExpressionResultType:)]
        pub unsafe fn setExpressionResultType(&self, expression_result_type: NSAttributeType);
    }
);
