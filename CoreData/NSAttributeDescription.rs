//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAttributeType {
        NSUndefinedAttributeType = 0,
        NSInteger16AttributeType = 100,
        NSInteger32AttributeType = 200,
        NSInteger64AttributeType = 300,
        NSDecimalAttributeType = 400,
        NSDoubleAttributeType = 500,
        NSFloatAttributeType = 600,
        NSStringAttributeType = 700,
        NSBooleanAttributeType = 800,
        NSDateAttributeType = 900,
        NSBinaryDataAttributeType = 1000,
        NSUUIDAttributeType = 1100,
        NSURIAttributeType = 1200,
        NSTransformableAttributeType = 1800,
        NSObjectIDAttributeType = 2000,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSAttributeDescription")]
    pub struct NSAttributeDescription;

    #[cfg(feature = "CoreData_NSAttributeDescription")]
    unsafe impl ClassType for NSAttributeDescription {
        #[inherits(NSObject)]
        type Super = NSPropertyDescription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSAttributeDescription")]
unsafe impl NSCoding for NSAttributeDescription {}

#[cfg(feature = "CoreData_NSAttributeDescription")]
unsafe impl NSCopying for NSAttributeDescription {}

#[cfg(feature = "CoreData_NSAttributeDescription")]
unsafe impl NSObjectProtocol for NSAttributeDescription {}

extern_methods!(
    #[cfg(feature = "CoreData_NSAttributeDescription")]
    unsafe impl NSAttributeDescription {
        #[method(attributeType)]
        pub unsafe fn attributeType(&self) -> NSAttributeType;

        #[method(setAttributeType:)]
        pub unsafe fn setAttributeType(&self, attribute_type: NSAttributeType);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attributeValueClassName)]
        pub unsafe fn attributeValueClassName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAttributeValueClassName:)]
        pub unsafe fn setAttributeValueClassName(
            &self,
            attribute_value_class_name: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other defaultValue)]
        pub unsafe fn defaultValue(&self) -> Option<Id<Object>>;

        #[method(setDefaultValue:)]
        pub unsafe fn setDefaultValue(&self, default_value: Option<&Object>);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other versionHash)]
        pub unsafe fn versionHash(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueTransformerName)]
        pub unsafe fn valueTransformerName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValueTransformerName:)]
        pub unsafe fn setValueTransformerName(&self, value_transformer_name: Option<&NSString>);

        #[method(allowsExternalBinaryDataStorage)]
        pub unsafe fn allowsExternalBinaryDataStorage(&self) -> bool;

        #[method(setAllowsExternalBinaryDataStorage:)]
        pub unsafe fn setAllowsExternalBinaryDataStorage(
            &self,
            allows_external_binary_data_storage: bool,
        );

        #[method(preservesValueInHistoryOnDeletion)]
        pub unsafe fn preservesValueInHistoryOnDeletion(&self) -> bool;

        #[method(setPreservesValueInHistoryOnDeletion:)]
        pub unsafe fn setPreservesValueInHistoryOnDeletion(
            &self,
            preserves_value_in_history_on_deletion: bool,
        );

        #[method(allowsCloudEncryption)]
        pub unsafe fn allowsCloudEncryption(&self) -> bool;

        #[method(setAllowsCloudEncryption:)]
        pub unsafe fn setAllowsCloudEncryption(&self, allows_cloud_encryption: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSAttributeDescription")]
    unsafe impl NSAttributeDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
