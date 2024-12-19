//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewcontrollerautomaticdimension?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSSplitViewControllerAutomaticDimension: CGFloat;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    pub struct NSSplitViewController;
);

#[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
unsafe impl NSCoding for NSSplitViewController {}

#[cfg(all(
    feature = "NSKeyValueBinding",
    feature = "NSResponder",
    feature = "NSViewController"
))]
unsafe impl NSEditor for NSSplitViewController {}

#[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
unsafe impl NSObjectProtocol for NSSplitViewController {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSStoryboardSegue",
    feature = "NSViewController"
))]
unsafe impl NSSeguePerforming for NSSplitViewController {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSSplitView",
    feature = "NSViewController"
))]
unsafe impl NSSplitViewDelegate for NSSplitViewController {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSplitViewController {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceValidation",
    feature = "NSViewController"
))]
unsafe impl NSUserInterfaceValidations for NSSplitViewController {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSSplitViewController {
        #[cfg(all(feature = "NSSplitView", feature = "NSView"))]
        #[method_id(@__retain_semantics Other splitView)]
        pub unsafe fn splitView(&self) -> Retained<NSSplitView>;

        #[cfg(all(feature = "NSSplitView", feature = "NSView"))]
        #[method(setSplitView:)]
        pub unsafe fn setSplitView(&self, split_view: &NSSplitView);

        #[cfg(feature = "NSSplitViewItem")]
        #[method_id(@__retain_semantics Other splitViewItems)]
        pub unsafe fn splitViewItems(&self) -> Retained<NSArray<NSSplitViewItem>>;

        #[cfg(feature = "NSSplitViewItem")]
        #[method(setSplitViewItems:)]
        pub unsafe fn setSplitViewItems(&self, split_view_items: &NSArray<NSSplitViewItem>);

        #[cfg(feature = "NSSplitViewItem")]
        #[method(addSplitViewItem:)]
        pub unsafe fn addSplitViewItem(&self, split_view_item: &NSSplitViewItem);

        #[cfg(feature = "NSSplitViewItem")]
        #[method(insertSplitViewItem:atIndex:)]
        pub unsafe fn insertSplitViewItem_atIndex(
            &self,
            split_view_item: &NSSplitViewItem,
            index: NSInteger,
        );

        #[cfg(feature = "NSSplitViewItem")]
        #[method(removeSplitViewItem:)]
        pub unsafe fn removeSplitViewItem(&self, split_view_item: &NSSplitViewItem);

        #[cfg(feature = "NSSplitViewItem")]
        #[method_id(@__retain_semantics Other splitViewItemForViewController:)]
        pub unsafe fn splitViewItemForViewController(
            &self,
            view_controller: &NSViewController,
        ) -> Option<Retained<NSSplitViewItem>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumThicknessForInlineSidebars)]
        pub unsafe fn minimumThicknessForInlineSidebars(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMinimumThicknessForInlineSidebars:)]
        pub unsafe fn setMinimumThicknessForInlineSidebars(
            &self,
            minimum_thickness_for_inline_sidebars: CGFloat,
        );

        #[cfg(feature = "NSUserInterfaceValidation")]
        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(
            &self,
            item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>,
        ) -> bool;

        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);

        #[cfg(all(feature = "NSSplitView", feature = "NSView"))]
        #[method(splitView:canCollapseSubview:)]
        pub unsafe fn splitView_canCollapseSubview(
            &self,
            split_view: &NSSplitView,
            subview: &NSView,
        ) -> bool;

        #[cfg(all(feature = "NSSplitView", feature = "NSView"))]
        #[deprecated = "NSSplitView no longer supports collapsing sections via double-click. This delegate method is never called, and NSSplitViewController's implementation always returns NO."]
        #[method(splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:)]
        pub unsafe fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            subview: &NSView,
            divider_index: NSInteger,
        ) -> bool;

        #[cfg(all(feature = "NSSplitView", feature = "NSView"))]
        #[method(splitView:shouldHideDividerAtIndex:)]
        pub unsafe fn splitView_shouldHideDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> bool;

        #[cfg(all(
            feature = "NSSplitView",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[method(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
        pub unsafe fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            proposed_effective_rect: NSRect,
            drawn_rect: NSRect,
            divider_index: NSInteger,
        ) -> NSRect;

        #[cfg(all(
            feature = "NSSplitView",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[method(splitView:additionalEffectiveRectOfDividerAtIndex:)]
        pub unsafe fn splitView_additionalEffectiveRectOfDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSSplitViewController {
        #[cfg(feature = "NSNib")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSSplitViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSSplitViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSSplitViewControllerToggleSidebarAction
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSSplitViewController {
        #[method(toggleSidebar:)]
        pub unsafe fn toggleSidebar(&self, sender: Option<&AnyObject>);

        #[method(toggleInspector:)]
        pub unsafe fn toggleInspector(&self, sender: Option<&AnyObject>);
    }
);
