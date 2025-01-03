//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuserinterfacelayoutdirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSUserInterfaceLayoutDirection(pub NSInteger);
impl NSUserInterfaceLayoutDirection {
    #[doc(alias = "NSUserInterfaceLayoutDirectionLeftToRight")]
    pub const LeftToRight: Self = Self(0);
    #[doc(alias = "NSUserInterfaceLayoutDirectionRightToLeft")]
    pub const RightToLeft: Self = Self(1);
}

unsafe impl Encode for NSUserInterfaceLayoutDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSUserInterfaceLayoutDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuserinterfacelayoutorientation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSUserInterfaceLayoutOrientation(pub NSInteger);
impl NSUserInterfaceLayoutOrientation {
    #[doc(alias = "NSUserInterfaceLayoutOrientationHorizontal")]
    pub const Horizontal: Self = Self(0);
    #[doc(alias = "NSUserInterfaceLayoutOrientationVertical")]
    pub const Vertical: Self = Self(1);
}

unsafe impl Encode for NSUserInterfaceLayoutOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSUserInterfaceLayoutOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
