//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityzoomtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityZoomType(pub NSInteger);
impl UIAccessibilityZoomType {
    #[doc(alias = "UIAccessibilityZoomTypeInsertionPoint")]
    pub const InsertionPoint: Self = Self(0);
}

unsafe impl Encode for UIAccessibilityZoomType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityZoomType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub fn UIAccessibilityZoomFocusChanged(
        r#type: UIAccessibilityZoomType,
        frame: CGRect,
        view: &UIView,
    );
}

extern "C-unwind" {
    pub fn UIAccessibilityRegisterGestureConflictWithZoom();
}
