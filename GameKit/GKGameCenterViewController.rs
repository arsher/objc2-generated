//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKGameCenterViewControllerState(pub NSInteger);
impl GKGameCenterViewControllerState {
    #[doc(alias = "GKGameCenterViewControllerStateDefault")]
    pub const Default: Self = Self(-1);
    #[doc(alias = "GKGameCenterViewControllerStateLeaderboards")]
    pub const Leaderboards: Self = Self(0);
    #[doc(alias = "GKGameCenterViewControllerStateAchievements")]
    pub const Achievements: Self = Self(1);
    #[doc(alias = "GKGameCenterViewControllerStateChallenges")]
    pub const Challenges: Self = Self(2);
    #[doc(alias = "GKGameCenterViewControllerStateLocalPlayerProfile")]
    pub const LocalPlayerProfile: Self = Self(3);
    #[doc(alias = "GKGameCenterViewControllerStateDashboard")]
    pub const Dashboard: Self = Self(4);
    #[doc(alias = "GKGameCenterViewControllerStateLocalPlayerFriendsList")]
    pub const LocalPlayerFriendsList: Self = Self(5);
}

unsafe impl Encode for GKGameCenterViewControllerState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKGameCenterViewControllerState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct GKGameCenterViewController;
);

#[cfg(all(feature = "GKDialogController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl GKViewController for GKGameCenterViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for GKGameCenterViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for GKGameCenterViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for GKGameCenterViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for GKGameCenterViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for GKGameCenterViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKGameCenterViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKGameCenterViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKGameCenterViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKGameCenterViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKGameCenterViewController {
        #[method_id(@__retain_semantics Other gameCenterDelegate)]
        pub unsafe fn gameCenterDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn GKGameCenterControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setGameCenterDelegate:)]
        pub unsafe fn setGameCenterDelegate(
            &self,
            game_center_delegate: Option<&ProtocolObject<dyn GKGameCenterControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Init initWithState:)]
        pub unsafe fn initWithState(
            this: Allocated<Self>,
            state: GKGameCenterViewControllerState,
        ) -> Retained<Self>;

        #[cfg(feature = "GKLeaderboard")]
        #[method_id(@__retain_semantics Init initWithLeaderboardID:playerScope:timeScope:)]
        pub unsafe fn initWithLeaderboardID_playerScope_timeScope(
            this: Allocated<Self>,
            leaderboard_id: &NSString,
            player_scope: GKLeaderboardPlayerScope,
            time_scope: GKLeaderboardTimeScope,
        ) -> Retained<Self>;

        #[cfg(feature = "GKLeaderboard")]
        #[method_id(@__retain_semantics Init initWithLeaderboard:playerScope:)]
        pub unsafe fn initWithLeaderboard_playerScope(
            this: Allocated<Self>,
            leaderboard: &GKLeaderboard,
            player_scope: GKLeaderboardPlayerScope,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLeaderboardSetID:)]
        pub unsafe fn initWithLeaderboardSetID(
            this: Allocated<Self>,
            leaderboard_set_id: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAchievementID:)]
        pub unsafe fn initWithAchievementID(
            this: Allocated<Self>,
            achievement_id: &NSString,
        ) -> Retained<Self>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Init initWithPlayer:)]
        pub unsafe fn initWithPlayer(this: Allocated<Self>, player: &GKPlayer) -> Retained<Self>;
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKGameCenterViewController {
        #[deprecated]
        #[method(viewState)]
        pub unsafe fn viewState(&self) -> GKGameCenterViewControllerState;

        #[deprecated]
        #[method(setViewState:)]
        pub unsafe fn setViewState(&self, view_state: GKGameCenterViewControllerState);

        #[cfg(feature = "GKLeaderboard")]
        #[deprecated]
        #[method(leaderboardTimeScope)]
        pub unsafe fn leaderboardTimeScope(&self) -> GKLeaderboardTimeScope;

        #[cfg(feature = "GKLeaderboard")]
        #[deprecated]
        #[method(setLeaderboardTimeScope:)]
        pub unsafe fn setLeaderboardTimeScope(
            &self,
            leaderboard_time_scope: GKLeaderboardTimeScope,
        );

        #[deprecated]
        #[method_id(@__retain_semantics Other leaderboardIdentifier)]
        pub unsafe fn leaderboardIdentifier(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method(setLeaderboardIdentifier:)]
        pub unsafe fn setLeaderboardIdentifier(&self, leaderboard_identifier: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other leaderboardCategory)]
        pub unsafe fn leaderboardCategory(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method(setLeaderboardCategory:)]
        pub unsafe fn setLeaderboardCategory(&self, leaderboard_category: Option<&NSString>);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkgamecentercontrollerdelegate?language=objc)
    pub unsafe trait GKGameCenterControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(gameCenterViewControllerDidFinish:)]
        unsafe fn gameCenterViewControllerDidFinish(
            &self,
            game_center_view_controller: &GKGameCenterViewController,
        );
    }

    unsafe impl ProtocolType for dyn GKGameCenterControllerDelegate {}
);
