//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOperation")]
    pub struct CKFetchRecordZoneChangesOperation;

    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOperation")]
    unsafe impl ClassType for CKFetchRecordZoneChangesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOperation")]
    unsafe impl CKFetchRecordZoneChangesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "CloudKit_CKFetchRecordZoneChangesConfiguration",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordZoneIDs:configurationsByRecordZoneID:)]
        pub unsafe fn initWithRecordZoneIDs_configurationsByRecordZoneID(
            this: Option<Allocated<Self>>,
            record_zone_i_ds: &NSArray<CKRecordZoneID>,
            configurations_by_record_zone_id: Option<
                &NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesConfiguration>,
            >,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordZoneIDs)]
        pub unsafe fn recordZoneIDs(&self) -> Option<Id<NSArray<CKRecordZoneID>, Shared>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method(setRecordZoneIDs:)]
        pub unsafe fn setRecordZoneIDs(&self, record_zone_i_ds: Option<&NSArray<CKRecordZoneID>>);

        #[cfg(all(
            feature = "CloudKit_CKFetchRecordZoneChangesConfiguration",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other configurationsByRecordZoneID)]
        pub unsafe fn configurationsByRecordZoneID(
            &self,
        ) -> Option<Id<NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesConfiguration>, Shared>>;

        #[cfg(all(
            feature = "CloudKit_CKFetchRecordZoneChangesConfiguration",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(setConfigurationsByRecordZoneID:)]
        pub unsafe fn setConfigurationsByRecordZoneID(
            &self,
            configurations_by_record_zone_id: Option<
                &NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesConfiguration>,
            >,
        );

        #[method(fetchAllChanges)]
        pub unsafe fn fetchAllChanges(&self) -> bool;

        #[method(setFetchAllChanges:)]
        pub unsafe fn setFetchAllChanges(&self, fetch_all_changes: bool);

        #[cfg(feature = "CloudKit_CKRecord")]
        #[deprecated = "Use recordWasChangedBlock instead, which surfaces per-record errors"]
        #[method(recordChangedBlock)]
        pub unsafe fn recordChangedBlock(&self) -> *mut Block<(NonNull<CKRecord>,), ()>;

        #[cfg(feature = "CloudKit_CKRecord")]
        #[deprecated = "Use recordWasChangedBlock instead, which surfaces per-record errors"]
        #[method(setRecordChangedBlock:)]
        pub unsafe fn setRecordChangedBlock(
            &self,
            record_changed_block: Option<&Block<(NonNull<CKRecord>,), ()>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSError"
        ))]
        #[method(recordWasChangedBlock)]
        pub unsafe fn recordWasChangedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordID>, *mut CKRecord, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSError"
        ))]
        #[method(setRecordWasChangedBlock:)]
        pub unsafe fn setRecordWasChangedBlock(
            &self,
            record_was_changed_block: Option<
                &Block<(NonNull<CKRecordID>, *mut CKRecord, *mut NSError), ()>,
            >,
        );

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method(recordWithIDWasDeletedBlock)]
        pub unsafe fn recordWithIDWasDeletedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordID>, NonNull<CKRecordType>), ()>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method(setRecordWithIDWasDeletedBlock:)]
        pub unsafe fn setRecordWithIDWasDeletedBlock(
            &self,
            record_with_id_was_deleted_block: Option<
                &Block<(NonNull<CKRecordID>, NonNull<CKRecordType>), ()>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZoneID",
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSData"
        ))]
        #[method(recordZoneChangeTokensUpdatedBlock)]
        pub unsafe fn recordZoneChangeTokensUpdatedBlock(
            &self,
        ) -> *mut Block<
            (
                NonNull<CKRecordZoneID>,
                *mut CKServerChangeToken,
                *mut NSData,
            ),
            (),
        >;

        #[cfg(all(
            feature = "CloudKit_CKRecordZoneID",
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSData"
        ))]
        #[method(setRecordZoneChangeTokensUpdatedBlock:)]
        pub unsafe fn setRecordZoneChangeTokensUpdatedBlock(
            &self,
            record_zone_change_tokens_updated_block: Option<
                &Block<
                    (
                        NonNull<CKRecordZoneID>,
                        *mut CKServerChangeToken,
                        *mut NSData,
                    ),
                    (),
                >,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZoneID",
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method(recordZoneFetchCompletionBlock)]
        pub unsafe fn recordZoneFetchCompletionBlock(
            &self,
        ) -> *mut Block<
            (
                NonNull<CKRecordZoneID>,
                *mut CKServerChangeToken,
                *mut NSData,
                Bool,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "CloudKit_CKRecordZoneID",
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method(setRecordZoneFetchCompletionBlock:)]
        pub unsafe fn setRecordZoneFetchCompletionBlock(
            &self,
            record_zone_fetch_completion_block: Option<
                &Block<
                    (
                        NonNull<CKRecordZoneID>,
                        *mut CKServerChangeToken,
                        *mut NSData,
                        Bool,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(fetchRecordZoneChangesCompletionBlock)]
        pub unsafe fn fetchRecordZoneChangesCompletionBlock(
            &self,
        ) -> *mut Block<(*mut NSError,), ()>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(setFetchRecordZoneChangesCompletionBlock:)]
        pub unsafe fn setFetchRecordZoneChangesCompletionBlock(
            &self,
            fetch_record_zone_changes_completion_block: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOperation")]
    unsafe impl CKFetchRecordZoneChangesOperation {
        #[cfg(all(
            feature = "CloudKit_CKFetchRecordZoneChangesOptions",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithRecordZoneIDs:optionsByRecordZoneID:)]
        pub unsafe fn initWithRecordZoneIDs_optionsByRecordZoneID(
            this: Option<Allocated<Self>>,
            record_zone_i_ds: &NSArray<CKRecordZoneID>,
            options_by_record_zone_id: Option<
                &NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesOptions>,
            >,
        ) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "CloudKit_CKFetchRecordZoneChangesOptions",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSDictionary"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other optionsByRecordZoneID)]
        pub unsafe fn optionsByRecordZoneID(
            &self,
        ) -> Option<Id<NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesOptions>, Shared>>;

        #[cfg(all(
            feature = "CloudKit_CKFetchRecordZoneChangesOptions",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSDictionary"
        ))]
        #[deprecated]
        #[method(setOptionsByRecordZoneID:)]
        pub unsafe fn setOptionsByRecordZoneID(
            &self,
            options_by_record_zone_id: Option<
                &NSDictionary<CKRecordZoneID, CKFetchRecordZoneChangesOptions>,
            >,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesConfiguration")]
    pub struct CKFetchRecordZoneChangesConfiguration;

    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesConfiguration")]
    unsafe impl ClassType for CKFetchRecordZoneChangesConfiguration {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesConfiguration")]
    unsafe impl CKFetchRecordZoneChangesConfiguration {
        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Id<CKServerChangeToken, Shared>>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Id<NSArray<CKRecordFieldKey>, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOptions")]
    #[deprecated]
    pub struct CKFetchRecordZoneChangesOptions;

    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOptions")]
    unsafe impl ClassType for CKFetchRecordZoneChangesOptions {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOptions")]
    unsafe impl CKFetchRecordZoneChangesOptions {
        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Id<CKServerChangeToken, Shared>>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Id<NSArray<CKRecordFieldKey>, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);
    }
);