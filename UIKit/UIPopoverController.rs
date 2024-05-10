//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
    pub struct UIPopoverController;

    unsafe impl ClassType for UIPopoverController {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIPopoverController {}

#[cfg(feature = "UIAppearance")]
unsafe impl UIAppearanceContainer for UIPopoverController {}

extern_methods!(
    unsafe impl UIPopoverController {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method_id(@__retain_semantics Init initWithContentViewController:)]
        pub unsafe fn initWithContentViewController(
            this: Allocated<Self>,
            view_controller: &UIViewController,
        ) -> Id<Self>;

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UIPopoverControllerDelegate>>>;

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPopoverControllerDelegate>>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method_id(@__retain_semantics Other contentViewController)]
        pub unsafe fn contentViewController(&self) -> Id<UIViewController>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(&self, content_view_controller: &UIViewController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setContentViewController:animated:)]
        pub unsafe fn setContentViewController_animated(
            &self,
            view_controller: &UIViewController,
            animated: bool,
        );

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(popoverContentSize)]
        pub unsafe fn popoverContentSize(&self) -> CGSize;

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setPopoverContentSize:)]
        pub unsafe fn setPopoverContentSize(&self, popover_content_size: CGSize);

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setPopoverContentSize:animated:)]
        pub unsafe fn setPopoverContentSize_animated(&self, size: CGSize, animated: bool);

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(isPopoverVisible)]
        pub unsafe fn isPopoverVisible(&self) -> bool;

        #[cfg(feature = "UIPopoverSupport")]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(popoverArrowDirection)]
        pub unsafe fn popoverArrowDirection(&self) -> UIPopoverArrowDirection;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method_id(@__retain_semantics Other passthroughViews)]
        pub unsafe fn passthroughViews(&self) -> Option<Id<NSArray<UIView>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setPassthroughViews:)]
        pub unsafe fn setPassthroughViews(&self, passthrough_views: Option<&NSArray<UIView>>);

        #[cfg(all(
            feature = "UIPopoverSupport",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(presentPopoverFromRect:inView:permittedArrowDirections:animated:)]
        pub unsafe fn presentPopoverFromRect_inView_permittedArrowDirections_animated(
            &self,
            rect: CGRect,
            view: &UIView,
            arrow_directions: UIPopoverArrowDirection,
            animated: bool,
        );

        #[cfg(all(
            feature = "UIBarButtonItem",
            feature = "UIBarItem",
            feature = "UIPopoverSupport"
        ))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(presentPopoverFromBarButtonItem:permittedArrowDirections:animated:)]
        pub unsafe fn presentPopoverFromBarButtonItem_permittedArrowDirections_animated(
            &self,
            item: &UIBarButtonItem,
            arrow_directions: UIPopoverArrowDirection,
            animated: bool,
        );

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(dismissPopoverAnimated:)]
        pub unsafe fn dismissPopoverAnimated(&self, animated: bool);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&UIColor>);

        #[cfg(feature = "UIGeometry")]
        #[method(popoverLayoutMargins)]
        pub unsafe fn popoverLayoutMargins(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setPopoverLayoutMargins:)]
        pub unsafe fn setPopoverLayoutMargins(&self, popover_layout_margins: UIEdgeInsets);

        #[method(popoverBackgroundViewClass)]
        pub unsafe fn popoverBackgroundViewClass(&self) -> Option<&'static AnyClass>;

        #[method(setPopoverBackgroundViewClass:)]
        pub unsafe fn setPopoverBackgroundViewClass(
            &self,
            popover_background_view_class: Option<&AnyClass>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPopoverController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UIPopoverControllerDelegate:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[deprecated]
        #[optional]
        #[method(popoverControllerShouldDismissPopover:)]
        unsafe fn popoverControllerShouldDismissPopover(
            &self,
            popover_controller: &UIPopoverController,
        ) -> bool;

        #[deprecated]
        #[optional]
        #[method(popoverControllerDidDismissPopover:)]
        unsafe fn popoverControllerDidDismissPopover(
            &self,
            popover_controller: &UIPopoverController,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated]
        #[optional]
        #[method(popoverController:willRepositionPopoverToRect:inView:)]
        unsafe fn popoverController_willRepositionPopoverToRect_inView(
            &self,
            popover_controller: &UIPopoverController,
            rect: NonNull<CGRect>,
            view: &mut Id<UIView>,
        );
    }

    unsafe impl ProtocolType for dyn UIPopoverControllerDelegate {}
);