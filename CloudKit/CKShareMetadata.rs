//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKShareMetadata")]
    pub struct CKShareMetadata;

    #[cfg(feature = "CloudKit_CKShareMetadata")]
    unsafe impl ClassType for CKShareMetadata {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKShareMetadata")]
unsafe impl NSCoding for CKShareMetadata {}

#[cfg(feature = "CloudKit_CKShareMetadata")]
unsafe impl NSObjectProtocol for CKShareMetadata {}

#[cfg(feature = "CloudKit_CKShareMetadata")]
unsafe impl NSSecureCoding for CKShareMetadata {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKShareMetadata")]
    unsafe impl CKShareMetadata {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "CloudKit_CKShare")]
        #[method_id(@__retain_semantics Other share)]
        pub unsafe fn share(&self) -> Id<CKShare, Shared>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other hierarchicalRootRecordID)]
        pub unsafe fn hierarchicalRootRecordID(&self) -> Option<Id<CKRecordID, Shared>>;

        #[method(participantRole)]
        pub unsafe fn participantRole(&self) -> CKShareParticipantRole;

        #[method(participantStatus)]
        pub unsafe fn participantStatus(&self) -> CKShareParticipantAcceptanceStatus;

        #[method(participantPermission)]
        pub unsafe fn participantPermission(&self) -> CKShareParticipantPermission;

        #[cfg(feature = "CloudKit_CKUserIdentity")]
        #[method_id(@__retain_semantics Other ownerIdentity)]
        pub unsafe fn ownerIdentity(&self) -> Id<CKUserIdentity, Shared>;

        #[cfg(feature = "CloudKit_CKRecord")]
        #[method_id(@__retain_semantics Other rootRecord)]
        pub unsafe fn rootRecord(&self) -> Option<Id<CKRecord, Shared>>;

        #[deprecated]
        #[method(participantType)]
        pub unsafe fn participantType(&self) -> CKShareParticipantType;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[deprecated]
        #[method_id(@__retain_semantics Other rootRecordID)]
        pub unsafe fn rootRecordID(&self) -> Id<CKRecordID, Shared>;
    }
);
