//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonType(pub NSInteger);
impl UIButtonType {
    #[doc(alias = "UIButtonTypeCustom")]
    pub const Custom: Self = Self(0);
    #[doc(alias = "UIButtonTypeSystem")]
    pub const System: Self = Self(1);
    #[doc(alias = "UIButtonTypeDetailDisclosure")]
    pub const DetailDisclosure: Self = Self(2);
    #[doc(alias = "UIButtonTypeInfoLight")]
    pub const InfoLight: Self = Self(3);
    #[doc(alias = "UIButtonTypeInfoDark")]
    pub const InfoDark: Self = Self(4);
    #[doc(alias = "UIButtonTypeContactAdd")]
    pub const ContactAdd: Self = Self(5);
    #[doc(alias = "UIButtonTypePlain")]
    pub const Plain: Self = Self(6);
    #[doc(alias = "UIButtonTypeClose")]
    pub const Close: Self = Self(7);
    #[doc(alias = "UIButtonTypeRoundedRect")]
    pub const RoundedRect: Self = Self(UIButtonType::System.0);
}

unsafe impl Encode for UIButtonType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIButtonRole(pub NSInteger);
impl UIButtonRole {
    #[doc(alias = "UIButtonRoleNormal")]
    pub const Normal: Self = Self(0);
    #[doc(alias = "UIButtonRolePrimary")]
    pub const Primary: Self = Self(1);
    #[doc(alias = "UIButtonRoleCancel")]
    pub const Cancel: Self = Self(2);
    #[doc(alias = "UIButtonRoleDestructive")]
    pub const Destructive: Self = Self(3);
}

