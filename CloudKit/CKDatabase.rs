//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKDatabaseScope(pub NSInteger);
impl CKDatabaseScope {
    #[doc(alias = "CKDatabaseScopePublic")]
    pub const Public: Self = Self(1);
    #[doc(alias = "CKDatabaseScopePrivate")]
    pub const Private: Self = Self(2);
    #[doc(alias = "CKDatabaseScopeShared")]
    pub const Shared: Self = Self(3);
}

unsafe impl Encode for CKDatabaseScope {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKDatabaseScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKDatabase;

    unsafe impl ClassType for CKDatabase {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKDatabase {}

unsafe impl Sync for CKDatabase {}

unsafe impl NSObjectProtocol for CKDatabase {}

extern_methods!(
    unsafe impl CKDatabase {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
        #[method(addOperation:)]
        pub unsafe fn addOperation(&self, operation: &CKDatabaseOperation);

        #[method(databaseScope)]
        pub unsafe fn databaseScope(&self) -> CKDatabaseScope;
    }
);

extern_methods!(
    /// ConvenienceMethods
    unsafe impl CKDatabase {
        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(fetchRecordWithID:completionHandler:)]
        pub unsafe fn fetchRecordWithID_completionHandler(
            &self,
            record_id: &CKRecordID,
            completion_handler: &block2::Block<dyn Fn(*mut CKRecord, *mut NSError)>,
        );

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        #[method(saveRecord:completionHandler:)]
        pub unsafe fn saveRecord_completionHandler(
            &self,
            record: &CKRecord,
            completion_handler: &block2::Block<dyn Fn(*mut CKRecord, *mut NSError)>,
        );

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        #[method(deleteRecordWithID:completionHandler:)]
        pub unsafe fn deleteRecordWithID_completionHandler(
            &self,
            record_id: &CKRecordID,
            completion_handler: &block2::Block<dyn Fn(*mut CKRecordID, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CKQuery",
            feature = "CKRecord",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(performQuery:inZoneWithID:completionHandler:)]
        pub unsafe fn performQuery_inZoneWithID_completionHandler(
            &self,
            query: &CKQuery,
            zone_id: Option<&CKRecordZoneID>,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<CKRecord>, *mut NSError)>,
        );

        #[cfg(all(feature = "CKRecordZone", feature = "block2"))]
        #[method(fetchAllRecordZonesWithCompletionHandler:)]
        pub unsafe fn fetchAllRecordZonesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<CKRecordZone>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(fetchRecordZoneWithID:completionHandler:)]
        pub unsafe fn fetchRecordZoneWithID_completionHandler(
            &self,
            zone_id: &CKRecordZoneID,
            completion_handler: &block2::Block<dyn Fn(*mut CKRecordZone, *mut NSError)>,
        );

        #[cfg(all(feature = "CKRecordZone", feature = "block2"))]
        #[method(saveRecordZone:completionHandler:)]
        pub unsafe fn saveRecordZone_completionHandler(
            &self,
            zone: &CKRecordZone,
            completion_handler: &block2::Block<dyn Fn(*mut CKRecordZone, *mut NSError)>,
        );

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(deleteRecordZoneWithID:completionHandler:)]
        pub unsafe fn deleteRecordZoneWithID_completionHandler(
            &self,
            zone_id: &CKRecordZoneID,
            completion_handler: &block2::Block<dyn Fn(*mut CKRecordZoneID, *mut NSError)>,
        );

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        #[method(fetchSubscriptionWithID:completionHandler:)]
        pub unsafe fn fetchSubscriptionWithID_completionHandler(
            &self,
            subscription_id: &CKSubscriptionID,
            completion_handler: &block2::Block<dyn Fn(*mut CKSubscription, *mut NSError)>,
        );

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        #[method(fetchAllSubscriptionsWithCompletionHandler:)]
        pub unsafe fn fetchAllSubscriptionsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<CKSubscription>, *mut NSError)>,
        );

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        #[method(saveSubscription:completionHandler:)]
        pub unsafe fn saveSubscription_completionHandler(
            &self,
            subscription: &CKSubscription,
            completion_handler: &block2::Block<dyn Fn(*mut CKSubscription, *mut NSError)>,
        );

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        #[method(deleteSubscriptionWithID:completionHandler:)]
        pub unsafe fn deleteSubscriptionWithID_completionHandler(
            &self,
            subscription_id: &CKSubscriptionID,
            completion_handler: &block2::Block<dyn Fn(*mut CKSubscriptionID, *mut NSError)>,
        );
    }
);
