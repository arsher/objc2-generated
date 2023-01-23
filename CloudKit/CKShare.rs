//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_static!(CKRecordTypeShare: &'static CKRecordType);

extern_static!(CKRecordNameZoneWideShare: &'static NSString);

extern_static!(CKShareTitleKey: &'static CKRecordFieldKey);

extern_static!(CKShareThumbnailImageDataKey: &'static CKRecordFieldKey);

extern_static!(CKShareTypeKey: &'static CKRecordFieldKey);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKShare")]
    pub struct CKShare;

    #[cfg(feature = "CloudKit_CKShare")]
    unsafe impl ClassType for CKShare {
        #[inherits(NSObject)]
        type Super = CKRecord;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKShare")]
    unsafe impl CKShare {
        #[method_id(@__retain_semantics Init initWithRootRecord:)]
        pub unsafe fn initWithRootRecord(
            this: Option<Allocated<Self>>,
            root_record: &CKRecord,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Init initWithRootRecord:shareID:)]
        pub unsafe fn initWithRootRecord_shareID(
            this: Option<Allocated<Self>>,
            root_record: &CKRecord,
            share_id: &CKRecordID,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithRecordZoneID:)]
        pub unsafe fn initWithRecordZoneID(
            this: Option<Allocated<Self>>,
            record_zone_id: &CKRecordZoneID,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method(publicPermission)]
        pub unsafe fn publicPermission(&self) -> CKShareParticipantPermission;

        #[method(setPublicPermission:)]
        pub unsafe fn setPublicPermission(&self, public_permission: CKShareParticipantPermission);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other participants)]
        pub unsafe fn participants(&self) -> Id<NSArray<CKShareParticipant>, Shared>;

        #[cfg(feature = "CloudKit_CKShareParticipant")]
        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Id<CKShareParticipant, Shared>;

        #[cfg(feature = "CloudKit_CKShareParticipant")]
        #[method_id(@__retain_semantics Other currentUserParticipant)]
        pub unsafe fn currentUserParticipant(&self) -> Option<Id<CKShareParticipant, Shared>>;

        #[cfg(feature = "CloudKit_CKShareParticipant")]
        #[method(addParticipant:)]
        pub unsafe fn addParticipant(&self, participant: &CKShareParticipant);

        #[cfg(feature = "CloudKit_CKShareParticipant")]
        #[method(removeParticipant:)]
        pub unsafe fn removeParticipant(&self, participant: &CKShareParticipant);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithRecordType:)]
        pub unsafe fn initWithRecordType(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Init initWithRecordType:recordID:)]
        pub unsafe fn initWithRecordType_recordID(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
            record_id: &CKRecordID,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithRecordType:zoneID:)]
        pub unsafe fn initWithRecordType_zoneID(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
            zone_id: &CKRecordZoneID,
        ) -> Id<Self, Shared>;
    }
);