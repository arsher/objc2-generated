//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipreferredpresentationstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPreferredPresentationStyle(pub NSInteger);
impl UIPreferredPresentationStyle {
    #[doc(alias = "UIPreferredPresentationStyleUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIPreferredPresentationStyleInline")]
    pub const Inline: Self = Self(1);
    #[doc(alias = "UIPreferredPresentationStyleAttachment")]
    pub const Attachment: Self = Self(2);
}

unsafe impl Encode for UIPreferredPresentationStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPreferredPresentationStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_category!(
    /// Category "UIKitAdditions" on [`NSItemProvider`].
    #[doc(alias = "UIKitAdditions")]
    pub unsafe trait NSItemProviderUIKitAdditions {
        #[method_id(@__retain_semantics Other teamData)]
        unsafe fn teamData(&self) -> Option<Retained<NSData>>;

        #[method(setTeamData:)]
        unsafe fn setTeamData(&self, team_data: Option<&NSData>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredPresentationSize)]
        unsafe fn preferredPresentationSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setPreferredPresentationSize:)]
        unsafe fn setPreferredPresentationSize(&self, preferred_presentation_size: CGSize);

        #[method(preferredPresentationStyle)]
        unsafe fn preferredPresentationStyle(&self) -> UIPreferredPresentationStyle;

        #[method(setPreferredPresentationStyle:)]
        unsafe fn setPreferredPresentationStyle(
            &self,
            preferred_presentation_style: UIPreferredPresentationStyle,
        );
    }

    unsafe impl NSItemProviderUIKitAdditions for NSItemProvider {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiitemproviderpresentationsizeproviding?language=objc)
    pub unsafe trait UIItemProviderPresentationSizeProviding:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredPresentationSizeForItemProvider)]
        unsafe fn preferredPresentationSizeForItemProvider(&self) -> CGSize;
    }

    unsafe impl ProtocolType for dyn UIItemProviderPresentationSizeProviding {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiitemproviderreadingaugmentationproviding?language=objc)
    pub unsafe trait UIItemProviderReadingAugmentationProviding {
        #[method_id(@__retain_semantics Other objectWithItemProviderData:typeIdentifier:requestedClass:error:_)]
        unsafe fn objectWithItemProviderData_typeIdentifier_requestedClass_error(
            data: &NSData,
            type_identifier: &NSString,
            requested_class: &AnyClass,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other additionalLeadingReadableTypeIdentifiersForItemProvider)]
        unsafe fn additionalLeadingReadableTypeIdentifiersForItemProvider(
        ) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other additionalTrailingReadableTypeIdentifiersForItemProvider)]
        unsafe fn additionalTrailingReadableTypeIdentifiersForItemProvider(
        ) -> Retained<NSArray<NSString>>;
    }

    unsafe impl ProtocolType for dyn UIItemProviderReadingAugmentationProviding {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiitemproviderreadingaugmentationdesignating?language=objc)
    pub unsafe trait UIItemProviderReadingAugmentationDesignating:
        NSItemProviderReading
    {
        #[method(_ui_augmentingNSItemProviderReadingClass)]
        unsafe fn _ui_augmentingNSItemProviderReadingClass() -> &'static AnyClass;
    }

    unsafe impl ProtocolType for dyn UIItemProviderReadingAugmentationDesignating {}
);
