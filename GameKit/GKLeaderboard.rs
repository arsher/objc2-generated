//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKLeaderboardTimeScope(pub NSInteger);
impl GKLeaderboardTimeScope {
    #[doc(alias = "GKLeaderboardTimeScopeToday")]
    pub const Today: Self = Self(0);
    #[doc(alias = "GKLeaderboardTimeScopeWeek")]
    pub const Week: Self = Self(1);
    #[doc(alias = "GKLeaderboardTimeScopeAllTime")]
    pub const AllTime: Self = Self(2);
}

unsafe impl Encode for GKLeaderboardTimeScope {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKLeaderboardTimeScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKLeaderboardPlayerScope(pub NSInteger);
impl GKLeaderboardPlayerScope {
    #[doc(alias = "GKLeaderboardPlayerScopeGlobal")]
    pub const Global: Self = Self(0);
    #[doc(alias = "GKLeaderboardPlayerScopeFriendsOnly")]
    pub const FriendsOnly: Self = Self(1);
}

unsafe impl Encode for GKLeaderboardPlayerScope {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKLeaderboardPlayerScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKLeaderboardType(pub NSInteger);
impl GKLeaderboardType {
    #[doc(alias = "GKLeaderboardTypeClassic")]
    pub const Classic: Self = Self(0);
    #[doc(alias = "GKLeaderboardTypeRecurring")]
    pub const Recurring: Self = Self(1);
}

unsafe impl Encode for GKLeaderboardType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKLeaderboardType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKLeaderboard;

    unsafe impl ClassType for GKLeaderboard {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKLeaderboard {}

extern_methods!(
    unsafe impl GKLeaderboard {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other baseLeaderboardID)]
        pub unsafe fn baseLeaderboardID(&self) -> Id<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> GKLeaderboardType;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Option<Id<NSDate>>;

        #[method_id(@__retain_semantics Other nextStartDate)]
        pub unsafe fn nextStartDate(&self) -> Option<Id<NSDate>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(feature = "block2")]
        #[method(loadLeaderboardsWithIDs:completionHandler:)]
        pub unsafe fn loadLeaderboardsWithIDs_completionHandler(
            leaderboard_i_ds: Option<&NSArray<NSString>>,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<GKLeaderboard>, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(loadPreviousOccurrenceWithCompletionHandler:)]
        pub unsafe fn loadPreviousOccurrenceWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut GKLeaderboard, *mut NSError)>,
        );

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(submitScore:context:player:leaderboardIDs:completionHandler:)]
        pub unsafe fn submitScore_context_player_leaderboardIDs_completionHandler(
            score: NSInteger,
            context: NSUInteger,
            player: &GKPlayer,
            leaderboard_i_ds: &NSArray<NSString>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(submitScore:context:player:completionHandler:)]
        pub unsafe fn submitScore_context_player_completionHandler(
            &self,
            score: NSInteger,
            context: NSUInteger,
            player: &GKPlayer,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "GKLeaderboardEntry", feature = "block2"))]
        #[method(loadEntriesForPlayerScope:timeScope:range:completionHandler:)]
        pub unsafe fn loadEntriesForPlayerScope_timeScope_range_completionHandler(
            &self,
            player_scope: GKLeaderboardPlayerScope,
            time_scope: GKLeaderboardTimeScope,
            range: NSRange,
            completion_handler: &block2::Block<
                dyn Fn(
                    *mut GKLeaderboardEntry,
                    *mut NSArray<GKLeaderboardEntry>,
                    NSInteger,
                    *mut NSError,
                ),
            >,
        );

        #[cfg(all(
            feature = "GKBasePlayer",
            feature = "GKLeaderboardEntry",
            feature = "GKPlayer",
            feature = "block2"
        ))]
        #[method(loadEntriesForPlayers:timeScope:completionHandler:)]
        pub unsafe fn loadEntriesForPlayers_timeScope_completionHandler(
            &self,
            players: &NSArray<GKPlayer>,
            time_scope: GKLeaderboardTimeScope,
            completion_handler: &block2::Block<
                dyn Fn(*mut GKLeaderboardEntry, *mut NSArray<GKLeaderboardEntry>, *mut NSError),
            >,
        );
    }
);

extern_methods!(
    /// Deprecated
    unsafe impl GKLeaderboard {
        #[deprecated]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Option<Id<NSString>>;

        #[deprecated]
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithPlayerIDs:)]
        pub unsafe fn initWithPlayerIDs(
            this: Allocated<Self>,
            player_i_ds: Option<&NSArray<NSString>>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(loadCategoriesWithCompletionHandler:)]
        pub unsafe fn loadCategoriesWithCompletionHandler(
            completion_handler: Option<
                &block2::Block<
                    dyn Fn(*mut NSArray<NSString>, *mut NSArray<NSString>, *mut NSError),
                >,
            >,
        );

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(setDefaultLeaderboard:withCompletionHandler:)]
        pub unsafe fn setDefaultLeaderboard_withCompletionHandler(
            leaderboard_identifier: Option<&NSString>,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[deprecated]
        #[method(timeScope)]
        pub unsafe fn timeScope(&self) -> GKLeaderboardTimeScope;

        #[deprecated]
        #[method(setTimeScope:)]
        pub unsafe fn setTimeScope(&self, time_scope: GKLeaderboardTimeScope);

        #[deprecated]
        #[method(playerScope)]
        pub unsafe fn playerScope(&self) -> GKLeaderboardPlayerScope;

        #[deprecated]
        #[method(setPlayerScope:)]
        pub unsafe fn setPlayerScope(&self, player_scope: GKLeaderboardPlayerScope);

        #[deprecated]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[deprecated]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[deprecated]
        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;

        #[deprecated]
        #[method(setRange:)]
        pub unsafe fn setRange(&self, range: NSRange);

        #[cfg(feature = "GKScore")]
        #[deprecated]
        #[method_id(@__retain_semantics Other scores)]
        pub unsafe fn scores(&self) -> Option<Id<NSArray<GKScore>>>;

        #[deprecated]
        #[method(maxRange)]
        pub unsafe fn maxRange(&self) -> NSUInteger;

        #[cfg(feature = "GKScore")]
        #[deprecated]
        #[method_id(@__retain_semantics Other localPlayerScore)]
        pub unsafe fn localPlayerScore(&self) -> Option<Id<GKScore>>;

        #[deprecated]
        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithPlayers:)]
        pub unsafe fn initWithPlayers(
            this: Allocated<Self>,
            players: &NSArray<GKPlayer>,
        ) -> Id<Self>;

        #[cfg(all(feature = "GKScore", feature = "block2"))]
        #[deprecated]
        #[method(loadScoresWithCompletionHandler:)]
        pub unsafe fn loadScoresWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSArray<GKScore>, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(loadLeaderboardsWithCompletionHandler:)]
        pub unsafe fn loadLeaderboardsWithCompletionHandler(
            completion_handler: Option<
                &block2::Block<dyn Fn(*mut NSArray<GKLeaderboard>, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// UI
    unsafe impl GKLeaderboard {
        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        #[method(loadImageWithCompletionHandler:)]
        pub unsafe fn loadImageWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSImage, *mut NSError)>>,
        );
    }
);
