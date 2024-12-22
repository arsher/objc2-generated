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
    /// The item uses the default collapsing behavior for its set `behavior`. The default may change over time.
    #[doc(alias = "NSSplitViewItemCollapseBehaviorDefault")]
    pub const Default: Self = Self(0);
    /// The item prefers to keep the other panes at their current size and position on screen, potentially growing or shrinking the window in the direction to best preserve that. But it will break that preference in order to keep the window fully on screen or when in full screen.
    #[doc(alias = "NSSplitViewItemCollapseBehaviorPreferResizingSplitViewWithFixedSiblings")]
    pub const PreferResizingSplitViewWithFixedSiblings: Self = Self(1);
    /// The item prefers to resize the other split panes. This will be broken when uncollapsing if the item can't fully uncollapse before hitting the minimum size of the other panes or the window.
    #[doc(alias = "NSSplitViewItemCollapseBehaviorPreferResizingSiblingsWithFixedSplitView")]
    pub const PreferResizingSiblingsWithFixedSplitView: Self = Self(2);
    /// The item will collapse/uncollapse purely from a constraint animation, with a constraint priority of the item’s `holdingPriority`. This could result in a partial internal content resize and window resize, and has no implications for keeping the window on screen. External constraints can be used to tweak exactly how the animation affects item, sibling, and window size and positions.
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
    /// This constant can be used with any sizing related
    /// `NSSplitViewItem`properties to unset their values.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewitemunspecifieddimension?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSSplitViewItemUnspecifiedDimension: CGFloat;
}

