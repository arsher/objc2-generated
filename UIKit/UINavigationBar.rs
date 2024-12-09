//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uinavigationbarnstoolbarsection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UINavigationBarNSToolbarSection(pub NSInteger);
impl UINavigationBarNSToolbarSection {
    #[doc(alias = "UINavigationBarNSToolbarSectionNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UINavigationBarNSToolbarSectionSidebar")]
    pub const Sidebar: Self = Self(1);
    #[doc(alias = "UINavigationBarNSToolbarSectionSupplementary")]
    pub const Supplementary: Self = Self(2);
    #[doc(alias = "UINavigationBarNSToolbarSectionContent")]
    pub const Content: Self = Self(3);
}

unsafe impl Encode for UINavigationBarNSToolbarSection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UINavigationBarNSToolbarSection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uinavigationbar?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UINavigationBar;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UINavigationBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UINavigationBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UINavigationBar {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UINavigationBar {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UINavigationBar {}

#[cfg(all(feature = "UIBarCommon", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIBarPositioning for UINavigationBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UINavigationBar {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UINavigationBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UINavigationBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UINavigationBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UINavigationBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UINavigationBar {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UINavigationBar {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UINavigationBar {
        #[cfg(feature = "UIInterface")]
        #[method(barStyle)]
        pub unsafe fn barStyle(&self) -> UIBarStyle;

        #[cfg(feature = "UIInterface")]
        #[method(setBarStyle:)]
        pub unsafe fn setBarStyle(&self, bar_style: UIBarStyle);

        #[cfg(feature = "UIBarCommon")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UINavigationBarDelegate>>>;

        #[cfg(feature = "UIBarCommon")]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UINavigationBarDelegate>>,
        );

        #[method(isTranslucent)]
        pub unsafe fn isTranslucent(&self) -> bool;

        #[method(setTranslucent:)]
        pub unsafe fn setTranslucent(&self, translucent: bool);

        #[cfg(feature = "UINavigationItem")]
        #[method(pushNavigationItem:animated:)]
        pub unsafe fn pushNavigationItem_animated(&self, item: &UINavigationItem, animated: bool);

        #[cfg(feature = "UINavigationItem")]
        #[method_id(@__retain_semantics Other popNavigationItemAnimated:)]
        pub unsafe fn popNavigationItemAnimated(
            &self,
            animated: bool,
        ) -> Option<Retained<UINavigationItem>>;

        #[cfg(feature = "UINavigationItem")]
        #[method_id(@__retain_semantics Other topItem)]
        pub unsafe fn topItem(&self) -> Option<Retained<UINavigationItem>>;

        #[cfg(feature = "UINavigationItem")]
        #[method_id(@__retain_semantics Other backItem)]
        pub unsafe fn backItem(&self) -> Option<Retained<UINavigationItem>>;

        #[cfg(feature = "UINavigationItem")]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Option<Retained<NSArray<UINavigationItem>>>;

        #[cfg(feature = "UINavigationItem")]
        #[method(setItems:)]
        pub unsafe fn setItems(&self, items: Option<&NSArray<UINavigationItem>>);

        #[cfg(feature = "UINavigationItem")]
        #[method(setItems:animated:)]
        pub unsafe fn setItems_animated(
            &self,
            items: Option<&NSArray<UINavigationItem>>,
            animated: bool,
        );

        #[method(prefersLargeTitles)]
        pub unsafe fn prefersLargeTitles(&self) -> bool;

        #[method(setPrefersLargeTitles:)]
        pub unsafe fn setPrefersLargeTitles(&self, prefers_large_titles: bool);

        #[method(currentNSToolbarSection)]
        pub unsafe fn currentNSToolbarSection(&self) -> UINavigationBarNSToolbarSection;

        #[cfg(feature = "UIBehavioralStyle")]
        #[method(behavioralStyle)]
        pub unsafe fn behavioralStyle(&self) -> UIBehavioralStyle;

        #[cfg(feature = "UIBehavioralStyle")]
        #[method(preferredBehavioralStyle)]
        pub unsafe fn preferredBehavioralStyle(&self) -> UIBehavioralStyle;

