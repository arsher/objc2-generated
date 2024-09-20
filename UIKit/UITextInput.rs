//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    #[cfg(feature = "UITextInputTraits")]
    pub unsafe trait UIKeyInput: UITextInputTraits + MainThreadOnly {
        #[method(hasText)]
        unsafe fn hasText(&self) -> bool;

        #[method(insertText:)]
        unsafe fn insertText(&self, text: &NSString);

        #[method(deleteBackward)]
        unsafe fn deleteBackward(&self);
    }

    #[cfg(feature = "UITextInputTraits")]
    unsafe impl ProtocolType for dyn UIKeyInput {}
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextStorageDirection(pub NSInteger);
impl UITextStorageDirection {
    #[doc(alias = "UITextStorageDirectionForward")]
    pub const Forward: Self = Self(0);
    #[doc(alias = "UITextStorageDirectionBackward")]
    pub const Backward: Self = Self(1);
}

unsafe impl Encode for UITextStorageDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextStorageDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextLayoutDirection(pub NSInteger);
impl UITextLayoutDirection {
    #[doc(alias = "UITextLayoutDirectionRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "UITextLayoutDirectionLeft")]
    pub const Left: Self = Self(3);
    #[doc(alias = "UITextLayoutDirectionUp")]
    pub const Up: Self = Self(4);
    #[doc(alias = "UITextLayoutDirectionDown")]
    pub const Down: Self = Self(5);
}

unsafe impl Encode for UITextLayoutDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextLayoutDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type UITextDirection = NSInteger;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextGranularity(pub NSInteger);
impl UITextGranularity {
    #[doc(alias = "UITextGranularityCharacter")]
    pub const Character: Self = Self(0);
    #[doc(alias = "UITextGranularityWord")]
    pub const Word: Self = Self(1);
    #[doc(alias = "UITextGranularitySentence")]
    pub const Sentence: Self = Self(2);
    #[doc(alias = "UITextGranularityParagraph")]
    pub const Paragraph: Self = Self(3);
    #[doc(alias = "UITextGranularityLine")]
    pub const Line: Self = Self(4);
    #[doc(alias = "UITextGranularityDocument")]
    pub const Document: Self = Self(5);
}

