//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITabBarItemStateAppearance;

    unsafe impl ClassType for UITabBarItemStateAppearance {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITabBarItemStateAppearance {}

extern_methods!(
    unsafe impl UITabBarItemStateAppearance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;

        #[method_id(@__retain_semantics Other titleTextAttributes)]
        pub unsafe fn titleTextAttributes(
            &self,
        ) -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method(setTitleTextAttributes:)]
        pub unsafe fn setTitleTextAttributes(
            &self,
            title_text_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        );

        #[cfg(feature = "UIGeometry")]
        #[method(titlePositionAdjustment)]
        pub unsafe fn titlePositionAdjustment(&self) -> UIOffset;

        #[cfg(feature = "UIGeometry")]
        #[method(setTitlePositionAdjustment:)]
        pub unsafe fn setTitlePositionAdjustment(&self, title_position_adjustment: UIOffset);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other iconColor)]
        pub unsafe fn iconColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setIconColor:)]
        pub unsafe fn setIconColor(&self, icon_color: Option<&UIColor>);

        #[cfg(feature = "UIGeometry")]
        #[method(badgePositionAdjustment)]
        pub unsafe fn badgePositionAdjustment(&self) -> UIOffset;

        #[cfg(feature = "UIGeometry")]
        #[method(setBadgePositionAdjustment:)]
        pub unsafe fn setBadgePositionAdjustment(&self, badge_position_adjustment: UIOffset);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other badgeBackgroundColor)]
        pub unsafe fn badgeBackgroundColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBadgeBackgroundColor:)]
        pub unsafe fn setBadgeBackgroundColor(&self, badge_background_color: Option<&UIColor>);

        #[method_id(@__retain_semantics Other badgeTextAttributes)]
        pub unsafe fn badgeTextAttributes(
            &self,
        ) -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method(setBadgeTextAttributes:)]
        pub unsafe fn setBadgeTextAttributes(
            &self,
            badge_text_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        );

        #[cfg(feature = "UIGeometry")]
        #[method(badgeTitlePositionAdjustment)]
        pub unsafe fn badgeTitlePositionAdjustment(&self) -> UIOffset;

        #[cfg(feature = "UIGeometry")]
        #[method(setBadgeTitlePositionAdjustment:)]
        pub unsafe fn setBadgeTitlePositionAdjustment(
            &self,
            badge_title_position_adjustment: UIOffset,
        );
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITabBarItemAppearanceStyle(pub NSInteger);
impl UITabBarItemAppearanceStyle {
    #[doc(alias = "UITabBarItemAppearanceStyleStacked")]
    pub const Stacked: Self = Self(0);
    #[doc(alias = "UITabBarItemAppearanceStyleInline")]
    pub const Inline: Self = Self(1);
    #[doc(alias = "UITabBarItemAppearanceStyleCompactInline")]
    pub const CompactInline: Self = Self(2);
}

