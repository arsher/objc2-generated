//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-cloud-kit")]
use objc2_cloud_kit::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// Sharing
    #[cfg(all(
        feature = "NSPersistentCloudKitContainer",
        feature = "NSPersistentContainer"
    ))]
    unsafe impl NSPersistentCloudKitContainer {
        #[cfg(all(
            feature = "NSPersistentStore",
            feature = "block2",
            feature = "objc2-cloud-kit"
        ))]
        #[method(acceptShareInvitationsFromMetadata:intoPersistentStore:completion:)]
        pub unsafe fn acceptShareInvitationsFromMetadata_intoPersistentStore_completion(
            &self,
            metadata: &NSArray<CKShareMetadata>,
            persistent_store: &NSPersistentStore,
            completion: Option<&block2::Block<dyn Fn(*mut NSArray<CKShareMetadata>, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "NSPersistentStore",
            feature = "block2",
            feature = "objc2-cloud-kit"
        ))]
        #[method(purgeObjectsAndRecordsInZoneWithID:inPersistentStore:completion:)]
        pub unsafe fn purgeObjectsAndRecordsInZoneWithID_inPersistentStore_completion(
            &self,
            zone_id: &CKRecordZoneID,
            persistent_store: Option<&NSPersistentStore>,
            completion: Option<&block2::Block<dyn Fn(*mut CKRecordZoneID, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "NSPersistentStore",
            feature = "block2",
            feature = "objc2-cloud-kit"
        ))]
        #[method(persistUpdatedShare:inPersistentStore:completion:)]
        pub unsafe fn persistUpdatedShare_inPersistentStore_completion(
            &self,
            share: &CKShare,
            persistent_store: &NSPersistentStore,
            completion: Option<&block2::Block<dyn Fn(*mut CKShare, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "NSPersistentStore",
            feature = "block2",
            feature = "objc2-cloud-kit"
        ))]
        #[method(fetchParticipantsMatchingLookupInfos:intoPersistentStore:completion:)]
        pub unsafe fn fetchParticipantsMatchingLookupInfos_intoPersistentStore_completion(
            &self,
            lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
            persistent_store: &NSPersistentStore,
            completion: &block2::Block<dyn Fn(*mut NSArray<CKShareParticipant>, *mut NSError)>,
        );

        #[cfg(all(feature = "NSManagedObjectID", feature = "objc2-cloud-kit"))]
        #[method_id(@__retain_semantics Other fetchSharesMatchingObjectIDs:error:_)]
        pub unsafe fn fetchSharesMatchingObjectIDs_error(
            &self,
            object_i_ds: &NSArray<NSManagedObjectID>,
        ) -> Result<Retained<NSDictionary<NSManagedObjectID, CKShare>>, Retained<NSError>>;

        #[cfg(all(feature = "NSPersistentStore", feature = "objc2-cloud-kit"))]
        #[method_id(@__retain_semantics Other fetchSharesInPersistentStore:error:_)]
        pub unsafe fn fetchSharesInPersistentStore_error(
            &self,
            persistent_store: Option<&NSPersistentStore>,
        ) -> Result<Retained<NSArray<CKShare>>, Retained<NSError>>;

        #[cfg(all(
            feature = "NSManagedObject",
            feature = "NSManagedObjectID",
            feature = "block2",
            feature = "objc2-cloud-kit"
        ))]
        #[method(shareManagedObjects:toShare:completion:)]
        pub unsafe fn shareManagedObjects_toShare_completion(
            &self,
            managed_objects: &NSArray<NSManagedObject>,
            share: Option<&CKShare>,
            completion: &block2::Block<
                dyn Fn(*mut NSSet<NSManagedObjectID>, *mut CKShare, *mut CKContainer, *mut NSError),
            >,
        );
    }
);
