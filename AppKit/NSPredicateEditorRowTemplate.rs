//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPredicateEditorRowTemplate")]
    pub struct NSPredicateEditorRowTemplate;

    #[cfg(feature = "AppKit_NSPredicateEditorRowTemplate")]
    unsafe impl ClassType for NSPredicateEditorRowTemplate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSPredicateEditorRowTemplate")]
unsafe impl NSCoding for NSPredicateEditorRowTemplate {}

#[cfg(feature = "AppKit_NSPredicateEditorRowTemplate")]
unsafe impl NSCopying for NSPredicateEditorRowTemplate {}

#[cfg(feature = "AppKit_NSPredicateEditorRowTemplate")]
unsafe impl NSObjectProtocol for NSPredicateEditorRowTemplate {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPredicateEditorRowTemplate")]
    unsafe impl NSPredicateEditorRowTemplate {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(matchForPredicate:)]
        pub unsafe fn matchForPredicate(&self, predicate: &NSPredicate) -> c_double;

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other templateViews)]
        pub unsafe fn templateViews(&self) -> Id<NSArray<NSView>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: &NSPredicate);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPredicate"))]
        #[method_id(@__retain_semantics Other predicateWithSubpredicates:)]
        pub unsafe fn predicateWithSubpredicates(
            &self,
            subpredicates: Option<&NSArray<NSPredicate>>,
        ) -> Id<NSPredicate>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPredicate"))]
        #[method_id(@__retain_semantics Other displayableSubpredicatesOfPredicate:)]
        pub unsafe fn displayableSubpredicatesOfPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Option<Id<NSArray<NSPredicate>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSExpression",
            feature = "Foundation_NSNumber"
        ))]
        #[method_id(@__retain_semantics Init initWithLeftExpressions:rightExpressions:modifier:operators:options:)]
        pub unsafe fn initWithLeftExpressions_rightExpressions_modifier_operators_options(
            this: Option<Allocated<Self>>,
            left_expressions: &NSArray<NSExpression>,
            right_expressions: &NSArray<NSExpression>,
            modifier: NSComparisonPredicateModifier,
            operators: &NSArray<NSNumber>,
            options: NSUInteger,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSExpression",
            feature = "Foundation_NSNumber"
        ))]
        #[method_id(@__retain_semantics Init initWithLeftExpressions:rightExpressionAttributeType:modifier:operators:options:)]
        pub unsafe fn initWithLeftExpressions_rightExpressionAttributeType_modifier_operators_options(
            this: Option<Allocated<Self>>,
            left_expressions: &NSArray<NSExpression>,
            attribute_type: NSAttributeType,
            modifier: NSComparisonPredicateModifier,
            operators: &NSArray<NSNumber>,
            options: NSUInteger,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Init initWithCompoundTypes:)]
        pub unsafe fn initWithCompoundTypes(
            this: Option<Allocated<Self>>,
            compound_types: &NSArray<NSNumber>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSExpression"))]
        #[method_id(@__retain_semantics Other leftExpressions)]
        pub unsafe fn leftExpressions(&self) -> Option<Id<NSArray<NSExpression>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSExpression"))]
        #[method_id(@__retain_semantics Other rightExpressions)]
        pub unsafe fn rightExpressions(&self) -> Option<Id<NSArray<NSExpression>>>;

        #[method(rightExpressionAttributeType)]
        pub unsafe fn rightExpressionAttributeType(&self) -> NSAttributeType;

        #[method(modifier)]
        pub unsafe fn modifier(&self) -> NSComparisonPredicateModifier;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other operators)]
        pub unsafe fn operators(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other compoundTypes)]
        pub unsafe fn compoundTypes(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other templatesWithAttributeKeyPaths:inEntityDescription:)]
        pub unsafe fn templatesWithAttributeKeyPaths_inEntityDescription(
            key_paths: &NSArray<NSString>,
            entity_description: &NSEntityDescription,
        ) -> Id<NSArray<NSPredicateEditorRowTemplate>>;
    }
);
