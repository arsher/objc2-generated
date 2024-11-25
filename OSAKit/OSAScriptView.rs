//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/osakit/osascriptview?language=objc)
    #[unsafe(super(NSTextView, NSText, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSAScriptView;
);

unsafe impl NSAccessibility for OSAScriptView {}

unsafe impl NSAccessibilityElementProtocol for OSAScriptView {}

unsafe impl NSAccessibilityNavigableStaticText for OSAScriptView {}

unsafe impl NSAccessibilityStaticText for OSAScriptView {}

unsafe impl NSAnimatablePropertyContainer for OSAScriptView {}

unsafe impl NSAppearanceCustomization for OSAScriptView {}

unsafe impl NSChangeSpelling for OSAScriptView {}

unsafe impl NSCoding for OSAScriptView {}

unsafe impl NSColorChanging for OSAScriptView {}

unsafe impl NSDraggingDestination for OSAScriptView {}

unsafe impl NSDraggingSource for OSAScriptView {}

unsafe impl NSIgnoreMisspelledWords for OSAScriptView {}

unsafe impl NSMenuItemValidation for OSAScriptView {}

unsafe impl NSObjectProtocol for OSAScriptView {}

unsafe impl NSStandardKeyBindingResponding for OSAScriptView {}

unsafe impl NSTextContent for OSAScriptView {}

unsafe impl NSTextInput for OSAScriptView {}

unsafe impl NSTextInputClient for OSAScriptView {}

unsafe impl NSTextLayoutOrientationProvider for OSAScriptView {}

unsafe impl NSUserInterfaceItemIdentification for OSAScriptView {}

unsafe impl NSUserInterfaceValidations for OSAScriptView {}

extern_methods!(
    unsafe impl OSAScriptView {
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Option<Retained<NSString>>;

        #[method(setSource:)]
        pub unsafe fn setSource(&self, source: Option<&NSString>);

        #[method(usesScriptAssistant)]
        pub unsafe fn usesScriptAssistant(&self) -> bool;

        #[method(setUsesScriptAssistant:)]
        pub unsafe fn setUsesScriptAssistant(&self, uses_script_assistant: bool);

        #[method(usesTabs)]
        pub unsafe fn usesTabs(&self) -> bool;

        #[method(setUsesTabs:)]
        pub unsafe fn setUsesTabs(&self, uses_tabs: bool);

        #[method(tabWidth)]
        pub unsafe fn tabWidth(&self) -> NSUInteger;

        #[method(setTabWidth:)]
        pub unsafe fn setTabWidth(&self, tab_width: NSUInteger);

        #[method(wrapsLines)]
        pub unsafe fn wrapsLines(&self) -> bool;

        #[method(setWrapsLines:)]
        pub unsafe fn setWrapsLines(&self, wraps_lines: bool);

        #[method(indentsWrappedLines)]
        pub unsafe fn indentsWrappedLines(&self) -> bool;

        #[method(setIndentsWrappedLines:)]
        pub unsafe fn setIndentsWrappedLines(&self, indents_wrapped_lines: bool);

        #[method(indentWidth)]
        pub unsafe fn indentWidth(&self) -> NSUInteger;

        #[method(setIndentWidth:)]
        pub unsafe fn setIndentWidth(&self, indent_width: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextView`
    unsafe impl OSAScriptView {
        #[method_id(@__retain_semantics Init initWithFrame:textContainer:)]
        pub unsafe fn initWithFrame_textContainer(
            this: Allocated<Self>,
            frame_rect: NSRect,
            container: Option<&NSTextContainer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initUsingTextLayoutManager:)]
        pub unsafe fn initUsingTextLayoutManager(
            this: Allocated<Self>,
            using_text_layout_manager: bool,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other textViewUsingTextLayoutManager:)]
        pub unsafe fn textViewUsingTextLayoutManager(
            using_text_layout_manager: bool,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    unsafe impl OSAScriptView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl OSAScriptView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
