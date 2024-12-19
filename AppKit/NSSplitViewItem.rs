//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewitembehavior?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSplitViewItemBehavior(pub NSInteger);
impl NSSplitViewItemBehavior {
    #[doc(alias = "NSSplitViewItemBehaviorDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSSplitViewItemBehaviorSidebar")]
    pub const Sidebar: Self = Self(1);
    #[doc(alias = "NSSplitViewItemBehaviorContentList")]
    pub const ContentList: Self = Self(2);
    #[doc(alias = "NSSplitViewItemBehaviorInspector")]
    pub const Inspector: Self = Self(3);
}

unsafe impl Encode for NSSplitViewItemBehavior {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSSplitViewItemBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewitemcollapsebehavior?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSplitViewItemCollapseBehavior(pub NSInteger);
impl NSSplitViewItemCollapseBehavior {
    #[doc(alias = "NSSplitViewItemCollapseBehaviorDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSSplitViewItemCollapseBehaviorPreferResizingSplitViewWithFixedSiblings")]
    pub const PreferResizingSplitViewWithFixedSiblings: Self = Self(1);
    #[doc(alias = "NSSplitViewItemCollapseBehaviorPreferResizingSiblingsWithFixedSplitView")]
    pub const PreferResizingSiblingsWithFixedSplitView: Self = Self(2);
    #[doc(alias = "NSSplitViewItemCollapseBehaviorUseConstraints")]
    pub const UseConstraints: Self = Self(3);
}

unsafe impl Encode for NSSplitViewItemCollapseBehavior {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSSplitViewItemCollapseBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewitemunspecifieddimension?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSSplitViewItemUnspecifiedDimension: CGFloat;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSplitViewItem;
);

#[cfg(feature = "NSAnimation")]
unsafe impl NSAnimatablePropertyContainer for NSSplitViewItem {}

unsafe impl NSCoding for NSSplitViewItem {}

unsafe impl NSObjectProtocol for NSSplitViewItem {}

extern_methods!(
    unsafe impl NSSplitViewItem {
        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other splitViewItemWithViewController:)]
        pub unsafe fn splitViewItemWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other sidebarWithViewController:)]
        pub unsafe fn sidebarWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other contentListWithViewController:)]
        pub unsafe fn contentListWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other inspectorWithViewController:)]
        pub unsafe fn inspectorWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSSplitViewItemBehavior;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self, mtm: MainThreadMarker) -> Retained<NSViewController>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, view_controller: &NSViewController);

        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;

        #[method(setCollapsed:)]
        pub unsafe fn setCollapsed(&self, collapsed: bool);

        #[method(canCollapse)]
        pub unsafe fn canCollapse(&self) -> bool;

        #[method(setCanCollapse:)]
        pub unsafe fn setCanCollapse(&self, can_collapse: bool);

        #[method(collapseBehavior)]
        pub unsafe fn collapseBehavior(&self) -> NSSplitViewItemCollapseBehavior;

        #[method(setCollapseBehavior:)]
        pub unsafe fn setCollapseBehavior(
            &self,
            collapse_behavior: NSSplitViewItemCollapseBehavior,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumThickness)]
        pub unsafe fn minimumThickness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMinimumThickness:)]
        pub unsafe fn setMinimumThickness(&self, minimum_thickness: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(maximumThickness)]
        pub unsafe fn maximumThickness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMaximumThickness:)]
        pub unsafe fn setMaximumThickness(&self, maximum_thickness: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredThicknessFraction)]
        pub unsafe fn preferredThicknessFraction(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setPreferredThicknessFraction:)]
        pub unsafe fn setPreferredThicknessFraction(&self, preferred_thickness_fraction: CGFloat);

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(holdingPriority)]
        pub unsafe fn holdingPriority(&self) -> NSLayoutPriority;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(setHoldingPriority:)]
        pub unsafe fn setHoldingPriority(&self, holding_priority: NSLayoutPriority);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(automaticMaximumThickness)]
        pub unsafe fn automaticMaximumThickness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAutomaticMaximumThickness:)]
        pub unsafe fn setAutomaticMaximumThickness(&self, automatic_maximum_thickness: CGFloat);

        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;

        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, spring_loaded: bool);

        #[method(canCollapseFromWindowResize)]
        pub unsafe fn canCollapseFromWindowResize(&self) -> bool;

        #[method(setCanCollapseFromWindowResize:)]
        pub unsafe fn setCanCollapseFromWindowResize(&self, can_collapse_from_window_resize: bool);

        #[method(allowsFullHeightLayout)]
        pub unsafe fn allowsFullHeightLayout(&self) -> bool;

        #[method(setAllowsFullHeightLayout:)]
        pub unsafe fn setAllowsFullHeightLayout(&self, allows_full_height_layout: bool);

        #[cfg(feature = "NSWindow")]
        #[method(titlebarSeparatorStyle)]
        pub unsafe fn titlebarSeparatorStyle(&self) -> NSTitlebarSeparatorStyle;

        #[cfg(feature = "NSWindow")]
        #[method(setTitlebarSeparatorStyle:)]
        pub unsafe fn setTitlebarSeparatorStyle(
            &self,
            titlebar_separator_style: NSTitlebarSeparatorStyle,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSplitViewItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
