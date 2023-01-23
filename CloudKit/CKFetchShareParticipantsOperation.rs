//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
    pub struct CKFetchShareParticipantsOperation;

    #[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
    unsafe impl ClassType for CKFetchShareParticipantsOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
    unsafe impl CKFetchShareParticipantsOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithUserIdentityLookupInfos:)]
        pub unsafe fn initWithUserIdentityLookupInfos(
            this: Option<Allocated<Self>>,
            user_identity_lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
        ) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other userIdentityLookupInfos)]
        pub unsafe fn userIdentityLookupInfos(
            &self,
        ) -> Option<Id<NSArray<CKUserIdentityLookupInfo>, Shared>>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[method(setUserIdentityLookupInfos:)]
        pub unsafe fn setUserIdentityLookupInfos(
            &self,
            user_identity_lookup_infos: Option<&NSArray<CKUserIdentityLookupInfo>>,
        );

        #[cfg(feature = "CloudKit_CKShareParticipant")]
        #[deprecated = "Use perShareParticipantCompletionBlock instead, which surfaces per-share-participant errors"]
        #[method(shareParticipantFetchedBlock)]
        pub unsafe fn shareParticipantFetchedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKShareParticipant>,), ()>;

        #[cfg(feature = "CloudKit_CKShareParticipant")]
        #[deprecated = "Use perShareParticipantCompletionBlock instead, which surfaces per-share-participant errors"]
        #[method(setShareParticipantFetchedBlock:)]
        pub unsafe fn setShareParticipantFetchedBlock(
            &self,
            share_participant_fetched_block: Option<&Block<(NonNull<CKShareParticipant>,), ()>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSError"
        ))]
        #[method(perShareParticipantCompletionBlock)]
        pub unsafe fn perShareParticipantCompletionBlock(
            &self,
        ) -> *mut Block<
            (
                NonNull<CKUserIdentityLookupInfo>,
                *mut CKShareParticipant,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSError"
        ))]
        #[method(setPerShareParticipantCompletionBlock:)]
        pub unsafe fn setPerShareParticipantCompletionBlock(
            &self,
            per_share_participant_completion_block: Option<
                &Block<
                    (
                        NonNull<CKUserIdentityLookupInfo>,
                        *mut CKShareParticipant,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(fetchShareParticipantsCompletionBlock)]
        pub unsafe fn fetchShareParticipantsCompletionBlock(
            &self,
        ) -> *mut Block<(*mut NSError,), ()>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(setFetchShareParticipantsCompletionBlock:)]
        pub unsafe fn setFetchShareParticipantsCompletionBlock(
            &self,
            fetch_share_participants_completion_block: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);