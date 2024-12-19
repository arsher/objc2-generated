//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncengineeventtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKSyncEngineEventType(pub NSInteger);
impl CKSyncEngineEventType {
    #[doc(alias = "CKSyncEngineEventTypeStateUpdate")]
    pub const StateUpdate: Self = Self(0);
    #[doc(alias = "CKSyncEngineEventTypeAccountChange")]
    pub const AccountChange: Self = Self(1);
    #[doc(alias = "CKSyncEngineEventTypeFetchedDatabaseChanges")]
    pub const FetchedDatabaseChanges: Self = Self(2);
    #[doc(alias = "CKSyncEngineEventTypeFetchedRecordZoneChanges")]
    pub const FetchedRecordZoneChanges: Self = Self(3);
    #[doc(alias = "CKSyncEngineEventTypeSentDatabaseChanges")]
    pub const SentDatabaseChanges: Self = Self(4);
    #[doc(alias = "CKSyncEngineEventTypeSentRecordZoneChanges")]
    pub const SentRecordZoneChanges: Self = Self(5);
    #[doc(alias = "CKSyncEngineEventTypeWillFetchChanges")]
    pub const WillFetchChanges: Self = Self(6);
    #[doc(alias = "CKSyncEngineEventTypeWillFetchRecordZoneChanges")]
    pub const WillFetchRecordZoneChanges: Self = Self(7);
    #[doc(alias = "CKSyncEngineEventTypeDidFetchRecordZoneChanges")]
    pub const DidFetchRecordZoneChanges: Self = Self(8);
    #[doc(alias = "CKSyncEngineEventTypeDidFetchChanges")]
    pub const DidFetchChanges: Self = Self(9);
    #[doc(alias = "CKSyncEngineEventTypeWillSendChanges")]
    pub const WillSendChanges: Self = Self(10);
    #[doc(alias = "CKSyncEngineEventTypeDidSendChanges")]
    pub const DidSendChanges: Self = Self(11);
}

unsafe impl Encode for CKSyncEngineEventType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKSyncEngineEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncengineevent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineEvent;
);

unsafe impl Send for CKSyncEngineEvent {}

unsafe impl Sync for CKSyncEngineEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineEvent {}

extern_methods!(
    unsafe impl CKSyncEngineEvent {
        #[method(type)]
        pub unsafe fn r#type(&self) -> CKSyncEngineEventType;

        #[method_id(@__retain_semantics Other stateUpdateEvent)]
        pub unsafe fn stateUpdateEvent(&self) -> Retained<CKSyncEngineStateUpdateEvent>;

        #[method_id(@__retain_semantics Other accountChangeEvent)]
        pub unsafe fn accountChangeEvent(&self) -> Retained<CKSyncEngineAccountChangeEvent>;

        #[method_id(@__retain_semantics Other fetchedDatabaseChangesEvent)]
        pub unsafe fn fetchedDatabaseChangesEvent(
            &self,
        ) -> Retained<CKSyncEngineFetchedDatabaseChangesEvent>;

        #[method_id(@__retain_semantics Other fetchedRecordZoneChangesEvent)]
        pub unsafe fn fetchedRecordZoneChangesEvent(
            &self,
        ) -> Retained<CKSyncEngineFetchedRecordZoneChangesEvent>;

        #[method_id(@__retain_semantics Other sentDatabaseChangesEvent)]
        pub unsafe fn sentDatabaseChangesEvent(
            &self,
        ) -> Retained<CKSyncEngineSentDatabaseChangesEvent>;

        #[method_id(@__retain_semantics Other sentRecordZoneChangesEvent)]
        pub unsafe fn sentRecordZoneChangesEvent(
            &self,
        ) -> Retained<CKSyncEngineSentRecordZoneChangesEvent>;

        #[method_id(@__retain_semantics Other willFetchChangesEvent)]
        pub unsafe fn willFetchChangesEvent(&self) -> Retained<CKSyncEngineWillFetchChangesEvent>;

        #[method_id(@__retain_semantics Other willFetchRecordZoneChangesEvent)]
        pub unsafe fn willFetchRecordZoneChangesEvent(
            &self,
        ) -> Retained<CKSyncEngineWillFetchRecordZoneChangesEvent>;

        #[method_id(@__retain_semantics Other didFetchRecordZoneChangesEvent)]
        pub unsafe fn didFetchRecordZoneChangesEvent(
            &self,
        ) -> Retained<CKSyncEngineDidFetchRecordZoneChangesEvent>;

        #[method_id(@__retain_semantics Other didFetchChangesEvent)]
        pub unsafe fn didFetchChangesEvent(&self) -> Retained<CKSyncEngineDidFetchChangesEvent>;

        #[method_id(@__retain_semantics Other willSendChangesEvent)]
        pub unsafe fn willSendChangesEvent(&self) -> Retained<CKSyncEngineWillSendChangesEvent>;

        #[method_id(@__retain_semantics Other didSendChangesEvent)]
        pub unsafe fn didSendChangesEvent(&self) -> Retained<CKSyncEngineDidSendChangesEvent>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginestateupdateevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineStateUpdateEvent;
);

unsafe impl Send for CKSyncEngineStateUpdateEvent {}

unsafe impl Sync for CKSyncEngineStateUpdateEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineStateUpdateEvent {}

extern_methods!(
    unsafe impl CKSyncEngineStateUpdateEvent {
        #[cfg(feature = "CKSyncEngineState")]
        #[method_id(@__retain_semantics Other stateSerialization)]
        pub unsafe fn stateSerialization(&self) -> Retained<CKSyncEngineStateSerialization>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineStateUpdateEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncengineaccountchangetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKSyncEngineAccountChangeType(pub NSInteger);
impl CKSyncEngineAccountChangeType {
    #[doc(alias = "CKSyncEngineAccountChangeTypeSignIn")]
    pub const SignIn: Self = Self(0);
    #[doc(alias = "CKSyncEngineAccountChangeTypeSignOut")]
    pub const SignOut: Self = Self(1);
    #[doc(alias = "CKSyncEngineAccountChangeTypeSwitchAccounts")]
    pub const SwitchAccounts: Self = Self(2);
}

unsafe impl Encode for CKSyncEngineAccountChangeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKSyncEngineAccountChangeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncengineaccountchangeevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineAccountChangeEvent;
);

unsafe impl Send for CKSyncEngineAccountChangeEvent {}

unsafe impl Sync for CKSyncEngineAccountChangeEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineAccountChangeEvent {}

extern_methods!(
    unsafe impl CKSyncEngineAccountChangeEvent {
        #[method(changeType)]
        pub unsafe fn changeType(&self) -> CKSyncEngineAccountChangeType;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other previousUser)]
        pub unsafe fn previousUser(&self) -> Option<Retained<CKRecordID>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other currentUser)]
        pub unsafe fn currentUser(&self) -> Option<Retained<CKRecordID>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineAccountChangeEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefetcheddatabasechangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchedDatabaseChangesEvent;
);

