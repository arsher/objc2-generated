//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwebviewnavigationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIWebViewNavigationType(pub NSInteger);
impl UIWebViewNavigationType {
    #[doc(alias = "UIWebViewNavigationTypeLinkClicked")]
    pub const LinkClicked: Self = Self(0);
    #[doc(alias = "UIWebViewNavigationTypeFormSubmitted")]
    pub const FormSubmitted: Self = Self(1);
    #[doc(alias = "UIWebViewNavigationTypeBackForward")]
    pub const BackForward: Self = Self(2);
    #[doc(alias = "UIWebViewNavigationTypeReload")]
    pub const Reload: Self = Self(3);
    #[doc(alias = "UIWebViewNavigationTypeFormResubmitted")]
    pub const FormResubmitted: Self = Self(4);
    #[doc(alias = "UIWebViewNavigationTypeOther")]
    pub const Other: Self = Self(5);
}

unsafe impl Encode for UIWebViewNavigationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIWebViewNavigationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwebpaginationmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIWebPaginationMode(pub NSInteger);
impl UIWebPaginationMode {
    #[doc(alias = "UIWebPaginationModeUnpaginated")]
    pub const Unpaginated: Self = Self(0);
    #[doc(alias = "UIWebPaginationModeLeftToRight")]
    pub const LeftToRight: Self = Self(1);
    #[doc(alias = "UIWebPaginationModeTopToBottom")]
    pub const TopToBottom: Self = Self(2);
    #[doc(alias = "UIWebPaginationModeBottomToTop")]
    pub const BottomToTop: Self = Self(3);
    #[doc(alias = "UIWebPaginationModeRightToLeft")]
    pub const RightToLeft: Self = Self(4);
}

unsafe impl Encode for UIWebPaginationMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIWebPaginationMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwebpaginationbreakingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIWebPaginationBreakingMode(pub NSInteger);
impl UIWebPaginationBreakingMode {
    #[doc(alias = "UIWebPaginationBreakingModePage")]
    pub const Page: Self = Self(0);
    #[doc(alias = "UIWebPaginationBreakingModeColumn")]
    pub const Column: Self = Self(1);
}

unsafe impl Encode for UIWebPaginationBreakingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIWebPaginationBreakingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwebview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    #[deprecated = "No longer supported; please adopt WKWebView."]
    pub struct UIWebView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIWebView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIWebView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIWebView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIWebView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIWebView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIWebView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIWebView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIWebView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIWebView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIWebView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIWebView {}

