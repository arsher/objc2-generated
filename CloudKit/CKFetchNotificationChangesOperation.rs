//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
    pub struct CKFetchNotificationChangesOperation;

    #[cfg(feature = "CKOperation")]
    unsafe impl ClassType for CKFetchNotificationChangesOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKFetchNotificationChangesOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchNotificationChangesOperation {
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "CKServerChangeToken")]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method_id(@__retain_semantics Init initWithPreviousServerChangeToken:)]
        pub unsafe fn initWithPreviousServerChangeToken(
            this: Allocated<Self>,
            previous_server_change_token: Option<&CKServerChangeToken>,
        ) -> Id<Self>;

        #[cfg(feature = "CKServerChangeToken")]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Id<CKServerChangeToken>>;

        #[cfg(feature = "CKServerChangeToken")]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(moreComing)]
        pub unsafe fn moreComing(&self) -> bool;

        #[cfg(all(feature = "CKNotification", feature = "block2"))]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(notificationChangedBlock)]
        pub unsafe fn notificationChangedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKNotification>)>;

        #[cfg(all(feature = "CKNotification", feature = "block2"))]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(setNotificationChangedBlock:)]
        pub unsafe fn setNotificationChangedBlock(
            &self,
            notification_changed_block: Option<&block2::Block<dyn Fn(NonNull<CKNotification>)>>,
        );

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(fetchNotificationChangesCompletionBlock)]
        pub unsafe fn fetchNotificationChangesCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut CKServerChangeToken, *mut NSError)>;

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(setFetchNotificationChangesCompletionBlock:)]
        pub unsafe fn setFetchNotificationChangesCompletionBlock(
            &self,
            fetch_notification_changes_completion_block: Option<
                &block2::Block<dyn Fn(*mut CKServerChangeToken, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchNotificationChangesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
