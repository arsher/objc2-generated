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

extern "C" {
    /// Deprecated methods that previously returned player IDs will return GKPlayerIDNoLongerAvailable instead.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkplayeridnolongeravailable?language=objc)
    pub static GKPlayerIDNoLongerAvailable: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkplayer?language=objc)
    #[unsafe(super(GKBasePlayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GKBasePlayer")]
    pub struct GKPlayer;
);

#[cfg(feature = "GKBasePlayer")]
unsafe impl NSObjectProtocol for GKPlayer {}

extern_methods!(
    #[cfg(feature = "GKBasePlayer")]
    unsafe impl GKPlayer {
        /// This convenience method checks if the gamePlayerID and the teamPlayerID (scopedIDs) are persistent or unique for the instantiation of this app.
        #[method(scopedIDsArePersistent)]
        pub unsafe fn scopedIDsArePersistent(&self) -> bool;

        /// This is the player's unique and persistent ID that is scoped to this application.
        #[method_id(@__retain_semantics Other gamePlayerID)]
        pub unsafe fn gamePlayerID(&self) -> Retained<NSString>;

        /// This is the player's unique and persistent ID that is scoped to the Apple Store Connect Team identifier of this application.
        #[method_id(@__retain_semantics Other teamPlayerID)]
        pub unsafe fn teamPlayerID(&self) -> Retained<NSString>;

        /// This is player's alias to be displayed. The display name may be very long, so be sure to use appropriate string truncation API when drawing.
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Retained<NSString>;

        /// The alias property contains the player's nickname. When you need to display the name to the user, consider using displayName instead. The nickname is unique but not invariant: the player may change their nickname. The nickname may be very long, so be sure to use appropriate string truncation API when drawing.
        #[method_id(@__retain_semantics Other alias)]
        pub unsafe fn alias(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other anonymousGuestPlayerWithIdentifier:)]
        pub unsafe fn anonymousGuestPlayerWithIdentifier(
            guest_identifier: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other guestIdentifier)]
        pub unsafe fn guestIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(isInvitable)]
        pub unsafe fn isInvitable(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GKBasePlayer")]
    unsafe impl GKPlayer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkphotosize?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKPhotoSize(pub NSInteger);
impl GKPhotoSize {
    #[doc(alias = "GKPhotoSizeSmall")]
    pub const Small: Self = Self(0);
    #[doc(alias = "GKPhotoSizeNormal")]
    pub const Normal: Self = Self(1);
}

unsafe impl Encode for GKPhotoSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKPhotoSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// UI
    #[cfg(feature = "GKBasePlayer")]
    unsafe impl GKPlayer {
        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method(loadPhotoForSize:withCompletionHandler:)]
        pub unsafe fn loadPhotoForSize_withCompletionHandler(
            &self,
            size: GKPhotoSize,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSImage, *mut NSError)>>,
        );
    }
);

extern "C" {
    /// Notification will be posted whenever the player details changes. The object of the notification will be the player.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkplayerdidchangenotificationname?language=objc)
    pub static GKPlayerDidChangeNotificationName: &'static NSNotificationName;
}

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GKBasePlayer")]
    unsafe impl GKPlayer {
        #[deprecated]
        #[method(isFriend)]
        pub unsafe fn isFriend(&self) -> bool;

        #[deprecated = "Use either the gamePlayerID or teamPlayerID property to identify a player."]
        #[method_id(@__retain_semantics Other playerID)]
        pub unsafe fn playerID(&self) -> Retained<NSString>;

        #[cfg(feature = "block2")]
        /// Load the Game Center players for the playerIDs provided. Error will be nil on success.
        /// Possible reasons for error:
        /// 1. Unauthenticated local player
        /// 2. Communications failure
        /// 3. Invalid player identifier
        #[deprecated]
        #[method(loadPlayersForIdentifiers:withCompletionHandler:)]
        pub unsafe fn loadPlayersForIdentifiers_withCompletionHandler(
            identifiers: &NSArray<NSString>,
            completion_handler: Option<
                &block2::Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>,
            >,
        );
    }
);