#[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
unsafe impl UIScrollViewDelegate for UIWebView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIWebView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIWebView {
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn UIWebViewDelegate>>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn UIWebViewDelegate>>);

        #[cfg(feature = "UIScrollView")]
        #[method_id(@__retain_semantics Other scrollView)]
        pub unsafe fn scrollView(&self) -> Retained<UIScrollView>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(loadRequest:)]
        pub unsafe fn loadRequest(&self, request: &NSURLRequest);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(loadHTMLString:baseURL:)]
        pub unsafe fn loadHTMLString_baseURL(&self, string: &NSString, base_url: Option<&NSURL>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(loadData:MIMEType:textEncodingName:baseURL:)]
        pub unsafe fn loadData_MIMEType_textEncodingName_baseURL(
            &self,
            data: &NSData,
            mime_type: &NSString,
            text_encoding_name: &NSString,
            base_url: &NSURL,
        );

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Option<Retained<NSURLRequest>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(reload)]
        pub unsafe fn reload(&self);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(stopLoading)]
        pub unsafe fn stopLoading(&self);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(goBack)]
        pub unsafe fn goBack(&self);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(goForward)]
        pub unsafe fn goForward(&self);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canGoBack)]
        pub unsafe fn canGoBack(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canGoForward)]
        pub unsafe fn canGoForward(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other stringByEvaluatingJavaScriptFromString:)]
        pub unsafe fn stringByEvaluatingJavaScriptFromString(
            &self,
            script: &NSString,
        ) -> Option<Retained<NSString>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(scalesPageToFit)]
        pub unsafe fn scalesPageToFit(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setScalesPageToFit:)]
        pub unsafe fn setScalesPageToFit(&self, scales_page_to_fit: bool);

        #[deprecated]
        #[method(detectsPhoneNumbers)]
        pub unsafe fn detectsPhoneNumbers(&self) -> bool;

        #[deprecated]
        #[method(setDetectsPhoneNumbers:)]
        pub unsafe fn setDetectsPhoneNumbers(&self, detects_phone_numbers: bool);

        #[cfg(feature = "UIDataDetectors")]
        #[method(dataDetectorTypes)]
        pub unsafe fn dataDetectorTypes(&self) -> UIDataDetectorTypes;

        #[cfg(feature = "UIDataDetectors")]
        #[method(setDataDetectorTypes:)]
        pub unsafe fn setDataDetectorTypes(&self, data_detector_types: UIDataDetectorTypes);

        #[method(allowsInlineMediaPlayback)]
        pub unsafe fn allowsInlineMediaPlayback(&self) -> bool;

        #[method(setAllowsInlineMediaPlayback:)]
        pub unsafe fn setAllowsInlineMediaPlayback(&self, allows_inline_media_playback: bool);

        #[method(mediaPlaybackRequiresUserAction)]
        pub unsafe fn mediaPlaybackRequiresUserAction(&self) -> bool;

        #[method(setMediaPlaybackRequiresUserAction:)]
        pub unsafe fn setMediaPlaybackRequiresUserAction(
            &self,
            media_playback_requires_user_action: bool,
        );

        #[method(mediaPlaybackAllowsAirPlay)]
        pub unsafe fn mediaPlaybackAllowsAirPlay(&self) -> bool;

        #[method(setMediaPlaybackAllowsAirPlay:)]
        pub unsafe fn setMediaPlaybackAllowsAirPlay(&self, media_playback_allows_air_play: bool);

        #[method(suppressesIncrementalRendering)]
        pub unsafe fn suppressesIncrementalRendering(&self) -> bool;

        #[method(setSuppressesIncrementalRendering:)]
        pub unsafe fn setSuppressesIncrementalRendering(
            &self,
            suppresses_incremental_rendering: bool,
        );

        #[method(keyboardDisplayRequiresUserAction)]
        pub unsafe fn keyboardDisplayRequiresUserAction(&self) -> bool;

        #[method(setKeyboardDisplayRequiresUserAction:)]
        pub unsafe fn setKeyboardDisplayRequiresUserAction(
            &self,
            keyboard_display_requires_user_action: bool,
        );

        #[method(paginationMode)]
        pub unsafe fn paginationMode(&self) -> UIWebPaginationMode;

        #[method(setPaginationMode:)]
        pub unsafe fn setPaginationMode(&self, pagination_mode: UIWebPaginationMode);

        #[method(paginationBreakingMode)]
        pub unsafe fn paginationBreakingMode(&self) -> UIWebPaginationBreakingMode;

        #[method(setPaginationBreakingMode:)]
        pub unsafe fn setPaginationBreakingMode(
            &self,
            pagination_breaking_mode: UIWebPaginationBreakingMode,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(pageLength)]
        pub unsafe fn pageLength(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setPageLength:)]
        pub unsafe fn setPageLength(&self, page_length: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(gapBetweenPages)]
        pub unsafe fn gapBetweenPages(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setGapBetweenPages:)]
        pub unsafe fn setGapBetweenPages(&self, gap_between_pages: CGFloat);

        #[method(pageCount)]
        pub unsafe fn pageCount(&self) -> NSUInteger;

        #[method(allowsPictureInPictureMediaPlayback)]
        pub unsafe fn allowsPictureInPictureMediaPlayback(&self) -> bool;

        #[method(setAllowsPictureInPictureMediaPlayback:)]
        pub unsafe fn setAllowsPictureInPictureMediaPlayback(
            &self,
            allows_picture_in_picture_media_playback: bool,
        );

        #[method(allowsLinkPreview)]
        pub unsafe fn allowsLinkPreview(&self) -> bool;

        #[method(setAllowsLinkPreview:)]
        pub unsafe fn setAllowsLinkPreview(&self, allows_link_preview: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIWebView {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIWebView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwebviewdelegate?language=objc)
    pub unsafe trait UIWebViewDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "No longer supported."]
        #[optional]
        #[method(webView:shouldStartLoadWithRequest:navigationType:)]
        unsafe fn webView_shouldStartLoadWithRequest_navigationType(
            &self,
            web_view: &UIWebView,
            request: &NSURLRequest,
            navigation_type: UIWebViewNavigationType,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "No longer supported."]
        #[optional]
        #[method(webViewDidStartLoad:)]
        unsafe fn webViewDidStartLoad(&self, web_view: &UIWebView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "No longer supported."]
        #[optional]
        #[method(webViewDidFinishLoad:)]
        unsafe fn webViewDidFinishLoad(&self, web_view: &UIWebView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "No longer supported."]
        #[optional]
        #[method(webView:didFailLoadWithError:)]
        unsafe fn webView_didFailLoadWithError(&self, web_view: &UIWebView, error: &NSError);
    }

    unsafe impl ProtocolType for dyn UIWebViewDelegate {}
);
