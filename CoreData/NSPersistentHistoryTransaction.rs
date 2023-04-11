//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
    pub struct NSPersistentHistoryTransaction;

    #[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
    unsafe impl ClassType for NSPersistentHistoryTransaction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
unsafe impl NSCopying for NSPersistentHistoryTransaction {}

#[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
unsafe impl NSObjectProtocol for NSPersistentHistoryTransaction {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
    unsafe impl NSPersistentHistoryTransaction {
        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSManagedObjectContext"
        ))]
        #[method_id(@__retain_semantics Other entityDescriptionWithContext:)]
        pub unsafe fn entityDescriptionWithContext(
            context: &NSManagedObjectContext,
        ) -> Option<Id<NSEntityDescription>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entityDescription)]
        pub unsafe fn entityDescription() -> Option<Id<NSEntityDescription>>;

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest() -> Option<Id<NSFetchRequest>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSDate>;

        #[cfg(all(
            feature = "CoreData_NSPersistentHistoryChange",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other changes)]
        pub unsafe fn changes(&self) -> Option<Id<NSArray<NSPersistentHistoryChange>>>;

        #[method(transactionNumber)]
        pub unsafe fn transactionNumber(&self) -> i64;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other storeID)]
        pub unsafe fn storeID(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bundleID)]
        pub unsafe fn bundleID(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other processID)]
        pub unsafe fn processID(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other contextName)]
        pub unsafe fn contextName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other author)]
        pub unsafe fn author(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "CoreData_NSPersistentHistoryToken")]
        #[method_id(@__retain_semantics Other token)]
        pub unsafe fn token(&self) -> Id<NSPersistentHistoryToken>;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method_id(@__retain_semantics Other objectIDNotification)]
        pub unsafe fn objectIDNotification(&self) -> Id<NSNotification>;
    }
);
