//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSViewControllerTransitionOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSViewControllerTransitionOptions: NSUInteger {
        const NSViewControllerTransitionNone = 0x0;
        const NSViewControllerTransitionCrossfade = 0x1;
        const NSViewControllerTransitionSlideUp = 0x10;
        const NSViewControllerTransitionSlideDown = 0x20;
        const NSViewControllerTransitionSlideLeft = 0x40;
        const NSViewControllerTransitionSlideRight = 0x80;
        const NSViewControllerTransitionSlideForward = 0x140;
        const NSViewControllerTransitionSlideBackward = 0x180;
        const NSViewControllerTransitionAllowUserInteraction = 0x1000;
    }
}

unsafe impl Encode for NSViewControllerTransitionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSViewControllerTransitionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSResponder")]
    pub struct NSViewController;

    #[cfg(feature = "NSResponder")]
    unsafe impl ClassType for NSViewController {
        #[inherits(NSObject)]
        type Super = NSResponder;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "NSResponder")]
unsafe impl NSCoding for NSViewController {}

#[cfg(all(feature = "NSKeyValueBinding", feature = "NSResponder"))]
unsafe impl NSEditor for NSViewController {}

#[cfg(feature = "NSResponder")]
unsafe impl NSObjectProtocol for NSViewController {}

#[cfg(all(feature = "NSResponder", feature = "NSStoryboardSegue"))]
unsafe impl NSSeguePerforming for NSViewController {}

#[cfg(all(feature = "NSResponder", feature = "NSUserInterfaceItemIdentification"))]
unsafe impl NSUserInterfaceItemIdentification for NSViewController {}

extern_methods!(
    #[cfg(feature = "NSResponder")]
    unsafe impl NSViewController {
        #[cfg(feature = "NSNib")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(feature = "NSNib")]
        #[method_id(@__retain_semantics Other nibName)]
        pub unsafe fn nibName(&self) -> Option<Id<NSNibName>>;

        #[method_id(@__retain_semantics Other nibBundle)]
        pub unsafe fn nibBundle(&self) -> Option<Id<NSBundle>>;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<AnyObject>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Id<NSView>;

        #[cfg(feature = "NSView")]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: &NSView);

        #[cfg(feature = "NSView")]
        #[method_id(@__retain_semantics Other viewIfLoaded)]
        pub unsafe fn viewIfLoaded(&self) -> Option<Id<NSView>>;

        #[method(loadView)]
        pub unsafe fn loadView(&self);

        #[method(loadViewIfNeeded)]
        pub unsafe fn loadViewIfNeeded(&self);

        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        pub unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&AnyObject>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(commitEditing)]
        pub unsafe fn commitEditing(&self) -> bool;

        #[method(discardEditing)]
        pub unsafe fn discardEditing(&self);

        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);

        #[method(isViewLoaded)]
        pub unsafe fn isViewLoaded(&self) -> bool;

        #[method(viewWillAppear)]
        pub unsafe fn viewWillAppear(&self);

        #[method(viewDidAppear)]
        pub unsafe fn viewDidAppear(&self);

        #[method(viewWillDisappear)]
        pub unsafe fn viewWillDisappear(&self);

        #[method(viewDidDisappear)]
        pub unsafe fn viewDidDisappear(&self);

        #[method(preferredContentSize)]
        pub unsafe fn preferredContentSize(&self) -> NSSize;

        #[method(setPreferredContentSize:)]
        pub unsafe fn setPreferredContentSize(&self, preferred_content_size: NSSize);

        #[method(updateViewConstraints)]
        pub unsafe fn updateViewConstraints(&self);

        #[method(viewWillLayout)]
        pub unsafe fn viewWillLayout(&self);

        #[method(viewDidLayout)]
        pub unsafe fn viewDidLayout(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "NSResponder")]
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSResponder")]
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSViewControllerPresentation
    #[cfg(feature = "NSResponder")]
    unsafe impl NSViewController {
        #[method(presentViewController:animator:)]
        pub unsafe fn presentViewController_animator(
            &self,
            view_controller: &NSViewController,
            animator: &ProtocolObject<dyn NSViewControllerPresentationAnimator>,
        );

        #[method(dismissViewController:)]
        pub unsafe fn dismissViewController(&self, view_controller: &NSViewController);

        #[method(dismissController:)]
        pub unsafe fn dismissController(&self, sender: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other presentedViewControllers)]
        pub unsafe fn presentedViewControllers(&self) -> Option<Id<NSArray<NSViewController>>>;

        #[method_id(@__retain_semantics Other presentingViewController)]
        pub unsafe fn presentingViewController(&self) -> Option<Id<NSViewController>>;
    }
);