extern_class!(
    /// NSSplitViewItem implements the items used in an NSSplitViewController.
    /// The item describes a child ViewController's state in a SplitViewController, e.g. its collapsibility, holding priority and other metrics, and collapsed state.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewitem?language=objc)
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
        /// Creates an autoreleased SplitViewItem that represents the provided ViewController. All other properties are left at their default.
        ///
        /// Parameter `viewController`: The view controller used to set the viewController property
        ///
        /// Returns: An autoreleased SplitViewItem.
        #[method_id(@__retain_semantics Other splitViewItemWithViewController:)]
        pub unsafe fn splitViewItemWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        /// Creates a split view item representing a sidebar for the provided ViewController.
        /// Sidebars have standard system behavior, specifically:
        /// - Translucent material background
        /// - The ability to collapse/uncollapse on split view size changes
        /// - The ability to overlay at small split view sizes when in fullscreen
        /// - canCollapse is set to YES
        /// - minimumThickness and maximumThickness are set to the standard minimum and maximum sidebar size
        /// - preferredThicknessFraction is set to the standard fraction for sidebars (0.15)
        /// - springLoaded is set to YES
        ///
        /// Parameter `viewController`: The view controller used to set the viewController property
        ///
        /// Returns: An autoreleased SplitViewItem that acts as a sidebar.
        #[method_id(@__retain_semantics Other sidebarWithViewController:)]
        pub unsafe fn sidebarWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        /// Creates a split view item representing a content list for the provided ViewController, akin to Mail's message list, Note's note list.
        /// Content lists have system standard defaults, specifically:
        /// - minimumThickness and maximumThickness are set to the system standard for content lists
        /// - automaticMaximumThickness is set to the system standard for content lists
        /// - preferredThicknessFraction is set to the standard fraction for content lists (0.28 when a neighbor sidebar is visible, 0.33 if not)
        ///
        /// Parameter `viewController`: The view controller used to set the viewController property
        ///
        /// Returns: An autoreleased SplitViewItem that acts as a content list.
        #[method_id(@__retain_semantics Other contentListWithViewController:)]
        pub unsafe fn contentListWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        /// Creates a split view item representing an inspector for the provided ViewController.
        /// On macOS 14.0 and above inspectors have the following standard system behavior:
        /// - canCollapse is set to YES
        /// - minimumThickness and maximumThickness are set to the standard inspector size (270) and are not resizable by default
        ///
        /// Parameter `viewController`: The view controller used to set the viewController property
        ///
        /// Returns: An autoreleased SplitViewItem that acts as an inspector.
        #[method_id(@__retain_semantics Other inspectorWithViewController:)]
        pub unsafe fn inspectorWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        /// The standard behavior type of the receiver. See initializers for descriptions of each behavior.
        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSSplitViewItemBehavior;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        /// The view controller represented by the SplitViewItem. An exception will be thrown if a new viewController is set while the receiving SplitViewItem is added to a SplitViewController.
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self, mtm: MainThreadMarker) -> Retained<NSViewController>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        /// Setter for [`viewController`][Self::viewController].
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, view_controller: &NSViewController);

        /// Whether or not the child ViewController corresponding to the SplitViewItem is collapsed in the SplitViewController. The default is
        /// `NO.`This can be set with the animator proxy to animate the collapse or uncollapse. The exact animation used can be customized by setting it in the -animations dictionary with a key of "collapsed". If this is set to YES before it is added to the SplitViewController, it will be initially collapsed and the SplitViewController will not cause the view to be loaded until it is uncollapsed. This is KVC/KVO compliant and will be updated if the value changes from user interaction.
        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;

        /// Setter for [`isCollapsed`][Self::isCollapsed].
        #[method(setCollapsed:)]
        pub unsafe fn setCollapsed(&self, collapsed: bool);

        /// Whether or not the child view controller is collapsible from user interaction - whether by dragging or double clicking a divider. The default is
        /// `NO.`
        #[method(canCollapse)]
        pub unsafe fn canCollapse(&self) -> bool;

        /// Setter for [`canCollapse`][Self::canCollapse].
        #[method(setCanCollapse:)]
        pub unsafe fn setCanCollapse(&self, can_collapse: bool);

        /// The resize behavior when the receiver toggles its `collapsed` state programmatically, both animatedly and not. Defaults to `.Default`.
        #[method(collapseBehavior)]
        pub unsafe fn collapseBehavior(&self) -> NSSplitViewItemCollapseBehavior;

        /// Setter for [`collapseBehavior`][Self::collapseBehavior].
        #[method(setCollapseBehavior:)]
        pub unsafe fn setCollapseBehavior(
            &self,
            collapse_behavior: NSSplitViewItemCollapseBehavior,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// A convenience to set the minimum thickness of the split view item -- width for "vertical" split views, height otherwise. If NSSplitViewItemUnspecifiedDimension, no minimum size is enforced by the SplitViewItem, although constraints in the contained view hierarchy might have constraints specify some minimum size on their own. Defaults to NSSplitViewItemUnspecifiedDimension.
        #[method(minimumThickness)]
        pub unsafe fn minimumThickness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumThickness`][Self::minimumThickness].
        #[method(setMinimumThickness:)]
        pub unsafe fn setMinimumThickness(&self, minimum_thickness: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// A convenience to set the maximum thickness of the split view item -- width for "vertical" split views, height otherwise. If NSSplitViewItemUnspecifiedDimension, no maximum size is enforced by the SplitViewItem, although constraints in the contained view hierarchy might have constraints specify some maximum size on their own. Defaults to NSSplitViewItemUnspecifiedDimension.
        #[method(maximumThickness)]
        pub unsafe fn maximumThickness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maximumThickness`][Self::maximumThickness].
        #[method(setMaximumThickness:)]
        pub unsafe fn setMaximumThickness(&self, maximum_thickness: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The percentage of the contained NSSplitView that the NSSplitViewItem prefers to encompass. This is used when double-clicking on a neighbor divider to return to that standard ratio. As well as after entering fullscreen to determine the initial size of the receiver. Defaults to NSSplitViewItemUnspecifiedDimension, which means no resize will occur on double-clicks, and the absolute size is preserved when entering fullscreen.
        #[method(preferredThicknessFraction)]
        pub unsafe fn preferredThicknessFraction(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`preferredThicknessFraction`][Self::preferredThicknessFraction].
        #[method(setPreferredThicknessFraction:)]
        pub unsafe fn setPreferredThicknessFraction(&self, preferred_thickness_fraction: CGFloat);

        #[cfg(feature = "NSLayoutConstraint")]
        /// Sets the priority under which a SplitViewItem will hold its width (for a vertical split view) or height (for a horizontal split view). The view with the lowest priority will be the first to take on additional width if the split view grows or shrinks. The default is
        /// `NSLayoutPriorityDefaultLow.`
        #[method(holdingPriority)]
        pub unsafe fn holdingPriority(&self) -> NSLayoutPriority;

        #[cfg(feature = "NSLayoutConstraint")]
        /// Setter for [`holdingPriority`][Self::holdingPriority].
        #[method(setHoldingPriority:)]
        pub unsafe fn setHoldingPriority(&self, holding_priority: NSLayoutPriority);

        #[cfg(feature = "objc2-core-foundation")]
        /// The maximum thickness of the split view item when resizing due to automatic sizing, such as entering fullscreen with a set preferredThicknessFraction or proportional sizing. The user can still resize up to the absolute maximum size by dragging the divider or otherwise. If NSSplitViewItemUnspecifiedDimension, no automatic maximum is enforced. Defaults to NSSplitViewItemUnspecifiedDimension.
        #[method(automaticMaximumThickness)]
        pub unsafe fn automaticMaximumThickness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`automaticMaximumThickness`][Self::automaticMaximumThickness].
        #[method(setAutomaticMaximumThickness:)]
        pub unsafe fn setAutomaticMaximumThickness(&self, automatic_maximum_thickness: CGFloat);

        /// If YES, the split view item can be temporarily uncollapsed during a drag by hovering or deep clicking on its neighboring divider. Defaults to NO.
        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;

        /// Setter for [`isSpringLoaded`][Self::isSpringLoaded].
        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, spring_loaded: bool);

        /// If YES, the item can be collapsed from a window resize. This can differ from `canCollapse`, to allow divider collapsing but not window resize collapsing or vice versa.
        /// Defaults to YES for Sidebars and NO for Inspectors.
        /// - Note: Setting `canCollapse` for sidebars will reset this value to that new value.
        #[method(canCollapseFromWindowResize)]
        pub unsafe fn canCollapseFromWindowResize(&self) -> bool;

        /// Setter for [`canCollapseFromWindowResize`][Self::canCollapseFromWindowResize].
        #[method(setCanCollapseFromWindowResize:)]
        pub unsafe fn setCanCollapseFromWindowResize(&self, can_collapse_from_window_resize: bool);

        /// Whether or not a sidebar or inspector is allowed to be full height in the window when the `NSFullSizeContentViewWindowMask` style mask is also set. Only applies to NSSplitViewItemBehaviorSidebar and NSSplitViewItemBehaviorInspector. Defaults to YES.
        #[method(allowsFullHeightLayout)]
        pub unsafe fn allowsFullHeightLayout(&self) -> bool;

        /// Setter for [`allowsFullHeightLayout`][Self::allowsFullHeightLayout].
        #[method(setAllowsFullHeightLayout:)]
        pub unsafe fn setAllowsFullHeightLayout(&self, allows_full_height_layout: bool);

        #[cfg(feature = "NSWindow")]
        /// Specifies a preference for the style of separator displayed between the titlebar and the content of the split view item.
        ///
        /// For this value to be applicable, the item's view must be associated with its own titlebar section (see `NSTrackingSeparatorToolbarItem` for more info).
        /// The default value is NSTitlebarSeparatorStyleAutomatic. This value is subject to the containing window's preference and can be overridden.
        #[method(titlebarSeparatorStyle)]
        pub unsafe fn titlebarSeparatorStyle(&self) -> NSTitlebarSeparatorStyle;

        #[cfg(feature = "NSWindow")]
        /// Setter for [`titlebarSeparatorStyle`][Self::titlebarSeparatorStyle].
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
