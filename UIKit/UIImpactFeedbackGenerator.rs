//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimpactfeedbackstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImpactFeedbackStyle(pub NSInteger);
impl UIImpactFeedbackStyle {
    #[doc(alias = "UIImpactFeedbackStyleLight")]
    pub const Light: Self = Self(0);
    #[doc(alias = "UIImpactFeedbackStyleMedium")]
    pub const Medium: Self = Self(1);
    #[doc(alias = "UIImpactFeedbackStyleHeavy")]
    pub const Heavy: Self = Self(2);
    #[doc(alias = "UIImpactFeedbackStyleSoft")]
    pub const Soft: Self = Self(3);
    #[doc(alias = "UIImpactFeedbackStyleRigid")]
    pub const Rigid: Self = Self(4);
}

unsafe impl Encode for UIImpactFeedbackStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImpactFeedbackStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator?language=objc)
    #[unsafe(super(UIFeedbackGenerator, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIFeedbackGenerator")]
    pub struct UIImpactFeedbackGenerator;
);

#[cfg(feature = "UIFeedbackGenerator")]
unsafe impl NSObjectProtocol for UIImpactFeedbackGenerator {}

extern_methods!(
    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl UIImpactFeedbackGenerator {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// set a style on the feedback generator and attach it to the provided view as an interaction.
        #[method_id(@__retain_semantics Other feedbackGeneratorWithStyle:forView:)]
        pub unsafe fn feedbackGeneratorWithStyle_forView(
            style: UIImpactFeedbackStyle,
            view: &UIView,
        ) -> Retained<Self>;

        /// call when your UI element impacts something else
        #[method(impactOccurred)]
        pub unsafe fn impactOccurred(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(impactOccurredAtLocation:)]
        pub unsafe fn impactOccurredAtLocation(&self, location: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        /// call when your UI element impacts something else with a specific intensity [0.0, 1.0]
        #[method(impactOccurredWithIntensity:)]
        pub unsafe fn impactOccurredWithIntensity(&self, intensity: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(impactOccurredWithIntensity:atLocation:)]
        pub unsafe fn impactOccurredWithIntensity_atLocation(
            &self,
            intensity: CGFloat,
            location: CGPoint,
        );

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithStyle:)]
        pub unsafe fn initWithStyle(
            this: Allocated<Self>,
            style: UIImpactFeedbackStyle,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIFeedbackGenerator`
    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl UIImpactFeedbackGenerator {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// initalize the generator with a view to attach it to the provided view as an interaction.
        #[method_id(@__retain_semantics Other feedbackGeneratorForView:)]
        pub unsafe fn feedbackGeneratorForView(view: &UIView) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl UIImpactFeedbackGenerator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
