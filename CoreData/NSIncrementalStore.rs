//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsincrementalstore?language=objc)
    #[unsafe(super(NSPersistentStore, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStore")]
    pub struct NSIncrementalStore;
);

#[cfg(feature = "NSPersistentStore")]
unsafe impl NSObjectProtocol for NSIncrementalStore {}

extern_methods!(
    #[cfg(feature = "NSPersistentStore")]
    unsafe impl NSIncrementalStore {
        #[method(loadMetadata:_)]
        pub unsafe fn loadMetadata(&self) -> Result<(), Retained<NSError>>;

        #[cfg(all(
            feature = "NSManagedObjectContext",
            feature = "NSPersistentStoreRequest"
        ))]
        #[method_id(@__retain_semantics Other executeRequest:withContext:error:_)]
        pub unsafe fn executeRequest_withContext_error(
            &self,
            request: &NSPersistentStoreRequest,
            context: Option<&NSManagedObjectContext>,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(all(
            feature = "NSIncrementalStoreNode",
            feature = "NSManagedObjectContext",
            feature = "NSManagedObjectID"
        ))]
        #[method_id(@__retain_semantics New newValuesForObjectWithID:withContext:error:_)]
        pub unsafe fn newValuesForObjectWithID_withContext_error(
            &self,
            object_id: &NSManagedObjectID,
            context: &NSManagedObjectContext,
        ) -> Result<Retained<NSIncrementalStoreNode>, Retained<NSError>>;

        #[cfg(all(
            feature = "NSManagedObjectContext",
            feature = "NSManagedObjectID",
            feature = "NSPropertyDescription",
            feature = "NSRelationshipDescription"
        ))]
        #[method_id(@__retain_semantics New newValueForRelationship:forObjectWithID:withContext:error:_)]
        pub unsafe fn newValueForRelationship_forObjectWithID_withContext_error(
            &self,
            relationship: &NSRelationshipDescription,
            object_id: &NSManagedObjectID,
            context: Option<&NSManagedObjectContext>,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other identifierForNewStoreAtURL:)]
        pub unsafe fn identifierForNewStoreAtURL(store_url: &NSURL) -> Retained<AnyObject>;

        #[cfg(all(feature = "NSManagedObject", feature = "NSManagedObjectID"))]
        #[method_id(@__retain_semantics Other obtainPermanentIDsForObjects:error:_)]
        pub unsafe fn obtainPermanentIDsForObjects_error(
            &self,
            array: &NSArray<NSManagedObject>,
        ) -> Result<Retained<NSArray<NSManagedObjectID>>, Retained<NSError>>;

        #[cfg(feature = "NSManagedObjectID")]
        #[method(managedObjectContextDidRegisterObjectsWithIDs:)]
        pub unsafe fn managedObjectContextDidRegisterObjectsWithIDs(
            &self,
            object_i_ds: &NSArray<NSManagedObjectID>,
        );

        #[cfg(feature = "NSManagedObjectID")]
        #[method(managedObjectContextDidUnregisterObjectsWithIDs:)]
        pub unsafe fn managedObjectContextDidUnregisterObjectsWithIDs(
            &self,
            object_i_ds: &NSArray<NSManagedObjectID>,
        );

        #[cfg(all(feature = "NSEntityDescription", feature = "NSManagedObjectID"))]
        #[method_id(@__retain_semantics New newObjectIDForEntity:referenceObject:)]
        pub unsafe fn newObjectIDForEntity_referenceObject(
            &self,
            entity: &NSEntityDescription,
            data: &AnyObject,
        ) -> Retained<NSManagedObjectID>;

        #[cfg(feature = "NSManagedObjectID")]
        #[method_id(@__retain_semantics Other referenceObjectForObjectID:)]
        pub unsafe fn referenceObjectForObjectID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> Retained<AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSPersistentStore`
    #[cfg(feature = "NSPersistentStore")]
    unsafe impl NSIncrementalStore {
        #[cfg(feature = "NSPersistentStoreCoordinator")]
        #[method_id(@__retain_semantics Init initWithPersistentStoreCoordinator:configurationName:URL:options:)]
        pub unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options(
            this: Allocated<Self>,
            root: Option<&NSPersistentStoreCoordinator>,
            name: Option<&NSString>,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStore")]
    unsafe impl NSIncrementalStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
