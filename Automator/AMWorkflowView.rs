//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Automator_AMWorkflowView")]
    pub struct AMWorkflowView;

    #[cfg(feature = "Automator_AMWorkflowView")]
    unsafe impl ClassType for AMWorkflowView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Automator_AMWorkflowView")]
unsafe impl NSAccessibility for AMWorkflowView {}

#[cfg(feature = "Automator_AMWorkflowView")]
unsafe impl NSAccessibilityElementProtocol for AMWorkflowView {}

#[cfg(feature = "Automator_AMWorkflowView")]
unsafe impl NSAnimatablePropertyContainer for AMWorkflowView {}

#[cfg(feature = "Automator_AMWorkflowView")]
unsafe impl NSAppearanceCustomization for AMWorkflowView {}

#[cfg(feature = "Automator_AMWorkflowView")]
unsafe impl NSCoding for AMWorkflowView {}

#[cfg(feature = "Automator_AMWorkflowView")]
unsafe impl NSDraggingDestination for AMWorkflowView {}

#[cfg(feature = "Automator_AMWorkflowView")]
unsafe impl NSObjectProtocol for AMWorkflowView {}

#[cfg(feature = "Automator_AMWorkflowView")]
unsafe impl NSUserInterfaceItemIdentification for AMWorkflowView {}

extern_methods!(
    #[cfg(feature = "Automator_AMWorkflowView")]
    unsafe impl AMWorkflowView {
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[cfg(feature = "Automator_AMWorkflowController")]
        #[method_id(@__retain_semantics Other workflowController)]
        pub unsafe fn workflowController(&self) -> Option<Id<AMWorkflowController>>;

        #[cfg(feature = "Automator_AMWorkflowController")]
        #[method(setWorkflowController:)]
        pub unsafe fn setWorkflowController(
            &self,
            workflow_controller: Option<&AMWorkflowController>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "Automator_AMWorkflowView")]
    unsafe impl AMWorkflowView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "Automator_AMWorkflowView")]
    unsafe impl AMWorkflowView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Automator_AMWorkflowView")]
    unsafe impl AMWorkflowView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
