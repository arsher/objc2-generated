//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-photos")]
#[cfg(not(target_os = "watchos"))]
use objc2_photos::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/photosui/phlivephotoviewplaybackstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHLivePhotoViewPlaybackStyle(pub NSInteger);
impl PHLivePhotoViewPlaybackStyle {
    #[doc(alias = "PHLivePhotoViewPlaybackStyleUndefined")]
    pub const Undefined: Self = Self(0);
    #[doc(alias = "PHLivePhotoViewPlaybackStyleFull")]
    pub const Full: Self = Self(1);
    #[doc(alias = "PHLivePhotoViewPlaybackStyleHint")]
    pub const Hint: Self = Self(2);
}

unsafe impl Encode for PHLivePhotoViewPlaybackStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHLivePhotoViewPlaybackStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for PHLivePhotoViewPlaybackStyle {}

unsafe impl Sync for PHLivePhotoViewPlaybackStyle {}

/// [Apple's documentation](https://developer.apple.com/documentation/photosui/phlivephotoviewcontentmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHLivePhotoViewContentMode(pub NSInteger);
impl PHLivePhotoViewContentMode {
    #[doc(alias = "PHLivePhotoViewContentModeAspectFit")]
    pub const AspectFit: Self = Self(0);
    #[doc(alias = "PHLivePhotoViewContentModeAspectFill")]
    pub const AspectFill: Self = Self(1);
}

unsafe impl Encode for PHLivePhotoViewContentMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHLivePhotoViewContentMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for PHLivePhotoViewContentMode {}

unsafe impl Sync for PHLivePhotoViewContentMode {}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phlivephotoview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct PHLivePhotoView;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for PHLivePhotoView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for PHLivePhotoView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for PHLivePhotoView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for PHLivePhotoView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for PHLivePhotoView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for PHLivePhotoView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for PHLivePhotoView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for PHLivePhotoView {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl PHLivePhotoView {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn PHLivePhotoViewDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn PHLivePhotoViewDelegate>>,
        );

        #[cfg(feature = "objc2-photos")]
        #[method_id(@__retain_semantics Other livePhoto)]
        pub unsafe fn livePhoto(&self) -> Option<Retained<PHLivePhoto>>;

        #[cfg(feature = "objc2-photos")]
        #[method(setLivePhoto:)]
        pub unsafe fn setLivePhoto(&self, live_photo: Option<&PHLivePhoto>);

        #[method(contentMode)]
        pub unsafe fn contentMode(&self) -> PHLivePhotoViewContentMode;

        #[method(setContentMode:)]
        pub unsafe fn setContentMode(&self, content_mode: PHLivePhotoViewContentMode);

        #[method(contentsRect)]
        pub unsafe fn contentsRect(&self) -> CGRect;

        #[method(setContentsRect:)]
        pub unsafe fn setContentsRect(&self, contents_rect: CGRect);

        #[method(audioVolume)]
        pub unsafe fn audioVolume(&self) -> c_float;

        #[method(setAudioVolume:)]
        pub unsafe fn setAudioVolume(&self, audio_volume: c_float);

        #[method(isMuted)]
        pub unsafe fn isMuted(&self) -> bool;

        #[method(setMuted:)]
        pub unsafe fn setMuted(&self, muted: bool);

        #[method(startPlaybackWithStyle:)]
        pub unsafe fn startPlaybackWithStyle(&self, playback_style: PHLivePhotoViewPlaybackStyle);

        #[method(stopPlayback)]
        pub unsafe fn stopPlayback(&self);

        #[method(stopPlaybackAnimated:)]
        pub unsafe fn stopPlaybackAnimated(&self, animated: bool);

        #[method_id(@__retain_semantics Other livePhotoBadgeView)]
        pub unsafe fn livePhotoBadgeView(&self) -> Option<Retained<NSView>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl PHLivePhotoView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

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
    unsafe impl PHLivePhotoView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl PHLivePhotoView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phlivephotoviewdelegate?language=objc)
    pub unsafe trait PHLivePhotoViewDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(livePhotoView:canBeginPlaybackWithStyle:)]
        unsafe fn livePhotoView_canBeginPlaybackWithStyle(
            &self,
            live_photo_view: &PHLivePhotoView,
            playback_style: PHLivePhotoViewPlaybackStyle,
        ) -> bool;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(livePhotoView:willBeginPlaybackWithStyle:)]
        unsafe fn livePhotoView_willBeginPlaybackWithStyle(
            &self,
            live_photo_view: &PHLivePhotoView,
            playback_style: PHLivePhotoViewPlaybackStyle,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(livePhotoView:didEndPlaybackWithStyle:)]
        unsafe fn livePhotoView_didEndPlaybackWithStyle(
            &self,
            live_photo_view: &PHLivePhotoView,
            playback_style: PHLivePhotoViewPlaybackStyle,
        );
    }

    unsafe impl ProtocolType for dyn PHLivePhotoViewDelegate {}
);
