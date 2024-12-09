//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webframeview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    #[deprecated]
    pub struct WebFrameView;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for WebFrameView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for WebFrameView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for WebFrameView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for WebFrameView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for WebFrameView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for WebFrameView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for WebFrameView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for WebFrameView {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl WebFrameView {
        #[cfg(feature = "WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webFrame)]
        pub unsafe fn webFrame(&self) -> Option<Retained<WebFrame>>;

        #[cfg(feature = "WebDocument")]
        #[deprecated]
        #[method_id(@__retain_semantics Other documentView)]
        pub unsafe fn documentView(&self) -> Option<Retained<NSView>>;

        #[deprecated]
        #[method(allowsScrolling)]
        pub unsafe fn allowsScrolling(&self) -> bool;

        #[deprecated]
        #[method(setAllowsScrolling:)]
        pub unsafe fn setAllowsScrolling(&self, allows_scrolling: bool);

        #[deprecated]
        #[method(canPrintHeadersAndFooters)]
        pub unsafe fn canPrintHeadersAndFooters(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other printOperationWithPrintInfo:)]
        pub unsafe fn printOperationWithPrintInfo(
            &self,
            print_info: Option<&NSPrintInfo>,
        ) -> Option<Retained<NSPrintOperation>>;

        #[deprecated]
        #[method(documentViewShouldHandlePrint)]
        pub unsafe fn documentViewShouldHandlePrint(&self) -> bool;

        #[deprecated]
        #[method(printDocumentView)]
        pub unsafe fn printDocumentView(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl WebFrameView {
        #[cfg(feature = "objc2-core-foundation")]
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
    unsafe impl WebFrameView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl WebFrameView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
