// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "MediaPlayer", kind = "framework")]
extern "C" {}

#[cfg(feature = "AVFoundation_MPNowPlayingInfoLanguageOptionAdditions")]
#[path = "AVFoundation_MPNowPlayingInfoLanguageOptionAdditions.rs"]
mod __AVFoundation_MPNowPlayingInfoLanguageOptionAdditions;
#[cfg(feature = "AVPlayerItem_MediaPlayerAdditions")]
#[path = "AVPlayerItem_MediaPlayerAdditions.rs"]
mod __AVPlayerItem_MediaPlayerAdditions;
#[cfg(feature = "MPContentItem")]
#[path = "MPContentItem.rs"]
mod __MPContentItem;
#[cfg(feature = "MPError")]
#[path = "MPError.rs"]
mod __MPError;
#[cfg(feature = "MPMediaEntity")]
#[path = "MPMediaEntity.rs"]
mod __MPMediaEntity;
#[cfg(feature = "MPMediaItem")]
#[path = "MPMediaItem.rs"]
mod __MPMediaItem;
#[cfg(feature = "MPMediaItemCollection")]
#[path = "MPMediaItemCollection.rs"]
mod __MPMediaItemCollection;
#[cfg(feature = "MPMediaLibrary")]
#[path = "MPMediaLibrary.rs"]
mod __MPMediaLibrary;
#[cfg(feature = "MPMediaPickerController")]
#[path = "MPMediaPickerController.rs"]
mod __MPMediaPickerController;
#[cfg(feature = "MPMediaPlayback")]
#[path = "MPMediaPlayback.rs"]
mod __MPMediaPlayback;
#[cfg(feature = "MPMediaPlaylist")]
#[path = "MPMediaPlaylist.rs"]
mod __MPMediaPlaylist;
#[cfg(feature = "MPMediaQuery")]
#[path = "MPMediaQuery.rs"]
mod __MPMediaQuery;
#[cfg(feature = "MPMediaQuerySection")]
#[path = "MPMediaQuerySection.rs"]
mod __MPMediaQuerySection;
#[cfg(feature = "MPMoviePlayerController")]
#[path = "MPMoviePlayerController.rs"]
mod __MPMoviePlayerController;
#[cfg(feature = "MPMoviePlayerViewController")]
#[path = "MPMoviePlayerViewController.rs"]
mod __MPMoviePlayerViewController;
#[cfg(feature = "MPMusicPlayerApplicationController")]
#[path = "MPMusicPlayerApplicationController.rs"]
mod __MPMusicPlayerApplicationController;
#[cfg(feature = "MPMusicPlayerController")]
#[path = "MPMusicPlayerController.rs"]
mod __MPMusicPlayerController;
#[cfg(feature = "MPMusicPlayerQueueDescriptor")]
#[path = "MPMusicPlayerQueueDescriptor.rs"]
mod __MPMusicPlayerQueueDescriptor;
#[cfg(feature = "MPNowPlayingInfoCenter")]
#[path = "MPNowPlayingInfoCenter.rs"]
mod __MPNowPlayingInfoCenter;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
#[path = "MPNowPlayingInfoLanguageOption.rs"]
mod __MPNowPlayingInfoLanguageOption;
#[cfg(feature = "MPNowPlayingSession")]
#[path = "MPNowPlayingSession.rs"]
mod __MPNowPlayingSession;
#[cfg(feature = "MPPlayableContentDataSource")]
#[path = "MPPlayableContentDataSource.rs"]
mod __MPPlayableContentDataSource;
#[cfg(feature = "MPPlayableContentDelegate")]
#[path = "MPPlayableContentDelegate.rs"]
mod __MPPlayableContentDelegate;
#[cfg(feature = "MPPlayableContentManager")]
#[path = "MPPlayableContentManager.rs"]
mod __MPPlayableContentManager;
#[cfg(feature = "MPPlayableContentManagerContext")]
#[path = "MPPlayableContentManagerContext.rs"]
mod __MPPlayableContentManagerContext;
#[cfg(feature = "MPRemoteCommand")]
#[path = "MPRemoteCommand.rs"]
mod __MPRemoteCommand;
#[cfg(feature = "MPRemoteCommandCenter")]
#[path = "MPRemoteCommandCenter.rs"]
mod __MPRemoteCommandCenter;
#[cfg(feature = "MPRemoteCommandEvent")]
#[path = "MPRemoteCommandEvent.rs"]
mod __MPRemoteCommandEvent;
#[cfg(feature = "MPRemoteControlTypes")]
#[path = "MPRemoteControlTypes.rs"]
mod __MPRemoteControlTypes;
#[cfg(feature = "MPVolumeSettings")]
#[path = "MPVolumeSettings.rs"]
mod __MPVolumeSettings;
#[cfg(feature = "MPVolumeView")]
#[path = "MPVolumeView.rs"]
mod __MPVolumeView;
#[cfg(feature = "MediaPlayerDefines")]
#[path = "MediaPlayerDefines.rs"]
mod __MediaPlayerDefines;
#[cfg(feature = "NSUserActivity_MediaPlayerAdditions")]
#[path = "NSUserActivity_MediaPlayerAdditions.rs"]
mod __NSUserActivity_MediaPlayerAdditions;

