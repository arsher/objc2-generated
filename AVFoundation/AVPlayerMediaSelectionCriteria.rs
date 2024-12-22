//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The preferred languages and media characteristics for a player.
    ///
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayermediaselectioncriteria?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPlayerMediaSelectionCriteria;
);

unsafe impl Send for AVPlayerMediaSelectionCriteria {}

unsafe impl Sync for AVPlayerMediaSelectionCriteria {}

unsafe impl NSObjectProtocol for AVPlayerMediaSelectionCriteria {}

extern_methods!(
    unsafe impl AVPlayerMediaSelectionCriteria {
        #[method_id(@__retain_semantics Other preferredLanguages)]
        pub unsafe fn preferredLanguages(&self) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(feature = "AVMediaFormat")]
        #[method_id(@__retain_semantics Other preferredMediaCharacteristics)]
        pub unsafe fn preferredMediaCharacteristics(
            &self,
        ) -> Option<Retained<NSArray<AVMediaCharacteristic>>>;

        #[cfg(feature = "AVMediaFormat")]
        #[method_id(@__retain_semantics Other principalMediaCharacteristics)]
        pub unsafe fn principalMediaCharacteristics(
            &self,
        ) -> Option<Retained<NSArray<AVMediaCharacteristic>>>;

        #[cfg(feature = "AVMediaFormat")]
        /// Creates an instance of AVPlayerMediaSelectionCriteria.
        ///
        /// Parameter `preferredLanguages`: An NSArray of NSStrings containing language identifiers, in order of desirability, that are preferred for selection. Can be nil.
        ///
        /// Parameter `preferredMediaCharacteristics`: An NSArray of AVMediaCharacteristics indicating additional media characteristics, in order of desirability, that are preferred when selecting media with the characteristic for which the receiver is set on the AVPlayer as the selection criteria. Can be nil.
        ///
        /// Returns: An instance of AVPlayerMediaSelectionCriteria.
        #[method_id(@__retain_semantics Init initWithPreferredLanguages:preferredMediaCharacteristics:)]
        pub unsafe fn initWithPreferredLanguages_preferredMediaCharacteristics(
            this: Allocated<Self>,
            preferred_languages: Option<&NSArray<NSString>>,
            preferred_media_characteristics: Option<&NSArray<AVMediaCharacteristic>>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVMediaFormat")]
        /// Creates an instance of AVPlayerMediaSelectionCriteria.
        ///
        /// Parameter `principalMediaCharacteristics`: An NSArray of AVMediaCharacteristics indicating media characteristics that are considered essential when selecting media with the characteristic for which the receiver is set on the AVPlayer as the selection criteria. Can be nil.
        ///
        /// Parameter `preferredLanguages`: An NSArray of NSStrings containing language identifiers, in order of desirability, that are preferred for selection. Can be nil.
        ///
        /// Parameter `preferredMediaCharacteristics`: An NSArray of AVMediaCharacteristics indicating additional media characteristics, in order of desirability, that are preferred when selecting media with the characteristic for which the receiver is set on the AVPlayer as the selection criteria. Can be nil.
        ///
        /// Returns: An instance of AVPlayerMediaSelectionCriteria.
        ///
        /// Note that even though principal media characteristics, when present, will override language preferences when making a selection within a specific media selection group, language preferences may still pertain to selections in other groups. For example, language preferences for the group that corresponds to the audible characteristic may be considered when choosing whether or not to select non-forced subtitles for translation purposes.
        #[method_id(@__retain_semantics Init initWithPrincipalMediaCharacteristics:preferredLanguages:preferredMediaCharacteristics:)]
        pub unsafe fn initWithPrincipalMediaCharacteristics_preferredLanguages_preferredMediaCharacteristics(
            this: Allocated<Self>,
            principal_media_characteristics: Option<&NSArray<AVMediaCharacteristic>>,
            preferred_languages: Option<&NSArray<NSString>>,
            preferred_media_characteristics: Option<&NSArray<AVMediaCharacteristic>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVPlayerMediaSelectionCriteria {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
