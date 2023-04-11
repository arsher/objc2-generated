//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchRecordZonesOperation")]
    pub struct CKFetchRecordZonesOperation;

    #[cfg(feature = "CloudKit_CKFetchRecordZonesOperation")]
    unsafe impl ClassType for CKFetchRecordZonesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKFetchRecordZonesOperation")]
unsafe impl NSObjectProtocol for CKFetchRecordZonesOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchRecordZonesOperation")]
    unsafe impl CKFetchRecordZonesOperation {
        #[method_id(@__retain_semantics Other fetchAllRecordZonesOperation)]
        pub unsafe fn fetchAllRecordZonesOperation() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithRecordZoneIDs:)]
        pub unsafe fn initWithRecordZoneIDs(
            this: Option<Allocated<Self>>,
            zone_i_ds: &NSArray<CKRecordZoneID>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordZoneIDs)]
        pub unsafe fn recordZoneIDs(&self) -> Option<Id<NSArray<CKRecordZoneID>>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method(setRecordZoneIDs:)]
        pub unsafe fn setRecordZoneIDs(&self, record_zone_i_ds: Option<&NSArray<CKRecordZoneID>>);

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSError"
        ))]
        #[method(perRecordZoneCompletionBlock)]
        pub unsafe fn perRecordZoneCompletionBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSError"
        ))]
        #[method(setPerRecordZoneCompletionBlock:)]
        pub unsafe fn setPerRecordZoneCompletionBlock(
            &self,
            per_record_zone_completion_block: Option<
                &Block<(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError), ()>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchRecordZonesCompletionBlock)]
        pub unsafe fn fetchRecordZonesCompletionBlock(
            &self,
        ) -> *mut Block<
            (
                *mut NSDictionary<CKRecordZoneID, CKRecordZone>,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method(setFetchRecordZonesCompletionBlock:)]
        pub unsafe fn setFetchRecordZonesCompletionBlock(
            &self,
            fetch_record_zones_completion_block: Option<
                &Block<
                    (
                        *mut NSDictionary<CKRecordZoneID, CKRecordZone>,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );
    }
);
