//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static NSManagedObjectContextWillSaveNotification: &'static NSString;
}

extern "C" {
    pub static NSManagedObjectContextDidSaveNotification: &'static NSString;
}

extern "C" {
    pub static NSManagedObjectContextObjectsDidChangeNotification: &'static NSString;
}

extern "C" {
    pub static NSManagedObjectContextDidSaveObjectIDsNotification: &'static NSString;
}

extern "C" {
    pub static NSManagedObjectContextDidMergeChangesObjectIDsNotification: &'static NSString;
}

extern "C" {
    pub static NSInsertedObjectsKey: &'static NSString;
}

extern "C" {
    pub static NSUpdatedObjectsKey: &'static NSString;
}

extern "C" {
    pub static NSDeletedObjectsKey: &'static NSString;
}

extern "C" {
    pub static NSRefreshedObjectsKey: &'static NSString;
}

extern "C" {
    pub static NSInvalidatedObjectsKey: &'static NSString;
}

extern "C" {
    pub static NSManagedObjectContextQueryGenerationKey: &'static NSString;
}

extern "C" {
    pub static NSInvalidatedAllObjectsKey: &'static NSString;
}

extern "C" {
    pub static NSInsertedObjectIDsKey: &'static NSString;
}

extern "C" {
    pub static NSUpdatedObjectIDsKey: &'static NSString;
}

extern "C" {
    pub static NSDeletedObjectIDsKey: &'static NSString;
}

extern "C" {
    pub static NSRefreshedObjectIDsKey: &'static NSString;
}

