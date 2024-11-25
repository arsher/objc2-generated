//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiguidedaccesserrordomain?language=objc)
    pub static UIGuidedAccessErrorDomain: &'static NSErrorDomain;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiguidedaccesserrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIGuidedAccessErrorCode(pub NSInteger);
impl UIGuidedAccessErrorCode {
    pub const UIGuidedAccessErrorPermissionDenied: Self = Self(0);
    pub const UIGuidedAccessErrorFailed: Self = Self(NSIntegerMax as _);
}

unsafe impl Encode for UIGuidedAccessErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIGuidedAccessErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiguidedaccessrestrictionstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIGuidedAccessRestrictionState(pub NSInteger);
impl UIGuidedAccessRestrictionState {
    #[doc(alias = "UIGuidedAccessRestrictionStateAllow")]
    pub const Allow: Self = Self(0);
    #[doc(alias = "UIGuidedAccessRestrictionStateDeny")]
    pub const Deny: Self = Self(1);
}

unsafe impl Encode for UIGuidedAccessRestrictionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIGuidedAccessRestrictionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiguidedaccessrestrictiondelegate?language=objc)
    pub unsafe trait UIGuidedAccessRestrictionDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[method_id(@__retain_semantics Other guidedAccessRestrictionIdentifiers)]
        unsafe fn guidedAccessRestrictionIdentifiers(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(guidedAccessRestrictionWithIdentifier:didChangeState:)]
        unsafe fn guidedAccessRestrictionWithIdentifier_didChangeState(
            &self,
            restriction_identifier: &NSString,
            new_restriction_state: UIGuidedAccessRestrictionState,
        );

        #[method_id(@__retain_semantics Other textForGuidedAccessRestrictionWithIdentifier:)]
        unsafe fn textForGuidedAccessRestrictionWithIdentifier(
            &self,
            restriction_identifier: &NSString,
        ) -> Option<Retained<NSString>>;

        #[optional]
        #[method_id(@__retain_semantics Other detailTextForGuidedAccessRestrictionWithIdentifier:)]
        unsafe fn detailTextForGuidedAccessRestrictionWithIdentifier(
            &self,
            restriction_identifier: &NSString,
        ) -> Option<Retained<NSString>>;
    }

    unsafe impl ProtocolType for dyn UIGuidedAccessRestrictionDelegate {}
);

extern "C-unwind" {
    pub fn UIGuidedAccessRestrictionStateForIdentifier(
        restriction_identifier: &NSString,
    ) -> UIGuidedAccessRestrictionState;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiguidedaccessaccessibilityfeature?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIGuidedAccessAccessibilityFeature(pub NSUInteger);
bitflags::bitflags! {
    impl UIGuidedAccessAccessibilityFeature: NSUInteger {
        #[doc(alias = "UIGuidedAccessAccessibilityFeatureVoiceOver")]
        const VoiceOver = 1<<0;
        #[doc(alias = "UIGuidedAccessAccessibilityFeatureZoom")]
        const Zoom = 1<<1;
        #[doc(alias = "UIGuidedAccessAccessibilityFeatureAssistiveTouch")]
        const AssistiveTouch = 1<<2;
        #[doc(alias = "UIGuidedAccessAccessibilityFeatureInvertColors")]
        const InvertColors = 1<<3;
        #[doc(alias = "UIGuidedAccessAccessibilityFeatureGrayscaleDisplay")]
        const GrayscaleDisplay = 1<<4;
    }
}

unsafe impl Encode for UIGuidedAccessAccessibilityFeature {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIGuidedAccessAccessibilityFeature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "block2")]
    pub fn UIGuidedAccessConfigureAccessibilityFeatures(
        features: UIGuidedAccessAccessibilityFeature,
        enabled: Bool,
        completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
    );
}
