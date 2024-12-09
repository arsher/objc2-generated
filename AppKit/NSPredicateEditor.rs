//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspredicateeditor?language=objc)
    #[unsafe(super(NSRuleEditor, NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSRuleEditor",
        feature = "NSView"
    ))]
    pub struct NSPredicateEditor;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSRuleEditor",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSPredicateEditor {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSRuleEditor",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSPredicateEditor {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSRuleEditor",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSPredicateEditor {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSRuleEditor",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSPredicateEditor {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSRuleEditor",
    feature = "NSView"
))]
unsafe impl NSCoding for NSPredicateEditor {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSRuleEditor",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSPredicateEditor {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSRuleEditor",
    feature = "NSView"
))]
unsafe impl NSObjectProtocol for NSPredicateEditor {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSRuleEditor",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSPredicateEditor {}

extern_methods!(
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSRuleEditor",
        feature = "NSView"
    ))]
    unsafe impl NSPredicateEditor {
        #[cfg(feature = "NSPredicateEditorRowTemplate")]
        #[method_id(@__retain_semantics Other rowTemplates)]
        pub unsafe fn rowTemplates(&self) -> Retained<NSArray<NSPredicateEditorRowTemplate>>;

        #[cfg(feature = "NSPredicateEditorRowTemplate")]
        #[method(setRowTemplates:)]
        pub unsafe fn setRowTemplates(&self, row_templates: &NSArray<NSPredicateEditorRowTemplate>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSRuleEditor",
        feature = "NSView"
    ))]
    unsafe impl NSPredicateEditor {
        #[cfg(feature = "objc2-core-foundation")]
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
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSRuleEditor",
        feature = "NSView"
    ))]
    unsafe impl NSPredicateEditor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSRuleEditor",
        feature = "NSView"
    ))]
    unsafe impl NSPredicateEditor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