#[cfg(feature = "AVFoundation_MPNowPlayingInfoLanguageOptionAdditions")]
pub use self::__AVFoundation_MPNowPlayingInfoLanguageOptionAdditions::AVMediaSelectionGroupMPNowPlayingInfoLanguageOptionAdditions;
#[cfg(feature = "AVFoundation_MPNowPlayingInfoLanguageOptionAdditions")]
pub use self::__AVFoundation_MPNowPlayingInfoLanguageOptionAdditions::AVMediaSelectionOptionMPNowPlayingInfoLanguageOptionAdditions;
#[cfg(feature = "AVPlayerItem_MediaPlayerAdditions")]
pub use self::__AVPlayerItem_MediaPlayerAdditions::AVPlayerItemMPAdditions;
#[cfg(feature = "MPContentItem")]
pub use self::__MPContentItem::MPContentItem;
#[cfg(feature = "MPError")]
pub use self::__MPError::MPErrorCode;
#[cfg(feature = "MPError")]
pub use self::__MPError::MPErrorDomain;
#[cfg(feature = "MPMediaEntity")]
pub use self::__MPMediaEntity::MPMediaEntity;
#[cfg(feature = "MPMediaEntity")]
pub use self::__MPMediaEntity::MPMediaEntityPersistentID;
#[cfg(feature = "MPMediaEntity")]
pub use self::__MPMediaEntity::MPMediaEntityPropertyPersistentID;
#[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItem"))]
pub use self::__MPMediaItem::MPMediaItem;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemArtwork;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumArtist;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumArtistPersistentID;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumPersistentID;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumTitle;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumTrackCount;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumTrackNumber;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyArtist;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyArtistPersistentID;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyArtwork;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAssetURL;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyBeatsPerMinute;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyBookmarkTime;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyComments;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyComposer;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyComposerPersistentID;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyDateAdded;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyDiscCount;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyDiscNumber;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyGenre;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyGenrePersistentID;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyHasProtectedAsset;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyIsCloudItem;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyIsCompilation;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyIsExplicit;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyIsPreorder;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyLastPlayedDate;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyLyrics;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyMediaType;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPersistentID;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPlayCount;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPlaybackDuration;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPlaybackStoreID;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPodcastPersistentID;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPodcastTitle;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyRating;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyReleaseDate;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertySkipCount;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyTitle;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyUserGrouping;
#[cfg(feature = "MPMediaItem")]
pub use self::__MPMediaItem::MPMediaType;
#[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
pub use self::__MPMediaItemCollection::MPMediaItemCollection;
#[cfg(feature = "MPMediaLibrary")]
pub use self::__MPMediaLibrary::MPMediaLibrary;
#[cfg(feature = "MPMediaLibrary")]
pub use self::__MPMediaLibrary::MPMediaLibraryAuthorizationStatus;
#[cfg(feature = "MPMediaLibrary")]
pub use self::__MPMediaLibrary::MPMediaLibraryDidChangeNotification;
#[cfg(feature = "MPMediaPlayback")]
pub use self::__MPMediaPlayback::MPMediaPlayback;
#[cfg(feature = "MPMediaPlayback")]
pub use self::__MPMediaPlayback::MPMediaPlaybackIsPreparedToPlayDidChangeNotification;
#[cfg(all(
    feature = "MPMediaEntity",
    feature = "MPMediaItemCollection",
    feature = "MPMediaPlaylist"
))]
pub use self::__MPMediaPlaylist::MPMediaPlaylist;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistAttribute;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistCreationMetadata;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyAuthorDisplayName;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyCloudGlobalID;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyDescriptionText;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyName;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyPersistentID;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyPlaylistAttributes;
#[cfg(feature = "MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertySeedItems;
#[cfg(feature = "MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaGrouping;
#[cfg(feature = "MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaPredicate;
#[cfg(feature = "MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaPredicateComparison;
#[cfg(feature = "MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaPropertyPredicate;
#[cfg(feature = "MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaQuery;
#[cfg(feature = "MPMediaQuerySection")]
pub use self::__MPMediaQuerySection::MPMediaQuerySection;
#[cfg(all(
    feature = "MPMusicPlayerApplicationController",
    feature = "MPMusicPlayerController"
))]
pub use self::__MPMusicPlayerApplicationController::MPMusicPlayerApplicationController;
#[cfg(feature = "MPMusicPlayerApplicationController")]
pub use self::__MPMusicPlayerApplicationController::MPMusicPlayerControllerMutableQueue;
#[cfg(feature = "MPMusicPlayerApplicationController")]
pub use self::__MPMusicPlayerApplicationController::MPMusicPlayerControllerQueue;
#[cfg(feature = "MPMusicPlayerApplicationController")]
pub use self::__MPMusicPlayerApplicationController::MPMusicPlayerControllerQueueDidChangeNotification;
#[cfg(feature = "MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlaybackState;
#[cfg(feature = "MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlayerController;
#[cfg(feature = "MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlayerControllerNowPlayingItemDidChangeNotification;
#[cfg(feature = "MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlayerControllerPlaybackStateDidChangeNotification;
#[cfg(feature = "MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlayerControllerVolumeDidChangeNotification;
#[cfg(feature = "MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicRepeatMode;
#[cfg(feature = "MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicShuffleMode;
#[cfg(feature = "MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPSystemMusicPlayerController;
#[cfg(feature = "MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerMediaItemQueueDescriptor;
#[cfg(feature = "MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerPlayParameters;
#[cfg(feature = "MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerPlayParametersQueueDescriptor;
#[cfg(feature = "MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerQueueDescriptor;
#[cfg(feature = "MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerStoreQueueDescriptor;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoCenter;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoCollectionIdentifier;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoMediaType;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyAdTimeRanges;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyAssetURL;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyAvailableLanguageOptions;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyChapterCount;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyChapterNumber;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyCreditsStartTime;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyCurrentLanguageOptions;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyCurrentPlaybackDate;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyDefaultPlaybackRate;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyElapsedPlaybackTime;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyExcludeFromSuggestions;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyExternalContentIdentifier;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyExternalUserProfileIdentifier;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyInternationalStandardRecordingCode;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyIsLiveStream;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyMediaType;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyPlaybackProgress;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyPlaybackQueueCount;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyPlaybackQueueIndex;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyPlaybackRate;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyServiceIdentifier;
#[cfg(feature = "MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingPlaybackState;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicDescribesMusicAndSound;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicDescribesVideo;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicDubbedTranslation;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicEasyToRead;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicIsAuxiliaryContent;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicIsMainProgramContent;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicLanguageTranslation;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicTranscribesSpokenDialog;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicVoiceOverTranslation;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPNowPlayingInfoLanguageOption;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPNowPlayingInfoLanguageOptionGroup;
#[cfg(feature = "MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPNowPlayingInfoLanguageOptionType;
#[cfg(feature = "MPNowPlayingSession")]
pub use self::__MPNowPlayingSession::MPAdTimeRange;
#[cfg(feature = "MPNowPlayingSession")]
pub use self::__MPNowPlayingSession::MPNowPlayingSession;
#[cfg(feature = "MPNowPlayingSession")]
pub use self::__MPNowPlayingSession::MPNowPlayingSessionDelegate;
#[cfg(feature = "MPPlayableContentDataSource")]
pub use self::__MPPlayableContentDataSource::MPPlayableContentDataSource;
#[cfg(feature = "MPPlayableContentDelegate")]
pub use self::__MPPlayableContentDelegate::MPPlayableContentDelegate;
#[cfg(feature = "MPPlayableContentManager")]
pub use self::__MPPlayableContentManager::MPPlayableContentManager;
#[cfg(feature = "MPPlayableContentManagerContext")]
pub use self::__MPPlayableContentManagerContext::MPPlayableContentManagerContext;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPChangePlaybackPositionCommand;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPChangePlaybackRateCommand;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPChangeRepeatModeCommand;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPChangeShuffleModeCommand;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPFeedbackCommand;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPRatingCommand;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPRemoteCommand;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPRemoteCommandHandlerStatus;
#[cfg(feature = "MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPSkipIntervalCommand;
#[cfg(feature = "MPRemoteCommandCenter")]
pub use self::__MPRemoteCommandCenter::MPRemoteCommandCenter;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangeLanguageOptionCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangePlaybackPositionCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangePlaybackRateCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangeRepeatModeCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangeShuffleModeCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPFeedbackCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPRatingCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPRemoteCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPSeekCommandEvent;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPSeekCommandEventType;
#[cfg(feature = "MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPSkipIntervalCommandEvent;
#[cfg(feature = "MPRemoteControlTypes")]
pub use self::__MPRemoteControlTypes::MPChangeLanguageOptionSetting;
#[cfg(feature = "MPRemoteControlTypes")]
pub use self::__MPRemoteControlTypes::MPRepeatType;
#[cfg(feature = "MPRemoteControlTypes")]
pub use self::__MPRemoteControlTypes::MPShuffleType;
#[cfg(feature = "MPVolumeSettings")]
pub use self::__MPVolumeSettings::MPVolumeSettingsAlertHide;
#[cfg(feature = "MPVolumeSettings")]
pub use self::__MPVolumeSettings::MPVolumeSettingsAlertIsVisible;
#[cfg(feature = "MPVolumeSettings")]
pub use self::__MPVolumeSettings::MPVolumeSettingsAlertShow;
#[cfg(feature = "NSUserActivity_MediaPlayerAdditions")]
pub use self::__NSUserActivity_MediaPlayerAdditions::NSUserActivityMediaPlayerAdditions;
