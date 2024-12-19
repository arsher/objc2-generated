//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginerecordzonechangebatch?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineRecordZoneChangeBatch;
);

unsafe impl Send for CKSyncEngineRecordZoneChangeBatch {}

unsafe impl Sync for CKSyncEngineRecordZoneChangeBatch {}

unsafe impl NSObjectProtocol for CKSyncEngineRecordZoneChangeBatch {}

extern_methods!(
    unsafe impl CKSyncEngineRecordZoneChangeBatch {
        #[cfg(all(
            feature = "CKRecord",
            feature = "CKRecordID",
            feature = "CKSyncEngineState",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithPendingChanges:recordProvider:)]
        pub unsafe fn initWithPendingChanges_recordProvider(
            this: Allocated<Self>,
            pending_changes: &NSArray<CKSyncEnginePendingRecordZoneChange>,
            record_provider: &block2::Block<dyn Fn(NonNull<CKRecordID>) -> *mut CKRecord + '_>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID"))]
        #[method_id(@__retain_semantics Init initWithRecordsToSave:recordIDsToDelete:atomicByZone:)]
        pub unsafe fn initWithRecordsToSave_recordIDsToDelete_atomicByZone(
            this: Allocated<Self>,
            records_to_save: Option<&NSArray<CKRecord>>,
            record_i_ds_to_delete: Option<&NSArray<CKRecordID>>,
            atomic_by_zone: bool,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other recordsToSave)]
        pub unsafe fn recordsToSave(&self) -> Retained<NSArray<CKRecord>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other recordIDsToDelete)]
        pub unsafe fn recordIDsToDelete(&self) -> Retained<NSArray<CKRecordID>>;

        #[method(atomicByZone)]
        pub unsafe fn atomicByZone(&self) -> bool;

        #[method(setAtomicByZone:)]
        pub unsafe fn setAtomicByZone(&self, atomic_by_zone: bool);
    }
);
