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

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uislider?language=objc)
    #[unsafe(super(UIControl, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    pub struct UISlider;
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UISlider {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UISlider {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UISlider {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearance for UISlider {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UISlider {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UISlider {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UISlider {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UISlider {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UISlider {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UISlider {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UISlider {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UISlider {}

extern_methods!(
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UISlider {
        #[method(value)]
        pub unsafe fn value(&self) -> c_float;

        /// Setter for [`value`][Self::value].
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_float);

        #[method(minimumValue)]
        pub unsafe fn minimumValue(&self) -> c_float;

        /// Setter for [`minimumValue`][Self::minimumValue].
        #[method(setMinimumValue:)]
        pub unsafe fn setMinimumValue(&self, minimum_value: c_float);

        #[method(maximumValue)]
        pub unsafe fn maximumValue(&self) -> c_float;

        /// Setter for [`maximumValue`][Self::maximumValue].
        #[method(setMaximumValue:)]
        pub unsafe fn setMaximumValue(&self, maximum_value: c_float);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other minimumValueImage)]
        pub unsafe fn minimumValueImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`minimumValueImage`][Self::minimumValueImage].
        #[method(setMinimumValueImage:)]
        pub unsafe fn setMinimumValueImage(&self, minimum_value_image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other maximumValueImage)]
        pub unsafe fn maximumValueImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`maximumValueImage`][Self::maximumValueImage].
        #[method(setMaximumValueImage:)]
        pub unsafe fn setMaximumValueImage(&self, maximum_value_image: Option<&UIImage>);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        /// Setter for [`isContinuous`][Self::isContinuous].
        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other minimumTrackTintColor)]
        pub unsafe fn minimumTrackTintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`minimumTrackTintColor`][Self::minimumTrackTintColor].
        #[method(setMinimumTrackTintColor:)]
        pub unsafe fn setMinimumTrackTintColor(&self, minimum_track_tint_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other maximumTrackTintColor)]
        pub unsafe fn maximumTrackTintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`maximumTrackTintColor`][Self::maximumTrackTintColor].
        #[method(setMaximumTrackTintColor:)]
        pub unsafe fn setMaximumTrackTintColor(&self, maximum_track_tint_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other thumbTintColor)]
        pub unsafe fn thumbTintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`thumbTintColor`][Self::thumbTintColor].
        #[method(setThumbTintColor:)]
        pub unsafe fn setThumbTintColor(&self, thumb_tint_color: Option<&UIColor>);

        #[method(setValue:animated:)]
        pub unsafe fn setValue_animated(&self, value: c_float, animated: bool);

        #[cfg(feature = "UIImage")]
        #[method(setThumbImage:forState:)]
        pub unsafe fn setThumbImage_forState(&self, image: Option<&UIImage>, state: UIControlState);

        #[cfg(feature = "UIImage")]
        #[method(setMinimumTrackImage:forState:)]
        pub unsafe fn setMinimumTrackImage_forState(
            &self,
            image: Option<&UIImage>,
            state: UIControlState,
        );

        #[cfg(feature = "UIImage")]
        #[method(setMaximumTrackImage:forState:)]
        pub unsafe fn setMaximumTrackImage_forState(
            &self,
            image: Option<&UIImage>,
            state: UIControlState,
        );

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other thumbImageForState:)]
        pub unsafe fn thumbImageForState(&self, state: UIControlState)
            -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other minimumTrackImageForState:)]
        pub unsafe fn minimumTrackImageForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other maximumTrackImageForState:)]
        pub unsafe fn maximumTrackImageForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other currentThumbImage)]
        pub unsafe fn currentThumbImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other currentMinimumTrackImage)]
        pub unsafe fn currentMinimumTrackImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other currentMaximumTrackImage)]
        pub unsafe fn currentMaximumTrackImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumValueImageRectForBounds:)]
        pub unsafe fn minimumValueImageRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(maximumValueImageRectForBounds:)]
        pub unsafe fn maximumValueImageRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(trackRectForBounds:)]
        pub unsafe fn trackRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(thumbRectForBounds:trackRect:value:)]
        pub unsafe fn thumbRectForBounds_trackRect_value(
            &self,
            bounds: CGRect,
            rect: CGRect,
            value: c_float,
        ) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIControl`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UISlider {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "UIAction",
            feature = "UIMenuElement",
            feature = "objc2-core-foundation"
        ))]
        /// Initializes the control and adds primaryAction for the UIControlEventPrimaryActionTriggered control event. Subclasses of UIControl may alter or add behaviors around the usage of primaryAction, see subclass documentation of this initializer for additional information.
        #[method_id(@__retain_semantics Init initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UISlider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
