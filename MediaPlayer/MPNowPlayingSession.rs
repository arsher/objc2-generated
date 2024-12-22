//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-av-foundation")]
use objc2_av_foundation::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpadtimerange?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPAdTimeRange;
);

unsafe impl NSCopying for MPAdTimeRange {}

unsafe impl CopyingHelper for MPAdTimeRange {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MPAdTimeRange {}

extern_methods!(
    unsafe impl MPAdTimeRange {
        #[cfg(feature = "objc2-core-media")]
        /// Represents a time range where an ad break exists in the current player item.
        /// This value must be in bounds of the duration of the current player item.
        #[method(timeRange)]
        pub unsafe fn timeRange(&self) -> CMTimeRange;

        #[cfg(feature = "objc2-core-media")]
        /// Setter for [`timeRange`][Self::timeRange].
        #[method(setTimeRange:)]
        pub unsafe fn setTimeRange(&self, time_range: CMTimeRange);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method_id(@__retain_semantics Init initWithTimeRange:)]
        pub unsafe fn initWithTimeRange(
            this: Allocated<Self>,
            time_range: CMTimeRange,
        ) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpnowplayingsession?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPNowPlayingSession;
);

unsafe impl NSObjectProtocol for MPNowPlayingSession {}

extern_methods!(
    unsafe impl MPNowPlayingSession {
        #[cfg(feature = "objc2-av-foundation")]
        /// Creates a session associated with a given AVPlayer instance. This will assert if players is nil or empty.
        #[method_id(@__retain_semantics Init initWithPlayers:)]
        pub unsafe fn initWithPlayers(
            this: Allocated<Self>,
            players: &NSArray<AVPlayer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-av-foundation")]
        /// AVPlayer instances associated with this session.
        #[method_id(@__retain_semantics Other players)]
        pub unsafe fn players(&self, mtm: MainThreadMarker) -> Retained<NSArray<AVPlayer>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MPNowPlayingSessionDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MPNowPlayingSessionDelegate>>,
        );

        /// When YES, now playing info will be automatically published, and nowPlayingInfoCenter must not be used.
        /// Now playing info keys to be incorporated by automatic publishing can be set on the AVPlayerItem's nowPlayingInfo property.
        #[method(automaticallyPublishesNowPlayingInfo)]
        pub unsafe fn automaticallyPublishesNowPlayingInfo(&self) -> bool;

        /// Setter for [`automaticallyPublishesNowPlayingInfo`][Self::automaticallyPublishesNowPlayingInfo].
        #[method(setAutomaticallyPublishesNowPlayingInfo:)]
        pub unsafe fn setAutomaticallyPublishesNowPlayingInfo(
            &self,
            automatically_publishes_now_playing_info: bool,
        );

        #[cfg(feature = "MPNowPlayingInfoCenter")]
        /// The now playing info center that is associated with this session.
        #[method_id(@__retain_semantics Other nowPlayingInfoCenter)]
        pub unsafe fn nowPlayingInfoCenter(&self) -> Retained<MPNowPlayingInfoCenter>;

        #[cfg(feature = "MPRemoteCommandCenter")]
        /// The remote command center that is associated with this session.
        #[method_id(@__retain_semantics Other remoteCommandCenter)]
        pub unsafe fn remoteCommandCenter(&self) -> Retained<MPRemoteCommandCenter>;

        /// Returns a Boolean value indicating whether this session can become the App's active now playing session.
        #[method(canBecomeActive)]
        pub unsafe fn canBecomeActive(&self) -> bool;

        /// Returns a Boolean value indicating whether this session is the App's active now playing session.
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[cfg(feature = "block2")]
        /// Asks the system to make this session the active now playing sessin for the App.
        #[method(becomeActiveIfPossibleWithCompletion:)]
        pub unsafe fn becomeActiveIfPossibleWithCompletion(
            &self,
            completion: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "objc2-av-foundation")]
        /// Add AVPlayer instance to this session.
        #[method(addPlayer:)]
        pub unsafe fn addPlayer(&self, player: &AVPlayer);

        #[cfg(feature = "objc2-av-foundation")]
        /// Remove AVPlayer instance from this session.
        #[method(removePlayer:)]
        pub unsafe fn removePlayer(&self, player: &AVPlayer);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpnowplayingsessiondelegate?language=objc)
    pub unsafe trait MPNowPlayingSessionDelegate: NSObjectProtocol {
        /// Tells the delegate that the session has changed its active status.
        #[optional]
        #[method(nowPlayingSessionDidChangeActive:)]
        unsafe fn nowPlayingSessionDidChangeActive(
            &self,
            now_playing_session: &MPNowPlayingSession,
        );

        /// Tells the delegate that the session has changed its can become active status.
        #[optional]
        #[method(nowPlayingSessionDidChangeCanBecomeActive:)]
        unsafe fn nowPlayingSessionDidChangeCanBecomeActive(
            &self,
            now_playing_session: &MPNowPlayingSession,
        );
    }

    unsafe impl ProtocolType for dyn MPNowPlayingSessionDelegate {}
);
