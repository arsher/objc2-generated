//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayeritemsegmenttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVPlayerItemSegmentType(pub NSInteger);
impl AVPlayerItemSegmentType {
    #[doc(alias = "AVPlayerItemSegmentTypePrimary")]
    pub const Primary: Self = Self(0);
    #[doc(alias = "AVPlayerItemSegmentTypeInterstitial")]
    pub const Interstitial: Self = Self(1);
}

unsafe impl Encode for AVPlayerItemSegmentType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVPlayerItemSegmentType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayeritemsegment?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPlayerItemSegment;
);

unsafe impl Send for AVPlayerItemSegment {}

unsafe impl Sync for AVPlayerItemSegment {}

unsafe impl NSObjectProtocol for AVPlayerItemSegment {}

extern_methods!(
    unsafe impl AVPlayerItemSegment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(segmentType)]
        pub unsafe fn segmentType(&self) -> AVPlayerItemSegmentType;

        #[cfg(feature = "objc2-core-media")]
        #[method(timeMapping)]
        pub unsafe fn timeMapping(&self) -> CMTimeMapping;

        #[method_id(@__retain_semantics Other loadedTimeRanges)]
        pub unsafe fn loadedTimeRanges(&self) -> Retained<NSArray<NSValue>>;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "AVPlayerInterstitialEventController")]
        #[method_id(@__retain_semantics Other interstitialEvent)]
        pub unsafe fn interstitialEvent(&self) -> Option<Retained<AVPlayerInterstitialEvent>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayeritemintegratedtimelinesnapshot?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPlayerItemIntegratedTimelineSnapshot;
);

unsafe impl Send for AVPlayerItemIntegratedTimelineSnapshot {}

unsafe impl Sync for AVPlayerItemIntegratedTimelineSnapshot {}

unsafe impl NSObjectProtocol for AVPlayerItemIntegratedTimelineSnapshot {}

extern_methods!(
    unsafe impl AVPlayerItemIntegratedTimelineSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method(duration)]
        pub unsafe fn duration(&self) -> CMTime;

        #[method_id(@__retain_semantics Other currentSegment)]
        pub unsafe fn currentSegment(&self) -> Option<Retained<AVPlayerItemSegment>>;

        #[method_id(@__retain_semantics Other segments)]
        pub unsafe fn segments(&self) -> Retained<NSArray<AVPlayerItemSegment>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(currentTime)]
        pub unsafe fn currentTime(&self) -> CMTime;

        #[method_id(@__retain_semantics Other currentDate)]
        pub unsafe fn currentDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(mapTime:toSegment:atSegmentOffset:)]
        pub unsafe fn mapTime_toSegment_atSegmentOffset(
            &self,
            time: CMTime,
            time_segment_out: Option<&mut Retained<AVPlayerItemSegment>>,
            segment_offset_out: NonNull<CMTime>,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayeritemintegratedtimeline?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPlayerItemIntegratedTimeline;
);

unsafe impl Send for AVPlayerItemIntegratedTimeline {}

unsafe impl Sync for AVPlayerItemIntegratedTimeline {}

unsafe impl NSObjectProtocol for AVPlayerItemIntegratedTimeline {}

extern_methods!(
    unsafe impl AVPlayerItemIntegratedTimeline {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other currentSnapshot)]
        pub unsafe fn currentSnapshot(&self) -> Retained<AVPlayerItemIntegratedTimelineSnapshot>;

        #[cfg(feature = "objc2-core-media")]
        #[method(currentTime)]
        pub unsafe fn currentTime(&self) -> CMTime;

        #[method_id(@__retain_semantics Other currentDate)]
        pub unsafe fn currentDate(&self) -> Option<Retained<NSDate>>;
    }
);

extern_methods!(
    /// AVPlayerItemIntegratedTimelineControl
    unsafe impl AVPlayerItemIntegratedTimeline {
        #[cfg(all(feature = "block2", feature = "objc2-core-media"))]
        #[method(seekToTime:toleranceBefore:toleranceAfter:completionHandler:)]
        pub unsafe fn seekToTime_toleranceBefore_toleranceAfter_completionHandler(
            &self,
            time: CMTime,
            tolerance_before: CMTime,
            tolerance_after: CMTime,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "block2")]
        #[method(seekToDate:completionHandler:)]
        pub unsafe fn seekToDate_completionHandler(
            &self,
            date: &NSDate,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayeritemintegratedtimelineobserver?language=objc)
    pub unsafe trait AVPlayerItemIntegratedTimelineObserver: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn AVPlayerItemIntegratedTimelineObserver {}
);

extern_methods!(
    /// AVPlayerItemIntegratedTimelineObserver
    unsafe impl AVPlayerItemIntegratedTimeline {
        #[method(removeTimeObserver:)]
        pub unsafe fn removeTimeObserver(
            &self,
            observer: &ProtocolObject<dyn AVPlayerItemIntegratedTimelineObserver>,
        );
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerintegratedtimelinesnapshotsoutofsyncnotification?language=objc)
    pub static AVPlayerIntegratedTimelineSnapshotsOutOfSyncNotification:
        &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerintegratedtimelinesnapshotsoutofsyncreasonkey?language=objc)
    pub static AVPlayerIntegratedTimelineSnapshotsOutOfSyncReasonKey: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerintegratedtimelinesnapshotsoutofsyncreason?language=objc)
// NS_TYPED_ENUM
pub type AVPlayerIntegratedTimelineSnapshotsOutOfSyncReason = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerintegratedtimelinesnapshotsoutofsyncreasonsegmentschanged?language=objc)
    pub static AVPlayerIntegratedTimelineSnapshotsOutOfSyncReasonSegmentsChanged:
        &'static AVPlayerIntegratedTimelineSnapshotsOutOfSyncReason;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerintegratedtimelinesnapshotsoutofsyncreasoncurrentsegmentchanged?language=objc)
    pub static AVPlayerIntegratedTimelineSnapshotsOutOfSyncReasonCurrentSegmentChanged:
        &'static AVPlayerIntegratedTimelineSnapshotsOutOfSyncReason;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerintegratedtimelinesnapshotsoutofsyncreasonloadedtimerangeschanged?language=objc)
    pub static AVPlayerIntegratedTimelineSnapshotsOutOfSyncReasonLoadedTimeRangesChanged:
        &'static AVPlayerIntegratedTimelineSnapshotsOutOfSyncReason;
}

extern_methods!(
    /// AVPlayerItemIntegratedTimelineSupport
    #[cfg(feature = "AVPlayerItem")]
    unsafe impl AVPlayerItem {
        #[method_id(@__retain_semantics Other integratedTimeline)]
        pub unsafe fn integratedTimeline(&self) -> Retained<AVPlayerItemIntegratedTimeline>;
    }
);