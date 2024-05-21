//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMediaType(pub NSUInteger);
bitflags::bitflags! {
    impl MPMediaType: NSUInteger {
        #[doc(alias = "MPMediaTypeMusic")]
        const Music = 1<<0;
        #[doc(alias = "MPMediaTypePodcast")]
        const Podcast = 1<<1;
        #[doc(alias = "MPMediaTypeAudioBook")]
        const AudioBook = 1<<2;
        #[doc(alias = "MPMediaTypeAudioITunesU")]
        const AudioITunesU = 1<<3;
        #[doc(alias = "MPMediaTypeAnyAudio")]
        const AnyAudio = 0x00ff;
        #[doc(alias = "MPMediaTypeMovie")]
        const Movie = 1<<8;
        #[doc(alias = "MPMediaTypeTVShow")]
        const TVShow = 1<<9;
        #[doc(alias = "MPMediaTypeVideoPodcast")]
        const VideoPodcast = 1<<10;
        #[doc(alias = "MPMediaTypeMusicVideo")]
        const MusicVideo = 1<<11;
        #[doc(alias = "MPMediaTypeVideoITunesU")]
        const VideoITunesU = 1<<12;
        #[doc(alias = "MPMediaTypeHomeVideo")]
        const HomeVideo = 1<<13;
        #[doc(alias = "MPMediaTypeAnyVideo")]
        const AnyVideo = 0xff00;
        #[doc(alias = "MPMediaTypeAny")]
        const Any = !0;
    }
}

unsafe impl Encode for MPMediaType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPMediaType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static MPMediaItemPropertyPersistentID: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyMediaType: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyTitle: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyAlbumTitle: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyAlbumPersistentID: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyArtist: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyArtistPersistentID: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyAlbumArtist: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyAlbumArtistPersistentID: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyGenre: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyGenrePersistentID: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyComposer: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyComposerPersistentID: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyPlaybackDuration: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyAlbumTrackNumber: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyAlbumTrackCount: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyDiscNumber: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyDiscCount: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyArtwork: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyIsExplicit: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyLyrics: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyIsCompilation: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyReleaseDate: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyBeatsPerMinute: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyComments: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyAssetURL: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyIsCloudItem: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyHasProtectedAsset: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyPodcastTitle: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyPodcastPersistentID: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyPlayCount: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertySkipCount: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyRating: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyLastPlayedDate: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyUserGrouping: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyBookmarkTime: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyDateAdded: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyPlaybackStoreID: &'static NSString;
}

