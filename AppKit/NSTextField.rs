//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextField")]
    pub struct NSTextField;

    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl ClassType for NSTextField {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSAccessibility for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSAccessibilityElement for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSAccessibilityNavigableStaticText for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSAccessibilityStaticText for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSAnimatablePropertyContainer for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSAppearanceCustomization for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSCoding for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSDraggingDestination for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSObjectProtocol for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSTextContent for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSUserInterfaceItemIdentification for NSTextField {}

#[cfg(feature = "AppKit_NSTextField")]
unsafe impl NSUserInterfaceValidations for NSTextField {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholder_string: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholder_attributed_string: Option<&NSAttributedString>,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: Option<&NSColor>);

        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;

        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;

        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);

        #[method(selectText:)]
        pub unsafe fn selectText(&self, sender: Option<&Object>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextFieldDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextFieldDelegate>>,
        );

        #[cfg(feature = "AppKit_NSText")]
        #[method(textShouldBeginEditing:)]
        pub unsafe fn textShouldBeginEditing(&self, text_object: &NSText) -> bool;

        #[cfg(feature = "AppKit_NSText")]
        #[method(textShouldEndEditing:)]
        pub unsafe fn textShouldEndEditing(&self, text_object: &NSText) -> bool;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(textDidBeginEditing:)]
        pub unsafe fn textDidBeginEditing(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(textDidEndEditing:)]
        pub unsafe fn textDidEndEditing(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(textDidChange:)]
        pub unsafe fn textDidChange(&self, notification: &NSNotification);

        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSTextFieldBezelStyle;

        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezel_style: NSTextFieldBezelStyle);

        #[method(preferredMaxLayoutWidth)]
        pub unsafe fn preferredMaxLayoutWidth(&self) -> CGFloat;

        #[method(setPreferredMaxLayoutWidth:)]
        pub unsafe fn setPreferredMaxLayoutWidth(&self, preferred_max_layout_width: CGFloat);

        #[method(maximumNumberOfLines)]
        pub unsafe fn maximumNumberOfLines(&self) -> NSInteger;

        #[method(setMaximumNumberOfLines:)]
        pub unsafe fn setMaximumNumberOfLines(&self, maximum_number_of_lines: NSInteger);

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allows_default_tightening_for_truncation: bool,
        );

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;

        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, line_break_strategy: NSLineBreakStrategy);
    }
);

extern_methods!(
    /// NSTouchBar
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[method(isAutomaticTextCompletionEnabled)]
        pub unsafe fn isAutomaticTextCompletionEnabled(&self) -> bool;

        #[method(setAutomaticTextCompletionEnabled:)]
        pub unsafe fn setAutomaticTextCompletionEnabled(
            &self,
            automatic_text_completion_enabled: bool,
        );

        #[method(allowsCharacterPickerTouchBarItem)]
        pub unsafe fn allowsCharacterPickerTouchBarItem(&self) -> bool;

        #[method(setAllowsCharacterPickerTouchBarItem:)]
        pub unsafe fn setAllowsCharacterPickerTouchBarItem(
            &self,
            allows_character_picker_touch_bar_item: bool,
        );
    }
);

extern_methods!(
    /// NSTextFieldConvenience
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelWithString:)]
        pub unsafe fn labelWithString(string_value: &NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(string_value: &NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributed_string_value: &NSAttributedString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textFieldWithString:)]
        pub unsafe fn textFieldWithString(string_value: &NSString) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSTextFieldAttributedStringMethods
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[method(allowsEditingTextAttributes)]
        pub unsafe fn allowsEditingTextAttributes(&self) -> bool;

        #[method(setAllowsEditingTextAttributes:)]
        pub unsafe fn setAllowsEditingTextAttributes(&self, allows_editing_text_attributes: bool);

        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;

        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, imports_graphics: bool);
    }
);

extern_protocol!(
    pub unsafe trait NSTextFieldDelegate: NSControlTextEditingDelegate {
        #[cfg(all(
            feature = "AppKit_NSTextField",
            feature = "AppKit_NSTextView",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textField:textView:candidatesForSelectedRange:)]
        unsafe fn textField_textView_candidatesForSelectedRange(
            &self,
            text_field: &NSTextField,
            text_view: &NSTextView,
            selected_range: NSRange,
        ) -> Option<Id<NSArray, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSTextField",
            feature = "AppKit_NSTextView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textField:textView:candidates:forSelectedRange:)]
        unsafe fn textField_textView_candidates_forSelectedRange(
            &self,
            text_field: &NSTextField,
            text_view: &NSTextView,
            candidates: &NSArray<NSTextCheckingResult>,
            selected_range: NSRange,
        ) -> Id<NSArray<NSTextCheckingResult>, Shared>;

        #[cfg(all(feature = "AppKit_NSTextField", feature = "AppKit_NSTextView"))]
        #[optional]
        #[method(textField:textView:shouldSelectCandidateAtIndex:)]
        unsafe fn textField_textView_shouldSelectCandidateAtIndex(
            &self,
            text_field: &NSTextField,
            text_view: &NSTextView,
            index: NSUInteger,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSTextFieldDelegate {}
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
