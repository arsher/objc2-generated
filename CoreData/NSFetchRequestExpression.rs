//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSFetchRequestExpressionType: NSExpressionType = 50);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSFetchRequestExpression")]
    pub struct NSFetchRequestExpression;

    #[cfg(feature = "CoreData_NSFetchRequestExpression")]
    unsafe impl ClassType for NSFetchRequestExpression {
        #[inherits(NSObject)]
        type Super = NSExpression;
    }
);

#[cfg(feature = "CoreData_NSFetchRequestExpression")]
unsafe impl NSCoding for NSFetchRequestExpression {}

#[cfg(feature = "CoreData_NSFetchRequestExpression")]
unsafe impl NSObjectProtocol for NSFetchRequestExpression {}

#[cfg(feature = "CoreData_NSFetchRequestExpression")]
unsafe impl NSSecureCoding for NSFetchRequestExpression {}

extern_methods!(
    #[cfg(feature = "CoreData_NSFetchRequestExpression")]
    unsafe impl NSFetchRequestExpression {
        #[method_id(@__retain_semantics Other expressionForFetch:context:countOnly:)]
        pub unsafe fn expressionForFetch_context_countOnly(
            fetch: &NSExpression,
            context: &NSExpression,
            count_flag: bool,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other requestExpression)]
        pub unsafe fn requestExpression(&self) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other contextExpression)]
        pub unsafe fn contextExpression(&self) -> Id<NSExpression, Shared>;

        #[method(isCountOnlyRequest)]
        pub unsafe fn isCountOnlyRequest(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSExpression`
    #[cfg(feature = "CoreData_NSFetchRequestExpression")]
    unsafe impl NSFetchRequestExpression {
        #[method_id(@__retain_semantics Init initWithExpressionType:)]
        pub unsafe fn initWithExpressionType(
            this: Option<Allocated<Self>>,
            r#type: NSExpressionType,
        ) -> Id<Self, Shared>;
    }
);
