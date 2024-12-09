//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextloupesession?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextLoupeSession;
);

unsafe impl NSObjectProtocol for UITextLoupeSession {}

extern_methods!(
    unsafe impl UITextLoupeSession {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method_id(@__retain_semantics Other beginLoupeSessionAtPoint:fromSelectionWidgetView:inView:)]
        pub unsafe fn beginLoupeSessionAtPoint_fromSelectionWidgetView_inView(
            point: CGPoint,
            selection_widget: Option<&UIView>,
            interaction_view: &UIView,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(moveToPoint:withCaretRect:trackingCaret:)]
        pub unsafe fn moveToPoint_withCaretRect_trackingCaret(
            &self,
            point: CGPoint,
            caret_rect: CGRect,
            tracks_caret: bool,
        );

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextLoupeSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
