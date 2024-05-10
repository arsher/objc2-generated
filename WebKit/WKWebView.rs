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
pub struct WKMediaPlaybackState(pub NSInteger);
impl WKMediaPlaybackState {
    #[doc(alias = "WKMediaPlaybackStateNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "WKMediaPlaybackStatePlaying")]
    pub const Playing: Self = Self(1);
    #[doc(alias = "WKMediaPlaybackStatePaused")]
    pub const Paused: Self = Self(2);
    #[doc(alias = "WKMediaPlaybackStateSuspended")]
    pub const Suspended: Self = Self(3);
}

unsafe impl Encode for WKMediaPlaybackState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKMediaPlaybackState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKMediaCaptureState(pub NSInteger);
impl WKMediaCaptureState {
    #[doc(alias = "WKMediaCaptureStateNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "WKMediaCaptureStateActive")]
    pub const Active: Self = Self(1);
    #[doc(alias = "WKMediaCaptureStateMuted")]
    pub const Muted: Self = Self(2);
}

unsafe impl Encode for WKMediaCaptureState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKMediaCaptureState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKFullscreenState(pub NSInteger);
impl WKFullscreenState {
    #[doc(alias = "WKFullscreenStateNotInFullscreen")]
    pub const NotInFullscreen: Self = Self(0);
    #[doc(alias = "WKFullscreenStateEnteringFullscreen")]
    pub const EnteringFullscreen: Self = Self(1);
    #[doc(alias = "WKFullscreenStateInFullscreen")]
    pub const InFullscreen: Self = Self(2);
    #[doc(alias = "WKFullscreenStateExitingFullscreen")]
    pub const ExitingFullscreen: Self = Self(3);
}

