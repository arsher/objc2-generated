//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKNotificationID")]
    pub struct CKNotificationID;

    #[cfg(feature = "CloudKit_CKNotificationID")]
    unsafe impl ClassType for CKNotificationID {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKNotificationID")]
unsafe impl NSCoding for CKNotificationID {}

#[cfg(feature = "CloudKit_CKNotificationID")]
unsafe impl NSCopying for CKNotificationID {}

#[cfg(feature = "CloudKit_CKNotificationID")]
unsafe impl NSObjectProtocol for CKNotificationID {}

#[cfg(feature = "CloudKit_CKNotificationID")]
unsafe impl NSSecureCoding for CKNotificationID {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKNotificationID")]
    unsafe impl CKNotificationID {}
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKNotificationType {
        CKNotificationTypeQuery = 1,
        CKNotificationTypeRecordZone = 2,
        CKNotificationTypeReadNotification = 3,
        CKNotificationTypeDatabase = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKNotification")]
    pub struct CKNotification;

    #[cfg(feature = "CloudKit_CKNotification")]
    unsafe impl ClassType for CKNotification {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKNotification")]
unsafe impl NSObjectProtocol for CKNotification {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKNotification")]
    unsafe impl CKNotification {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other notificationFromRemoteNotificationDictionary:)]
        pub unsafe fn notificationFromRemoteNotificationDictionary(
            notification_dictionary: &NSDictionary,
        ) -> Option<Id<Self>>;

        #[method(notificationType)]
        pub unsafe fn notificationType(&self) -> CKNotificationType;

        #[cfg(feature = "CloudKit_CKNotificationID")]
        #[method_id(@__retain_semantics Other notificationID)]
        pub unsafe fn notificationID(&self) -> Option<Id<CKNotificationID>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other subscriptionOwnerUserRecordID)]
        pub unsafe fn subscriptionOwnerUserRecordID(&self) -> Option<Id<CKRecordID>>;

        #[method(isPruned)]
        pub unsafe fn isPruned(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alertBody)]
        pub unsafe fn alertBody(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alertLocalizationKey)]
        pub unsafe fn alertLocalizationKey(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other alertLocalizationArgs)]
        pub unsafe fn alertLocalizationArgs(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other titleLocalizationKey)]
        pub unsafe fn titleLocalizationKey(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other titleLocalizationArgs)]
        pub unsafe fn titleLocalizationArgs(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitleLocalizationKey)]
        pub unsafe fn subtitleLocalizationKey(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other subtitleLocalizationArgs)]
        pub unsafe fn subtitleLocalizationArgs(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alertActionLocalizationKey)]
        pub unsafe fn alertActionLocalizationKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alertLaunchImage)]
        pub unsafe fn alertLaunchImage(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other badge)]
        pub unsafe fn badge(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other soundName)]
        pub unsafe fn soundName(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other subscriptionID)]
        pub unsafe fn subscriptionID(&self) -> Option<Id<CKSubscriptionID>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Option<Id<NSString>>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKQueryNotificationReason {
        CKQueryNotificationReasonRecordCreated = 1,
        CKQueryNotificationReasonRecordUpdated = 2,
        CKQueryNotificationReasonRecordDeleted = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKQueryNotification")]
    pub struct CKQueryNotification;

    #[cfg(feature = "CloudKit_CKQueryNotification")]
    unsafe impl ClassType for CKQueryNotification {
        #[inherits(NSObject)]
        type Super = CKNotification;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKQueryNotification")]
unsafe impl NSObjectProtocol for CKQueryNotification {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKQueryNotification")]
    unsafe impl CKQueryNotification {
        #[method(queryNotificationReason)]
        pub unsafe fn queryNotificationReason(&self) -> CKQueryNotificationReason;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other recordFields)]
        pub unsafe fn recordFields(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other recordID)]
        pub unsafe fn recordID(&self) -> Option<Id<CKRecordID>>;

        #[method(databaseScope)]
        pub unsafe fn databaseScope(&self) -> CKDatabaseScope;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKRecordZoneNotification")]
    pub struct CKRecordZoneNotification;

    #[cfg(feature = "CloudKit_CKRecordZoneNotification")]
    unsafe impl ClassType for CKRecordZoneNotification {
        #[inherits(NSObject)]
        type Super = CKNotification;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKRecordZoneNotification")]
unsafe impl NSObjectProtocol for CKRecordZoneNotification {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKRecordZoneNotification")]
    unsafe impl CKRecordZoneNotification {
        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other recordZoneID)]
        pub unsafe fn recordZoneID(&self) -> Option<Id<CKRecordZoneID>>;

        #[method(databaseScope)]
        pub unsafe fn databaseScope(&self) -> CKDatabaseScope;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKDatabaseNotification")]
    pub struct CKDatabaseNotification;

    #[cfg(feature = "CloudKit_CKDatabaseNotification")]
    unsafe impl ClassType for CKDatabaseNotification {
        #[inherits(NSObject)]
        type Super = CKNotification;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKDatabaseNotification")]
unsafe impl NSObjectProtocol for CKDatabaseNotification {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKDatabaseNotification")]
    unsafe impl CKDatabaseNotification {
        #[method(databaseScope)]
        pub unsafe fn databaseScope(&self) -> CKDatabaseScope;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKNotification`
    #[cfg(feature = "CloudKit_CKQueryNotification")]
    unsafe impl CKQueryNotification {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other notificationFromRemoteNotificationDictionary:)]
        pub unsafe fn notificationFromRemoteNotificationDictionary(
            notification_dictionary: &NSDictionary,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKNotification`
    #[cfg(feature = "CloudKit_CKRecordZoneNotification")]
    unsafe impl CKRecordZoneNotification {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other notificationFromRemoteNotificationDictionary:)]
        pub unsafe fn notificationFromRemoteNotificationDictionary(
            notification_dictionary: &NSDictionary,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKNotification`
    #[cfg(feature = "CloudKit_CKDatabaseNotification")]
    unsafe impl CKDatabaseNotification {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other notificationFromRemoteNotificationDictionary:)]
        pub unsafe fn notificationFromRemoteNotificationDictionary(
            notification_dictionary: &NSDictionary,
        ) -> Option<Id<Self>>;
    }
);
