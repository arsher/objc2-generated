//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C-unwind" {
    pub fn AXPrefersHorizontalTextLayout() -> Bool;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axprefershorizontaltextlayoutdidchangenotification?language=objc)
    pub static AXPrefersHorizontalTextLayoutDidChangeNotification: &'static NSNotificationName;
}

extern "C-unwind" {
    pub fn AXAnimatedImagesEnabled() -> Bool;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axanimatedimagesenableddidchangenotification?language=objc)
    pub static AXAnimatedImagesEnabledDidChangeNotification: &'static NSNotificationName;
}

extern "C-unwind" {
    pub fn AXAssistiveAccessEnabled() -> Bool;
}

extern "C-unwind" {
    pub fn AXPrefersNonBlinkingTextInsertionIndicator() -> Bool;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axprefersnonblinkingtextinsertionindicatordidchangenotification?language=objc)
    pub static AXPrefersNonBlinkingTextInsertionIndicatorDidChangeNotification:
        &'static NSNotificationName;
}

/// [Apple's documentation](https://developer.apple.com/documentation/accessibility/axsettingsfeature?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AXSettingsFeature(pub NSInteger);
impl AXSettingsFeature {
    #[doc(alias = "AXSettingsFeaturePersonalVoiceAllowAppsToRequestToUse")]
    pub const PersonalVoiceAllowAppsToRequestToUse: Self = Self(1);
}

unsafe impl Encode for AXSettingsFeature {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AXSettingsFeature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "block2")]
    pub fn AXOpenSettingsFeature(
        feature: AXSettingsFeature,
        completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
    );
}
