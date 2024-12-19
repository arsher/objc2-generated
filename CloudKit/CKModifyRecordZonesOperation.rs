//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckmodifyrecordzonesoperation?language=objc)
    #[unsafe(super(CKDatabaseOperation, CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKModifyRecordZonesOperation;
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
unsafe impl NSObjectProtocol for CKModifyRecordZonesOperation {}

extern_methods!(
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKModifyRecordZonesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "CKRecordZone", feature = "CKRecordZoneID"))]
        #[method_id(@__retain_semantics Init initWithRecordZonesToSave:recordZoneIDsToDelete:)]
        pub unsafe fn initWithRecordZonesToSave_recordZoneIDsToDelete(
            this: Allocated<Self>,
            record_zones_to_save: Option<&NSArray<CKRecordZone>>,
            record_zone_i_ds_to_delete: Option<&NSArray<CKRecordZoneID>>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZone")]
        #[method_id(@__retain_semantics Other recordZonesToSave)]
        pub unsafe fn recordZonesToSave(&self) -> Option<Retained<NSArray<CKRecordZone>>>;

        #[cfg(feature = "CKRecordZone")]
        #[method(setRecordZonesToSave:)]
        pub unsafe fn setRecordZonesToSave(
            &self,
            record_zones_to_save: Option<&NSArray<CKRecordZone>>,
        );

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other recordZoneIDsToDelete)]
        pub unsafe fn recordZoneIDsToDelete(&self) -> Option<Retained<NSArray<CKRecordZoneID>>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method(setRecordZoneIDsToDelete:)]
        pub unsafe fn setRecordZoneIDsToDelete(
            &self,
            record_zone_i_ds_to_delete: Option<&NSArray<CKRecordZoneID>>,
        );

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(perRecordZoneSaveBlock)]
        pub unsafe fn perRecordZoneSaveBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>;

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(setPerRecordZoneSaveBlock:)]
        pub unsafe fn setPerRecordZoneSaveBlock(
            &self,
            per_record_zone_save_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(perRecordZoneDeleteBlock)]
        pub unsafe fn perRecordZoneDeleteBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordZoneID>, *mut NSError)>;

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        #[method(setPerRecordZoneDeleteBlock:)]
        pub unsafe fn setPerRecordZoneDeleteBlock(
            &self,
            per_record_zone_delete_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordZoneID>, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(modifyRecordZonesCompletionBlock)]
        pub unsafe fn modifyRecordZonesCompletionBlock(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(*mut NSArray<CKRecordZone>, *mut NSArray<CKRecordZoneID>, *mut NSError),
        >;

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(setModifyRecordZonesCompletionBlock:)]
        pub unsafe fn setModifyRecordZonesCompletionBlock(
            &self,
            modify_record_zones_completion_block: Option<
                &block2::Block<
                    dyn Fn(*mut NSArray<CKRecordZone>, *mut NSArray<CKRecordZoneID>, *mut NSError),
                >,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKModifyRecordZonesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
