//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    pub struct GKLocalPlayer;

    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl ClassType for GKLocalPlayer {
        #[inherits(GKBasePlayer, NSObject)]
        type Super = GKPlayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKLocalPlayer")]
unsafe impl NSObjectProtocol for GKLocalPlayer {}

extern_methods!(
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[method_id(@__retain_semantics Other local)]
        pub unsafe fn local() -> Id<GKLocalPlayer>;

        #[method_id(@__retain_semantics Other localPlayer)]
        pub unsafe fn localPlayer() -> Id<GKLocalPlayer>;

        #[method(isAuthenticated)]
        pub unsafe fn isAuthenticated(&self) -> bool;

        #[method(isUnderage)]
        pub unsafe fn isUnderage(&self) -> bool;

        #[method(isMultiplayerGamingRestricted)]
        pub unsafe fn isMultiplayerGamingRestricted(&self) -> bool;

        #[method(isPersonalizedCommunicationRestricted)]
        pub unsafe fn isPersonalizedCommunicationRestricted(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadRecentPlayersWithCompletionHandler:)]
        pub unsafe fn loadRecentPlayersWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSArray<GKPlayer>, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadChallengableFriendsWithCompletionHandler:)]
        pub unsafe fn loadChallengableFriendsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSArray<GKPlayer>, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(setDefaultLeaderboardIdentifier:completionHandler:)]
        pub unsafe fn setDefaultLeaderboardIdentifier_completionHandler(
            &self,
            leaderboard_identifier: &NSString,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(loadDefaultLeaderboardIdentifierWithCompletionHandler:)]
        pub unsafe fn loadDefaultLeaderboardIdentifierWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSString, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(fetchItemsForIdentityVerificationSignature:)]
        pub unsafe fn fetchItemsForIdentityVerificationSignature(
            &self,
            completion_handler: Option<
                &Block<(*mut NSURL, *mut NSData, *mut NSData, u64, *mut NSError), ()>,
            >,
        );
    }
);

extern_protocol!(
    pub unsafe trait GKLocalPlayerListener:
        GKChallengeListener + GKInviteEventListener + GKSavedGameListener + GKTurnBasedEventListener
    {
    }

    unsafe impl ProtocolType for dyn GKLocalPlayerListener {}
);

extern_methods!(
    /// GKLocalPlayerEvents
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[method(registerListener:)]
        pub unsafe fn registerListener(&self, listener: &ProtocolObject<dyn GKLocalPlayerListener>);

        #[method(unregisterListener:)]
        pub unsafe fn unregisterListener(
            &self,
            listener: &ProtocolObject<dyn GKLocalPlayerListener>,
        );

        #[method(unregisterAllListeners)]
        pub unsafe fn unregisterAllListeners(&self);
    }
);

extern_static!(GKPlayerAuthenticationDidChangeNotificationName: &'static NSNotificationName);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated = "Use setDefaultLeaderboardIdentifier:completionHandler: instead"]
        #[method(setDefaultLeaderboardCategoryID:completionHandler:)]
        pub unsafe fn setDefaultLeaderboardCategoryID_completionHandler(
            &self,
            category_id: Option<&NSString>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated = "Use loadDefaultLeaderboardIdentifierWithCompletionHandler: instead"]
        #[method(loadDefaultLeaderboardCategoryIDWithCompletionHandler:)]
        pub unsafe fn loadDefaultLeaderboardCategoryIDWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSString, *mut NSError), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "Set the authenticateHandler instead"]
        #[method(authenticateWithCompletionHandler:)]
        pub unsafe fn authenticateWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKPlayer"
        ))]
        #[deprecated]
        #[method(loadFriendPlayersWithCompletionHandler:)]
        pub unsafe fn loadFriendPlayersWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSArray<GKPlayer>, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[deprecated = "API deprecated. Use fetchItemsForIdentityVerificationSignature: and the teamPlayerID value to verify a user identity."]
        #[method(generateIdentityVerificationSignatureWithCompletionHandler:)]
        pub unsafe fn generateIdentityVerificationSignatureWithCompletionHandler(
            &self,
            completion_handler: Option<
                &Block<(*mut NSURL, *mut NSData, *mut NSData, u64, *mut NSError), ()>,
            >,
        );
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "This is never invoked and its implementation does nothing, use loadRecentPlayersWithCompletionHandler: instead"]
        #[method(loadFriendsWithCompletionHandler:)]
        pub unsafe fn loadFriendsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSArray<NSString>, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = " This property is obsolete, Use loadFriendPlayersWithCompletionHandler: instead"]
        #[method_id(@__retain_semantics Other friends)]
        pub unsafe fn friends(&self) -> Option<Id<NSArray<NSString>>>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKFriendsAuthorizationStatus {
        GKFriendsAuthorizationStatusNotDetermined = 0,
        GKFriendsAuthorizationStatusRestricted = 1,
        GKFriendsAuthorizationStatusDenied = 2,
        GKFriendsAuthorizationStatusAuthorized = 3,
    }
);

extern_methods!(
    /// FriendsList
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(feature = "Foundation_NSError")]
        #[method(loadFriendsAuthorizationStatus:)]
        pub unsafe fn loadFriendsAuthorizationStatus(
            &self,
            completion_handler: &Block<(GKFriendsAuthorizationStatus, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKPlayer"
        ))]
        #[method(loadFriends:)]
        pub unsafe fn loadFriends(
            &self,
            completion_handler: &Block<(*mut NSArray<GKPlayer>, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKPlayer"
        ))]
        #[method(loadFriendsWithIdentifiers:completionHandler:)]
        pub unsafe fn loadFriendsWithIdentifiers_completionHandler(
            &self,
            identifiers: &NSArray<NSString>,
            completion_handler: &Block<(*mut NSArray<GKPlayer>, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// UI
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(all(feature = "AppKit_NSViewController", feature = "Foundation_NSError"))]
        #[method(authenticateHandler)]
        pub unsafe fn authenticateHandler(
            &self,
        ) -> *mut Block<(*mut NSViewController, *mut NSError), ()>;

        #[cfg(all(feature = "AppKit_NSViewController", feature = "Foundation_NSError"))]
        #[method(setAuthenticateHandler:)]
        pub unsafe fn setAuthenticateHandler(
            &self,
            authenticate_handler: Option<&Block<(*mut NSViewController, *mut NSError), ()>>,
        );

        #[method(isPresentingFriendRequestViewController)]
        pub unsafe fn isPresentingFriendRequestViewController(&self) -> bool;

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSError"))]
        #[method(presentFriendRequestCreatorFromWindow:error:_)]
        pub unsafe fn presentFriendRequestCreatorFromWindow_error(
            &self,
            window: Option<&NSWindow>,
        ) -> Result<(), Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `GKPlayer`
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    unsafe impl GKLocalPlayer {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other anonymousGuestPlayerWithIdentifier:)]
        pub unsafe fn anonymousGuestPlayerWithIdentifier(guest_identifier: &NSString) -> Id<Self>;
    }
);
