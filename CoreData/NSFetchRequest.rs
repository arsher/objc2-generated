//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFetchRequestResultType {
        NSManagedObjectResultType = 0x00,
        NSManagedObjectIDResultType = 0x01,
        NSDictionaryResultType = 0x02,
        NSCountResultType = 0x04,
    }
);

extern_protocol!(
    pub unsafe trait NSFetchRequestResult: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn NSFetchRequestResult {}
);

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "Foundation_NSNumber")]
    unsafe impl NSNumber {}
);

#[cfg(feature = "Foundation_NSNumber")]
unsafe impl NSFetchRequestResult for NSNumber {}

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "Foundation_NSDictionary")]
    unsafe impl NSDictionary {}
);

#[cfg(feature = "Foundation_NSDictionary")]
unsafe impl NSFetchRequestResult for NSDictionary {}

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "CoreData_NSManagedObject")]
    unsafe impl NSManagedObject {}
);

#[cfg(feature = "CoreData_NSManagedObject")]
unsafe impl NSFetchRequestResult for NSManagedObject {}

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "CoreData_NSManagedObjectID")]
    unsafe impl NSManagedObjectID {}
);

#[cfg(feature = "CoreData_NSManagedObjectID")]
unsafe impl NSFetchRequestResult for NSManagedObjectID {}

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSFetchRequest")]
    pub struct NSFetchRequest<ResultType: ?Sized = AnyObject> {
        __superclass: NSPersistentStoreRequest,
        _inner0: PhantomData<*mut ResultType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "CoreData_NSFetchRequest")]
    unsafe impl<ResultType: ?Sized + Message> ClassType for NSFetchRequest<ResultType> {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "CoreData_NSFetchRequest")]
unsafe impl<ResultType: ?Sized + NSCoding> NSCoding for NSFetchRequest<ResultType> {}

#[cfg(feature = "CoreData_NSFetchRequest")]
unsafe impl<ResultType: ?Sized + IsIdCloneable> NSCopying for NSFetchRequest<ResultType> {}

#[cfg(feature = "CoreData_NSFetchRequest")]
unsafe impl<ResultType: ?Sized> NSObjectProtocol for NSFetchRequest<ResultType> {}