extern "C" {
    pub static MPMediaItemPropertyIsPreorder: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPMediaEntity")]
    pub struct MPMediaItem;

    #[cfg(feature = "MPMediaEntity")]
    unsafe impl ClassType for MPMediaItem {
        #[inherits(NSObject)]
        type Super = MPMediaEntity;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MPMediaEntity")]
unsafe impl NSCoding for MPMediaItem {}

#[cfg(feature = "MPMediaEntity")]
unsafe impl NSObjectProtocol for MPMediaItem {}

#[cfg(feature = "MPMediaEntity")]
unsafe impl NSSecureCoding for MPMediaItem {}

extern_methods!(
    #[cfg(feature = "MPMediaEntity")]
    unsafe impl MPMediaItem {
        #[method(persistentID)]
        pub unsafe fn persistentID(&self) -> MPMediaEntityPersistentID;

        #[method(mediaType)]
        pub unsafe fn mediaType(&self) -> MPMediaType;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other albumTitle)]
        pub unsafe fn albumTitle(&self) -> Option<Retained<NSString>>;

        #[method(albumPersistentID)]
        pub unsafe fn albumPersistentID(&self) -> MPMediaEntityPersistentID;

        #[method_id(@__retain_semantics Other artist)]
        pub unsafe fn artist(&self) -> Option<Retained<NSString>>;

        #[method(artistPersistentID)]
        pub unsafe fn artistPersistentID(&self) -> MPMediaEntityPersistentID;

        #[method_id(@__retain_semantics Other albumArtist)]
        pub unsafe fn albumArtist(&self) -> Option<Retained<NSString>>;

        #[method(albumArtistPersistentID)]
        pub unsafe fn albumArtistPersistentID(&self) -> MPMediaEntityPersistentID;

        #[method_id(@__retain_semantics Other genre)]
        pub unsafe fn genre(&self) -> Option<Retained<NSString>>;

        #[method(genrePersistentID)]
        pub unsafe fn genrePersistentID(&self) -> MPMediaEntityPersistentID;

        #[method_id(@__retain_semantics Other composer)]
        pub unsafe fn composer(&self) -> Option<Retained<NSString>>;

        #[method(composerPersistentID)]
        pub unsafe fn composerPersistentID(&self) -> MPMediaEntityPersistentID;

        #[method(playbackDuration)]
        pub unsafe fn playbackDuration(&self) -> NSTimeInterval;

        #[method(albumTrackNumber)]
        pub unsafe fn albumTrackNumber(&self) -> NSUInteger;

        #[method(albumTrackCount)]
        pub unsafe fn albumTrackCount(&self) -> NSUInteger;

        #[method(discNumber)]
        pub unsafe fn discNumber(&self) -> NSUInteger;

        #[method(discCount)]
        pub unsafe fn discCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other artwork)]
        pub unsafe fn artwork(&self) -> Option<Retained<MPMediaItemArtwork>>;

        #[method(isExplicitItem)]
        pub unsafe fn isExplicitItem(&self) -> bool;

        #[method_id(@__retain_semantics Other lyrics)]
        pub unsafe fn lyrics(&self) -> Option<Retained<NSString>>;

        #[method(isCompilation)]
        pub unsafe fn isCompilation(&self) -> bool;

        #[method_id(@__retain_semantics Other releaseDate)]
        pub unsafe fn releaseDate(&self) -> Option<Retained<NSDate>>;

        #[method(beatsPerMinute)]
        pub unsafe fn beatsPerMinute(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other comments)]
        pub unsafe fn comments(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other assetURL)]
        pub unsafe fn assetURL(&self) -> Option<Retained<NSURL>>;

        #[method(isCloudItem)]
        pub unsafe fn isCloudItem(&self) -> bool;

        #[method(hasProtectedAsset)]
        pub unsafe fn hasProtectedAsset(&self) -> bool;

        #[method_id(@__retain_semantics Other podcastTitle)]
        pub unsafe fn podcastTitle(&self) -> Option<Retained<NSString>>;

        #[method(podcastPersistentID)]
        pub unsafe fn podcastPersistentID(&self) -> MPMediaEntityPersistentID;

        #[method(playCount)]
        pub unsafe fn playCount(&self) -> NSUInteger;

        #[method(skipCount)]
        pub unsafe fn skipCount(&self) -> NSUInteger;

        #[method(rating)]
        pub unsafe fn rating(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other lastPlayedDate)]
        pub unsafe fn lastPlayedDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other userGrouping)]
        pub unsafe fn userGrouping(&self) -> Option<Retained<NSString>>;

        #[method(bookmarkTime)]
        pub unsafe fn bookmarkTime(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Other dateAdded)]
        pub unsafe fn dateAdded(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other playbackStoreID)]
        pub unsafe fn playbackStoreID(&self) -> Retained<NSString>;

        #[method(isPreorder)]
        pub unsafe fn isPreorder(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPMediaEntity")]
    unsafe impl MPMediaItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMediaItemArtwork;

    unsafe impl ClassType for MPMediaItemArtwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPMediaItemArtwork {}

extern_methods!(
    unsafe impl MPMediaItemArtwork {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Init initWithBoundsSize:requestHandler:)]
        pub unsafe fn initWithBoundsSize_requestHandler(
            this: Allocated<Self>,
            bounds_size: CGSize,
            request_handler: &block2::Block<dyn Fn(CGSize) -> NonNull<NSImage>>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other imageWithSize:)]
        pub unsafe fn imageWithSize(&self, size: CGSize) -> Option<Retained<NSImage>>;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[deprecated = "cropRect is no longer used"]
        #[method(imageCropRect)]
        pub unsafe fn imageCropRect(&self) -> CGRect;
    }
);