unsafe impl Send for CKSyncEngineFetchedDatabaseChangesEvent {}

unsafe impl Sync for CKSyncEngineFetchedDatabaseChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchedDatabaseChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineFetchedDatabaseChangesEvent {
        #[cfg(feature = "CKRecordZone")]
        #[method_id(@__retain_semantics Other modifications)]
        pub unsafe fn modifications(&self) -> Retained<NSArray<CKRecordZone>>;

        #[method_id(@__retain_semantics Other deletions)]
        pub unsafe fn deletions(&self) -> Retained<NSArray<CKSyncEngineFetchedZoneDeletion>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineFetchedDatabaseChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefetchedrecordzonechangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchedRecordZoneChangesEvent;
);

unsafe impl Send for CKSyncEngineFetchedRecordZoneChangesEvent {}

unsafe impl Sync for CKSyncEngineFetchedRecordZoneChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchedRecordZoneChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineFetchedRecordZoneChangesEvent {
        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other modifications)]
        pub unsafe fn modifications(&self) -> Retained<NSArray<CKRecord>>;

        #[method_id(@__retain_semantics Other deletions)]
        pub unsafe fn deletions(&self) -> Retained<NSArray<CKSyncEngineFetchedRecordDeletion>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineFetchedRecordZoneChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginesentdatabasechangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineSentDatabaseChangesEvent;
);

unsafe impl Send for CKSyncEngineSentDatabaseChangesEvent {}

unsafe impl Sync for CKSyncEngineSentDatabaseChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineSentDatabaseChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineSentDatabaseChangesEvent {
        #[cfg(feature = "CKRecordZone")]
        #[method_id(@__retain_semantics Other savedZones)]
        pub unsafe fn savedZones(&self) -> Retained<NSArray<CKRecordZone>>;

        #[method_id(@__retain_semantics Other failedZoneSaves)]
        pub unsafe fn failedZoneSaves(&self) -> Retained<NSArray<CKSyncEngineFailedZoneSave>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other deletedZoneIDs)]
        pub unsafe fn deletedZoneIDs(&self) -> Retained<NSArray<CKRecordZoneID>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other failedZoneDeletes)]
        pub unsafe fn failedZoneDeletes(&self) -> Retained<NSDictionary<CKRecordZoneID, NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineSentDatabaseChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginesentrecordzonechangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineSentRecordZoneChangesEvent;
);

unsafe impl Send for CKSyncEngineSentRecordZoneChangesEvent {}

unsafe impl Sync for CKSyncEngineSentRecordZoneChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineSentRecordZoneChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineSentRecordZoneChangesEvent {
        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other savedRecords)]
        pub unsafe fn savedRecords(&self) -> Retained<NSArray<CKRecord>>;

        #[method_id(@__retain_semantics Other failedRecordSaves)]
        pub unsafe fn failedRecordSaves(&self) -> Retained<NSArray<CKSyncEngineFailedRecordSave>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other deletedRecordIDs)]
        pub unsafe fn deletedRecordIDs(&self) -> Retained<NSArray<CKRecordID>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other failedRecordDeletes)]
        pub unsafe fn failedRecordDeletes(&self) -> Retained<NSDictionary<CKRecordID, NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineSentRecordZoneChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginewillfetchchangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineWillFetchChangesEvent;
);

