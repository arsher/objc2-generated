//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKModifyRecordZonesOperation")]
    pub struct CKModifyRecordZonesOperation;

    #[cfg(feature = "CloudKit_CKModifyRecordZonesOperation")]
    unsafe impl ClassType for CKModifyRecordZonesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

#[cfg(feature = "CloudKit_CKModifyRecordZonesOperation")]
unsafe impl NSObjectProtocol for CKModifyRecordZonesOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKModifyRecordZonesOperation")]
    unsafe impl CKModifyRecordZonesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordZonesToSave:recordZoneIDsToDelete:)]
        pub unsafe fn initWithRecordZonesToSave_recordZoneIDsToDelete(
            this: Option<Allocated<Self>>,
            record_zones_to_save: Option<&NSArray<CKRecordZone>>,
            record_zone_i_ds_to_delete: Option<&NSArray<CKRecordZoneID>>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "CloudKit_CKRecordZone", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordZonesToSave)]
        pub unsafe fn recordZonesToSave(&self) -> Option<Id<NSArray<CKRecordZone>, Shared>>;

        #[cfg(all(feature = "CloudKit_CKRecordZone", feature = "Foundation_NSArray"))]
        #[method(setRecordZonesToSave:)]
        pub unsafe fn setRecordZonesToSave(
            &self,
            record_zones_to_save: Option<&NSArray<CKRecordZone>>,
        );

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordZoneIDsToDelete)]
        pub unsafe fn recordZoneIDsToDelete(&self) -> Option<Id<NSArray<CKRecordZoneID>, Shared>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method(setRecordZoneIDsToDelete:)]
        pub unsafe fn setRecordZoneIDsToDelete(
            &self,
            record_zone_i_ds_to_delete: Option<&NSArray<CKRecordZoneID>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSError"
        ))]
        #[method(perRecordZoneSaveBlock)]
        pub unsafe fn perRecordZoneSaveBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSError"
        ))]
        #[method(setPerRecordZoneSaveBlock:)]
        pub unsafe fn setPerRecordZoneSaveBlock(
            &self,
            per_record_zone_save_block: Option<
                &Block<(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError), ()>,
            >,
        );

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSError"))]
        #[method(perRecordZoneDeleteBlock)]
        pub unsafe fn perRecordZoneDeleteBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordZoneID>, *mut NSError), ()>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSError"))]
        #[method(setPerRecordZoneDeleteBlock:)]
        pub unsafe fn setPerRecordZoneDeleteBlock(
            &self,
            per_record_zone_delete_block: Option<
                &Block<(NonNull<CKRecordZoneID>, *mut NSError), ()>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(modifyRecordZonesCompletionBlock)]
        pub unsafe fn modifyRecordZonesCompletionBlock(
            &self,
        ) -> *mut Block<
            (
                *mut NSArray<CKRecordZone>,
                *mut NSArray<CKRecordZoneID>,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(setModifyRecordZonesCompletionBlock:)]
        pub unsafe fn setModifyRecordZonesCompletionBlock(
            &self,
            modify_record_zones_completion_block: Option<
                &Block<
                    (
                        *mut NSArray<CKRecordZone>,
                        *mut NSArray<CKRecordZoneID>,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );
    }
);
