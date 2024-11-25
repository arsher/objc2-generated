//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextselectionhighlightview?language=objc)
    #[cfg(feature = "UIView")]
    pub unsafe trait UITextSelectionHighlightView:
        UICoordinateSpace + MainThreadOnly
    {
        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other selectionRects)]
        unsafe fn selectionRects(&self) -> Retained<NSArray<UITextSelectionRect>>;

        #[cfg(feature = "UITextInput")]
        #[method(setSelectionRects:)]
        unsafe fn setSelectionRects(&self, selection_rects: &NSArray<UITextSelectionRect>);
    }

    #[cfg(feature = "UIView")]
    unsafe impl ProtocolType for dyn UITextSelectionHighlightView {}
);