unsafe impl Send for CKSyncEngineWillFetchChangesEvent {}

unsafe impl Sync for CKSyncEngineWillFetchChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineWillFetchChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineWillFetchChangesEvent {
        #[cfg(feature = "CKSyncEngine")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Retained<CKSyncEngineFetchChangesContext>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineWillFetchChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginewillfetchrecordzonechangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineWillFetchRecordZoneChangesEvent;
);

unsafe impl Send for CKSyncEngineWillFetchRecordZoneChangesEvent {}

unsafe impl Sync for CKSyncEngineWillFetchRecordZoneChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineWillFetchRecordZoneChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineWillFetchRecordZoneChangesEvent {
        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Retained<CKRecordZoneID>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineWillFetchRecordZoneChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginedidfetchrecordzonechangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineDidFetchRecordZoneChangesEvent;
);

unsafe impl Send for CKSyncEngineDidFetchRecordZoneChangesEvent {}

unsafe impl Sync for CKSyncEngineDidFetchRecordZoneChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineDidFetchRecordZoneChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineDidFetchRecordZoneChangesEvent {
        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Retained<CKRecordZoneID>;

        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineDidFetchRecordZoneChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginedidfetchchangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineDidFetchChangesEvent;
);

unsafe impl Send for CKSyncEngineDidFetchChangesEvent {}

unsafe impl Sync for CKSyncEngineDidFetchChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineDidFetchChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineDidFetchChangesEvent {
        #[cfg(feature = "CKSyncEngine")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Retained<CKSyncEngineFetchChangesContext>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineDidFetchChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginewillsendchangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineWillSendChangesEvent;
);

unsafe impl Send for CKSyncEngineWillSendChangesEvent {}

unsafe impl Sync for CKSyncEngineWillSendChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineWillSendChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineWillSendChangesEvent {
        #[cfg(feature = "CKSyncEngine")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Retained<CKSyncEngineSendChangesContext>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineWillSendChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginedidsendchangesevent?language=objc)
    #[unsafe(super(CKSyncEngineEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineDidSendChangesEvent;
);

unsafe impl Send for CKSyncEngineDidSendChangesEvent {}

unsafe impl Sync for CKSyncEngineDidSendChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineDidSendChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineDidSendChangesEvent {
        #[cfg(feature = "CKSyncEngine")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Retained<CKSyncEngineSendChangesContext>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineDidSendChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefetchedrecorddeletion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchedRecordDeletion;
);

unsafe impl Send for CKSyncEngineFetchedRecordDeletion {}

unsafe impl Sync for CKSyncEngineFetchedRecordDeletion {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchedRecordDeletion {}

extern_methods!(
    unsafe impl CKSyncEngineFetchedRecordDeletion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other recordID)]
        pub unsafe fn recordID(&self) -> Retained<CKRecordID>;

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Retained<CKRecordType>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginezonedeletionreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKSyncEngineZoneDeletionReason(pub NSInteger);
impl CKSyncEngineZoneDeletionReason {
    #[doc(alias = "CKSyncEngineZoneDeletionReasonDeleted")]
    pub const Deleted: Self = Self(0);
    #[doc(alias = "CKSyncEngineZoneDeletionReasonPurged")]
    pub const Purged: Self = Self(1);
    #[doc(alias = "CKSyncEngineZoneDeletionReasonEncryptedDataReset")]
    pub const EncryptedDataReset: Self = Self(2);
}

unsafe impl Encode for CKSyncEngineZoneDeletionReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKSyncEngineZoneDeletionReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefetchedzonedeletion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchedZoneDeletion;
);

unsafe impl Send for CKSyncEngineFetchedZoneDeletion {}

unsafe impl Sync for CKSyncEngineFetchedZoneDeletion {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchedZoneDeletion {}

extern_methods!(
    unsafe impl CKSyncEngineFetchedZoneDeletion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Retained<CKRecordZoneID>;

        #[method(reason)]
        pub unsafe fn reason(&self) -> CKSyncEngineZoneDeletionReason;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefailedrecordsave?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFailedRecordSave;
);

unsafe impl Send for CKSyncEngineFailedRecordSave {}

unsafe impl Sync for CKSyncEngineFailedRecordSave {}

unsafe impl NSObjectProtocol for CKSyncEngineFailedRecordSave {}

extern_methods!(
    unsafe impl CKSyncEngineFailedRecordSave {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other record)]
        pub unsafe fn record(&self) -> Retained<CKRecord>;

        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Retained<NSError>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefailedzonesave?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFailedZoneSave;
);

unsafe impl Send for CKSyncEngineFailedZoneSave {}

unsafe impl Sync for CKSyncEngineFailedZoneSave {}

unsafe impl NSObjectProtocol for CKSyncEngineFailedZoneSave {}

extern_methods!(
    unsafe impl CKSyncEngineFailedZoneSave {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKRecordZone")]
        #[method_id(@__retain_semantics Other recordZone)]
        pub unsafe fn recordZone(&self) -> Retained<CKRecordZone>;

        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Retained<NSError>;
    }
);
