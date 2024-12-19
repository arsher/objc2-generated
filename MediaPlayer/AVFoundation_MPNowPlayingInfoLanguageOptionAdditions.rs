//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-av-foundation")]
use objc2_av_foundation::*;

use crate::*;

extern_category!(
    /// Category "MPNowPlayingInfoLanguageOptionAdditions" on [`AVMediaSelectionOption`].
    #[doc(alias = "MPNowPlayingInfoLanguageOptionAdditions")]
    pub unsafe trait AVMediaSelectionOptionMPNowPlayingInfoLanguageOptionAdditions {
        #[cfg(feature = "MPNowPlayingInfoLanguageOption")]
        #[method_id(@__retain_semantics Other makeNowPlayingInfoLanguageOption)]
        unsafe fn makeNowPlayingInfoLanguageOption(
            &self,
        ) -> Option<Retained<MPNowPlayingInfoLanguageOption>>;
    }

    #[cfg(feature = "objc2-av-foundation")]
    unsafe impl AVMediaSelectionOptionMPNowPlayingInfoLanguageOptionAdditions
        for AVMediaSelectionOption
    {
    }
);

extern_category!(
    /// Category "MPNowPlayingInfoLanguageOptionAdditions" on [`AVMediaSelectionGroup`].
    #[doc(alias = "MPNowPlayingInfoLanguageOptionAdditions")]
    pub unsafe trait AVMediaSelectionGroupMPNowPlayingInfoLanguageOptionAdditions {
        #[cfg(feature = "MPNowPlayingInfoLanguageOption")]
        #[method_id(@__retain_semantics Other makeNowPlayingInfoLanguageOptionGroup)]
        unsafe fn makeNowPlayingInfoLanguageOptionGroup(
            &self,
        ) -> Retained<MPNowPlayingInfoLanguageOptionGroup>;
    }

    #[cfg(feature = "objc2-av-foundation")]
    unsafe impl AVMediaSelectionGroupMPNowPlayingInfoLanguageOptionAdditions for AVMediaSelectionGroup {}
);
