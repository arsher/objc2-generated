//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;
#[cfg(feature = "objc2-symbols")]
use objc2_symbols::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimageview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIImageView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIImageView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIImageView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIImageView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIImageView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIImageView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIImageView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIImageView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIImageView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIImageView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIImageView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIImageView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIImageView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIImageView {
        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Init initWithImage:)]
        pub unsafe fn initWithImage(
            this: Allocated<Self>,
            image: Option<&UIImage>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Init initWithImage:highlightedImage:)]
        pub unsafe fn initWithImage_highlightedImage(
            this: Allocated<Self>,
            image: Option<&UIImage>,
            highlighted_image: Option<&UIImage>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other highlightedImage)]
        pub unsafe fn highlightedImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setHighlightedImage:)]
        pub unsafe fn setHighlightedImage(&self, highlighted_image: Option<&UIImage>);

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method_id(@__retain_semantics Other preferredSymbolConfiguration)]
        pub unsafe fn preferredSymbolConfiguration(
            &self,
        ) -> Option<Retained<UIImageSymbolConfiguration>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method(setPreferredSymbolConfiguration:)]
        pub unsafe fn setPreferredSymbolConfiguration(
            &self,
            preferred_symbol_configuration: Option<&UIImageSymbolConfiguration>,
        );

        #[method(isUserInteractionEnabled)]
        pub unsafe fn isUserInteractionEnabled(&self) -> bool;

        #[method(setUserInteractionEnabled:)]
        pub unsafe fn setUserInteractionEnabled(&self, user_interaction_enabled: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other animationImages)]
        pub unsafe fn animationImages(&self) -> Option<Retained<NSArray<UIImage>>>;

        #[cfg(feature = "UIImage")]
        #[method(setAnimationImages:)]
        pub unsafe fn setAnimationImages(&self, animation_images: Option<&NSArray<UIImage>>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other highlightedAnimationImages)]
        pub unsafe fn highlightedAnimationImages(&self) -> Option<Retained<NSArray<UIImage>>>;

        #[cfg(feature = "UIImage")]
        #[method(setHighlightedAnimationImages:)]
        pub unsafe fn setHighlightedAnimationImages(
            &self,
            highlighted_animation_images: Option<&NSArray<UIImage>>,
        );

        #[method(animationDuration)]
        pub unsafe fn animationDuration(&self) -> NSTimeInterval;

        #[method(setAnimationDuration:)]
        pub unsafe fn setAnimationDuration(&self, animation_duration: NSTimeInterval);

        #[method(animationRepeatCount)]
        pub unsafe fn animationRepeatCount(&self) -> NSInteger;

        #[method(setAnimationRepeatCount:)]
        pub unsafe fn setAnimationRepeatCount(&self, animation_repeat_count: NSInteger);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[method(startAnimating)]
        pub unsafe fn startAnimating(&self);

        #[method(stopAnimating)]
        pub unsafe fn stopAnimating(&self);

        #[method(isAnimating)]
        pub unsafe fn isAnimating(&self) -> bool;

        #[cfg(feature = "UIInterface")]
        #[method(preferredImageDynamicRange)]
        pub unsafe fn preferredImageDynamicRange(&self) -> UIImageDynamicRange;

        #[cfg(feature = "UIInterface")]
        #[method(setPreferredImageDynamicRange:)]
        pub unsafe fn setPreferredImageDynamicRange(
            &self,
            preferred_image_dynamic_range: UIImageDynamicRange,
        );

        #[cfg(feature = "UIInterface")]
        #[method(imageDynamicRange)]
        pub unsafe fn imageDynamicRange(&self) -> UIImageDynamicRange;

        #[method(adjustsImageWhenAncestorFocused)]
        pub unsafe fn adjustsImageWhenAncestorFocused(&self) -> bool;

        #[method(setAdjustsImageWhenAncestorFocused:)]
        pub unsafe fn setAdjustsImageWhenAncestorFocused(
            &self,
            adjusts_image_when_ancestor_focused: bool,
        );

        #[cfg(feature = "UILayoutGuide")]
        #[method_id(@__retain_semantics Other focusedFrameGuide)]
        pub unsafe fn focusedFrameGuide(&self) -> Retained<UILayoutGuide>;

        #[method_id(@__retain_semantics Other overlayContentView)]
        pub unsafe fn overlayContentView(&self) -> Retained<UIView>;

        #[method(masksFocusEffectToContents)]
        pub unsafe fn masksFocusEffectToContents(&self) -> bool;

        #[method(setMasksFocusEffectToContents:)]
        pub unsafe fn setMasksFocusEffectToContents(&self, masks_focus_effect_to_contents: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIImageView {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIImageView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIImageView {
        #[cfg(feature = "objc2-symbols")]
        #[method(addSymbolEffect:)]
        pub unsafe fn addSymbolEffect(&self, symbol_effect: &NSSymbolEffect);

        #[cfg(feature = "objc2-symbols")]
        #[method(addSymbolEffect:options:)]
        pub unsafe fn addSymbolEffect_options(
            &self,
            symbol_effect: &NSSymbolEffect,
            options: &NSSymbolEffectOptions,
        );

        #[cfg(feature = "objc2-symbols")]
        #[method(addSymbolEffect:options:animated:)]
        pub unsafe fn addSymbolEffect_options_animated(
            &self,
            symbol_effect: &NSSymbolEffect,
            options: &NSSymbolEffectOptions,
            animated: bool,
        );

        #[cfg(all(
            feature = "UISymbolEffectCompletion",
            feature = "block2",
            feature = "objc2-symbols"
        ))]
        #[method(addSymbolEffect:options:animated:completion:)]
        pub unsafe fn addSymbolEffect_options_animated_completion(
            &self,
            symbol_effect: &NSSymbolEffect,
            options: &NSSymbolEffectOptions,
            animated: bool,
            completion_handler: UISymbolEffectCompletion,
        );

        #[cfg(feature = "objc2-symbols")]
        #[method(removeSymbolEffectOfType:)]
        pub unsafe fn removeSymbolEffectOfType(&self, symbol_effect: &NSSymbolEffect);

        #[cfg(feature = "objc2-symbols")]
        #[method(removeSymbolEffectOfType:options:)]
        pub unsafe fn removeSymbolEffectOfType_options(
            &self,
            symbol_effect: &NSSymbolEffect,
            options: &NSSymbolEffectOptions,
        );

        #[cfg(feature = "objc2-symbols")]
        #[method(removeSymbolEffectOfType:options:animated:)]
        pub unsafe fn removeSymbolEffectOfType_options_animated(
            &self,
            symbol_effect: &NSSymbolEffect,
            options: &NSSymbolEffectOptions,
            animated: bool,
        );

        #[cfg(all(
            feature = "UISymbolEffectCompletion",
            feature = "block2",
            feature = "objc2-symbols"
        ))]
        #[method(removeSymbolEffectOfType:options:animated:completion:)]
        pub unsafe fn removeSymbolEffectOfType_options_animated_completion(
            &self,
            symbol_effect: &NSSymbolEffect,
            options: &NSSymbolEffectOptions,
            animated: bool,
            completion_handler: UISymbolEffectCompletion,
        );

        #[method(removeAllSymbolEffects)]
        pub unsafe fn removeAllSymbolEffects(&self);

        #[cfg(feature = "objc2-symbols")]
        #[method(removeAllSymbolEffectsWithOptions:)]
        pub unsafe fn removeAllSymbolEffectsWithOptions(&self, options: &NSSymbolEffectOptions);

        #[cfg(feature = "objc2-symbols")]
        #[method(removeAllSymbolEffectsWithOptions:animated:)]
        pub unsafe fn removeAllSymbolEffectsWithOptions_animated(
            &self,
            options: &NSSymbolEffectOptions,
            animated: bool,
        );

        #[cfg(all(feature = "UIImage", feature = "objc2-symbols"))]
        #[method(setSymbolImage:withContentTransition:)]
        pub unsafe fn setSymbolImage_withContentTransition(
            &self,
            symbol_image: &UIImage,
            transition: &NSSymbolContentTransition,
        );

        #[cfg(all(feature = "UIImage", feature = "objc2-symbols"))]
        #[method(setSymbolImage:withContentTransition:options:)]
        pub unsafe fn setSymbolImage_withContentTransition_options(
            &self,
            symbol_image: &UIImage,
            transition: &NSSymbolContentTransition,
            options: &NSSymbolEffectOptions,
        );

        #[cfg(all(
            feature = "UIImage",
            feature = "UISymbolEffectCompletion",
            feature = "block2",
            feature = "objc2-symbols"
        ))]
        #[method(setSymbolImage:withContentTransition:options:completion:)]
        pub unsafe fn setSymbolImage_withContentTransition_options_completion(
            &self,
            symbol_image: &UIImage,
            transition: &NSSymbolContentTransition,
            options: &NSSymbolEffectOptions,
            completion_handler: UISymbolEffectCompletion,
        );
    }
);