unsafe impl Encode for UIButtonRole {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIButtonRole {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(all(
    feature = "UIControl",
    feature = "UIHoverStyle",
    feature = "UIPointerStyle",
    feature = "UIResponder",
    feature = "UIView",
    feature = "block2"
))]
pub type UIButtonPointerStyleProvider = *mut block2::Block<
    dyn Fn(
        NonNull<UIButton>,
        NonNull<UIPointerEffect>,
        NonNull<UIPointerShape>,
    ) -> *mut UIPointerStyle,
>;

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView",
    feature = "block2"
))]
pub type UIButtonConfigurationUpdateHandler = *mut block2::Block<dyn Fn(NonNull<UIButton>)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    pub struct UIButton;

    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl ClassType for UIButton {
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
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIButton {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIButton {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIButton {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearance for UIButton {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UIButton {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIButton {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIButton {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UIButton {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UIButton {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UIButton {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIButton {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIButton {}

extern_methods!(
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIButton {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Init initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other buttonWithType:)]
        pub unsafe fn buttonWithType(button_type: UIButtonType, mtm: MainThreadMarker) -> Id<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other systemButtonWithImage:target:action:)]
        pub unsafe fn systemButtonWithImage_target_action(
            image: &UIImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other systemButtonWithPrimaryAction:)]
        pub unsafe fn systemButtonWithPrimaryAction(
            primary_action: Option<&UIAction>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other buttonWithType:primaryAction:)]
        pub unsafe fn buttonWithType_primaryAction(
            button_type: UIButtonType,
            primary_action: Option<&UIAction>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "UIAction",
            feature = "UIButtonConfiguration",
            feature = "UIMenuElement"
        ))]
        #[method_id(@__retain_semantics Other buttonWithConfiguration:primaryAction:)]
        pub unsafe fn buttonWithConfiguration_primaryAction(
            configuration: &UIButtonConfiguration,
            primary_action: Option<&UIAction>,
        ) -> Id<Self>;

        #[cfg(feature = "UIButtonConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Option<Id<UIButtonConfiguration>>;

        #[cfg(feature = "UIButtonConfiguration")]
        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: Option<&UIButtonConfiguration>);

        #[method(setNeedsUpdateConfiguration)]
        pub unsafe fn setNeedsUpdateConfiguration(&self);

        #[method(updateConfiguration)]
        pub unsafe fn updateConfiguration(&self);

        #[cfg(feature = "block2")]
        #[method(configurationUpdateHandler)]
        pub unsafe fn configurationUpdateHandler(&self) -> UIButtonConfigurationUpdateHandler;

        #[cfg(feature = "block2")]
        #[method(setConfigurationUpdateHandler:)]
        pub unsafe fn setConfigurationUpdateHandler(
            &self,
            configuration_update_handler: UIButtonConfigurationUpdateHandler,
        );

        #[method(automaticallyUpdatesConfiguration)]
        pub unsafe fn automaticallyUpdatesConfiguration(&self) -> bool;

        #[method(setAutomaticallyUpdatesConfiguration:)]
        pub unsafe fn setAutomaticallyUpdatesConfiguration(
            &self,
            automatically_updates_configuration: bool,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[method(buttonType)]
        pub unsafe fn buttonType(&self) -> UIButtonType;

        #[method(isHovered)]
        pub unsafe fn isHovered(&self) -> bool;

        #[method(isHeld)]
        pub unsafe fn isHeld(&self) -> bool;

        #[method(role)]
        pub unsafe fn role(&self) -> UIButtonRole;

        #[method(setRole:)]
        pub unsafe fn setRole(&self, role: UIButtonRole);

        #[method(isPointerInteractionEnabled)]
        pub unsafe fn isPointerInteractionEnabled(&self) -> bool;

        #[method(setPointerInteractionEnabled:)]
        pub unsafe fn setPointerInteractionEnabled(&self, pointer_interaction_enabled: bool);

        #[cfg(all(
            feature = "UIHoverStyle",
            feature = "UIPointerStyle",
            feature = "block2"
        ))]
        #[method(pointerStyleProvider)]
        pub unsafe fn pointerStyleProvider(&self) -> UIButtonPointerStyleProvider;

        #[cfg(all(
            feature = "UIHoverStyle",
            feature = "UIPointerStyle",
            feature = "block2"
        ))]
        #[method(setPointerStyleProvider:)]
        pub unsafe fn setPointerStyleProvider(
            &self,
            pointer_style_provider: UIButtonPointerStyleProvider,
        );

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<UIMenu>>;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&UIMenu>);

        #[cfg(feature = "UIContextMenuConfiguration")]
        #[method(preferredMenuElementOrder)]
        pub unsafe fn preferredMenuElementOrder(&self) -> UIContextMenuConfigurationElementOrder;

        #[cfg(feature = "UIContextMenuConfiguration")]
        #[method(setPreferredMenuElementOrder:)]
        pub unsafe fn setPreferredMenuElementOrder(
            &self,
            preferred_menu_element_order: UIContextMenuConfigurationElementOrder,
        );

        #[method(changesSelectionAsPrimaryAction)]
        pub unsafe fn changesSelectionAsPrimaryAction(&self) -> bool;

        #[method(setChangesSelectionAsPrimaryAction:)]
        pub unsafe fn setChangesSelectionAsPrimaryAction(
            &self,
            changes_selection_as_primary_action: bool,
        );

        #[method(setTitle:forState:)]
        pub unsafe fn setTitle_forState(&self, title: Option<&NSString>, state: UIControlState);

        #[cfg(feature = "UIColor")]
        #[method(setTitleColor:forState:)]
        pub unsafe fn setTitleColor_forState(&self, color: Option<&UIColor>, state: UIControlState);

        #[cfg(feature = "UIColor")]
        #[method(setTitleShadowColor:forState:)]
        pub unsafe fn setTitleShadowColor_forState(
            &self,
            color: Option<&UIColor>,
            state: UIControlState,
        );

        #[cfg(feature = "UIImage")]
        #[method(setImage:forState:)]
        pub unsafe fn setImage_forState(&self, image: Option<&UIImage>, state: UIControlState);

        #[cfg(feature = "UIImage")]
        #[method(setBackgroundImage:forState:)]
        pub unsafe fn setBackgroundImage_forState(
            &self,
            image: Option<&UIImage>,
            state: UIControlState,
        );

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method(setPreferredSymbolConfiguration:forImageInState:)]
        pub unsafe fn setPreferredSymbolConfiguration_forImageInState(
            &self,
            configuration: Option<&UIImageSymbolConfiguration>,
            state: UIControlState,
        );

        #[method(setAttributedTitle:forState:)]
        pub unsafe fn setAttributedTitle_forState(
            &self,
            title: Option<&NSAttributedString>,
            state: UIControlState,
        );

        #[method_id(@__retain_semantics Other titleForState:)]
        pub unsafe fn titleForState(&self, state: UIControlState) -> Option<Id<NSString>>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other titleColorForState:)]
        pub unsafe fn titleColorForState(&self, state: UIControlState) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other titleShadowColorForState:)]
        pub unsafe fn titleShadowColorForState(&self, state: UIControlState)
            -> Option<Id<UIColor>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other imageForState:)]
        pub unsafe fn imageForState(&self, state: UIControlState) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other backgroundImageForState:)]
        pub unsafe fn backgroundImageForState(&self, state: UIControlState) -> Option<Id<UIImage>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method_id(@__retain_semantics Other preferredSymbolConfigurationForImageInState:)]
        pub unsafe fn preferredSymbolConfigurationForImageInState(
            &self,
            state: UIControlState,
        ) -> Option<Id<UIImageSymbolConfiguration>>;

        #[method_id(@__retain_semantics Other attributedTitleForState:)]
        pub unsafe fn attributedTitleForState(
            &self,
            state: UIControlState,
        ) -> Option<Id<NSAttributedString>>;

        #[method_id(@__retain_semantics Other currentTitle)]
        pub unsafe fn currentTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other currentTitleColor)]
        pub unsafe fn currentTitleColor(&self) -> Id<UIColor>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other currentTitleShadowColor)]
        pub unsafe fn currentTitleShadowColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other currentImage)]
        pub unsafe fn currentImage(&self) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other currentBackgroundImage)]
        pub unsafe fn currentBackgroundImage(&self) -> Option<Id<UIImage>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method_id(@__retain_semantics Other currentPreferredSymbolConfiguration)]
        pub unsafe fn currentPreferredSymbolConfiguration(
            &self,
        ) -> Option<Id<UIImageSymbolConfiguration>>;

        #[method_id(@__retain_semantics Other currentAttributedTitle)]
        pub unsafe fn currentAttributedTitle(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "UILabel")]
        #[method_id(@__retain_semantics Other titleLabel)]
        pub unsafe fn titleLabel(&self) -> Option<Id<UILabel>>;

        #[cfg(feature = "UIImageView")]
        #[method_id(@__retain_semantics Other imageView)]
        pub unsafe fn imageView(&self) -> Option<Id<UIImageView>>;

        #[cfg(feature = "UILabel")]
        #[method_id(@__retain_semantics Other subtitleLabel)]
        pub unsafe fn subtitleLabel(&self) -> Option<Id<UILabel>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIButton {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIButton {
        #[cfg(feature = "UIFont")]
        #[deprecated = "Specify an attributed title with a custom font"]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Id<UIFont>;

        #[cfg(feature = "UIFont")]
        #[deprecated = "Specify an attributed title with a custom font"]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: &UIFont);

        #[cfg(feature = "NSParagraphStyle")]
        #[deprecated = "Specify an attributed title with a customized paragraph style"]
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        #[deprecated = "Specify an attributed title with a customized paragraph style"]
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[deprecated = "Specify an attributed title with a customized shadow style"]
        #[method(titleShadowOffset)]
        pub unsafe fn titleShadowOffset(&self) -> CGSize;

        #[deprecated = "Specify an attributed title with a customized shadow style"]
        #[method(setTitleShadowOffset:)]
        pub unsafe fn setTitleShadowOffset(&self, title_shadow_offset: CGSize);

        #[cfg(feature = "UIGeometry")]
        #[deprecated = "This property is ignored when using UIButtonConfiguration"]
        #[method(contentEdgeInsets)]
        pub unsafe fn contentEdgeInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[deprecated = "This property is ignored when using UIButtonConfiguration"]
        #[method(setContentEdgeInsets:)]
        pub unsafe fn setContentEdgeInsets(&self, content_edge_insets: UIEdgeInsets);

        #[cfg(feature = "UIGeometry")]
        #[deprecated = "This property is ignored when using UIButtonConfiguration"]
        #[method(titleEdgeInsets)]
        pub unsafe fn titleEdgeInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[deprecated = "This property is ignored when using UIButtonConfiguration"]
        #[method(setTitleEdgeInsets:)]
        pub unsafe fn setTitleEdgeInsets(&self, title_edge_insets: UIEdgeInsets);

        #[cfg(feature = "UIGeometry")]
        #[deprecated = "This property is ignored when using UIButtonConfiguration"]
        #[method(imageEdgeInsets)]
        pub unsafe fn imageEdgeInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[deprecated = "This property is ignored when using UIButtonConfiguration"]
        #[method(setImageEdgeInsets:)]
        pub unsafe fn setImageEdgeInsets(&self, image_edge_insets: UIEdgeInsets);

        #[deprecated = "This property is ignored when using UIButtonConfiguration, you may customize to replicate this behavior via a configurationUpdateHandler"]
        #[method(reversesTitleShadowWhenHighlighted)]
        pub unsafe fn reversesTitleShadowWhenHighlighted(&self) -> bool;

        #[deprecated = "This property is ignored when using UIButtonConfiguration, you may customize to replicate this behavior via a configurationUpdateHandler"]
        #[method(setReversesTitleShadowWhenHighlighted:)]
        pub unsafe fn setReversesTitleShadowWhenHighlighted(
            &self,
            reverses_title_shadow_when_highlighted: bool,
        );

        #[deprecated = "This property is ignored when using UIButtonConfiguration, you may customize to replicate this behavior via a configurationUpdateHandler"]
        #[method(adjustsImageWhenHighlighted)]
        pub unsafe fn adjustsImageWhenHighlighted(&self) -> bool;

        #[deprecated = "This property is ignored when using UIButtonConfiguration, you may customize to replicate this behavior via a configurationUpdateHandler"]
        #[method(setAdjustsImageWhenHighlighted:)]
        pub unsafe fn setAdjustsImageWhenHighlighted(&self, adjusts_image_when_highlighted: bool);

        #[deprecated = "This property is ignored when using UIButtonConfiguration, you may customize to replicate this behavior via a configurationUpdateHandler"]
        #[method(adjustsImageWhenDisabled)]
        pub unsafe fn adjustsImageWhenDisabled(&self) -> bool;

        #[deprecated = "This property is ignored when using UIButtonConfiguration, you may customize to replicate this behavior via a configurationUpdateHandler"]
        #[method(setAdjustsImageWhenDisabled:)]
        pub unsafe fn setAdjustsImageWhenDisabled(&self, adjusts_image_when_disabled: bool);

        #[deprecated = "This property is ignored when using UIButtonConfiguration"]
        #[method(showsTouchWhenHighlighted)]
        pub unsafe fn showsTouchWhenHighlighted(&self) -> bool;

        #[deprecated = "This property is ignored when using UIButtonConfiguration"]
        #[method(setShowsTouchWhenHighlighted:)]
        pub unsafe fn setShowsTouchWhenHighlighted(&self, shows_touch_when_highlighted: bool);

        #[deprecated = "Override layoutSubviews, call super, and position views as you desire."]
        #[method(backgroundRectForBounds:)]
        pub unsafe fn backgroundRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[deprecated = "Override layoutSubviews, call super, and position views as you desire."]
        #[method(contentRectForBounds:)]
        pub unsafe fn contentRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[deprecated = "Override layoutSubviews, call super, and position views as you desire."]
        #[method(titleRectForContentRect:)]
        pub unsafe fn titleRectForContentRect(&self, content_rect: CGRect) -> CGRect;

        #[deprecated = "Override layoutSubviews, call super, and position views as you desire."]
        #[method(imageRectForContentRect:)]
        pub unsafe fn imageRectForContentRect(&self, content_rect: CGRect) -> CGRect;
    }
);

extern_methods!(
    /// SpringLoading
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIButton {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UISpringLoadedInteractionSupporting",
    feature = "UIView"
))]
unsafe impl UISpringLoadedInteractionSupporting for UIButton {}
