//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfield?language=objc)
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSTextField;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSTextField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSTextField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityNavigableStaticText for NSTextField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityStaticText for NSTextField {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSTextField {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSTextField {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSTextField {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextContent",
    feature = "NSView"
))]
unsafe impl NSTextContent for NSTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTextField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceValidation",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceValidations for NSTextField {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTextField {
        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Retained<NSString>>;

        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholder_string: Option<&NSString>);

        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholder_attributed_string: Option<&NSAttributedString>,
        );

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
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
        pub unsafe fn selectText(&self, sender: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSTextFieldDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextFieldDelegate>>,
        );

        #[cfg(feature = "NSText")]
        #[method(textShouldBeginEditing:)]
        pub unsafe fn textShouldBeginEditing(&self, text_object: &NSText) -> bool;

        #[cfg(feature = "NSText")]
        #[method(textShouldEndEditing:)]
        pub unsafe fn textShouldEndEditing(&self, text_object: &NSText) -> bool;

        #[method(textDidBeginEditing:)]
        pub unsafe fn textDidBeginEditing(&self, notification: &NSNotification);

        #[method(textDidEndEditing:)]
        pub unsafe fn textDidEndEditing(&self, notification: &NSNotification);

        #[method(textDidChange:)]
        pub unsafe fn textDidChange(&self, notification: &NSNotification);

        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[cfg(feature = "NSTextFieldCell")]
        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSTextFieldBezelStyle;

        #[cfg(feature = "NSTextFieldCell")]
        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezel_style: NSTextFieldBezelStyle);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredMaxLayoutWidth)]
        pub unsafe fn preferredMaxLayoutWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
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

        #[cfg(feature = "NSParagraphStyle")]
        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;

        #[cfg(feature = "NSParagraphStyle")]
        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, line_break_strategy: NSLineBreakStrategy);

        #[method(allowsWritingTools)]
        pub unsafe fn allowsWritingTools(&self) -> bool;

        #[method(setAllowsWritingTools:)]
        pub unsafe fn setAllowsWritingTools(&self, allows_writing_tools: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTextField {
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
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTextField {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTextField {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSTouchBar
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
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
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTextField {
        #[method_id(@__retain_semantics Other labelWithString:)]
        pub unsafe fn labelWithString(
            string_value: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(
            string_value: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributed_string_value: &NSAttributedString,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other textFieldWithString:)]
        pub unsafe fn textFieldWithString(
            string_value: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSTextFieldAttributedStringMethods
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
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
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfielddelegate?language=objc)
    #[cfg(feature = "NSControl")]
    pub unsafe trait NSTextFieldDelegate:
        NSControlTextEditingDelegate + MainThreadOnly
    {
        #[cfg(all(
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSTextView",
            feature = "NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textField:textView:candidatesForSelectedRange:)]
        unsafe fn textField_textView_candidatesForSelectedRange(
            &self,
            text_field: &NSTextField,
            text_view: &NSTextView,
            selected_range: NSRange,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSTextView",
            feature = "NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textField:textView:candidates:forSelectedRange:)]
        unsafe fn textField_textView_candidates_forSelectedRange(
            &self,
            text_field: &NSTextField,
            text_view: &NSTextView,
            candidates: &NSArray<NSTextCheckingResult>,
            selected_range: NSRange,
        ) -> Retained<NSArray<NSTextCheckingResult>>;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSTextView",
            feature = "NSView"
        ))]
        #[optional]
        #[method(textField:textView:shouldSelectCandidateAtIndex:)]
        unsafe fn textField_textView_shouldSelectCandidateAtIndex(
            &self,
            text_field: &NSTextField,
            text_view: &NSTextView,
            index: NSUInteger,
        ) -> bool;
    }

    #[cfg(feature = "NSControl")]
    unsafe impl ProtocolType for dyn NSTextFieldDelegate {}
);

extern_methods!(
    /// NSDeprecated
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTextField {
        #[deprecated = "Use `-setTitle:` instead"]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);
    }
);