unsafe impl Encode for UITabBarItemAppearanceStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITabBarItemAppearanceStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITabBarItemAppearance;

    unsafe impl ClassType for UITabBarItemAppearance {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UITabBarItemAppearance {}

unsafe impl NSCopying for UITabBarItemAppearance {}

unsafe impl NSObjectProtocol for UITabBarItemAppearance {}

unsafe impl NSSecureCoding for UITabBarItemAppearance {}

extern_methods!(
    unsafe impl UITabBarItemAppearance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithStyle:)]
        pub unsafe fn initWithStyle(
            this: Allocated<Self>,
            style: UITabBarItemAppearanceStyle,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[method_id(@__retain_semantics CopyOrMutCopy copy)]
        pub unsafe fn copy(&self) -> Id<Self>;

        #[method(configureWithDefaultForStyle:)]
        pub unsafe fn configureWithDefaultForStyle(&self, style: UITabBarItemAppearanceStyle);

        #[method_id(@__retain_semantics Other normal)]
        pub unsafe fn normal(&self) -> Id<UITabBarItemStateAppearance>;

        #[method_id(@__retain_semantics Other selected)]
        pub unsafe fn selected(&self) -> Id<UITabBarItemStateAppearance>;

        #[method_id(@__retain_semantics Other disabled)]
        pub unsafe fn disabled(&self) -> Id<UITabBarItemStateAppearance>;

        #[method_id(@__retain_semantics Other focused)]
        pub unsafe fn focused(&self) -> Id<UITabBarItemStateAppearance>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITabBarItemAppearance {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIBarAppearance")]
    pub struct UITabBarAppearance;

    #[cfg(feature = "UIBarAppearance")]
    unsafe impl ClassType for UITabBarAppearance {
        #[inherits(NSObject)]
        type Super = UIBarAppearance;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSCoding for UITabBarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSCopying for UITabBarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSObjectProtocol for UITabBarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSSecureCoding for UITabBarAppearance {}

extern_methods!(
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UITabBarAppearance {
        #[method_id(@__retain_semantics Other stackedLayoutAppearance)]
        pub unsafe fn stackedLayoutAppearance(&self) -> Id<UITabBarItemAppearance>;

        #[method(setStackedLayoutAppearance:)]
        pub unsafe fn setStackedLayoutAppearance(
            &self,
            stacked_layout_appearance: &UITabBarItemAppearance,
        );

        #[method_id(@__retain_semantics Other inlineLayoutAppearance)]
        pub unsafe fn inlineLayoutAppearance(&self) -> Id<UITabBarItemAppearance>;

        #[method(setInlineLayoutAppearance:)]
        pub unsafe fn setInlineLayoutAppearance(
            &self,
            inline_layout_appearance: &UITabBarItemAppearance,
        );

        #[method_id(@__retain_semantics Other compactInlineLayoutAppearance)]
        pub unsafe fn compactInlineLayoutAppearance(&self) -> Id<UITabBarItemAppearance>;

        #[method(setCompactInlineLayoutAppearance:)]
        pub unsafe fn setCompactInlineLayoutAppearance(
            &self,
            compact_inline_layout_appearance: &UITabBarItemAppearance,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other selectionIndicatorTintColor)]
        pub unsafe fn selectionIndicatorTintColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setSelectionIndicatorTintColor:)]
        pub unsafe fn setSelectionIndicatorTintColor(
            &self,
            selection_indicator_tint_color: Option<&UIColor>,
        );

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other selectionIndicatorImage)]
        pub unsafe fn selectionIndicatorImage(&self) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setSelectionIndicatorImage:)]
        pub unsafe fn setSelectionIndicatorImage(
            &self,
            selection_indicator_image: Option<&UIImage>,
        );

        #[cfg(feature = "UITabBar")]
        #[method(stackedItemPositioning)]
        pub unsafe fn stackedItemPositioning(&self) -> UITabBarItemPositioning;

        #[cfg(feature = "UITabBar")]
        #[method(setStackedItemPositioning:)]
        pub unsafe fn setStackedItemPositioning(
            &self,
            stacked_item_positioning: UITabBarItemPositioning,
        );

        #[method(stackedItemWidth)]
        pub unsafe fn stackedItemWidth(&self) -> CGFloat;

        #[method(setStackedItemWidth:)]
        pub unsafe fn setStackedItemWidth(&self, stacked_item_width: CGFloat);

        #[method(stackedItemSpacing)]
        pub unsafe fn stackedItemSpacing(&self) -> CGFloat;

        #[method(setStackedItemSpacing:)]
        pub unsafe fn setStackedItemSpacing(&self, stacked_item_spacing: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIBarAppearance`
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UITabBarAppearance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "UIDevice")]
        #[method_id(@__retain_semantics Init initWithIdiom:)]
        pub unsafe fn initWithIdiom(this: Allocated<Self>, idiom: UIUserInterfaceIdiom)
            -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBarAppearance:)]
        pub unsafe fn initWithBarAppearance(
            this: Allocated<Self>,
            bar_appearance: &UIBarAppearance,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UITabBarAppearance {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
