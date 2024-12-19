//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistseparatorvisibility?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIListSeparatorVisibility(pub NSInteger);
impl UIListSeparatorVisibility {
    #[doc(alias = "UIListSeparatorVisibilityAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIListSeparatorVisibilityVisible")]
    pub const Visible: Self = Self(1);
    #[doc(alias = "UIListSeparatorVisibilityHidden")]
    pub const Hidden: Self = Self(2);
}

unsafe impl Encode for UIListSeparatorVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIListSeparatorVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistseparatorautomaticinsets?language=objc)
    #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
    pub static UIListSeparatorAutomaticInsets: NSDirectionalEdgeInsets;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistseparatorconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIListSeparatorConfiguration;
);

unsafe impl NSCoding for UIListSeparatorConfiguration {}

unsafe impl NSCopying for UIListSeparatorConfiguration {}

unsafe impl CopyingHelper for UIListSeparatorConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIListSeparatorConfiguration {}

unsafe impl NSSecureCoding for UIListSeparatorConfiguration {}

extern_methods!(
    unsafe impl UIListSeparatorConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "UICollectionLayoutList")]
        #[method_id(@__retain_semantics Init initWithListAppearance:)]
        pub unsafe fn initWithListAppearance(
            this: Allocated<Self>,
            list_appearance: UICollectionLayoutListAppearance,
        ) -> Retained<Self>;

        #[method(topSeparatorVisibility)]
        pub unsafe fn topSeparatorVisibility(&self) -> UIListSeparatorVisibility;

        #[method(setTopSeparatorVisibility:)]
        pub unsafe fn setTopSeparatorVisibility(
            &self,
            top_separator_visibility: UIListSeparatorVisibility,
        );

        #[method(bottomSeparatorVisibility)]
        pub unsafe fn bottomSeparatorVisibility(&self) -> UIListSeparatorVisibility;

        #[method(setBottomSeparatorVisibility:)]
        pub unsafe fn setBottomSeparatorVisibility(
            &self,
            bottom_separator_visibility: UIListSeparatorVisibility,
        );

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(topSeparatorInsets)]
        pub unsafe fn topSeparatorInsets(&self) -> NSDirectionalEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(setTopSeparatorInsets:)]
        pub unsafe fn setTopSeparatorInsets(&self, top_separator_insets: NSDirectionalEdgeInsets);

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(bottomSeparatorInsets)]
        pub unsafe fn bottomSeparatorInsets(&self) -> NSDirectionalEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(setBottomSeparatorInsets:)]
        pub unsafe fn setBottomSeparatorInsets(
            &self,
            bottom_separator_insets: NSDirectionalEdgeInsets,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<UIColor>;

        #[cfg(feature = "UIColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &UIColor);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other multipleSelectionColor)]
        pub unsafe fn multipleSelectionColor(&self) -> Retained<UIColor>;

        #[cfg(feature = "UIColor")]
        #[method(setMultipleSelectionColor:)]
        pub unsafe fn setMultipleSelectionColor(&self, multiple_selection_color: &UIColor);

        #[cfg(feature = "UIVisualEffect")]
        #[method_id(@__retain_semantics Other visualEffect)]
        pub unsafe fn visualEffect(&self) -> Option<Retained<UIVisualEffect>>;

        #[cfg(feature = "UIVisualEffect")]
        #[method(setVisualEffect:)]
        pub unsafe fn setVisualEffect(&self, visual_effect: Option<&UIVisualEffect>);
    }
);