extern_methods!(
    /// NSViewControllerPresentationAndTransitionStyles
    #[cfg(feature = "NSResponder")]
    unsafe impl NSViewController {
        #[method(presentViewControllerAsSheet:)]
        pub unsafe fn presentViewControllerAsSheet(&self, view_controller: &NSViewController);

        #[method(presentViewControllerAsModalWindow:)]
        pub unsafe fn presentViewControllerAsModalWindow(&self, view_controller: &NSViewController);

        #[cfg(all(feature = "NSPopover", feature = "NSView"))]
        #[method(presentViewController:asPopoverRelativeToRect:ofView:preferredEdge:behavior:)]
        pub unsafe fn presentViewController_asPopoverRelativeToRect_ofView_preferredEdge_behavior(
            &self,
            view_controller: &NSViewController,
            positioning_rect: NSRect,
            positioning_view: &NSView,
            preferred_edge: NSRectEdge,
            behavior: NSPopoverBehavior,
        );

        #[cfg(all(feature = "NSPopover", feature = "NSView"))]
        #[method(presentViewController:asPopoverRelativeToRect:ofView:preferredEdge:behavior:hasFullSizeContent:)]
        pub unsafe fn presentViewController_asPopoverRelativeToRect_ofView_preferredEdge_behavior_hasFullSizeContent(
            &self,
            view_controller: &NSViewController,
            positioning_rect: NSRect,
            positioning_view: &NSView,
            preferred_edge: NSRectEdge,
            behavior: NSPopoverBehavior,
            has_full_size_content: bool,
        );

        #[cfg(feature = "block2")]
        #[method(transitionFromViewController:toViewController:options:completionHandler:)]
        pub unsafe fn transitionFromViewController_toViewController_options_completionHandler(
            &self,
            from_view_controller: &NSViewController,
            to_view_controller: &NSViewController,
            options: NSViewControllerTransitionOptions,
            completion: Option<&block2::Block<dyn Fn()>>,
        );
    }
);

extern_methods!(
    /// NSViewControllerContainer
    #[cfg(feature = "NSResponder")]
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Other parentViewController)]
        pub unsafe fn parentViewController(&self) -> Option<Id<NSViewController>>;

        #[method_id(@__retain_semantics Other childViewControllers)]
        pub unsafe fn childViewControllers(&self) -> Id<NSArray<NSViewController>>;

        #[method(setChildViewControllers:)]
        pub unsafe fn setChildViewControllers(
            &self,
            child_view_controllers: &NSArray<NSViewController>,
        );

        #[method(addChildViewController:)]
        pub unsafe fn addChildViewController(&self, child_view_controller: &NSViewController);

        #[method(removeFromParentViewController)]
        pub unsafe fn removeFromParentViewController(&self);

        #[method(insertChildViewController:atIndex:)]
        pub unsafe fn insertChildViewController_atIndex(
            &self,
            child_view_controller: &NSViewController,
            index: NSInteger,
        );

        #[method(removeChildViewControllerAtIndex:)]
        pub unsafe fn removeChildViewControllerAtIndex(&self, index: NSInteger);

        #[method(preferredContentSizeDidChangeForViewController:)]
        pub unsafe fn preferredContentSizeDidChangeForViewController(
            &self,
            view_controller: &NSViewController,
        );

        #[method(viewWillTransitionToSize:)]
        pub unsafe fn viewWillTransitionToSize(&self, new_size: NSSize);
    }
);

extern_protocol!(
    pub unsafe trait NSViewControllerPresentationAnimator:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(feature = "NSResponder")]
        #[method(animatePresentationOfViewController:fromViewController:)]
        unsafe fn animatePresentationOfViewController_fromViewController(
            &self,
            view_controller: &NSViewController,
            from_view_controller: &NSViewController,
        );

        #[cfg(feature = "NSResponder")]
        #[method(animateDismissalOfViewController:fromViewController:)]
        unsafe fn animateDismissalOfViewController_fromViewController(
            &self,
            view_controller: &NSViewController,
            from_view_controller: &NSViewController,
        );
    }

    unsafe impl ProtocolType for dyn NSViewControllerPresentationAnimator {}
);

extern_methods!(
    /// NSViewControllerStoryboardingMethods
    #[cfg(feature = "NSResponder")]
    unsafe impl NSViewController {
        #[cfg(feature = "NSStoryboard")]
        #[method_id(@__retain_semantics Other storyboard)]
        pub unsafe fn storyboard(&self) -> Option<Id<NSStoryboard>>;
    }
);

extern_methods!(
    /// NSExtensionAdditions
    #[cfg(feature = "NSResponder")]
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(&self) -> Option<Id<NSExtensionContext>>;

        #[cfg(feature = "NSView")]
        #[method_id(@__retain_semantics Other sourceItemView)]
        pub unsafe fn sourceItemView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "NSView")]
        #[method(setSourceItemView:)]
        pub unsafe fn setSourceItemView(&self, source_item_view: Option<&NSView>);

        #[method(preferredScreenOrigin)]
        pub unsafe fn preferredScreenOrigin(&self) -> NSPoint;

        #[method(setPreferredScreenOrigin:)]
        pub unsafe fn setPreferredScreenOrigin(&self, preferred_screen_origin: NSPoint);

        #[method(preferredMinimumSize)]
        pub unsafe fn preferredMinimumSize(&self) -> NSSize;

        #[method(preferredMaximumSize)]
        pub unsafe fn preferredMaximumSize(&self) -> NSSize;
    }
);

#[cfg(feature = "NSResponder")]
unsafe impl NSExtensionRequestHandling for NSViewController {}
