//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// Stand-in for the current user's ID; most often used in RecordZoneID->ownerName
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckcurrentuserdefaultname?language=objc)
    pub static CKCurrentUserDefaultName: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckownerdefaultname?language=objc)
    pub static CKOwnerDefaultName: &'static NSString;
}

extern_class!(
    /// A CKContainer, and its CKDatabases, are the main entry points into the CloudKit framework.
    ///
    ///
    /// Several methods in CloudKit accept completion handlers to indicate when they're completed.
    /// All CKOperation subclasses include progress and completion blocks to report significant events in their lifecycles.
    /// Each of these handlers and blocks is invoked on a non-main serial queue.  The receiver is responsible for handling the message on a different queue or thread if it is required.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckcontainer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKContainer;
);

unsafe impl Send for CKContainer {}

unsafe impl Sync for CKContainer {}

unsafe impl NSObjectProtocol for CKContainer {}

extern_methods!(
    unsafe impl CKContainer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Convenience method that uses the calling process' "iCloud.\(application-identifier)" as the container identifier
        ///
        ///
        /// application-identifier is the calling process'
        /// `application-identifier`entitlement on iOS / tvOS / watchOS.
        /// application-identifier is the calling process'
        /// `com.apple.application-identifier`entitlement on macOS.
        /// On all OSes, if an
        /// `com.apple.developer.associated-application-identifier`entitlement is present, its value will be preferred over the
        /// `application-identifier`variants.
        #[method_id(@__retain_semantics Other defaultContainer)]
        pub unsafe fn defaultContainer() -> Retained<CKContainer>;

        /// Obtain a CKContainer for the given containerIdentifier
        ///
        ///
        /// If the application is in production mode (aka,
        /// `com.apple.developer.icloud-container-environment`is set to Production in your entitlements plist, and you have no override in
        /// `com.apple.developer.icloud-container-development-container-identifiers),`then the production environment is used.
        #[method_id(@__retain_semantics Other containerWithIdentifier:)]
        pub unsafe fn containerWithIdentifier(
            container_identifier: &NSString,
        ) -> Retained<CKContainer>;

        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "CKOperation")]
        #[method(addOperation:)]
        pub unsafe fn addOperation(&self, operation: &CKOperation);
    }
);

extern_methods!(
    /// Database
    /// Database properties:
    /// Records in a public database
    /// - By default are world readable, owner writable.
    /// - Can be locked down by Roles, a process done in the Developer Portal, a web interface.  Roles are not present in the client API.
    /// - Are visible to the application developer via the Developer Portal.
    /// - Do not contribute to the owner's iCloud account storage quota.
    /// Records in a private database
    /// - By default are only owner readable and owner writable.
    /// - Are not visible to the application developer via the Developer Portal.
    /// - Are counted towards the owner's iCloud account storage quota.
    /// Records in a shared database
    /// - Are available to share participants based on the permissions of the enclosing CKShare
    /// - Are not visible to the application developer via the Developer Portal.
    /// - Are counted towards the originating owner's iCloud account storage quota.
    unsafe impl CKContainer {
        #[cfg(feature = "CKDatabase")]
        #[method_id(@__retain_semantics Other privateCloudDatabase)]
        pub unsafe fn privateCloudDatabase(&self) -> Retained<CKDatabase>;

        #[cfg(feature = "CKDatabase")]
        #[method_id(@__retain_semantics Other publicCloudDatabase)]
        pub unsafe fn publicCloudDatabase(&self) -> Retained<CKDatabase>;

        #[cfg(feature = "CKDatabase")]
        #[method_id(@__retain_semantics Other sharedCloudDatabase)]
        pub unsafe fn sharedCloudDatabase(&self) -> Retained<CKDatabase>;

        #[cfg(feature = "CKDatabase")]
        /// Convenience methods
        ///
        ///
        /// Returns: a database that's pointer-equal to one of the above properties
        #[method_id(@__retain_semantics Other databaseWithDatabaseScope:)]
        pub unsafe fn databaseWithDatabaseScope(
            &self,
            database_scope: CKDatabaseScope,
        ) -> Retained<CKDatabase>;
    }
);

