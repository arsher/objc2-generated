//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckquerycursor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKQueryCursor;
);

unsafe impl NSCoding for CKQueryCursor {}

unsafe impl NSCopying for CKQueryCursor {}

unsafe impl CopyingHelper for CKQueryCursor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKQueryCursor {}

unsafe impl NSSecureCoding for CKQueryCursor {}

extern_methods!(
    unsafe impl CKQueryCursor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckqueryoperationmaximumresults?language=objc)
    pub static CKQueryOperationMaximumResults: NSUInteger;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckqueryoperation?language=objc)
    #[unsafe(super(CKDatabaseOperation, CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKQueryOperation;
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
unsafe impl NSObjectProtocol for CKQueryOperation {}

extern_methods!(
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKQueryOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKQuery")]
        #[method_id(@__retain_semantics Init initWithQuery:)]
        pub unsafe fn initWithQuery(this: Allocated<Self>, query: &CKQuery) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCursor:)]
        pub unsafe fn initWithCursor(
            this: Allocated<Self>,
            cursor: &CKQueryCursor,
        ) -> Retained<Self>;

        #[cfg(feature = "CKQuery")]
        #[method_id(@__retain_semantics Other query)]
        pub unsafe fn query(&self) -> Option<Retained<CKQuery>>;

        #[cfg(feature = "CKQuery")]
        #[method(setQuery:)]
        pub unsafe fn setQuery(&self, query: Option<&CKQuery>);

        #[method_id(@__retain_semantics Other cursor)]
        pub unsafe fn cursor(&self) -> Option<Retained<CKQueryCursor>>;

        #[method(setCursor:)]
        pub unsafe fn setCursor(&self, cursor: Option<&CKQueryCursor>);

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Option<Retained<CKRecordZoneID>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method(setZoneID:)]
        pub unsafe fn setZoneID(&self, zone_id: Option<&CKRecordZoneID>);

        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Retained<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "CKRecord")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        #[deprecated = "Use recordMatchedBlock instead, which surfaces per-record errors"]
        #[method(recordFetchedBlock)]
        pub unsafe fn recordFetchedBlock(&self) -> *mut block2::Block<dyn Fn(NonNull<CKRecord>)>;

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        #[deprecated = "Use recordMatchedBlock instead, which surfaces per-record errors"]
        #[method(setRecordFetchedBlock:)]
        pub unsafe fn setRecordFetchedBlock(
            &self,
            record_fetched_block: Option<&block2::Block<dyn Fn(NonNull<CKRecord>)>>,
        );

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(recordMatchedBlock)]
        pub unsafe fn recordMatchedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordID>, *mut CKRecord, *mut NSError)>;

        #[cfg(all(feature = "CKRecord", feature = "CKRecordID", feature = "block2"))]
        #[method(setRecordMatchedBlock:)]
        pub unsafe fn setRecordMatchedBlock(
            &self,
            record_matched_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordID>, *mut CKRecord, *mut NSError)>,
            >,
        );

        #[cfg(feature = "block2")]
        #[method(queryCompletionBlock)]
        pub unsafe fn queryCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut CKQueryCursor, *mut NSError)>;

        #[cfg(feature = "block2")]
        #[method(setQueryCompletionBlock:)]
        pub unsafe fn setQueryCompletionBlock(
            &self,
            query_completion_block: Option<
                &block2::Block<dyn Fn(*mut CKQueryCursor, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKQueryOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
