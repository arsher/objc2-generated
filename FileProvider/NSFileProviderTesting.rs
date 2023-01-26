//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSFileProviderTestingOperationType {
        NSFileProviderTestingOperationTypeIngestion = 0,
        NSFileProviderTestingOperationTypeLookup = 1,
        NSFileProviderTestingOperationTypeCreation = 2,
        NSFileProviderTestingOperationTypeModification = 3,
        NSFileProviderTestingOperationTypeDeletion = 4,
        NSFileProviderTestingOperationTypeContentFetch = 5,
        NSFileProviderTestingOperationTypeChildrenEnumeration = 6,
        NSFileProviderTestingOperationTypeCollisionResolution = 7,
    }
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingOperation: NSObjectProtocol {
        #[method(type)]
        unsafe fn r#type(&self) -> NSFileProviderTestingOperationType;

        #[method_id(@__retain_semantics Other asIngestion)]
        unsafe fn asIngestion(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFileProviderTestingIngestion>, Shared>>;

        #[method_id(@__retain_semantics Other asLookup)]
        unsafe fn asLookup(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFileProviderTestingLookup>, Shared>>;

        #[method_id(@__retain_semantics Other asCreation)]
        unsafe fn asCreation(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFileProviderTestingCreation>, Shared>>;

        #[method_id(@__retain_semantics Other asModification)]
        unsafe fn asModification(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFileProviderTestingModification>, Shared>>;

        #[method_id(@__retain_semantics Other asDeletion)]
        unsafe fn asDeletion(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFileProviderTestingDeletion>, Shared>>;

        #[method_id(@__retain_semantics Other asContentFetch)]
        unsafe fn asContentFetch(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFileProviderTestingContentFetch>, Shared>>;

        #[method_id(@__retain_semantics Other asChildrenEnumeration)]
        unsafe fn asChildrenEnumeration(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFileProviderTestingChildrenEnumeration>, Shared>>;

        #[method_id(@__retain_semantics Other asCollisionResolution)]
        unsafe fn asCollisionResolution(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFileProviderTestingCollisionResolution>, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingOperation {}
);

extern_methods!(
    /// TestingModeInteractive
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other listAvailableTestingOperationsWithError:_)]
        pub unsafe fn listAvailableTestingOperationsWithError(
            &self,
        ) -> Result<
            Id<NSArray<ProtocolObject<dyn NSFileProviderTestingOperation>>, Shared>,
            Id<NSError, Shared>,
        >;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other runTestingOperations:error:_)]
        pub unsafe fn runTestingOperations_error(
            &self,
            operations: &NSArray<ProtocolObject<dyn NSFileProviderTestingOperation>>,
        ) -> Result<
            Id<NSDictionary<ProtocolObject<dyn NSFileProviderTestingOperation>, NSError>, Shared>,
            Id<NSError, Shared>,
        >;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFileProviderTestingOperationSide {
        NSFileProviderTestingOperationSideDisk = 0,
        NSFileProviderTestingOperationSideFileProvider = 1,
    }
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingIngestion: NSFileProviderTestingOperation {
        #[method(side)]
        unsafe fn side(&self) -> NSFileProviderTestingOperationSide;

        #[method_id(@__retain_semantics Other itemIdentifier)]
        unsafe fn itemIdentifier(&self) -> Id<NSFileProviderItemIdentifier, Shared>;

        #[method_id(@__retain_semantics Other item)]
        unsafe fn item(&self) -> Option<Id<NSFileProviderItem, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingIngestion {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingLookup: NSFileProviderTestingOperation {
        #[method(side)]
        unsafe fn side(&self) -> NSFileProviderTestingOperationSide;

        #[method_id(@__retain_semantics Other itemIdentifier)]
        unsafe fn itemIdentifier(&self) -> Id<NSFileProviderItemIdentifier, Shared>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingLookup {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingCreation: NSFileProviderTestingOperation {
        #[method(targetSide)]
        unsafe fn targetSide(&self) -> NSFileProviderTestingOperationSide;

        #[method_id(@__retain_semantics Other sourceItem)]
        unsafe fn sourceItem(&self) -> Id<NSFileProviderItem, Shared>;

        #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
        #[method_id(@__retain_semantics Other domainVersion)]
        unsafe fn domainVersion(&self) -> Option<Id<NSFileProviderDomainVersion, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingCreation {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingModification:
        NSFileProviderTestingOperation
    {
        #[method(targetSide)]
        unsafe fn targetSide(&self) -> NSFileProviderTestingOperationSide;

        #[method_id(@__retain_semantics Other sourceItem)]
        unsafe fn sourceItem(&self) -> Id<NSFileProviderItem, Shared>;

        #[method_id(@__retain_semantics Other targetItemIdentifier)]
        unsafe fn targetItemIdentifier(&self) -> Id<NSFileProviderItemIdentifier, Shared>;

        #[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
        #[method_id(@__retain_semantics Other targetItemBaseVersion)]
        unsafe fn targetItemBaseVersion(&self) -> Id<NSFileProviderItemVersion, Shared>;

        #[method(changedFields)]
        unsafe fn changedFields(&self) -> NSFileProviderItemFields;

        #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
        #[method_id(@__retain_semantics Other domainVersion)]
        unsafe fn domainVersion(&self) -> Option<Id<NSFileProviderDomainVersion, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingModification {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingDeletion: NSFileProviderTestingOperation {
        #[method(targetSide)]
        unsafe fn targetSide(&self) -> NSFileProviderTestingOperationSide;

        #[method_id(@__retain_semantics Other sourceItemIdentifier)]
        unsafe fn sourceItemIdentifier(&self) -> Id<NSFileProviderItemIdentifier, Shared>;

        #[method_id(@__retain_semantics Other targetItemIdentifier)]
        unsafe fn targetItemIdentifier(&self) -> Id<NSFileProviderItemIdentifier, Shared>;

        #[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
        #[method_id(@__retain_semantics Other targetItemBaseVersion)]
        unsafe fn targetItemBaseVersion(&self) -> Id<NSFileProviderItemVersion, Shared>;

        #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
        #[method_id(@__retain_semantics Other domainVersion)]
        unsafe fn domainVersion(&self) -> Option<Id<NSFileProviderDomainVersion, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingDeletion {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingContentFetch:
        NSFileProviderTestingOperation
    {
        #[method(side)]
        unsafe fn side(&self) -> NSFileProviderTestingOperationSide;

        #[method_id(@__retain_semantics Other itemIdentifier)]
        unsafe fn itemIdentifier(&self) -> Id<NSFileProviderItemIdentifier, Shared>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingContentFetch {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingChildrenEnumeration:
        NSFileProviderTestingOperation
    {
        #[method(side)]
        unsafe fn side(&self) -> NSFileProviderTestingOperationSide;

        #[method_id(@__retain_semantics Other itemIdentifier)]
        unsafe fn itemIdentifier(&self) -> Id<NSFileProviderItemIdentifier, Shared>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingChildrenEnumeration {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderTestingCollisionResolution:
        NSFileProviderTestingOperation
    {
        #[method(side)]
        unsafe fn side(&self) -> NSFileProviderTestingOperationSide;

        #[method_id(@__retain_semantics Other renamedItem)]
        unsafe fn renamedItem(&self) -> Id<NSFileProviderItem, Shared>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderTestingCollisionResolution {}
);