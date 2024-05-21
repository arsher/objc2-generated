//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    pub struct AMWorkflowView;

    #[cfg(feature = "objc2-app-kit")]
    unsafe impl ClassType for AMWorkflowView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAccessibility for AMWorkflowView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAccessibilityElementProtocol for AMWorkflowView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAnimatablePropertyContainer for AMWorkflowView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAppearanceCustomization for AMWorkflowView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSCoding for AMWorkflowView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSDraggingDestination for AMWorkflowView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSObjectProtocol for AMWorkflowView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSUserInterfaceItemIdentification for AMWorkflowView {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AMWorkflowView {
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[cfg(feature = "AMWorkflowController")]
        #[method_id(@__retain_semantics Other workflowController)]
        pub unsafe fn workflowController(&self) -> Option<Retained<AMWorkflowController>>;

        #[cfg(feature = "AMWorkflowController")]
        #[method(setWorkflowController:)]
        pub unsafe fn setWorkflowController(
            &self,
            workflow_controller: Option<&AMWorkflowController>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AMWorkflowView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AMWorkflowView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AMWorkflowView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
