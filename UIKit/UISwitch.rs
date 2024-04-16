//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
use objc2_quartz_core::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISwitchStyle(pub NSInteger);
impl UISwitchStyle {
    #[doc(alias = "UISwitchStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UISwitchStyleCheckbox")]
    pub const Checkbox: Self = Self(1);
    #[doc(alias = "UISwitchStyleSliding")]
    pub const Sliding: Self = Self(2);
}

unsafe impl Encode for UISwitchStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISwitchStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    pub struct UISwitch;

    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl ClassType for UISwitch {
        #[inherits(UIView, UIResponder, NSObject)]
        type Super = UIControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
unsafe impl CALayerDelegate for UISwitch {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UISwitch {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UISwitch {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearance for UISwitch {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UISwitch {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UISwitch {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UISwitch {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UISwitch {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UISwitch {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UISwitch {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UISwitch {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UISwitch {}

extern_methods!(
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UISwitch {
        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other onTintColor)]
        pub unsafe fn onTintColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setOnTintColor:)]
        pub unsafe fn setOnTintColor(&self, on_tint_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other thumbTintColor)]
        pub unsafe fn thumbTintColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setThumbTintColor:)]
        pub unsafe fn setThumbTintColor(&self, thumb_tint_color: Option<&UIColor>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other onImage)]
        pub unsafe fn onImage(&self) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setOnImage:)]
        pub unsafe fn setOnImage(&self, on_image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other offImage)]
        pub unsafe fn offImage(&self) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setOffImage:)]
        pub unsafe fn setOffImage(&self, off_image: Option<&UIImage>);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method(style)]
        pub unsafe fn style(&self) -> UISwitchStyle;

        #[method(preferredStyle)]
        pub unsafe fn preferredStyle(&self) -> UISwitchStyle;

        #[method(setPreferredStyle:)]
        pub unsafe fn setPreferredStyle(&self, preferred_style: UISwitchStyle);

        #[method(isOn)]
        pub unsafe fn isOn(&self) -> bool;

        #[method(setOn:)]
        pub unsafe fn setOn(&self, on: bool);

        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method(setOn:animated:)]
        pub unsafe fn setOn_animated(&self, on: bool, animated: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIControl`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UISwitch {
        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Init initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UISwitch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
