//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uinavigationbarappearance?language=objc)
    #[unsafe(super(UIBarAppearance, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIBarAppearance")]
    pub struct UINavigationBarAppearance;
);

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSCoding for UINavigationBarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSCopying for UINavigationBarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl CopyingHelper for UINavigationBarAppearance {
    type Result = Self;
}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSObjectProtocol for UINavigationBarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSSecureCoding for UINavigationBarAppearance {}

extern_methods!(
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UINavigationBarAppearance {
        #[method_id(@__retain_semantics Other titleTextAttributes)]
        pub unsafe fn titleTextAttributes(
            &self,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method(setTitleTextAttributes:)]
        pub unsafe fn setTitleTextAttributes(
            &self,
            title_text_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        );

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(titlePositionAdjustment)]
        pub unsafe fn titlePositionAdjustment(&self) -> UIOffset;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(setTitlePositionAdjustment:)]
        pub unsafe fn setTitlePositionAdjustment(&self, title_position_adjustment: UIOffset);

        #[method_id(@__retain_semantics Other largeTitleTextAttributes)]
        pub unsafe fn largeTitleTextAttributes(
            &self,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method(setLargeTitleTextAttributes:)]
        pub unsafe fn setLargeTitleTextAttributes(
            &self,
            large_title_text_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        );

        #[cfg(feature = "UIBarButtonItemAppearance")]
        #[method_id(@__retain_semantics Other buttonAppearance)]
        pub unsafe fn buttonAppearance(&self) -> Retained<UIBarButtonItemAppearance>;

        #[cfg(feature = "UIBarButtonItemAppearance")]
        #[method(setButtonAppearance:)]
        pub unsafe fn setButtonAppearance(&self, button_appearance: &UIBarButtonItemAppearance);

        #[cfg(feature = "UIBarButtonItemAppearance")]
        #[method_id(@__retain_semantics Other doneButtonAppearance)]
        pub unsafe fn doneButtonAppearance(&self) -> Retained<UIBarButtonItemAppearance>;

        #[cfg(feature = "UIBarButtonItemAppearance")]
        #[method(setDoneButtonAppearance:)]
        pub unsafe fn setDoneButtonAppearance(
            &self,
            done_button_appearance: &UIBarButtonItemAppearance,
        );

        #[cfg(feature = "UIBarButtonItemAppearance")]
        #[method_id(@__retain_semantics Other backButtonAppearance)]
        pub unsafe fn backButtonAppearance(&self) -> Retained<UIBarButtonItemAppearance>;

        #[cfg(feature = "UIBarButtonItemAppearance")]
        #[method(setBackButtonAppearance:)]
        pub unsafe fn setBackButtonAppearance(
            &self,
            back_button_appearance: &UIBarButtonItemAppearance,
        );

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other backIndicatorImage)]
        pub unsafe fn backIndicatorImage(&self) -> Retained<UIImage>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other backIndicatorTransitionMaskImage)]
        pub unsafe fn backIndicatorTransitionMaskImage(&self) -> Retained<UIImage>;

        #[cfg(feature = "UIImage")]
        #[method(setBackIndicatorImage:transitionMaskImage:)]
        pub unsafe fn setBackIndicatorImage_transitionMaskImage(
            &self,
            back_indicator_image: Option<&UIImage>,
            back_indicator_transition_mask_image: Option<&UIImage>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIBarAppearance`
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UINavigationBarAppearance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIDevice")]
        #[method_id(@__retain_semantics Init initWithIdiom:)]
        pub unsafe fn initWithIdiom(
            this: Allocated<Self>,
            idiom: UIUserInterfaceIdiom,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBarAppearance:)]
        pub unsafe fn initWithBarAppearance(
            this: Allocated<Self>,
            bar_appearance: &UIBarAppearance,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UINavigationBarAppearance {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