unsafe impl Encode for WKFullscreenState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKFullscreenState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    pub struct WKWebView;

    #[cfg(feature = "objc2-app-kit")]
    unsafe impl ClassType for WKWebView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAccessibility for WKWebView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAccessibilityElementProtocol for WKWebView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAnimatablePropertyContainer for WKWebView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAppearanceCustomization for WKWebView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSCoding for WKWebView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSDraggingDestination for WKWebView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSObjectProtocol for WKWebView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSUserInterfaceItemIdentification for WKWebView {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl WKWebView {
        #[cfg(feature = "WKWebViewConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Id<WKWebViewConfiguration>;

        #[cfg(feature = "WKNavigationDelegate")]
        #[method_id(@__retain_semantics Other navigationDelegate)]
        pub unsafe fn navigationDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn WKNavigationDelegate>>>;

        #[cfg(feature = "WKNavigationDelegate")]
        #[method(setNavigationDelegate:)]
        pub unsafe fn setNavigationDelegate(
            &self,
            navigation_delegate: Option<&ProtocolObject<dyn WKNavigationDelegate>>,
        );

        #[cfg(feature = "WKUIDelegate")]
        #[method_id(@__retain_semantics Other UIDelegate)]
        pub unsafe fn UIDelegate(&self) -> Option<Id<ProtocolObject<dyn WKUIDelegate>>>;

        #[cfg(feature = "WKUIDelegate")]
        #[method(setUIDelegate:)]
        pub unsafe fn setUIDelegate(&self, ui_delegate: Option<&ProtocolObject<dyn WKUIDelegate>>);

        #[cfg(feature = "WKBackForwardList")]
        #[method_id(@__retain_semantics Other backForwardList)]
        pub unsafe fn backForwardList(&self) -> Id<WKBackForwardList>;

        #[cfg(feature = "WKWebViewConfiguration")]
        #[method_id(@__retain_semantics Init initWithFrame:configuration:)]
        pub unsafe fn initWithFrame_configuration(
            this: Allocated<Self>,
            frame: CGRect,
            configuration: &WKWebViewConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other loadRequest:)]
        pub unsafe fn loadRequest(&self, request: &NSURLRequest) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other loadFileURL:allowingReadAccessToURL:)]
        pub unsafe fn loadFileURL_allowingReadAccessToURL(
            &self,
            url: &NSURL,
            read_access_url: &NSURL,
        ) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other loadHTMLString:baseURL:)]
        pub unsafe fn loadHTMLString_baseURL(
            &self,
            string: &NSString,
            base_url: Option<&NSURL>,
        ) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other loadData:MIMEType:characterEncodingName:baseURL:)]
        pub unsafe fn loadData_MIMEType_characterEncodingName_baseURL(
            &self,
            data: &NSData,
            mime_type: &NSString,
            character_encoding_name: &NSString,
            base_url: &NSURL,
        ) -> Option<Id<WKNavigation>>;

        #[cfg(all(feature = "WKBackForwardListItem", feature = "WKNavigation"))]
        #[method_id(@__retain_semantics Other goToBackForwardListItem:)]
        pub unsafe fn goToBackForwardListItem(
            &self,
            item: &WKBackForwardListItem,
        ) -> Option<Id<WKNavigation>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;

        #[method(estimatedProgress)]
        pub unsafe fn estimatedProgress(&self) -> c_double;

        #[method(hasOnlySecureContent)]
        pub unsafe fn hasOnlySecureContent(&self) -> bool;

        #[method(canGoBack)]
        pub unsafe fn canGoBack(&self) -> bool;

        #[method(canGoForward)]
        pub unsafe fn canGoForward(&self) -> bool;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other goBack)]
        pub unsafe fn goBack(&self) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other goForward)]
        pub unsafe fn goForward(&self) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other reload)]
        pub unsafe fn reload(&self) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other reloadFromOrigin)]
        pub unsafe fn reloadFromOrigin(&self) -> Option<Id<WKNavigation>>;

        #[method(stopLoading)]
        pub unsafe fn stopLoading(&self);

        #[cfg(feature = "block2")]
        #[method(evaluateJavaScript:completionHandler:)]
        pub unsafe fn evaluateJavaScript_completionHandler(
            &self,
            java_script_string: &NSString,
            completion_handler: Option<&block2::Block<dyn Fn(*mut AnyObject, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "WKContentWorld",
            feature = "WKFrameInfo",
            feature = "block2"
        ))]
        #[method(evaluateJavaScript:inFrame:inContentWorld:completionHandler:)]
        pub unsafe fn evaluateJavaScript_inFrame_inContentWorld_completionHandler(
            &self,
            java_script_string: &NSString,
            frame: Option<&WKFrameInfo>,
            content_world: &WKContentWorld,
            completion_handler: Option<&block2::Block<dyn Fn(*mut AnyObject, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "WKContentWorld",
            feature = "WKFrameInfo",
            feature = "block2"
        ))]
        #[method(callAsyncJavaScript:arguments:inFrame:inContentWorld:completionHandler:)]
        pub unsafe fn callAsyncJavaScript_arguments_inFrame_inContentWorld_completionHandler(
            &self,
            function_body: &NSString,
            arguments: Option<&NSDictionary<NSString, AnyObject>>,
            frame: Option<&WKFrameInfo>,
            content_world: &WKContentWorld,
            completion_handler: Option<&block2::Block<dyn Fn(*mut AnyObject, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(closeAllMediaPresentationsWithCompletionHandler:)]
        pub unsafe fn closeAllMediaPresentationsWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[deprecated]
        #[method(closeAllMediaPresentations)]
        pub unsafe fn closeAllMediaPresentations(&self);

        #[cfg(feature = "block2")]
        #[method(pauseAllMediaPlaybackWithCompletionHandler:)]
        pub unsafe fn pauseAllMediaPlaybackWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(pauseAllMediaPlayback:)]
        pub unsafe fn pauseAllMediaPlayback(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[method(setAllMediaPlaybackSuspended:completionHandler:)]
        pub unsafe fn setAllMediaPlaybackSuspended_completionHandler(
            &self,
            suspended: bool,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(resumeAllMediaPlayback:)]
        pub unsafe fn resumeAllMediaPlayback(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(suspendAllMediaPlayback:)]
        pub unsafe fn suspendAllMediaPlayback(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[method(requestMediaPlaybackStateWithCompletionHandler:)]
        pub unsafe fn requestMediaPlaybackStateWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(WKMediaPlaybackState)>,
        );

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(requestMediaPlaybackState:)]
        pub unsafe fn requestMediaPlaybackState(
            &self,
            completion_handler: &block2::Block<dyn Fn(WKMediaPlaybackState)>,
        );

        #[method(cameraCaptureState)]
        pub unsafe fn cameraCaptureState(&self) -> WKMediaCaptureState;

        #[method(microphoneCaptureState)]
        pub unsafe fn microphoneCaptureState(&self) -> WKMediaCaptureState;

        #[cfg(feature = "block2")]
        #[method(setCameraCaptureState:completionHandler:)]
        pub unsafe fn setCameraCaptureState_completionHandler(
            &self,
            state: WKMediaCaptureState,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[method(setMicrophoneCaptureState:completionHandler:)]
        pub unsafe fn setMicrophoneCaptureState_completionHandler(
            &self,
            state: WKMediaCaptureState,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(all(feature = "WKSnapshotConfiguration", feature = "block2"))]
        #[method(takeSnapshotWithConfiguration:completionHandler:)]
        pub unsafe fn takeSnapshotWithConfiguration_completionHandler(
            &self,
            snapshot_configuration: Option<&WKSnapshotConfiguration>,
            completion_handler: &block2::Block<dyn Fn(*mut NSImage, *mut NSError)>,
        );

        #[cfg(all(feature = "WKPDFConfiguration", feature = "block2"))]
        #[method(createPDFWithConfiguration:completionHandler:)]
        pub unsafe fn createPDFWithConfiguration_completionHandler(
            &self,
            pdf_configuration: Option<&WKPDFConfiguration>,
            completion_handler: &block2::Block<dyn Fn(*mut NSData, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(createWebArchiveDataWithCompletionHandler:)]
        pub unsafe fn createWebArchiveDataWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(NonNull<NSData>, NonNull<NSError>)>,
        );

        #[method(allowsBackForwardNavigationGestures)]
        pub unsafe fn allowsBackForwardNavigationGestures(&self) -> bool;

        #[method(setAllowsBackForwardNavigationGestures:)]
        pub unsafe fn setAllowsBackForwardNavigationGestures(
            &self,
            allows_back_forward_navigation_gestures: bool,
        );

        #[method_id(@__retain_semantics Other customUserAgent)]
        pub unsafe fn customUserAgent(&self) -> Option<Id<NSString>>;

        #[method(setCustomUserAgent:)]
        pub unsafe fn setCustomUserAgent(&self, custom_user_agent: Option<&NSString>);

        #[method(allowsLinkPreview)]
        pub unsafe fn allowsLinkPreview(&self) -> bool;

        #[method(setAllowsLinkPreview:)]
        pub unsafe fn setAllowsLinkPreview(&self, allows_link_preview: bool);

        #[method(allowsMagnification)]
        pub unsafe fn allowsMagnification(&self) -> bool;

        #[method(setAllowsMagnification:)]
        pub unsafe fn setAllowsMagnification(&self, allows_magnification: bool);

        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;

        #[method(setMagnification:)]
        pub unsafe fn setMagnification(&self, magnification: CGFloat);

        #[method(setMagnification:centeredAtPoint:)]
        pub unsafe fn setMagnification_centeredAtPoint(
            &self,
            magnification: CGFloat,
            point: CGPoint,
        );

        #[method(pageZoom)]
        pub unsafe fn pageZoom(&self) -> CGFloat;

        #[method(setPageZoom:)]
        pub unsafe fn setPageZoom(&self, page_zoom: CGFloat);

        #[cfg(all(
            feature = "WKFindConfiguration",
            feature = "WKFindResult",
            feature = "block2"
        ))]
        #[method(findString:withConfiguration:completionHandler:)]
        pub unsafe fn findString_withConfiguration_completionHandler(
            &self,
            string: &NSString,
            configuration: Option<&WKFindConfiguration>,
            completion_handler: &block2::Block<dyn Fn(NonNull<WKFindResult>)>,
        );

        #[method(handlesURLScheme:)]
        pub unsafe fn handlesURLScheme(url_scheme: &NSString, mtm: MainThreadMarker) -> bool;

        #[cfg(all(feature = "WKDownload", feature = "block2"))]
        #[method(startDownloadUsingRequest:completionHandler:)]
        pub unsafe fn startDownloadUsingRequest_completionHandler(
            &self,
            request: &NSURLRequest,
            completion_handler: &block2::Block<dyn Fn(NonNull<WKDownload>)>,
        );

        #[cfg(all(feature = "WKDownload", feature = "block2"))]
        #[method(resumeDownloadFromResumeData:completionHandler:)]
        pub unsafe fn resumeDownloadFromResumeData_completionHandler(
            &self,
            resume_data: &NSData,
            completion_handler: &block2::Block<dyn Fn(NonNull<WKDownload>)>,
        );

        #[method_id(@__retain_semantics Other mediaType)]
        pub unsafe fn mediaType(&self) -> Option<Id<NSString>>;

        #[method(setMediaType:)]
        pub unsafe fn setMediaType(&self, media_type: Option<&NSString>);

        #[method_id(@__retain_semantics Other interactionState)]
        pub unsafe fn interactionState(&self) -> Option<Id<AnyObject>>;

        #[method(setInteractionState:)]
        pub unsafe fn setInteractionState(&self, interaction_state: Option<&AnyObject>);

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other loadSimulatedRequest:response:responseData:)]
        pub unsafe fn loadSimulatedRequest_response_responseData(
            &self,
            request: &NSURLRequest,
            response: &NSURLResponse,
            data: &NSData,
        ) -> Id<WKNavigation>;

        #[cfg(feature = "WKNavigation")]
        #[deprecated]
        #[method_id(@__retain_semantics Other loadSimulatedRequest:withResponse:responseData:)]
        pub unsafe fn loadSimulatedRequest_withResponse_responseData(
            &self,
            request: &NSURLRequest,
            response: &NSURLResponse,
            data: &NSData,
        ) -> Id<WKNavigation>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other loadFileRequest:allowingReadAccessToURL:)]
        pub unsafe fn loadFileRequest_allowingReadAccessToURL(
            &self,
            request: &NSURLRequest,
            read_access_url: &NSURL,
        ) -> Id<WKNavigation>;

        #[cfg(feature = "WKNavigation")]
        #[method_id(@__retain_semantics Other loadSimulatedRequest:responseHTMLString:)]
        pub unsafe fn loadSimulatedRequest_responseHTMLString(
            &self,
            request: &NSURLRequest,
            string: &NSString,
        ) -> Id<WKNavigation>;

        #[cfg(feature = "WKNavigation")]
        #[deprecated]
        #[method_id(@__retain_semantics Other loadSimulatedRequest:withResponseHTMLString:)]
        pub unsafe fn loadSimulatedRequest_withResponseHTMLString(
            &self,
            request: &NSURLRequest,
            string: &NSString,
        ) -> Id<WKNavigation>;

        #[method_id(@__retain_semantics Other printOperationWithPrintInfo:)]
        pub unsafe fn printOperationWithPrintInfo(
            &self,
            print_info: &NSPrintInfo,
        ) -> Id<NSPrintOperation>;

        #[method_id(@__retain_semantics Other themeColor)]
        pub unsafe fn themeColor(&self) -> Option<Id<NSColor>>;

        #[method_id(@__retain_semantics Other underPageBackgroundColor)]
        pub unsafe fn underPageBackgroundColor(&self) -> Id<NSColor>;

        #[method(setUnderPageBackgroundColor:)]
        pub unsafe fn setUnderPageBackgroundColor(
            &self,
            under_page_background_color: Option<&NSColor>,
        );

        #[method(fullscreenState)]
        pub unsafe fn fullscreenState(&self) -> WKFullscreenState;

        #[method(minimumViewportInset)]
        pub unsafe fn minimumViewportInset(&self) -> NSEdgeInsets;

        #[method(maximumViewportInset)]
        pub unsafe fn maximumViewportInset(&self) -> NSEdgeInsets;

        #[method(setMinimumViewportInset:maximumViewportInset:)]
        pub unsafe fn setMinimumViewportInset_maximumViewportInset(
            &self,
            minimum_viewport_inset: NSEdgeInsets,
            maximum_viewport_inset: NSEdgeInsets,
        );

        #[method(isInspectable)]
        pub unsafe fn isInspectable(&self) -> bool;

        #[method(setInspectable:)]
        pub unsafe fn setInspectable(&self, inspectable: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl WKWebView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl WKWebView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl WKWebView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// WKIBActions
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl WKWebView {
        #[method(goBack:)]
        pub unsafe fn goBack_(&self, sender: Option<&AnyObject>);

        #[method(goForward:)]
        pub unsafe fn goForward_(&self, sender: Option<&AnyObject>);

        #[method(reload:)]
        pub unsafe fn reload_(&self, sender: Option<&AnyObject>);

        #[method(reloadFromOrigin:)]
        pub unsafe fn reloadFromOrigin_(&self, sender: Option<&AnyObject>);

        #[method(stopLoading:)]
        pub unsafe fn stopLoading_(&self, sender: Option<&AnyObject>);
    }
);

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSUserInterfaceValidations for WKWebView {}

extern_methods!(
    /// WKNSTextFinderClient
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl WKWebView {}
);

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSTextFinderClient for WKWebView {}

extern_methods!(
    /// WKDeprecated
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl WKWebView {
        #[deprecated]
        #[method_id(@__retain_semantics Other certificateChain)]
        pub unsafe fn certificateChain(&self) -> Id<NSArray>;
    }
);
