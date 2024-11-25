//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsatomicstore?language=objc)
    #[unsafe(super(NSPersistentStore, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStore")]
    pub struct NSAtomicStore;
);

#[cfg(feature = "NSPersistentStore")]
unsafe impl NSObjectProtocol for NSAtomicStore {}

extern_methods!(
    #[cfg(feature = "NSPersistentStore")]
    unsafe impl NSAtomicStore {
        #[cfg(feature = "NSPersistentStoreCoordinator")]
        #[method_id(@__retain_semantics Init initWithPersistentStoreCoordinator:configurationName:URL:options:)]
        pub unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options(
            this: Allocated<Self>,
            coordinator: Option<&NSPersistentStoreCoordinator>,
            configuration_name: Option<&NSString>,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Retained<Self>;

        #[method(load:_)]
        pub unsafe fn load(&self) -> Result<(), Retained<NSError>>;

        #[method(save:_)]
        pub unsafe fn save(&self) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "NSAtomicStoreCacheNode", feature = "NSManagedObject"))]
        #[method_id(@__retain_semantics New newCacheNodeForManagedObject:)]
        pub unsafe fn newCacheNodeForManagedObject(
            &self,
            managed_object: &NSManagedObject,
        ) -> Retained<NSAtomicStoreCacheNode>;

        #[cfg(all(feature = "NSAtomicStoreCacheNode", feature = "NSManagedObject"))]
        #[method(updateCacheNode:fromManagedObject:)]
        pub unsafe fn updateCacheNode_fromManagedObject(
            &self,
            node: &NSAtomicStoreCacheNode,
            managed_object: &NSManagedObject,
        );

        #[cfg(feature = "NSAtomicStoreCacheNode")]
        #[method_id(@__retain_semantics Other cacheNodes)]
        pub unsafe fn cacheNodes(&self) -> Retained<NSSet<NSAtomicStoreCacheNode>>;

        #[cfg(feature = "NSAtomicStoreCacheNode")]
        #[method(addCacheNodes:)]
        pub unsafe fn addCacheNodes(&self, cache_nodes: &NSSet<NSAtomicStoreCacheNode>);

        #[cfg(feature = "NSAtomicStoreCacheNode")]
        #[method(willRemoveCacheNodes:)]
        pub unsafe fn willRemoveCacheNodes(&self, cache_nodes: &NSSet<NSAtomicStoreCacheNode>);

        #[cfg(all(feature = "NSAtomicStoreCacheNode", feature = "NSManagedObjectID"))]
        #[method_id(@__retain_semantics Other cacheNodeForObjectID:)]
        pub unsafe fn cacheNodeForObjectID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> Option<Retained<NSAtomicStoreCacheNode>>;

        #[cfg(all(feature = "NSEntityDescription", feature = "NSManagedObjectID"))]
        #[method_id(@__retain_semantics Other objectIDForEntity:referenceObject:)]
        pub unsafe fn objectIDForEntity_referenceObject(
            &self,
            entity: &NSEntityDescription,
            data: &AnyObject,
        ) -> Retained<NSManagedObjectID>;

        #[cfg(feature = "NSManagedObject")]
        #[method_id(@__retain_semantics New newReferenceObjectForManagedObject:)]
        pub unsafe fn newReferenceObjectForManagedObject(
            &self,
            managed_object: &NSManagedObject,
        ) -> Retained<AnyObject>;

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
    unsafe impl NSAtomicStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStore")]
    unsafe impl NSAtomicStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
