//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSValueTransformerName = NSString;
);

extern_static!(NSNegateBooleanTransformerName: &'static NSValueTransformerName);

extern_static!(NSIsNilTransformerName: &'static NSValueTransformerName);

extern_static!(NSIsNotNilTransformerName: &'static NSValueTransformerName);

extern_static!(NSUnarchiveFromDataTransformerName: &'static NSValueTransformerName);

extern_static!(NSKeyedUnarchiveFromDataTransformerName: &'static NSValueTransformerName);

extern_static!(NSSecureUnarchiveFromDataTransformerName: &'static NSValueTransformerName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSValueTransformer")]
    pub struct NSValueTransformer;

    #[cfg(feature = "Foundation_NSValueTransformer")]
    unsafe impl ClassType for NSValueTransformer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSValueTransformer")]
unsafe impl NSObjectProtocol for NSValueTransformer {}

extern_methods!(
    #[cfg(feature = "Foundation_NSValueTransformer")]
    unsafe impl NSValueTransformer {
        #[method(setValueTransformer:forName:)]
        pub unsafe fn setValueTransformer_forName(
            transformer: Option<&NSValueTransformer>,
            name: &NSValueTransformerName,
        );

        #[method_id(@__retain_semantics Other valueTransformerForName:)]
        pub unsafe fn valueTransformerForName(
            name: &NSValueTransformerName,
        ) -> Option<Id<NSValueTransformer>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other valueTransformerNames)]
        pub unsafe fn valueTransformerNames() -> Id<NSArray<NSValueTransformerName>>;

        #[method(transformedValueClass)]
        pub unsafe fn transformedValueClass() -> &'static Class;

        #[method(allowsReverseTransformation)]
        pub unsafe fn allowsReverseTransformation() -> bool;

        #[method_id(@__retain_semantics Other transformedValue:)]
        pub unsafe fn transformedValue(&self, value: Option<&Object>) -> Option<Id<Object>>;

        #[method_id(@__retain_semantics Other reverseTransformedValue:)]
        pub unsafe fn reverseTransformedValue(&self, value: Option<&Object>) -> Option<Id<Object>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSSecureUnarchiveFromDataTransformer")]
    pub struct NSSecureUnarchiveFromDataTransformer;

    #[cfg(feature = "Foundation_NSSecureUnarchiveFromDataTransformer")]
    unsafe impl ClassType for NSSecureUnarchiveFromDataTransformer {
        #[inherits(NSObject)]
        type Super = NSValueTransformer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSSecureUnarchiveFromDataTransformer")]
unsafe impl NSObjectProtocol for NSSecureUnarchiveFromDataTransformer {}

extern_methods!(
    #[cfg(feature = "Foundation_NSSecureUnarchiveFromDataTransformer")]
    unsafe impl NSSecureUnarchiveFromDataTransformer {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allowedTopLevelClasses)]
        pub unsafe fn allowedTopLevelClasses() -> Id<NSArray<TodoClass>>;
    }
);