/// credentials in Settings app.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckaccountstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKAccountStatus(pub NSInteger);
impl CKAccountStatus {
    #[doc(alias = "CKAccountStatusCouldNotDetermine")]
    pub const CouldNotDetermine: Self = Self(0);
    #[doc(alias = "CKAccountStatusAvailable")]
    pub const Available: Self = Self(1);
    #[doc(alias = "CKAccountStatusRestricted")]
    pub const Restricted: Self = Self(2);
    #[doc(alias = "CKAccountStatusNoAccount")]
    pub const NoAccount: Self = Self(3);
    #[doc(alias = "CKAccountStatusTemporarilyUnavailable")]
    pub const TemporarilyUnavailable: Self = Self(4);
}

unsafe impl Encode for CKAccountStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKAccountStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// This local notification is posted when there has been any change to the logged in iCloud account.
    ///
    ///
    /// On receipt, an updated account status should be obtained by calling
    /// `accountStatusWithCompletionHandler:`
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckaccountchangednotification?language=objc)
    pub static CKAccountChangedNotification: &'static NSString;
}

extern_methods!(
    /// AccountStatus
    unsafe impl CKContainer {
        #[cfg(feature = "block2")]
        #[method(accountStatusWithCompletionHandler:)]
        pub unsafe fn accountStatusWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(CKAccountStatus, *mut NSError)>,
        );
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckapplicationpermissions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKApplicationPermissions(pub NSUInteger);
bitflags::bitflags! {
    impl CKApplicationPermissions: NSUInteger {
/// Allows the user's record in CloudKit to be discoverable via the user's email address
#[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        const CKApplicationPermissionUserDiscoverability = 1<<0;
    }
}

unsafe impl Encode for CKApplicationPermissions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CKApplicationPermissions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckapplicationpermissionstatus?language=objc)
// NS_ENUM
#[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKApplicationPermissionStatus(pub NSInteger);
impl CKApplicationPermissionStatus {
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    #[doc(alias = "CKApplicationPermissionStatusInitialState")]
    pub const InitialState: Self = Self(0);
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    #[doc(alias = "CKApplicationPermissionStatusCouldNotComplete")]
    pub const CouldNotComplete: Self = Self(1);
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    #[doc(alias = "CKApplicationPermissionStatusDenied")]
    pub const Denied: Self = Self(2);
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    #[doc(alias = "CKApplicationPermissionStatusGranted")]
    pub const Granted: Self = Self(3);
}

unsafe impl Encode for CKApplicationPermissionStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKApplicationPermissionStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckapplicationpermissionblock?language=objc)
#[cfg(feature = "block2")]
pub type CKApplicationPermissionBlock =
    *mut block2::Block<dyn Fn(CKApplicationPermissionStatus, *mut NSError)>;

extern_methods!(
    /// ApplicationPermission
    unsafe impl CKContainer {
        #[cfg(feature = "block2")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(statusForApplicationPermission:completionHandler:)]
        pub unsafe fn statusForApplicationPermission_completionHandler(
            &self,
            application_permission: CKApplicationPermissions,
            completion_handler: CKApplicationPermissionBlock,
        );

        #[cfg(feature = "block2")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(requestApplicationPermission:completionHandler:)]
        pub unsafe fn requestApplicationPermission_completionHandler(
            &self,
            application_permission: CKApplicationPermissions,
            completion_handler: CKApplicationPermissionBlock,
        );
    }
);

