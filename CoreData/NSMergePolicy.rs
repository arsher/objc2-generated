//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSMergePolicyType {
        NSErrorMergePolicyType = 0x00,
        NSMergeByPropertyStoreTrumpMergePolicyType = 0x01,
        NSMergeByPropertyObjectTrumpMergePolicyType = 0x02,
        NSOverwriteMergePolicyType = 0x03,
        NSRollbackMergePolicyType = 0x04,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSMergeConflict")]
    pub struct NSMergeConflict;

    #[cfg(feature = "CoreData_NSMergeConflict")]
    unsafe impl ClassType for NSMergeConflict {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSMergeConflict")]
unsafe impl NSObjectProtocol for NSMergeConflict {}

extern_methods!(
    #[cfg(feature = "CoreData_NSMergeConflict")]
    unsafe impl NSMergeConflict {
        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method_id(@__retain_semantics Other sourceObject)]
        pub unsafe fn sourceObject(&self) -> Id<NSManagedObject>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other objectSnapshot)]
        pub unsafe fn objectSnapshot(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other cachedSnapshot)]
        pub unsafe fn cachedSnapshot(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other persistedSnapshot)]
        pub unsafe fn persistedSnapshot(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[method(newVersionNumber)]
        pub unsafe fn newVersionNumber(&self) -> NSUInteger;

        #[method(oldVersionNumber)]
        pub unsafe fn oldVersionNumber(&self) -> NSUInteger;

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithSource:newVersion:oldVersion:cachedSnapshot:persistedSnapshot:)]
        pub unsafe fn initWithSource_newVersion_oldVersion_cachedSnapshot_persistedSnapshot(
            this: Option<Allocated<Self>>,
            src_object: &NSManagedObject,
            newvers: NSUInteger,
            oldvers: NSUInteger,
            cachesnap: Option<&NSDictionary<NSString, Object>>,
            persnap: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSConstraintConflict")]
    pub struct NSConstraintConflict;

    #[cfg(feature = "CoreData_NSConstraintConflict")]
    unsafe impl ClassType for NSConstraintConflict {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSConstraintConflict")]
unsafe impl NSObjectProtocol for NSConstraintConflict {}

extern_methods!(
    #[cfg(feature = "CoreData_NSConstraintConflict")]
    unsafe impl NSConstraintConflict {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other constraint)]
        pub unsafe fn constraint(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other constraintValues)]
        pub unsafe fn constraintValues(&self) -> Id<NSDictionary<NSString, Object>>;

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method_id(@__retain_semantics Other databaseObject)]
        pub unsafe fn databaseObject(&self) -> Option<Id<NSManagedObject>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other databaseSnapshot)]
        pub unsafe fn databaseSnapshot(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other conflictingObjects)]
        pub unsafe fn conflictingObjects(&self) -> Id<NSArray<NSManagedObject>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other conflictingSnapshots)]
        pub unsafe fn conflictingSnapshots(&self) -> Id<NSArray<NSDictionary>>;

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithConstraint:databaseObject:databaseSnapshot:conflictingObjects:conflictingSnapshots:)]
        pub unsafe fn initWithConstraint_databaseObject_databaseSnapshot_conflictingObjects_conflictingSnapshots(
            this: Option<Allocated<Self>>,
            contraint: &NSArray<NSString>,
            database_object: Option<&NSManagedObject>,
            database_snapshot: Option<&NSDictionary>,
            conflicting_objects: &NSArray<NSManagedObject>,
            conflicting_snapshots: &NSArray,
        ) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSMergePolicy")]
    pub struct NSMergePolicy;

    #[cfg(feature = "CoreData_NSMergePolicy")]
    unsafe impl ClassType for NSMergePolicy {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSMergePolicy")]
unsafe impl NSObjectProtocol for NSMergePolicy {}

extern_methods!(
    #[cfg(feature = "CoreData_NSMergePolicy")]
    unsafe impl NSMergePolicy {
        #[method_id(@__retain_semantics Other errorMergePolicy)]
        pub unsafe fn errorMergePolicy() -> Id<NSMergePolicy>;

        #[method_id(@__retain_semantics Other rollbackMergePolicy)]
        pub unsafe fn rollbackMergePolicy() -> Id<NSMergePolicy>;

        #[method_id(@__retain_semantics Other overwriteMergePolicy)]
        pub unsafe fn overwriteMergePolicy() -> Id<NSMergePolicy>;

        #[method_id(@__retain_semantics Other mergeByPropertyObjectTrumpMergePolicy)]
        pub unsafe fn mergeByPropertyObjectTrumpMergePolicy() -> Id<NSMergePolicy>;

        #[method_id(@__retain_semantics Other mergeByPropertyStoreTrumpMergePolicy)]
        pub unsafe fn mergeByPropertyStoreTrumpMergePolicy() -> Id<NSMergePolicy>;

        #[method(mergeType)]
        pub unsafe fn mergeType(&self) -> NSMergePolicyType;

        #[method_id(@__retain_semantics Init initWithMergeType:)]
        pub unsafe fn initWithMergeType(
            this: Option<Allocated<Self>>,
            ty: NSMergePolicyType,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(resolveConflicts:error:_)]
        pub unsafe fn resolveConflicts_error(&self, list: &NSArray) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSMergeConflict",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(resolveOptimisticLockingVersionConflicts:error:_)]
        pub unsafe fn resolveOptimisticLockingVersionConflicts_error(
            &self,
            list: &NSArray<NSMergeConflict>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSConstraintConflict",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(resolveConstraintConflicts:error:_)]
        pub unsafe fn resolveConstraintConflicts_error(
            &self,
            list: &NSArray<NSConstraintConflict>,
        ) -> Result<(), Id<NSError>>;
    }
);
