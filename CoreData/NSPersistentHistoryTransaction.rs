//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistenthistorytransaction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentHistoryTransaction;
);

unsafe impl NSCopying for NSPersistentHistoryTransaction {}

unsafe impl CopyingHelper for NSPersistentHistoryTransaction {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPersistentHistoryTransaction {}

extern_methods!(
    unsafe impl NSPersistentHistoryTransaction {
        #[cfg(all(feature = "NSEntityDescription", feature = "NSManagedObjectContext"))]
        #[method_id(@__retain_semantics Other entityDescriptionWithContext:)]
        pub unsafe fn entityDescriptionWithContext(
            context: &NSManagedObjectContext,
        ) -> Option<Retained<NSEntityDescription>>;

        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Other entityDescription)]
        pub unsafe fn entityDescription() -> Option<Retained<NSEntityDescription>>;

        #[cfg(all(feature = "NSFetchRequest", feature = "NSPersistentStoreRequest"))]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest() -> Option<Retained<NSFetchRequest>>;

        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Retained<NSDate>;

        #[cfg(feature = "NSPersistentHistoryChange")]
        #[method_id(@__retain_semantics Other changes)]
        pub unsafe fn changes(&self) -> Option<Retained<NSArray<NSPersistentHistoryChange>>>;

        #[method(transactionNumber)]
        pub unsafe fn transactionNumber(&self) -> i64;

        #[method_id(@__retain_semantics Other storeID)]
        pub unsafe fn storeID(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other bundleID)]
        pub unsafe fn bundleID(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other processID)]
        pub unsafe fn processID(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other contextName)]
        pub unsafe fn contextName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other author)]
        pub unsafe fn author(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSPersistentHistoryToken")]
        #[method_id(@__retain_semantics Other token)]
        pub unsafe fn token(&self) -> Retained<NSPersistentHistoryToken>;

        #[method_id(@__retain_semantics Other objectIDNotification)]
        pub unsafe fn objectIDNotification(&self) -> Retained<NSNotification>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentHistoryTransaction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
