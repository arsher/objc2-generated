//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisceneactivationstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISceneActivationState(pub NSInteger);
impl UISceneActivationState {
    #[doc(alias = "UISceneActivationStateUnattached")]
    pub const Unattached: Self = Self(-1);
    #[doc(alias = "UISceneActivationStateForegroundActive")]
    pub const ForegroundActive: Self = Self(0);
    #[doc(alias = "UISceneActivationStateForegroundInactive")]
    pub const ForegroundInactive: Self = Self(1);
    #[doc(alias = "UISceneActivationStateBackground")]
    pub const Background: Self = Self(2);
}

unsafe impl Encode for UISceneActivationState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISceneActivationState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscenecapturestate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISceneCaptureState(pub NSInteger);
impl UISceneCaptureState {
    #[doc(alias = "UISceneCaptureStateUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UISceneCaptureStateInactive")]
    pub const Inactive: Self = Self(0);
    #[doc(alias = "UISceneCaptureStateActive")]
    pub const Active: Self = Self(1);
}

unsafe impl Encode for UISceneCaptureState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISceneCaptureState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscenesessionrole?language=objc)
// NS_TYPED_ENUM
pub type UISceneSessionRole = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisceneerrordomain?language=objc)
    pub static UISceneErrorDomain: Option<&'static NSErrorDomain>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisceneerrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISceneErrorCode(pub NSInteger);
impl UISceneErrorCode {
    #[doc(alias = "UISceneErrorCodeMultipleScenesNotSupported")]
    pub const MultipleScenesNotSupported: Self = Self(0);
    #[doc(alias = "UISceneErrorCodeRequestDenied")]
    pub const RequestDenied: Self = Self(1);
    #[doc(alias = "UISceneErrorCodeGeometryRequestUnsupported")]
    pub const GeometryRequestUnsupported: Self = Self(100);
    #[doc(alias = "UISceneErrorCodeGeometryRequestDenied")]
    pub const GeometryRequestDenied: Self = Self(101);
}

unsafe impl Encode for UISceneErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISceneErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