extern_methods!(
    /// UserRecords
    unsafe impl CKContainer {
        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        /// If there is no iCloud account configured, or if access is restricted, a
        /// `CKErrorNotAuthenticated`error will be returned.
        ///
        /// This work is treated as having
        /// `NSQualityOfServiceUserInitiated`quality of service.
        #[method(fetchUserRecordIDWithCompletionHandler:)]
        pub unsafe fn fetchUserRecordIDWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut CKRecordID, *mut NSError)>,
        );

        #[cfg(all(feature = "CKUserIdentity", feature = "block2"))]
        /// Fetches all user identities that match an entry in the user's contacts database.
        ///
        ///
        /// `CKDiscoverAllUserIdentitiesOperation`is the more configurable,
        /// `CKOperation`-based alternative to this methods
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverAllIdentitiesWithCompletionHandler:)]
        pub unsafe fn discoverAllIdentitiesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<CKUserIdentity>, *mut NSError)>,
        );

        #[cfg(all(feature = "CKUserIdentity", feature = "block2"))]
        /// Fetches the user identity that corresponds to the given email address.
        ///
        ///
        /// Only users who have opted-in to user discoverability will have their identities returned by this method.  If a user with the inputted email exists in iCloud, but has not opted-in to user discoverability, this method completes with a nil
        /// `userInfo.``CKDiscoverUserIdentitiesOperation`is the more configurable,
        /// `CKOperation`-based alternative to this method
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverUserIdentityWithEmailAddress:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithEmailAddress_completionHandler(
            &self,
            email: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut CKUserIdentity, *mut NSError)>,
        );

        #[cfg(all(feature = "CKUserIdentity", feature = "block2"))]
        /// Fetches the user identity that corresponds to the given phone number.
        ///
        ///
        /// Only users who have opted-in to user discoverability will have their identities returned by this method.  If a user with the inputted phone number exists in iCloud, but has not opted-in to user discoverability, this method completes with a nil
        /// `userInfo.``CKDiscoverUserIdentitiesOperation`is the more configurable,
        /// `CKOperation`-based alternative to this method
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverUserIdentityWithPhoneNumber:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithPhoneNumber_completionHandler(
            &self,
            phone_number: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut CKUserIdentity, *mut NSError)>,
        );

        #[cfg(all(feature = "CKRecordID", feature = "CKUserIdentity", feature = "block2"))]
        /// Fetches the user identity that corresponds to the given user record id.
        ///
        ///
        /// Only users who have opted-in to user discoverability will have their identities returned by this method.  If a user has not opted-in to user discoverability, this method completes with a nil
        /// `userInfo.``CKDiscoverUserIdentitiesOperation`is the more configurable,
        /// `CKOperation`-based alternative to this method
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverUserIdentityWithUserRecordID:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithUserRecordID_completionHandler(
            &self,
            user_record_id: &CKRecordID,
            completion_handler: &block2::Block<dyn Fn(*mut CKUserIdentity, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Sharing
    unsafe impl CKContainer {
        #[cfg(all(feature = "CKShareParticipant", feature = "block2"))]
        /// Fetches share participants matching the provided info.
        ///
        ///
        /// `CKFetchShareParticipantsOperation`is the more configurable,
        /// `CKOperation`-based alternative to these methods.
        #[method(fetchShareParticipantWithEmailAddress:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithEmailAddress_completionHandler(
            &self,
            email_address: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut CKShareParticipant, *mut NSError)>,
        );

        #[cfg(all(feature = "CKShareParticipant", feature = "block2"))]
        #[method(fetchShareParticipantWithPhoneNumber:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithPhoneNumber_completionHandler(
            &self,
            phone_number: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut CKShareParticipant, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CKRecordID",
            feature = "CKShareParticipant",
            feature = "block2"
        ))]
        #[method(fetchShareParticipantWithUserRecordID:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithUserRecordID_completionHandler(
            &self,
            user_record_id: &CKRecordID,
            completion_handler: &block2::Block<dyn Fn(*mut CKShareParticipant, *mut NSError)>,
        );

        #[cfg(all(feature = "CKShareMetadata", feature = "block2"))]
        #[method(fetchShareMetadataWithURL:completionHandler:)]
        pub unsafe fn fetchShareMetadataWithURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &block2::Block<dyn Fn(*mut CKShareMetadata, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CKRecord",
            feature = "CKShare",
            feature = "CKShareMetadata",
            feature = "block2"
        ))]
        #[method(acceptShareMetadata:completionHandler:)]
        pub unsafe fn acceptShareMetadata_completionHandler(
            &self,
            metadata: &CKShareMetadata,
            completion_handler: &block2::Block<dyn Fn(*mut CKShare, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// CKLongLivedOperations
    unsafe impl CKContainer {
        #[cfg(all(feature = "CKOperation", feature = "block2"))]
        /// Long lived CKOperations returned by this call must be started on an operation queue.
        /// Remember to set the callback blocks before starting the operation.
        /// If an operation has already completed against the server, and is subsequently resumed, that operation will replay all of its callbacks from the start of the operation, but the request will not be re-sent to the server.
        /// If a long lived operation is cancelled or finishes completely it is no longer returned by these calls.
        #[method(fetchAllLongLivedOperationIDsWithCompletionHandler:)]
        pub unsafe fn fetchAllLongLivedOperationIDsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<CKOperationID>, *mut NSError)>,
        );

        #[cfg(all(feature = "CKOperation", feature = "block2"))]
        #[method(fetchLongLivedOperationWithID:completionHandler:)]
        pub unsafe fn fetchLongLivedOperationWithID_completionHandler(
            &self,
            operation_id: &CKOperationID,
            completion_handler: &block2::Block<dyn Fn(*mut CKOperation, *mut NSError)>,
        );
    }
);