unsafe impl Encode for UITextGranularity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextGranularity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDictationPhrase;

    unsafe impl ClassType for UIDictationPhrase {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIDictationPhrase {}

extern_methods!(
    unsafe impl UIDictationPhrase {
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other alternativeInterpretations)]
        pub unsafe fn alternativeInterpretations(&self) -> Option<Retained<NSArray<NSString>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDictationPhrase {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextInputAssistantItem;

    unsafe impl ClassType for UITextInputAssistantItem {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextInputAssistantItem {}

extern_methods!(
    unsafe impl UITextInputAssistantItem {
        #[method(allowsHidingShortcuts)]
        pub unsafe fn allowsHidingShortcuts(&self) -> bool;

        #[method(setAllowsHidingShortcuts:)]
        pub unsafe fn setAllowsHidingShortcuts(&self, allows_hiding_shortcuts: bool);

        #[cfg(feature = "UIBarButtonItemGroup")]
        #[method_id(@__retain_semantics Other leadingBarButtonGroups)]
        pub unsafe fn leadingBarButtonGroups(&self) -> Retained<NSArray<UIBarButtonItemGroup>>;

        #[cfg(feature = "UIBarButtonItemGroup")]
        #[method(setLeadingBarButtonGroups:)]
        pub unsafe fn setLeadingBarButtonGroups(
            &self,
            leading_bar_button_groups: &NSArray<UIBarButtonItemGroup>,
        );

        #[cfg(feature = "UIBarButtonItemGroup")]
        #[method_id(@__retain_semantics Other trailingBarButtonGroups)]
        pub unsafe fn trailingBarButtonGroups(&self) -> Retained<NSArray<UIBarButtonItemGroup>>;

        #[cfg(feature = "UIBarButtonItemGroup")]
        #[method(setTrailingBarButtonGroups:)]
        pub unsafe fn setTrailingBarButtonGroups(
            &self,
            trailing_bar_button_groups: &NSArray<UIBarButtonItemGroup>,
        );

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other keyboardActionButtonItem)]
        pub unsafe fn keyboardActionButtonItem(&self) -> Option<Retained<UIBarButtonItem>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(setKeyboardActionButtonItem:)]
        pub unsafe fn setKeyboardActionButtonItem(
            &self,
            keyboard_action_button_item: Option<&UIBarButtonItem>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextInputAssistantItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextPlaceholder;

    unsafe impl ClassType for UITextPlaceholder {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextPlaceholder {}

extern_methods!(
    unsafe impl UITextPlaceholder {
        #[method_id(@__retain_semantics Other rects)]
        pub unsafe fn rects(&self) -> Retained<NSArray<UITextSelectionRect>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextPlaceholder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextAlternativeStyle(pub NSInteger);
impl UITextAlternativeStyle {
    #[doc(alias = "UITextAlternativeStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UITextAlternativeStyleLowConfidence")]
    pub const LowConfidence: Self = Self(1);
}

unsafe impl Encode for UITextAlternativeStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextAlternativeStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    #[cfg(feature = "UITextInputTraits")]
    pub unsafe trait UITextInput: UIKeyInput + MainThreadOnly {
        #[method_id(@__retain_semantics Other textInRange:)]
        unsafe fn textInRange(&self, range: &UITextRange) -> Option<Retained<NSString>>;

        #[method(replaceRange:withText:)]
        unsafe fn replaceRange_withText(&self, range: &UITextRange, text: &NSString);

        #[method_id(@__retain_semantics Other selectedTextRange)]
        unsafe fn selectedTextRange(&self) -> Option<Retained<UITextRange>>;

        #[method(setSelectedTextRange:)]
        unsafe fn setSelectedTextRange(&self, selected_text_range: Option<&UITextRange>);

        #[method_id(@__retain_semantics Other markedTextRange)]
        unsafe fn markedTextRange(&self) -> Option<Retained<UITextRange>>;

        #[method_id(@__retain_semantics Other markedTextStyle)]
        unsafe fn markedTextStyle(
            &self,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;

        #[method(setMarkedTextStyle:)]
        unsafe fn setMarkedTextStyle(
            &self,
            marked_text_style: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[method(setMarkedText:selectedRange:)]
        unsafe fn setMarkedText_selectedRange(
            &self,
            marked_text: Option<&NSString>,
            selected_range: NSRange,
        );

        #[method(unmarkText)]
        unsafe fn unmarkText(&self);

        #[method_id(@__retain_semantics Other beginningOfDocument)]
        unsafe fn beginningOfDocument(&self) -> Retained<UITextPosition>;

        #[method_id(@__retain_semantics Other endOfDocument)]
        unsafe fn endOfDocument(&self) -> Retained<UITextPosition>;

        #[method_id(@__retain_semantics Other textRangeFromPosition:toPosition:)]
        unsafe fn textRangeFromPosition_toPosition(
            &self,
            from_position: &UITextPosition,
            to_position: &UITextPosition,
        ) -> Option<Retained<UITextRange>>;

        #[method_id(@__retain_semantics Other positionFromPosition:offset:)]
        unsafe fn positionFromPosition_offset(
            &self,
            position: &UITextPosition,
            offset: NSInteger,
        ) -> Option<Retained<UITextPosition>>;

        #[method_id(@__retain_semantics Other positionFromPosition:inDirection:offset:)]
        unsafe fn positionFromPosition_inDirection_offset(
            &self,
            position: &UITextPosition,
            direction: UITextLayoutDirection,
            offset: NSInteger,
        ) -> Option<Retained<UITextPosition>>;

        #[method(comparePosition:toPosition:)]
        unsafe fn comparePosition_toPosition(
            &self,
            position: &UITextPosition,
            other: &UITextPosition,
        ) -> NSComparisonResult;

        #[method(offsetFromPosition:toPosition:)]
        unsafe fn offsetFromPosition_toPosition(
            &self,
            from: &UITextPosition,
            to_position: &UITextPosition,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other inputDelegate)]
        unsafe fn inputDelegate(&self)
            -> Option<Retained<ProtocolObject<dyn UITextInputDelegate>>>;

        #[method(setInputDelegate:)]
        unsafe fn setInputDelegate(
            &self,
            input_delegate: Option<&ProtocolObject<dyn UITextInputDelegate>>,
        );

        #[method_id(@__retain_semantics Other tokenizer)]
        unsafe fn tokenizer(&self) -> Retained<ProtocolObject<dyn UITextInputTokenizer>>;

        #[method_id(@__retain_semantics Other positionWithinRange:farthestInDirection:)]
        unsafe fn positionWithinRange_farthestInDirection(
            &self,
            range: &UITextRange,
            direction: UITextLayoutDirection,
        ) -> Option<Retained<UITextPosition>>;

        #[method_id(@__retain_semantics Other characterRangeByExtendingPosition:inDirection:)]
        unsafe fn characterRangeByExtendingPosition_inDirection(
            &self,
            position: &UITextPosition,
            direction: UITextLayoutDirection,
        ) -> Option<Retained<UITextRange>>;

        #[cfg(feature = "NSText")]
        #[method(baseWritingDirectionForPosition:inDirection:)]
        unsafe fn baseWritingDirectionForPosition_inDirection(
            &self,
            position: &UITextPosition,
            direction: UITextStorageDirection,
        ) -> NSWritingDirection;

        #[cfg(feature = "NSText")]
        #[method(setBaseWritingDirection:forRange:)]
        unsafe fn setBaseWritingDirection_forRange(
            &self,
            writing_direction: NSWritingDirection,
            range: &UITextRange,
        );

        #[method(firstRectForRange:)]
        unsafe fn firstRectForRange(&self, range: &UITextRange) -> CGRect;

        #[method(caretRectForPosition:)]
        unsafe fn caretRectForPosition(&self, position: &UITextPosition) -> CGRect;

        #[method_id(@__retain_semantics Other selectionRectsForRange:)]
        unsafe fn selectionRectsForRange(
            &self,
            range: &UITextRange,
        ) -> Retained<NSArray<UITextSelectionRect>>;

        #[method_id(@__retain_semantics Other closestPositionToPoint:)]
        unsafe fn closestPositionToPoint(&self, point: CGPoint)
            -> Option<Retained<UITextPosition>>;

        #[method_id(@__retain_semantics Other closestPositionToPoint:withinRange:)]
        unsafe fn closestPositionToPoint_withinRange(
            &self,
            point: CGPoint,
            range: &UITextRange,
        ) -> Option<Retained<UITextPosition>>;

        #[method_id(@__retain_semantics Other characterRangeAtPoint:)]
        unsafe fn characterRangeAtPoint(&self, point: CGPoint) -> Option<Retained<UITextRange>>;

        #[optional]
        #[method(shouldChangeTextInRange:replacementText:)]
        unsafe fn shouldChangeTextInRange_replacementText(
            &self,
            range: &UITextRange,
            text: &NSString,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other textStylingAtPosition:inDirection:)]
        unsafe fn textStylingAtPosition_inDirection(
            &self,
            position: &UITextPosition,
            direction: UITextStorageDirection,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;

        #[optional]
        #[method_id(@__retain_semantics Other positionWithinRange:atCharacterOffset:)]
        unsafe fn positionWithinRange_atCharacterOffset(
            &self,
            range: &UITextRange,
            offset: NSInteger,
        ) -> Option<Retained<UITextPosition>>;

        #[optional]
        #[method(characterOffsetOfPosition:withinRange:)]
        unsafe fn characterOffsetOfPosition_withinRange(
            &self,
            position: &UITextPosition,
            range: &UITextRange,
        ) -> NSInteger;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method_id(@__retain_semantics Other textInputView)]
        unsafe fn textInputView(&self) -> Retained<UIView>;

        #[optional]
        #[method(selectionAffinity)]
        unsafe fn selectionAffinity(&self) -> UITextStorageDirection;

        #[optional]
        #[method(setSelectionAffinity:)]
        unsafe fn setSelectionAffinity(&self, selection_affinity: UITextStorageDirection);

        #[optional]
        #[method(insertDictationResult:)]
        unsafe fn insertDictationResult(&self, dictation_result: &NSArray<UIDictationPhrase>);

        #[optional]
        #[method(dictationRecordingDidEnd)]
        unsafe fn dictationRecordingDidEnd(&self);

        #[optional]
        #[method(dictationRecognitionFailed)]
        unsafe fn dictationRecognitionFailed(&self);

        #[optional]
        #[method_id(@__retain_semantics Other insertDictationResultPlaceholder)]
        unsafe fn insertDictationResultPlaceholder(&self) -> Retained<AnyObject>;

        #[optional]
        #[method(frameForDictationResultPlaceholder:)]
        unsafe fn frameForDictationResultPlaceholder(&self, placeholder: &AnyObject) -> CGRect;

        #[optional]
        #[method(removeDictationResultPlaceholder:willInsertResult:)]
        unsafe fn removeDictationResultPlaceholder_willInsertResult(
            &self,
            placeholder: &AnyObject,
            will_insert_result: bool,
        );

        #[optional]
        #[method(insertText:alternatives:style:)]
        unsafe fn insertText_alternatives_style(
            &self,
            text: &NSString,
            alternatives: &NSArray<NSString>,
            style: UITextAlternativeStyle,
        );

        #[optional]
        #[method(setAttributedMarkedText:selectedRange:)]
        unsafe fn setAttributedMarkedText_selectedRange(
            &self,
            marked_text: Option<&NSAttributedString>,
            selected_range: NSRange,
        );

        #[optional]
        #[method_id(@__retain_semantics Other insertTextPlaceholderWithSize:)]
        unsafe fn insertTextPlaceholderWithSize(&self, size: CGSize)
            -> Retained<UITextPlaceholder>;

        #[optional]
        #[method(removeTextPlaceholder:)]
        unsafe fn removeTextPlaceholder(&self, text_placeholder: &UITextPlaceholder);

        #[optional]
        #[method(beginFloatingCursorAtPoint:)]
        unsafe fn beginFloatingCursorAtPoint(&self, point: CGPoint);

        #[optional]
        #[method(updateFloatingCursorAtPoint:)]
        unsafe fn updateFloatingCursorAtPoint(&self, point: CGPoint);

        #[optional]
        #[method(endFloatingCursor)]
        unsafe fn endFloatingCursor(&self);

        #[optional]
        #[method(caretTransformForPosition:)]
        unsafe fn caretTransformForPosition(&self, position: &UITextPosition) -> CGAffineTransform;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[optional]
        #[method_id(@__retain_semantics Other editMenuForTextRange:suggestedActions:)]
        unsafe fn editMenuForTextRange_suggestedActions(
            &self,
            text_range: &UITextRange,
            suggested_actions: &NSArray<UIMenuElement>,
        ) -> Option<Retained<UIMenu>>;

        #[cfg(feature = "UIEditMenuInteraction")]
        #[optional]
        #[method(willPresentEditMenuWithAnimator:)]
        unsafe fn willPresentEditMenuWithAnimator(
            &self,
            animator: &ProtocolObject<dyn UIEditMenuInteractionAnimating>,
        );

        #[cfg(feature = "UIEditMenuInteraction")]
        #[optional]
        #[method(willDismissEditMenuWithAnimator:)]
        unsafe fn willDismissEditMenuWithAnimator(
            &self,
            animator: &ProtocolObject<dyn UIEditMenuInteractionAnimating>,
        );

        #[optional]
        #[method(supportsAdaptiveImageGlyph)]
        unsafe fn supportsAdaptiveImageGlyph(&self) -> bool;

        #[optional]
        #[method(setSupportsAdaptiveImageGlyph:)]
        unsafe fn setSupportsAdaptiveImageGlyph(&self, supports_adaptive_image_glyph: bool);

        #[cfg(feature = "NSAdaptiveImageGlyph")]
        #[optional]
        #[method(insertAdaptiveImageGlyph:replacementRange:)]
        unsafe fn insertAdaptiveImageGlyph_replacementRange(
            &self,
            adaptive_image_glyph: &NSAdaptiveImageGlyph,
            replacement_range: &UITextRange,
        );

        #[optional]
        #[method(isEditable)]
        unsafe fn isEditable(&self) -> bool;

        #[optional]
        #[method(insertAttributedText:)]
        unsafe fn insertAttributedText(&self, string: &NSAttributedString);

        #[optional]
        #[method(replaceRange:withAttributedText:)]
        unsafe fn replaceRange_withAttributedText(
            &self,
            range: &UITextRange,
            attributed_text: &NSAttributedString,
        );
    }

    #[cfg(feature = "UITextInputTraits")]
    unsafe impl ProtocolType for dyn UITextInput {}
);

extern "C" {
    pub static UITextInputTextBackgroundColorKey: &'static NSString;
}

extern "C" {
    pub static UITextInputTextColorKey: &'static NSString;
}

extern "C" {
    pub static UITextInputTextFontKey: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextPosition;

    unsafe impl ClassType for UITextPosition {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextPosition {}

extern_methods!(
    unsafe impl UITextPosition {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextPosition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextRange;

    unsafe impl ClassType for UITextRange {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextRange {}

extern_methods!(
    unsafe impl UITextRange {
        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method_id(@__retain_semantics Other start)]
        pub unsafe fn start(&self) -> Retained<UITextPosition>;

        #[method_id(@__retain_semantics Other end)]
        pub unsafe fn end(&self) -> Retained<UITextPosition>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextRange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextSelectionRect;

    unsafe impl ClassType for UITextSelectionRect {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextSelectionRect {}

extern_methods!(
    unsafe impl UITextSelectionRect {
        #[method(rect)]
        pub unsafe fn rect(&self) -> CGRect;

        #[cfg(feature = "NSText")]
        #[method(writingDirection)]
        pub unsafe fn writingDirection(&self) -> NSWritingDirection;

        #[method(containsStart)]
        pub unsafe fn containsStart(&self) -> bool;

        #[method(containsEnd)]
        pub unsafe fn containsEnd(&self) -> bool;

        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;

        #[method(transform)]
        pub unsafe fn transform(&self) -> CGAffineTransform;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextSelectionRect {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UITextInputDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "UITextInputTraits")]
        #[method(selectionWillChange:)]
        unsafe fn selectionWillChange(&self, text_input: Option<&ProtocolObject<dyn UITextInput>>);

        #[cfg(feature = "UITextInputTraits")]
        #[method(selectionDidChange:)]
        unsafe fn selectionDidChange(&self, text_input: Option<&ProtocolObject<dyn UITextInput>>);

        #[cfg(feature = "UITextInputTraits")]
        #[method(textWillChange:)]
        unsafe fn textWillChange(&self, text_input: Option<&ProtocolObject<dyn UITextInput>>);

        #[cfg(feature = "UITextInputTraits")]
        #[method(textDidChange:)]
        unsafe fn textDidChange(&self, text_input: Option<&ProtocolObject<dyn UITextInput>>);
    }

    unsafe impl ProtocolType for dyn UITextInputDelegate {}
);

extern_protocol!(
    pub unsafe trait UITextInputTokenizer: NSObjectProtocol + MainThreadOnly {
        #[method_id(@__retain_semantics Other rangeEnclosingPosition:withGranularity:inDirection:)]
        unsafe fn rangeEnclosingPosition_withGranularity_inDirection(
            &self,
            position: &UITextPosition,
            granularity: UITextGranularity,
            direction: UITextDirection,
        ) -> Option<Retained<UITextRange>>;

        #[method(isPosition:atBoundary:inDirection:)]
        unsafe fn isPosition_atBoundary_inDirection(
            &self,
            position: &UITextPosition,
            granularity: UITextGranularity,
            direction: UITextDirection,
        ) -> bool;

        #[method_id(@__retain_semantics Other positionFromPosition:toBoundary:inDirection:)]
        unsafe fn positionFromPosition_toBoundary_inDirection(
            &self,
            position: &UITextPosition,
            granularity: UITextGranularity,
            direction: UITextDirection,
        ) -> Option<Retained<UITextPosition>>;

        #[method(isPosition:withinTextUnit:inDirection:)]
        unsafe fn isPosition_withinTextUnit_inDirection(
            &self,
            position: &UITextPosition,
            granularity: UITextGranularity,
            direction: UITextDirection,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UITextInputTokenizer {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextInputStringTokenizer;

    unsafe impl ClassType for UITextInputStringTokenizer {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextInputStringTokenizer {}

unsafe impl UITextInputTokenizer for UITextInputStringTokenizer {}

extern_methods!(
    unsafe impl UITextInputStringTokenizer {
        #[cfg(all(feature = "UIResponder", feature = "UITextInputTraits"))]
        #[method_id(@__retain_semantics Init initWithTextInput:)]
        pub unsafe fn initWithTextInput(
            this: Allocated<Self>,
            text_input: &UIResponder,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextInputStringTokenizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextInputMode;

    unsafe impl ClassType for UITextInputMode {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCoding for UITextInputMode {}

unsafe impl NSObjectProtocol for UITextInputMode {}

unsafe impl NSSecureCoding for UITextInputMode {}

extern_methods!(
    unsafe impl UITextInputMode {
        #[method_id(@__retain_semantics Other primaryLanguage)]
        pub unsafe fn primaryLanguage(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other currentInputMode)]
        pub unsafe fn currentInputMode(mtm: MainThreadMarker) -> Option<Retained<UITextInputMode>>;

        #[method_id(@__retain_semantics Other activeInputModes)]
        pub unsafe fn activeInputModes(mtm: MainThreadMarker)
            -> Retained<NSArray<UITextInputMode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextInputMode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    pub static UITextInputCurrentInputModeDidChangeNotification: &'static NSNotificationName;
}
