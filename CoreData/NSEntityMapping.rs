//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSEntityMappingType {
        NSUndefinedEntityMappingType = 0x00,
        NSCustomEntityMappingType = 0x01,
        NSAddEntityMappingType = 0x02,
        NSRemoveEntityMappingType = 0x03,
        NSCopyEntityMappingType = 0x04,
        NSTransformEntityMappingType = 0x05,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSEntityMapping")]
    pub struct NSEntityMapping;

    #[cfg(feature = "CoreData_NSEntityMapping")]
    unsafe impl ClassType for NSEntityMapping {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSEntityMapping")]
unsafe impl NSObjectProtocol for NSEntityMapping {}

extern_methods!(
    #[cfg(feature = "CoreData_NSEntityMapping")]
    unsafe impl NSEntityMapping {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(mappingType)]
        pub unsafe fn mappingType(&self) -> NSEntityMappingType;

        #[method(setMappingType:)]
        pub unsafe fn setMappingType(&self, mapping_type: NSEntityMappingType);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sourceEntityName)]
        pub unsafe fn sourceEntityName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSourceEntityName:)]
        pub unsafe fn setSourceEntityName(&self, source_entity_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other sourceEntityVersionHash)]
        pub unsafe fn sourceEntityVersionHash(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setSourceEntityVersionHash:)]
        pub unsafe fn setSourceEntityVersionHash(
            &self,
            source_entity_version_hash: Option<&NSData>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other destinationEntityName)]
        pub unsafe fn destinationEntityName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDestinationEntityName:)]
        pub unsafe fn setDestinationEntityName(&self, destination_entity_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other destinationEntityVersionHash)]
        pub unsafe fn destinationEntityVersionHash(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setDestinationEntityVersionHash:)]
        pub unsafe fn setDestinationEntityVersionHash(
            &self,
            destination_entity_version_hash: Option<&NSData>,
        );

        #[cfg(all(feature = "CoreData_NSPropertyMapping", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other attributeMappings)]
        pub unsafe fn attributeMappings(&self) -> Option<Id<NSArray<NSPropertyMapping>>>;

        #[cfg(all(feature = "CoreData_NSPropertyMapping", feature = "Foundation_NSArray"))]
        #[method(setAttributeMappings:)]
        pub unsafe fn setAttributeMappings(
            &self,
            attribute_mappings: Option<&NSArray<NSPropertyMapping>>,
        );

        #[cfg(all(feature = "CoreData_NSPropertyMapping", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other relationshipMappings)]
        pub unsafe fn relationshipMappings(&self) -> Option<Id<NSArray<NSPropertyMapping>>>;

        #[cfg(all(feature = "CoreData_NSPropertyMapping", feature = "Foundation_NSArray"))]
        #[method(setRelationshipMappings:)]
        pub unsafe fn setRelationshipMappings(
            &self,
            relationship_mappings: Option<&NSArray<NSPropertyMapping>>,
        );

        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other sourceExpression)]
        pub unsafe fn sourceExpression(&self) -> Option<Id<NSExpression>>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method(setSourceExpression:)]
        pub unsafe fn setSourceExpression(&self, source_expression: Option<&NSExpression>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other entityMigrationPolicyClassName)]
        pub unsafe fn entityMigrationPolicyClassName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setEntityMigrationPolicyClassName:)]
        pub unsafe fn setEntityMigrationPolicyClassName(
            &self,
            entity_migration_policy_class_name: Option<&NSString>,
        );
    }
);
