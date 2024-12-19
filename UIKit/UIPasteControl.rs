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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipastecontroldisplaymode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPasteControlDisplayMode(pub NSUInteger);
impl UIPasteControlDisplayMode {
    #[doc(alias = "UIPasteControlDisplayModeIconAndLabel")]
    pub const IconAndLabel: Self = Self(0);
    #[doc(alias = "UIPasteControlDisplayModeIconOnly")]
    pub const IconOnly: Self = Self(1);
    #[doc(alias = "UIPasteControlDisplayModeLabelOnly")]
    pub const LabelOnly: Self = Self(2);
    #[doc(alias = "UIPasteControlDisplayModeArrowAndLabel")]
    pub const ArrowAndLabel: Self = Self(3);
}

unsafe impl Encode for UIPasteControlDisplayMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIPasteControlDisplayMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipastecontrolconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPasteControlConfiguration;
);

unsafe impl NSCoding for UIPasteControlConfiguration {}

unsafe impl NSObjectProtocol for UIPasteControlConfiguration {}

unsafe impl NSSecureCoding for UIPasteControlConfiguration {}

extern_methods!(
    unsafe impl UIPasteControlConfiguration {
        #[method(displayMode)]
        pub unsafe fn displayMode(&self) -> UIPasteControlDisplayMode;

        #[method(setDisplayMode:)]
        pub unsafe fn setDisplayMode(&self, display_mode: UIPasteControlDisplayMode);

        #[cfg(feature = "UIButtonConfiguration")]
        #[method(cornerStyle)]
        pub unsafe fn cornerStyle(&self) -> UIButtonConfigurationCornerStyle;

        #[cfg(feature = "UIButtonConfiguration")]
        #[method(setCornerStyle:)]
        pub unsafe fn setCornerStyle(&self, corner_style: UIButtonConfigurationCornerStyle);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[cfg(feature = "UIGeometry")]
        #[method(imagePlacement)]
        pub unsafe fn imagePlacement(&self) -> NSDirectionalRectEdge;

        #[cfg(feature = "UIGeometry")]
        #[method(setImagePlacement:)]
        pub unsafe fn setImagePlacement(&self, image_placement: NSDirectionalRectEdge);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other baseForegroundColor)]
        pub unsafe fn baseForegroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBaseForegroundColor:)]
        pub unsafe fn setBaseForegroundColor(&self, base_foreground_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other baseBackgroundColor)]
        pub unsafe fn baseBackgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBaseBackgroundColor:)]
        pub unsafe fn setBaseBackgroundColor(&self, base_background_color: Option<&UIColor>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPasteControlConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipastecontrol?language=objc)
    #[unsafe(super(UIControl, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    pub struct UIPasteControl;
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIPasteControl {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIPasteControl {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIPasteControl {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearance for UIPasteControl {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UIPasteControl {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIPasteControl {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIPasteControl {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UIPasteControl {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UIPasteControl {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UIPasteControl {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIPasteControl {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIPasteControl {}

extern_methods!(
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIPasteControl {
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Retained<UIPasteControlConfiguration>;

        #[cfg(feature = "UIPasteConfigurationSupporting")]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPasteConfigurationSupporting>>>;

        #[cfg(feature = "UIPasteConfigurationSupporting")]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setTarget:)]
        pub unsafe fn setTarget(
            &self,
            target: Option<&ProtocolObject<dyn UIPasteConfigurationSupporting>>,
        );

        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &UIPasteControlConfiguration,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIControl`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIPasteControl {
        #[cfg(all(
            feature = "UIAction",
            feature = "UIMenuElement",
            feature = "objc2-core-foundation"
        ))]
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
    unsafe impl UIPasteControl {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
