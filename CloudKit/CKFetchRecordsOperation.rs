//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckfetchrecordsoperation?language=objc)
    #[unsafe(super(CKDatabaseOperation, CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKFetchRecordsOperation;
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
unsafe impl NSObjectProtocol for CKFetchRecordsOperation {}

extern_methods!(
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordsOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Init initWithRecordIDs:)]
        pub unsafe fn initWithRecordIDs(
            this: Allocated<Self>,
            record_i_ds: &NSArray<CKRecordID>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other fetchCurrentUserRecordOperation)]
        pub unsafe fn fetchCurrentUserRecordOperation() -> Retained<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other recordIDs)]
        pub unsafe fn recordIDs(&self) -> Option<Retained<NSArray<CKRecordID>>>;

        #[cfg(feature = "CKRecordID")]
        #[method(setRecordIDs:)]
        pub unsafe fn setRecordIDs(&self, record_i_ds: Option<&NSArray<CKRecordID>>);

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Retained<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "CKRecord")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        #[method(perRecordProgressBlock)]
        pub unsafe fn perRecordProgressBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordID>, c_double)>;

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        #[method(setPerRecordProgressBlock:)]
        pub unsafe fn setPerRecordProgressBlock(
            &self,
            per_record_progress_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordID>, c_double)>,
            >,
        );

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(perRecordCompletionBlock)]
        pub unsafe fn perRecordCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut CKRecord, *mut CKRecordID, *mut NSError)>;

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(setPerRecordCompletionBlock:)]
        pub unsafe fn setPerRecordCompletionBlock(
            &self,
            per_record_completion_block: Option<
                &block2::Block<dyn Fn(*mut CKRecord, *mut CKRecordID, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(fetchRecordsCompletionBlock)]
        pub unsafe fn fetchRecordsCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSDictionary<CKRecordID, CKRecord>, *mut NSError)>;

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(setFetchRecordsCompletionBlock:)]
        pub unsafe fn setFetchRecordsCompletionBlock(
            &self,
            fetch_records_completion_block: Option<
                &block2::Block<dyn Fn(*mut NSDictionary<CKRecordID, CKRecord>, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchRecordsOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