extern_methods!(
    #[cfg(feature = "CoreData_NSFetchRequest")]
    unsafe impl<ResultType: Message> NSFetchRequest<ResultType> {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fetchRequestWithEntityName:)]
        pub unsafe fn fetchRequestWithEntityName(entity_name: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithEntityName:)]
        pub unsafe fn initWithEntityName(
            this: Option<Allocated<Self>>,
            entity_name: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other execute:_)]
        pub unsafe fn execute(&self) -> Result<Id<NSArray<ResultType>>, Id<NSError>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Option<Id<NSEntityDescription>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method(setEntity:)]
        pub unsafe fn setEntity(&self, entity: Option<&NSEntityDescription>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other entityName)]
        pub unsafe fn entityName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<NSPredicate>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Id<NSArray<NSSortDescriptor>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(
            &self,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
        );

        #[method(fetchLimit)]
        pub unsafe fn fetchLimit(&self) -> NSUInteger;

        #[method(setFetchLimit:)]
        pub unsafe fn setFetchLimit(&self, fetch_limit: NSUInteger);

        #[cfg(all(feature = "CoreData_NSPersistentStore", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other affectedStores)]
        pub unsafe fn affectedStores(&self) -> Option<Id<NSArray<NSPersistentStore>>>;

        #[cfg(all(feature = "CoreData_NSPersistentStore", feature = "Foundation_NSArray"))]
        #[method(setAffectedStores:)]
        pub unsafe fn setAffectedStores(
            &self,
            affected_stores: Option<&NSArray<NSPersistentStore>>,
        );

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSFetchRequestResultType;

        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSFetchRequestResultType);

        #[method(includesSubentities)]
        pub unsafe fn includesSubentities(&self) -> bool;

        #[method(setIncludesSubentities:)]
        pub unsafe fn setIncludesSubentities(&self, includes_subentities: bool);

        #[method(includesPropertyValues)]
        pub unsafe fn includesPropertyValues(&self) -> bool;

        #[method(setIncludesPropertyValues:)]
        pub unsafe fn setIncludesPropertyValues(&self, includes_property_values: bool);

        #[method(returnsObjectsAsFaults)]
        pub unsafe fn returnsObjectsAsFaults(&self) -> bool;

        #[method(setReturnsObjectsAsFaults:)]
        pub unsafe fn setReturnsObjectsAsFaults(&self, returns_objects_as_faults: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other relationshipKeyPathsForPrefetching)]
        pub unsafe fn relationshipKeyPathsForPrefetching(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setRelationshipKeyPathsForPrefetching:)]
        pub unsafe fn setRelationshipKeyPathsForPrefetching(
            &self,
            relationship_key_paths_for_prefetching: Option<&NSArray<NSString>>,
        );

        #[method(includesPendingChanges)]
        pub unsafe fn includesPendingChanges(&self) -> bool;

        #[method(setIncludesPendingChanges:)]
        pub unsafe fn setIncludesPendingChanges(&self, includes_pending_changes: bool);

        #[method(returnsDistinctResults)]
        pub unsafe fn returnsDistinctResults(&self) -> bool;

        #[method(setReturnsDistinctResults:)]
        pub unsafe fn setReturnsDistinctResults(&self, returns_distinct_results: bool);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other propertiesToFetch)]
        pub unsafe fn propertiesToFetch(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setPropertiesToFetch:)]
        pub unsafe fn setPropertiesToFetch(&self, properties_to_fetch: Option<&NSArray>);

        #[method(fetchOffset)]
        pub unsafe fn fetchOffset(&self) -> NSUInteger;

        #[method(setFetchOffset:)]
        pub unsafe fn setFetchOffset(&self, fetch_offset: NSUInteger);

        #[method(fetchBatchSize)]
        pub unsafe fn fetchBatchSize(&self) -> NSUInteger;

        #[method(setFetchBatchSize:)]
        pub unsafe fn setFetchBatchSize(&self, fetch_batch_size: NSUInteger);

        #[method(shouldRefreshRefetchedObjects)]
        pub unsafe fn shouldRefreshRefetchedObjects(&self) -> bool;

        #[method(setShouldRefreshRefetchedObjects:)]
        pub unsafe fn setShouldRefreshRefetchedObjects(
            &self,
            should_refresh_refetched_objects: bool,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other propertiesToGroupBy)]
        pub unsafe fn propertiesToGroupBy(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setPropertiesToGroupBy:)]
        pub unsafe fn setPropertiesToGroupBy(&self, properties_to_group_by: Option<&NSArray>);

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other havingPredicate)]
        pub unsafe fn havingPredicate(&self) -> Option<Id<NSPredicate>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setHavingPredicate:)]
        pub unsafe fn setHavingPredicate(&self, having_predicate: Option<&NSPredicate>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSFetchRequest")]
    unsafe impl<ResultType: Message> NSFetchRequest<ResultType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

pub type NSPersistentStoreAsynchronousFetchResultCompletionBlock =
    *mut Block<(NonNull<NSAsynchronousFetchResult>,), ()>;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
    pub struct NSAsynchronousFetchRequest<ResultType: ?Sized = AnyObject> {
        __superclass: NSPersistentStoreRequest,
        _inner0: PhantomData<*mut ResultType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
    unsafe impl<ResultType: ?Sized + Message> ClassType for NSAsynchronousFetchRequest<ResultType> {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
unsafe impl<ResultType: ?Sized + IsIdCloneable> NSCopying
    for NSAsynchronousFetchRequest<ResultType>
{
}

#[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
unsafe impl<ResultType: ?Sized> NSObjectProtocol for NSAsynchronousFetchRequest<ResultType> {}

extern_methods!(
    #[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
    unsafe impl<ResultType: Message> NSAsynchronousFetchRequest<ResultType> {
        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Id<NSFetchRequest<ResultType>>;

        #[method(completionBlock)]
        pub unsafe fn completionBlock(
            &self,
        ) -> NSPersistentStoreAsynchronousFetchResultCompletionBlock;

        #[method(estimatedResultCount)]
        pub unsafe fn estimatedResultCount(&self) -> NSInteger;

        #[method(setEstimatedResultCount:)]
        pub unsafe fn setEstimatedResultCount(&self, estimated_result_count: NSInteger);

        #[cfg(all(
            feature = "CoreData_NSAsynchronousFetchResult",
            feature = "CoreData_NSFetchRequest"
        ))]
        #[method_id(@__retain_semantics Init initWithFetchRequest:completionBlock:)]
        pub unsafe fn initWithFetchRequest_completionBlock(
            this: Option<Allocated<Self>>,
            request: &NSFetchRequest<ResultType>,
            blk: Option<&Block<(NonNull<NSAsynchronousFetchResult<ResultType>>,), ()>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
    unsafe impl<ResultType: Message> NSAsynchronousFetchRequest<ResultType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
