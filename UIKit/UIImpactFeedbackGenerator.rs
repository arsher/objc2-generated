//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

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
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIFeedbackGenerator")]
    pub struct UIImpactFeedbackGenerator;

    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl ClassType for UIImpactFeedbackGenerator {
        #[inherits(NSObject)]
        type Super = UIFeedbackGenerator;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIFeedbackGenerator")]
unsafe impl NSObjectProtocol for UIImpactFeedbackGenerator {}

extern_methods!(
    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl UIImpactFeedbackGenerator {
        #[method_id(@__retain_semantics Init initWithStyle:)]
        pub unsafe fn initWithStyle(
            this: Allocated<Self>,
            style: UIImpactFeedbackStyle,
        ) -> Id<Self>;

        #[method(impactOccurred)]
        pub unsafe fn impactOccurred(&self);

        #[method(impactOccurredWithIntensity:)]
        pub unsafe fn impactOccurredWithIntensity(&self, intensity: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl UIImpactFeedbackGenerator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