extern "C" {
    pub static NSInvalidatedObjectIDsKey: &'static NSString;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSManagedObjectContextConcurrencyType(pub NSUInteger);
impl NSManagedObjectContextConcurrencyType {
    #[deprecated = "Use another NSManagedObjectContextConcurrencyType"]
    pub const NSConfinementConcurrencyType: Self = Self(0x00);
    pub const NSPrivateQueueConcurrencyType: Self = Self(0x01);
    pub const NSMainQueueConcurrencyType: Self = Self(0x02);
}

unsafe impl Encode for NSManagedObjectContextConcurrencyType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSManagedObjectContextConcurrencyType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSManagedObjectContext;

    unsafe impl ClassType for NSManagedObjectContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSManagedObjectContext {}

unsafe impl NSLocking for NSManagedObjectContext {}

unsafe impl NSObjectProtocol for NSManagedObjectContext {}

extern_methods!(
    unsafe impl NSManagedObjectContext {
        #[deprecated = "Use -initWithConcurrencyType: instead"]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated = "Use -initWithConcurrencyType: instead"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithConcurrencyType:)]
        pub unsafe fn initWithConcurrencyType(
            this: Allocated<Self>,
            ct: NSManagedObjectContextConcurrencyType,
        ) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method(performBlock:)]
        pub unsafe fn performBlock(&self, block: &block2::Block<dyn Fn()>);

        #[cfg(feature = "block2")]
        #[method(performBlockAndWait:)]
        pub unsafe fn performBlockAndWait(&self, block: &block2::Block<dyn Fn() + '_>);

        #[cfg(feature = "NSPersistentStoreCoordinator")]
        #[method_id(@__retain_semantics Other persistentStoreCoordinator)]
        pub unsafe fn persistentStoreCoordinator(&self)
            -> Option<Id<NSPersistentStoreCoordinator>>;

        #[cfg(feature = "NSPersistentStoreCoordinator")]
        #[method(setPersistentStoreCoordinator:)]
        pub unsafe fn setPersistentStoreCoordinator(
            &self,
            persistent_store_coordinator: Option<&NSPersistentStoreCoordinator>,
        );

        #[method_id(@__retain_semantics Other parentContext)]
        pub unsafe fn parentContext(&self) -> Option<Id<NSManagedObjectContext>>;

        #[method(setParentContext:)]
        pub unsafe fn setParentContext(&self, parent_context: Option<&NSManagedObjectContext>);

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<NSUndoManager>>;

        #[method(setUndoManager:)]
        pub unsafe fn setUndoManager(&self, undo_manager: Option<&NSUndoManager>);

        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Id<NSMutableDictionary>;

        #[method(concurrencyType)]
        pub unsafe fn concurrencyType(&self) -> NSManagedObjectContextConcurrencyType;

        #[cfg(all(feature = "NSManagedObject", feature = "NSManagedObjectID"))]
        #[method_id(@__retain_semantics Other objectRegisteredForID:)]
        pub unsafe fn objectRegisteredForID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> Option<Id<NSManagedObject>>;

        #[cfg(all(feature = "NSManagedObject", feature = "NSManagedObjectID"))]
        #[method_id(@__retain_semantics Other objectWithID:)]
        pub unsafe fn objectWithID(&self, object_id: &NSManagedObjectID) -> Id<NSManagedObject>;

        #[cfg(all(feature = "NSManagedObject", feature = "NSManagedObjectID"))]
        #[method_id(@__retain_semantics Other existingObjectWithID:error:_)]
        pub unsafe fn existingObjectWithID_error(
            &self,
            object_id: &NSManagedObjectID,
        ) -> Result<Id<NSManagedObject>, Id<NSError>>;

        #[cfg(all(feature = "NSFetchRequest", feature = "NSPersistentStoreRequest"))]
        #[method_id(@__retain_semantics Other executeFetchRequest:error:_)]
        pub unsafe fn executeFetchRequest_error(
            &self,
            request: &NSFetchRequest,
        ) -> Result<Id<NSArray>, Id<NSError>>;

        #[cfg(all(
            feature = "NSPersistentStoreRequest",
            feature = "NSPersistentStoreResult"
        ))]
        #[method_id(@__retain_semantics Other executeRequest:error:_)]
        pub unsafe fn executeRequest_error(
            &self,
            request: &NSPersistentStoreRequest,
        ) -> Result<Id<NSPersistentStoreResult>, Id<NSError>>;

        #[cfg(feature = "NSManagedObject")]
        #[method(insertObject:)]
        pub unsafe fn insertObject(&self, object: &NSManagedObject);

        #[cfg(feature = "NSManagedObject")]
        #[method(deleteObject:)]
        pub unsafe fn deleteObject(&self, object: &NSManagedObject);

        #[cfg(feature = "NSManagedObject")]
        #[method(refreshObject:mergeChanges:)]
        pub unsafe fn refreshObject_mergeChanges(&self, object: &NSManagedObject, flag: bool);

        #[cfg(feature = "NSManagedObject")]
        #[method(detectConflictsForObject:)]
        pub unsafe fn detectConflictsForObject(&self, object: &NSManagedObject);

        #[method(observeValueForKeyPath:ofObject:change:context:)]
        pub unsafe fn observeValueForKeyPath_ofObject_change_context(
            &self,
            key_path: Option<&NSString>,
            object: Option<&AnyObject>,
            change: Option<&NSDictionary<NSString, AnyObject>>,
            context: *mut c_void,
        );

        #[method(processPendingChanges)]
        pub unsafe fn processPendingChanges(&self);

        #[cfg(feature = "NSPersistentStore")]
        #[method(assignObject:toPersistentStore:)]
        pub unsafe fn assignObject_toPersistentStore(
            &self,
            object: &AnyObject,
            store: &NSPersistentStore,
        );

        #[cfg(feature = "NSManagedObject")]
        #[method_id(@__retain_semantics Other insertedObjects)]
        pub unsafe fn insertedObjects(&self) -> Id<NSSet<NSManagedObject>>;

        #[cfg(feature = "NSManagedObject")]
        #[method_id(@__retain_semantics Other updatedObjects)]
        pub unsafe fn updatedObjects(&self) -> Id<NSSet<NSManagedObject>>;

        #[cfg(feature = "NSManagedObject")]
        #[method_id(@__retain_semantics Other deletedObjects)]
        pub unsafe fn deletedObjects(&self) -> Id<NSSet<NSManagedObject>>;

        #[cfg(feature = "NSManagedObject")]
        #[method_id(@__retain_semantics Other registeredObjects)]
        pub unsafe fn registeredObjects(&self) -> Id<NSSet<NSManagedObject>>;

        #[method(undo)]
        pub unsafe fn undo(&self);

        #[method(redo)]
        pub unsafe fn redo(&self);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(rollback)]
        pub unsafe fn rollback(&self);

        #[method(save:_)]
        pub unsafe fn save(&self) -> Result<(), Id<NSError>>;

        #[method(refreshAllObjects)]
        pub unsafe fn refreshAllObjects(&self);

        #[deprecated = "Use a queue style context and -performBlockAndWait: instead"]
        #[method(lock)]
        pub unsafe fn lock(&self);

        #[deprecated = "Use a queue style context and -performBlockAndWait: instead"]
        #[method(unlock)]
        pub unsafe fn unlock(&self);

        #[deprecated = "Use a queue style context and -performBlock: instead"]
        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[method(propagatesDeletesAtEndOfEvent)]
        pub unsafe fn propagatesDeletesAtEndOfEvent(&self) -> bool;

        #[method(setPropagatesDeletesAtEndOfEvent:)]
        pub unsafe fn setPropagatesDeletesAtEndOfEvent(
            &self,
            propagates_deletes_at_end_of_event: bool,
        );

        #[method(retainsRegisteredObjects)]
        pub unsafe fn retainsRegisteredObjects(&self) -> bool;

        #[method(setRetainsRegisteredObjects:)]
        pub unsafe fn setRetainsRegisteredObjects(&self, retains_registered_objects: bool);

        #[method(shouldDeleteInaccessibleFaults)]
        pub unsafe fn shouldDeleteInaccessibleFaults(&self) -> bool;

        #[method(setShouldDeleteInaccessibleFaults:)]
        pub unsafe fn setShouldDeleteInaccessibleFaults(
            &self,
            should_delete_inaccessible_faults: bool,
        );

        #[cfg(all(
            feature = "NSManagedObject",
            feature = "NSManagedObjectID",
            feature = "NSPropertyDescription"
        ))]
        #[method(shouldHandleInaccessibleFault:forObjectID:triggeredByProperty:)]
        pub unsafe fn shouldHandleInaccessibleFault_forObjectID_triggeredByProperty(
            &self,
            fault: &NSManagedObject,
            oid: &NSManagedObjectID,
            property: Option<&NSPropertyDescription>,
        ) -> bool;

        #[method(stalenessInterval)]
        pub unsafe fn stalenessInterval(&self) -> NSTimeInterval;

        #[method(setStalenessInterval:)]
        pub unsafe fn setStalenessInterval(&self, staleness_interval: NSTimeInterval);

        #[method_id(@__retain_semantics Other mergePolicy)]
        pub unsafe fn mergePolicy(&self) -> Id<AnyObject>;

        #[method(setMergePolicy:)]
        pub unsafe fn setMergePolicy(&self, merge_policy: &AnyObject);

        #[cfg(feature = "NSManagedObject")]
        #[method(obtainPermanentIDsForObjects:error:_)]
        pub unsafe fn obtainPermanentIDsForObjects_error(
            &self,
            objects: &NSArray<NSManagedObject>,
        ) -> Result<(), Id<NSError>>;

        #[method(mergeChangesFromContextDidSaveNotification:)]
        pub unsafe fn mergeChangesFromContextDidSaveNotification(
            &self,
            notification: &NSNotification,
        );

        #[method(mergeChangesFromRemoteContextSave:intoContexts:)]
        pub unsafe fn mergeChangesFromRemoteContextSave_intoContexts(
            change_notification_data: &NSDictionary,
            contexts: &NSArray<NSManagedObjectContext>,
        );

        #[cfg(feature = "NSQueryGenerationToken")]
        #[method_id(@__retain_semantics Other queryGenerationToken)]
        pub unsafe fn queryGenerationToken(&self) -> Option<Id<NSQueryGenerationToken>>;

        #[cfg(feature = "NSQueryGenerationToken")]
        #[method(setQueryGenerationFromToken:error:_)]
        pub unsafe fn setQueryGenerationFromToken_error(
            &self,
            generation: Option<&NSQueryGenerationToken>,
        ) -> Result<(), Id<NSError>>;

        #[method(automaticallyMergesChangesFromParent)]
        pub unsafe fn automaticallyMergesChangesFromParent(&self) -> bool;

        #[method(setAutomaticallyMergesChangesFromParent:)]
        pub unsafe fn setAutomaticallyMergesChangesFromParent(
            &self,
            automatically_merges_changes_from_parent: bool,
        );

        #[method_id(@__retain_semantics Other transactionAuthor)]
        pub unsafe fn transactionAuthor(&self) -> Option<Id<NSString>>;

        #[method(setTransactionAuthor:)]
        pub unsafe fn setTransactionAuthor(&self, transaction_author: Option<&NSString>);
    }
);
