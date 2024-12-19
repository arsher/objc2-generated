//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilargecontentvieweritem?language=objc)
    pub unsafe trait UILargeContentViewerItem: NSObjectProtocol + MainThreadOnly {
        #[method(showsLargeContentViewer)]
        unsafe fn showsLargeContentViewer(&self) -> bool;

        #[method_id(@__retain_semantics Other largeContentTitle)]
        unsafe fn largeContentTitle(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other largeContentImage)]
        unsafe fn largeContentImage(&self) -> Option<Retained<UIImage>>;

        #[method(scalesLargeContentImage)]
        unsafe fn scalesLargeContentImage(&self) -> bool;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(largeContentImageInsets)]
        unsafe fn largeContentImageInsets(&self) -> UIEdgeInsets;
    }

    unsafe impl ProtocolType for dyn UILargeContentViewerItem {}
);

extern_methods!(
    /// UILargeContentViewer
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIView {
        #[method(showsLargeContentViewer)]
        pub unsafe fn showsLargeContentViewer(&self) -> bool;

        #[method(setShowsLargeContentViewer:)]
        pub unsafe fn setShowsLargeContentViewer(&self, shows_large_content_viewer: bool);

        #[method_id(@__retain_semantics Other largeContentTitle)]
        pub unsafe fn largeContentTitle(&self) -> Option<Retained<NSString>>;

        #[method(setLargeContentTitle:)]
        pub unsafe fn setLargeContentTitle(&self, large_content_title: Option<&NSString>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other largeContentImage)]
        pub unsafe fn largeContentImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setLargeContentImage:)]
        pub unsafe fn setLargeContentImage(&self, large_content_image: Option<&UIImage>);

        #[method(scalesLargeContentImage)]
        pub unsafe fn scalesLargeContentImage(&self) -> bool;

        #[method(setScalesLargeContentImage:)]
        pub unsafe fn setScalesLargeContentImage(&self, scales_large_content_image: bool);

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(largeContentImageInsets)]
        pub unsafe fn largeContentImageInsets(&self) -> UIEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(setLargeContentImageInsets:)]
        pub unsafe fn setLargeContentImageInsets(&self, large_content_image_insets: UIEdgeInsets);
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UILargeContentViewerItem for UIView {}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilargecontentviewerinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UILargeContentViewerInteraction;
);

unsafe impl NSObjectProtocol for UILargeContentViewerInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UILargeContentViewerInteraction {}

extern_methods!(
    unsafe impl UILargeContentViewerInteraction {
        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: Option<&ProtocolObject<dyn UILargeContentViewerInteractionDelegate>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UILargeContentViewerInteractionDelegate>>>;

        #[cfg(feature = "UIGestureRecognizer")]
        #[method_id(@__retain_semantics Other gestureRecognizerForExclusionRelationship)]
        pub unsafe fn gestureRecognizerForExclusionRelationship(
            &self,
        ) -> Retained<UIGestureRecognizer>;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(mtm: MainThreadMarker) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UILargeContentViewerInteraction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilargecontentviewerinteractiondelegate?language=objc)
    pub unsafe trait UILargeContentViewerInteractionDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "objc2-core-foundation")]
        #[optional]
        #[method(largeContentViewerInteraction:didEndOnItem:atPoint:)]
        unsafe fn largeContentViewerInteraction_didEndOnItem_atPoint(
            &self,
            interaction: &UILargeContentViewerInteraction,
            item: Option<&ProtocolObject<dyn UILargeContentViewerItem>>,
            point: CGPoint,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[optional]
        #[method_id(@__retain_semantics Other largeContentViewerInteraction:itemAtPoint:)]
        unsafe fn largeContentViewerInteraction_itemAtPoint(
            &self,
            interaction: &UILargeContentViewerInteraction,
            point: CGPoint,
        ) -> Option<Retained<ProtocolObject<dyn UILargeContentViewerItem>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other viewControllerForLargeContentViewerInteraction:)]
        unsafe fn viewControllerForLargeContentViewerInteraction(
            &self,
            interaction: &UILargeContentViewerInteraction,
        ) -> Retained<UIViewController>;
    }

    unsafe impl ProtocolType for dyn UILargeContentViewerInteractionDelegate {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilargecontentviewerinteractionenabledstatusdidchangenotification?language=objc)
    pub static UILargeContentViewerInteractionEnabledStatusDidChangeNotification:
        &'static NSNotificationName;
}