        #[cfg(feature = "UIBehavioralStyle")]
        #[method(setPreferredBehavioralStyle:)]
        pub unsafe fn setPreferredBehavioralStyle(
            &self,
            preferred_behavioral_style: UIBehavioralStyle,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other barTintColor)]
        pub unsafe fn barTintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBarTintColor:)]
        pub unsafe fn setBarTintColor(&self, bar_tint_color: Option<&UIColor>);

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method(setBackgroundImage:forBarPosition:barMetrics:)]
        pub unsafe fn setBackgroundImage_forBarPosition_barMetrics(
            &self,
            background_image: Option<&UIImage>,
            bar_position: UIBarPosition,
            bar_metrics: UIBarMetrics,
        );

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other backgroundImageForBarPosition:barMetrics:)]
        pub unsafe fn backgroundImageForBarPosition_barMetrics(
            &self,
            bar_position: UIBarPosition,
            bar_metrics: UIBarMetrics,
        ) -> Option<Retained<UIImage>>;

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method(setBackgroundImage:forBarMetrics:)]
        pub unsafe fn setBackgroundImage_forBarMetrics(
            &self,
            background_image: Option<&UIImage>,
            bar_metrics: UIBarMetrics,
        );

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other backgroundImageForBarMetrics:)]
        pub unsafe fn backgroundImageForBarMetrics(
            &self,
            bar_metrics: UIBarMetrics,
        ) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other shadowImage)]
        pub unsafe fn shadowImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setShadowImage:)]
        pub unsafe fn setShadowImage(&self, shadow_image: Option<&UIImage>);

        #[method_id(@__retain_semantics Other titleTextAttributes)]
        pub unsafe fn titleTextAttributes(
            &self,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;

        #[method(setTitleTextAttributes:)]
        pub unsafe fn setTitleTextAttributes(
            &self,
            title_text_attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[method_id(@__retain_semantics Other largeTitleTextAttributes)]
        pub unsafe fn largeTitleTextAttributes(
            &self,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;

        #[method(setLargeTitleTextAttributes:)]
        pub unsafe fn setLargeTitleTextAttributes(
            &self,
            large_title_text_attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[cfg(all(feature = "UIBarCommon", feature = "objc2-core-foundation"))]
        #[method(setTitleVerticalPositionAdjustment:forBarMetrics:)]
        pub unsafe fn setTitleVerticalPositionAdjustment_forBarMetrics(
            &self,
            adjustment: CGFloat,
            bar_metrics: UIBarMetrics,
        );

        #[cfg(all(feature = "UIBarCommon", feature = "objc2-core-foundation"))]
        #[method(titleVerticalPositionAdjustmentForBarMetrics:)]
        pub unsafe fn titleVerticalPositionAdjustmentForBarMetrics(
            &self,
            bar_metrics: UIBarMetrics,
        ) -> CGFloat;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other backIndicatorImage)]
        pub unsafe fn backIndicatorImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setBackIndicatorImage:)]
        pub unsafe fn setBackIndicatorImage(&self, back_indicator_image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other backIndicatorTransitionMaskImage)]
        pub unsafe fn backIndicatorTransitionMaskImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setBackIndicatorTransitionMaskImage:)]
        pub unsafe fn setBackIndicatorTransitionMaskImage(
            &self,
            back_indicator_transition_mask_image: Option<&UIImage>,
        );

        #[cfg(all(feature = "UIBarAppearance", feature = "UINavigationBarAppearance"))]
        #[method_id(@__retain_semantics Other standardAppearance)]
        pub unsafe fn standardAppearance(&self) -> Retained<UINavigationBarAppearance>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UINavigationBarAppearance"))]
        #[method(setStandardAppearance:)]
        pub unsafe fn setStandardAppearance(&self, standard_appearance: &UINavigationBarAppearance);

        #[cfg(all(feature = "UIBarAppearance", feature = "UINavigationBarAppearance"))]
        #[method_id(@__retain_semantics Other compactAppearance)]
        pub unsafe fn compactAppearance(&self) -> Option<Retained<UINavigationBarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UINavigationBarAppearance"))]
        #[method(setCompactAppearance:)]
        pub unsafe fn setCompactAppearance(
            &self,
            compact_appearance: Option<&UINavigationBarAppearance>,
        );

        #[cfg(all(feature = "UIBarAppearance", feature = "UINavigationBarAppearance"))]
        #[method_id(@__retain_semantics Other scrollEdgeAppearance)]
        pub unsafe fn scrollEdgeAppearance(&self) -> Option<Retained<UINavigationBarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UINavigationBarAppearance"))]
        #[method(setScrollEdgeAppearance:)]
        pub unsafe fn setScrollEdgeAppearance(
            &self,
            scroll_edge_appearance: Option<&UINavigationBarAppearance>,
        );

        #[cfg(all(feature = "UIBarAppearance", feature = "UINavigationBarAppearance"))]
        #[method_id(@__retain_semantics Other compactScrollEdgeAppearance)]
        pub unsafe fn compactScrollEdgeAppearance(
            &self,
        ) -> Option<Retained<UINavigationBarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UINavigationBarAppearance"))]
        #[method(setCompactScrollEdgeAppearance:)]
        pub unsafe fn setCompactScrollEdgeAppearance(
            &self,
            compact_scroll_edge_appearance: Option<&UINavigationBarAppearance>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UINavigationBar {
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
    unsafe impl UINavigationBar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uinavigationbardelegate?language=objc)
    #[cfg(feature = "UIBarCommon")]
    pub unsafe trait UINavigationBarDelegate:
        UIBarPositioningDelegate + MainThreadOnly
    {
        #[cfg(all(
            feature = "UINavigationItem",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(navigationBar:shouldPushItem:)]
        unsafe fn navigationBar_shouldPushItem(
            &self,
            navigation_bar: &UINavigationBar,
            item: &UINavigationItem,
        ) -> bool;

        #[cfg(all(
            feature = "UINavigationItem",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(navigationBar:didPushItem:)]
        unsafe fn navigationBar_didPushItem(
            &self,
            navigation_bar: &UINavigationBar,
            item: &UINavigationItem,
        );

        #[cfg(all(
            feature = "UINavigationItem",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(navigationBar:shouldPopItem:)]
        unsafe fn navigationBar_shouldPopItem(
            &self,
            navigation_bar: &UINavigationBar,
            item: &UINavigationItem,
        ) -> bool;

        #[cfg(all(
            feature = "UINavigationItem",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(navigationBar:didPopItem:)]
        unsafe fn navigationBar_didPopItem(
            &self,
            navigation_bar: &UINavigationBar,
            item: &UINavigationItem,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(navigationBarNSToolbarSection:)]
        unsafe fn navigationBarNSToolbarSection(
            &self,
            navigation_bar: &UINavigationBar,
        ) -> UINavigationBarNSToolbarSection;
    }

    #[cfg(feature = "UIBarCommon")]
    unsafe impl ProtocolType for dyn UINavigationBarDelegate {}
);
