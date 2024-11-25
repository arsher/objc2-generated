//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/webcachemodel?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WebCacheModel(pub NSUInteger);
impl WebCacheModel {
    #[deprecated]
    #[doc(alias = "WebCacheModelDocumentViewer")]
    pub const DocumentViewer: Self = Self(0);
    #[deprecated]
    #[doc(alias = "WebCacheModelDocumentBrowser")]
    pub const DocumentBrowser: Self = Self(1);
    #[deprecated]
    #[doc(alias = "WebCacheModelPrimaryWebBrowser")]
    pub const PrimaryWebBrowser: Self = Self(2);
}

unsafe impl Encode for WebCacheModel {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for WebCacheModel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webpreferenceschangednotification?language=objc)
    pub static WebPreferencesChangedNotification: Option<&'static NSString>;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webpreferences?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct WebPreferences;
);

unsafe impl NSCoding for WebPreferences {}

unsafe impl NSObjectProtocol for WebPreferences {}

extern_methods!(
    unsafe impl WebPreferences {
        #[deprecated]
        #[method_id(@__retain_semantics Other standardPreferences)]
        pub unsafe fn standardPreferences() -> Option<Retained<WebPreferences>>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            an_identifier: Option<&NSString>,
        ) -> Option<Retained<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other standardFontFamily)]
        pub unsafe fn standardFontFamily(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setStandardFontFamily:)]
        pub unsafe fn setStandardFontFamily(&self, standard_font_family: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other fixedFontFamily)]
        pub unsafe fn fixedFontFamily(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setFixedFontFamily:)]
        pub unsafe fn setFixedFontFamily(&self, fixed_font_family: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other serifFontFamily)]
        pub unsafe fn serifFontFamily(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setSerifFontFamily:)]
        pub unsafe fn setSerifFontFamily(&self, serif_font_family: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other sansSerifFontFamily)]
        pub unsafe fn sansSerifFontFamily(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setSansSerifFontFamily:)]
        pub unsafe fn setSansSerifFontFamily(&self, sans_serif_font_family: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other cursiveFontFamily)]
        pub unsafe fn cursiveFontFamily(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setCursiveFontFamily:)]
        pub unsafe fn setCursiveFontFamily(&self, cursive_font_family: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other fantasyFontFamily)]
        pub unsafe fn fantasyFontFamily(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setFantasyFontFamily:)]
        pub unsafe fn setFantasyFontFamily(&self, fantasy_font_family: Option<&NSString>);

        #[deprecated]
        #[method(defaultFontSize)]
        pub unsafe fn defaultFontSize(&self) -> c_int;

        #[deprecated]
        #[method(setDefaultFontSize:)]
        pub unsafe fn setDefaultFontSize(&self, default_font_size: c_int);

        #[deprecated]
        #[method(defaultFixedFontSize)]
        pub unsafe fn defaultFixedFontSize(&self) -> c_int;

        #[deprecated]
        #[method(setDefaultFixedFontSize:)]
        pub unsafe fn setDefaultFixedFontSize(&self, default_fixed_font_size: c_int);

        #[deprecated]
        #[method(minimumFontSize)]
        pub unsafe fn minimumFontSize(&self) -> c_int;

        #[deprecated]
        #[method(setMinimumFontSize:)]
        pub unsafe fn setMinimumFontSize(&self, minimum_font_size: c_int);

        #[deprecated]
        #[method(minimumLogicalFontSize)]
        pub unsafe fn minimumLogicalFontSize(&self) -> c_int;

        #[deprecated]
        #[method(setMinimumLogicalFontSize:)]
        pub unsafe fn setMinimumLogicalFontSize(&self, minimum_logical_font_size: c_int);

        #[deprecated]
        #[method_id(@__retain_semantics Other defaultTextEncodingName)]
        pub unsafe fn defaultTextEncodingName(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setDefaultTextEncodingName:)]
        pub unsafe fn setDefaultTextEncodingName(
            &self,
            default_text_encoding_name: Option<&NSString>,
        );

        #[deprecated]
        #[method(userStyleSheetEnabled)]
        pub unsafe fn userStyleSheetEnabled(&self) -> bool;

        #[deprecated]
        #[method(setUserStyleSheetEnabled:)]
        pub unsafe fn setUserStyleSheetEnabled(&self, user_style_sheet_enabled: bool);

        #[deprecated]
        #[method_id(@__retain_semantics Other userStyleSheetLocation)]
        pub unsafe fn userStyleSheetLocation(&self) -> Option<Retained<NSURL>>;

        #[deprecated]
        #[method(setUserStyleSheetLocation:)]
        pub unsafe fn setUserStyleSheetLocation(&self, user_style_sheet_location: Option<&NSURL>);

        #[deprecated]
        #[method(isJavaEnabled)]
        pub unsafe fn isJavaEnabled(&self) -> bool;

        #[deprecated]
        #[method(setJavaEnabled:)]
        pub unsafe fn setJavaEnabled(&self, java_enabled: bool);

        #[deprecated]
        #[method(isJavaScriptEnabled)]
        pub unsafe fn isJavaScriptEnabled(&self) -> bool;

        #[deprecated]
        #[method(setJavaScriptEnabled:)]
        pub unsafe fn setJavaScriptEnabled(&self, java_script_enabled: bool);

        #[deprecated]
        #[method(javaScriptCanOpenWindowsAutomatically)]
        pub unsafe fn javaScriptCanOpenWindowsAutomatically(&self) -> bool;

        #[deprecated]
        #[method(setJavaScriptCanOpenWindowsAutomatically:)]
        pub unsafe fn setJavaScriptCanOpenWindowsAutomatically(
            &self,
            java_script_can_open_windows_automatically: bool,
        );

        #[deprecated]
        #[method(arePlugInsEnabled)]
        pub unsafe fn arePlugInsEnabled(&self) -> bool;

        #[deprecated]
        #[method(setPlugInsEnabled:)]
        pub unsafe fn setPlugInsEnabled(&self, plug_ins_enabled: bool);

        #[deprecated]
        #[method(allowsAnimatedImages)]
        pub unsafe fn allowsAnimatedImages(&self) -> bool;

        #[deprecated]
        #[method(setAllowsAnimatedImages:)]
        pub unsafe fn setAllowsAnimatedImages(&self, allows_animated_images: bool);

        #[deprecated]
        #[method(allowsAnimatedImageLooping)]
        pub unsafe fn allowsAnimatedImageLooping(&self) -> bool;

        #[deprecated]
        #[method(setAllowsAnimatedImageLooping:)]
        pub unsafe fn setAllowsAnimatedImageLooping(&self, allows_animated_image_looping: bool);

        #[deprecated]
        #[method(loadsImagesAutomatically)]
        pub unsafe fn loadsImagesAutomatically(&self) -> bool;

        #[deprecated]
        #[method(setLoadsImagesAutomatically:)]
        pub unsafe fn setLoadsImagesAutomatically(&self, loads_images_automatically: bool);

        #[deprecated]
        #[method(autosaves)]
        pub unsafe fn autosaves(&self) -> bool;

        #[deprecated]
        #[method(setAutosaves:)]
        pub unsafe fn setAutosaves(&self, autosaves: bool);

        #[deprecated]
        #[method(shouldPrintBackgrounds)]
        pub unsafe fn shouldPrintBackgrounds(&self) -> bool;

        #[deprecated]
        #[method(setShouldPrintBackgrounds:)]
        pub unsafe fn setShouldPrintBackgrounds(&self, should_print_backgrounds: bool);

        #[deprecated]
        #[method(privateBrowsingEnabled)]
        pub unsafe fn privateBrowsingEnabled(&self) -> bool;

        #[deprecated]
        #[method(setPrivateBrowsingEnabled:)]
        pub unsafe fn setPrivateBrowsingEnabled(&self, private_browsing_enabled: bool);

        #[deprecated]
        #[method(tabsToLinks)]
        pub unsafe fn tabsToLinks(&self) -> bool;

        #[deprecated]
        #[method(setTabsToLinks:)]
        pub unsafe fn setTabsToLinks(&self, tabs_to_links: bool);

        #[deprecated]
        #[method(usesPageCache)]
        pub unsafe fn usesPageCache(&self) -> bool;

        #[deprecated]
        #[method(setUsesPageCache:)]
        pub unsafe fn setUsesPageCache(&self, uses_page_cache: bool);

        #[deprecated]
        #[method(cacheModel)]
        pub unsafe fn cacheModel(&self) -> WebCacheModel;

        #[deprecated]
        #[method(setCacheModel:)]
        pub unsafe fn setCacheModel(&self, cache_model: WebCacheModel);

        #[deprecated]
        #[method(suppressesIncrementalRendering)]
        pub unsafe fn suppressesIncrementalRendering(&self) -> bool;

        #[deprecated]
        #[method(setSuppressesIncrementalRendering:)]
        pub unsafe fn setSuppressesIncrementalRendering(
            &self,
            suppresses_incremental_rendering: bool,
        );

        #[deprecated]
        #[method(allowsAirPlayForMediaPlayback)]
        pub unsafe fn allowsAirPlayForMediaPlayback(&self) -> bool;

        #[deprecated]
        #[method(setAllowsAirPlayForMediaPlayback:)]
        pub unsafe fn setAllowsAirPlayForMediaPlayback(
            &self,
            allows_air_play_for_media_playback: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WebPreferences {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
