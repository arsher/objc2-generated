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

extern_methods!(
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholderString: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholderAttributedString: Option<&NSAttributedString>,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, textColor: Option<&NSColor>);

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
        pub unsafe fn delegate(&self) -> Option<Id<NSTextFieldDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextFieldDelegate>);

        #[cfg(feature = "AppKit_NSText")]
        #[method(textShouldBeginEditing:)]
        pub unsafe fn textShouldBeginEditing(&self, textObject: &NSText) -> bool;

        #[cfg(feature = "AppKit_NSText")]
        #[method(textShouldEndEditing:)]
        pub unsafe fn textShouldEndEditing(&self, textObject: &NSText) -> bool;

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
        pub unsafe fn setBezelStyle(&self, bezelStyle: NSTextFieldBezelStyle);

        #[method(preferredMaxLayoutWidth)]
        pub unsafe fn preferredMaxLayoutWidth(&self) -> CGFloat;

        #[method(setPreferredMaxLayoutWidth:)]
        pub unsafe fn setPreferredMaxLayoutWidth(&self, preferredMaxLayoutWidth: CGFloat);

        #[method(maximumNumberOfLines)]
        pub unsafe fn maximumNumberOfLines(&self) -> NSInteger;

        #[method(setMaximumNumberOfLines:)]
        pub unsafe fn setMaximumNumberOfLines(&self, maximumNumberOfLines: NSInteger);

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allowsDefaultTighteningForTruncation: bool,
        );

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;

        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, lineBreakStrategy: NSLineBreakStrategy);
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
            automaticTextCompletionEnabled: bool,
        );

        #[method(allowsCharacterPickerTouchBarItem)]
        pub unsafe fn allowsCharacterPickerTouchBarItem(&self) -> bool;

        #[method(setAllowsCharacterPickerTouchBarItem:)]
        pub unsafe fn setAllowsCharacterPickerTouchBarItem(
            &self,
            allowsCharacterPickerTouchBarItem: bool,
        );
    }
);

extern_methods!(
    /// NSTextFieldConvenience
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelWithString:)]
        pub unsafe fn labelWithString(stringValue: &NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(stringValue: &NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributedStringValue: &NSAttributedString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textFieldWithString:)]
        pub unsafe fn textFieldWithString(stringValue: &NSString) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSTextFieldAttributedStringMethods
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[method(allowsEditingTextAttributes)]
        pub unsafe fn allowsEditingTextAttributes(&self) -> bool;

        #[method(setAllowsEditingTextAttributes:)]
        pub unsafe fn setAllowsEditingTextAttributes(&self, allowsEditingTextAttributes: bool);

        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;

        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, importsGraphics: bool);
    }
);

extern_protocol!(
    pub struct NSTextFieldDelegate;

    unsafe impl ProtocolType for NSTextFieldDelegate {
        #[cfg(all(
            feature = "AppKit_NSTextField",
            feature = "AppKit_NSTextView",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textField:textView:candidatesForSelectedRange:)]
        pub unsafe fn textField_textView_candidatesForSelectedRange(
            &self,
            textField: &NSTextField,
            textView: &NSTextView,
            selectedRange: NSRange,
        ) -> Option<Id<NSArray, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSTextField",
            feature = "AppKit_NSTextView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textField:textView:candidates:forSelectedRange:)]
        pub unsafe fn textField_textView_candidates_forSelectedRange(
            &self,
            textField: &NSTextField,
            textView: &NSTextView,
            candidates: &NSArray<NSTextCheckingResult>,
            selectedRange: NSRange,
        ) -> Id<NSArray<NSTextCheckingResult>, Shared>;

        #[cfg(all(feature = "AppKit_NSTextField", feature = "AppKit_NSTextView"))]
        #[optional]
        #[method(textField:textView:shouldSelectCandidateAtIndex:)]
        pub unsafe fn textField_textView_shouldSelectCandidateAtIndex(
            &self,
            textField: &NSTextField,
            textView: &NSTextView,
            index: NSUInteger,
        ) -> bool;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSTextField")]
    unsafe impl NSTextField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
