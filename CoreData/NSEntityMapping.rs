//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsentitymappingtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEntityMappingType(pub NSUInteger);
impl NSEntityMappingType {
    pub const NSUndefinedEntityMappingType: Self = Self(0x00);
    pub const NSCustomEntityMappingType: Self = Self(0x01);
    pub const NSAddEntityMappingType: Self = Self(0x02);
    pub const NSRemoveEntityMappingType: Self = Self(0x03);
    pub const NSCopyEntityMappingType: Self = Self(0x04);
    pub const NSTransformEntityMappingType: Self = Self(0x05);
}

unsafe impl Encode for NSEntityMappingType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSEntityMappingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsentitymapping?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEntityMapping;
);

unsafe impl NSObjectProtocol for NSEntityMapping {}

extern_methods!(
    unsafe impl NSEntityMapping {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(mappingType)]
        pub unsafe fn mappingType(&self) -> NSEntityMappingType;

        #[method(setMappingType:)]
        pub unsafe fn setMappingType(&self, mapping_type: NSEntityMappingType);

        #[method_id(@__retain_semantics Other sourceEntityName)]
        pub unsafe fn sourceEntityName(&self) -> Option<Retained<NSString>>;

        #[method(setSourceEntityName:)]
        pub unsafe fn setSourceEntityName(&self, source_entity_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other sourceEntityVersionHash)]
        pub unsafe fn sourceEntityVersionHash(&self) -> Option<Retained<NSData>>;

        #[method(setSourceEntityVersionHash:)]
        pub unsafe fn setSourceEntityVersionHash(
            &self,
            source_entity_version_hash: Option<&NSData>,
        );

        #[method_id(@__retain_semantics Other destinationEntityName)]
        pub unsafe fn destinationEntityName(&self) -> Option<Retained<NSString>>;

        #[method(setDestinationEntityName:)]
        pub unsafe fn setDestinationEntityName(&self, destination_entity_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other destinationEntityVersionHash)]
        pub unsafe fn destinationEntityVersionHash(&self) -> Option<Retained<NSData>>;

        #[method(setDestinationEntityVersionHash:)]
        pub unsafe fn setDestinationEntityVersionHash(
            &self,
            destination_entity_version_hash: Option<&NSData>,
        );

        #[cfg(feature = "NSPropertyMapping")]
        #[method_id(@__retain_semantics Other attributeMappings)]
        pub unsafe fn attributeMappings(&self) -> Option<Retained<NSArray<NSPropertyMapping>>>;

        #[cfg(feature = "NSPropertyMapping")]
        #[method(setAttributeMappings:)]
        pub unsafe fn setAttributeMappings(
            &self,
            attribute_mappings: Option<&NSArray<NSPropertyMapping>>,
        );

        #[cfg(feature = "NSPropertyMapping")]
        #[method_id(@__retain_semantics Other relationshipMappings)]
        pub unsafe fn relationshipMappings(&self) -> Option<Retained<NSArray<NSPropertyMapping>>>;

        #[cfg(feature = "NSPropertyMapping")]
        #[method(setRelationshipMappings:)]
        pub unsafe fn setRelationshipMappings(
            &self,
            relationship_mappings: Option<&NSArray<NSPropertyMapping>>,
        );

        #[method_id(@__retain_semantics Other sourceExpression)]
        pub unsafe fn sourceExpression(&self) -> Option<Retained<NSExpression>>;

        #[method(setSourceExpression:)]
        pub unsafe fn setSourceExpression(&self, source_expression: Option<&NSExpression>);

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);

        #[method_id(@__retain_semantics Other entityMigrationPolicyClassName)]
        pub unsafe fn entityMigrationPolicyClassName(&self) -> Option<Retained<NSString>>;

        #[method(setEntityMigrationPolicyClassName:)]
        pub unsafe fn setEntityMigrationPolicyClassName(
            &self,
            entity_migration_policy_class_name: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSEntityMapping {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
