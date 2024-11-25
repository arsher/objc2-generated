//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipopoverarrowdirection?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPopoverArrowDirection(pub NSUInteger);
bitflags::bitflags! {
    impl UIPopoverArrowDirection: NSUInteger {
        #[doc(alias = "UIPopoverArrowDirectionUp")]
        const Up = 1<<0;
        #[doc(alias = "UIPopoverArrowDirectionDown")]
        const Down = 1<<1;
        #[doc(alias = "UIPopoverArrowDirectionLeft")]
        const Left = 1<<2;
        #[doc(alias = "UIPopoverArrowDirectionRight")]
        const Right = 1<<3;
        #[doc(alias = "UIPopoverArrowDirectionAny")]
        const Any = UIPopoverArrowDirection::Up.0|UIPopoverArrowDirection::Down.0|UIPopoverArrowDirection::Left.0|UIPopoverArrowDirection::Right.0;
        #[doc(alias = "UIPopoverArrowDirectionUnknown")]
        const Unknown = NSUIntegerMax as _;
    }
}

unsafe impl Encode for UIPopoverArrowDirection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIPopoverArrowDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// UIPopoverController
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIViewController {
        #[deprecated]
        #[method(isModalInPopover)]
        pub unsafe fn isModalInPopover(&self) -> bool;

        #[deprecated]
        #[method(setModalInPopover:)]
        pub unsafe fn setModalInPopover(&self, modal_in_popover: bool);

        #[deprecated]
        #[method(contentSizeForViewInPopover)]
        pub unsafe fn contentSizeForViewInPopover(&self) -> CGSize;

        #[deprecated]
        #[method(setContentSizeForViewInPopover:)]
        pub unsafe fn setContentSizeForViewInPopover(
            &self,
            content_size_for_view_in_popover: CGSize,
        );
    }
);
