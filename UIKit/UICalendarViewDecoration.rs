//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarviewdecorationsize?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICalendarViewDecorationSize(pub NSInteger);
impl UICalendarViewDecorationSize {
    #[doc(alias = "UICalendarViewDecorationSizeSmall")]
    pub const Small: Self = Self(0);
    #[doc(alias = "UICalendarViewDecorationSizeMedium")]
    pub const Medium: Self = Self(1);
    #[doc(alias = "UICalendarViewDecorationSizeLarge")]
    pub const Large: Self = Self(2);
}

unsafe impl Encode for UICalendarViewDecorationSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICalendarViewDecorationSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarviewdecoration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICalendarViewDecoration;
);

unsafe impl NSObjectProtocol for UICalendarViewDecoration {}

extern_methods!(
    unsafe impl UICalendarViewDecoration {
        /// Creates a default decoration with a circle image.
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "UIColor", feature = "UIImage"))]
        /// Creates a new image-based decoration with the specified image, color, and size.
        ///
        ///
        /// Parameter `image`: The image of the decoration, defaults to
        /// `circ`lebadge.fill if nil.
        ///
        /// Parameter `color`: The color of the the decoration. defaults to
        /// `UIColor.systemFillColor`if nil.
        ///
        /// Parameter `size`: The preferred size of the decoration. The default is UICalendarViewDecorationSizeMedium
        #[method_id(@__retain_semantics Init initWithImage:color:size:)]
        pub unsafe fn initWithImage_color_size(
            this: Allocated<Self>,
            image: Option<&UIImage>,
            color: Option<&UIColor>,
            size: UICalendarViewDecorationSize,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "block2"))]
        /// Creates a new custom view decoration using the provided view provider. The provider will
        /// be called once when the decoration view is first loaded. The decoration will be clipped to
        /// its parent's bounds, and cannot have interaction.
        #[method_id(@__retain_semantics Init initWithCustomViewProvider:)]
        pub unsafe fn initWithCustomViewProvider(
            this: Allocated<Self>,
            custom_view_provider: &block2::Block<dyn Fn() -> NonNull<UIView>>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIColor")]
        /// Creates a default image accessory with a circle image, and the specified color and size.
        #[method_id(@__retain_semantics Other decorationWithColor:size:)]
        pub unsafe fn decorationWithColor_size(
            color: Option<&UIColor>,
            size: UICalendarViewDecorationSize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        /// Creates an accessory with the specified image, and default color, and size.
        #[method_id(@__retain_semantics Other decorationWithImage:)]
        pub unsafe fn decorationWithImage(
            image: Option<&UIImage>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIColor", feature = "UIImage"))]
        /// Creates an accessory with the specified image, color, and size.
        #[method_id(@__retain_semantics Other decorationWithImage:color:size:)]
        pub unsafe fn decorationWithImage_color_size(
            image: Option<&UIImage>,
            color: Option<&UIColor>,
            size: UICalendarViewDecorationSize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "block2"))]
        /// Creates a new custom view decoration using the specified custom view provider. The provider will be called once
        /// when the decoration view is first loaded. The decoration will be clipped to its parent's bounds, and cannot have
        /// user interaction.
        #[method_id(@__retain_semantics Other decorationWithCustomViewProvider:)]
        pub unsafe fn decorationWithCustomViewProvider(
            custom_view_provider: &block2::Block<dyn Fn() -> NonNull<UIView>>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICalendarViewDecoration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
