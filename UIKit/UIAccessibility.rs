//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub fn UIAccessibilityConvertFrameToScreenCoordinates(rect: CGRect, view: &UIView) -> CGRect;
}

extern "C" {
    #[cfg(all(feature = "UIBezierPath", feature = "UIResponder", feature = "UIView"))]
    pub fn UIAccessibilityConvertPathToScreenCoordinates(
        path: &UIBezierPath,
        view: &UIView,
    ) -> NonNull<UIBezierPath>;
}

#[cfg(feature = "block2")]
pub type AXBoolReturnBlock = *mut block2::Block<dyn Fn() -> Bool>;

#[cfg(feature = "block2")]
pub type AXStringReturnBlock = *mut block2::Block<dyn Fn() -> *mut NSString>;

#[cfg(feature = "block2")]
pub type AXStringArrayReturnBlock = *mut block2::Block<dyn Fn() -> *mut NSArray<NSString>>;

#[cfg(feature = "block2")]
pub type AXAttributedStringReturnBlock = *mut block2::Block<dyn Fn() -> *mut NSAttributedString>;

#[cfg(feature = "block2")]
pub type AXAttributedStringArrayReturnBlock =
    *mut block2::Block<dyn Fn() -> *mut NSArray<NSAttributedString>>;

#[cfg(feature = "block2")]
pub type AXRectReturnBlock = *mut block2::Block<dyn Fn() -> CGRect>;

#[cfg(all(feature = "UIBezierPath", feature = "block2"))]
pub type AXPathReturnBlock = *mut block2::Block<dyn Fn() -> *mut UIBezierPath>;

#[cfg(feature = "block2")]
pub type AXPointReturnBlock = *mut block2::Block<dyn Fn() -> CGPoint>;

#[cfg(feature = "block2")]
pub type AXObjectReturnBlock = *mut block2::Block<dyn Fn() -> *mut AnyObject>;

#[cfg(feature = "block2")]
pub type AXArrayReturnBlock = *mut block2::Block<dyn Fn() -> *mut NSArray>;

#[cfg(feature = "block2")]
pub type AXVoidReturnBlock = *mut block2::Block<dyn Fn()>;

#[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
pub type AXTraitsReturnBlock = *mut block2::Block<dyn Fn() -> UIAccessibilityTraits>;

#[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
pub type AXNavigationStyleReturnBlock =
    *mut block2::Block<dyn Fn() -> UIAccessibilityNavigationStyle>;

#[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
pub type AXContainerTypeReturnBlock = *mut block2::Block<dyn Fn() -> UIAccessibilityContainerType>;

#[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
pub type AXTextualContextReturnBlock =
    *mut block2::Block<dyn Fn() -> *mut UIAccessibilityTextualContext>;

#[cfg(all(feature = "UIAccessibilityCustomAction", feature = "block2"))]
pub type AXCustomActionsReturnBlock =
    *mut block2::Block<dyn Fn() -> *mut NSArray<UIAccessibilityCustomAction>>;

extern_category!(
    /// Category "UIAccessibility" on [`NSObject`].
    #[doc(alias = "UIAccessibility")]
    pub unsafe trait NSObjectUIAccessibility {
        #[method(isAccessibilityElement)]
        unsafe fn isAccessibilityElement(&self) -> bool;

        #[method(setIsAccessibilityElement:)]
        unsafe fn setIsAccessibilityElement(&self, is_accessibility_element: bool);

        #[method_id(@__retain_semantics Other accessibilityLabel)]
        unsafe fn accessibilityLabel(&self) -> Option<Id<NSString>>;

        #[method(setAccessibilityLabel:)]
        unsafe fn setAccessibilityLabel(&self, accessibility_label: Option<&NSString>);

        #[method_id(@__retain_semantics Other accessibilityAttributedLabel)]
        unsafe fn accessibilityAttributedLabel(&self) -> Option<Id<NSAttributedString>>;

        #[method(setAccessibilityAttributedLabel:)]
        unsafe fn setAccessibilityAttributedLabel(
            &self,
            accessibility_attributed_label: Option<&NSAttributedString>,
        );

        #[method_id(@__retain_semantics Other accessibilityHint)]
        unsafe fn accessibilityHint(&self) -> Option<Id<NSString>>;

        #[method(setAccessibilityHint:)]
        unsafe fn setAccessibilityHint(&self, accessibility_hint: Option<&NSString>);

        #[method_id(@__retain_semantics Other accessibilityAttributedHint)]
        unsafe fn accessibilityAttributedHint(&self) -> Option<Id<NSAttributedString>>;

        #[method(setAccessibilityAttributedHint:)]
        unsafe fn setAccessibilityAttributedHint(
            &self,
            accessibility_attributed_hint: Option<&NSAttributedString>,
        );

        #[method_id(@__retain_semantics Other accessibilityValue)]
        unsafe fn accessibilityValue(&self) -> Option<Id<NSString>>;

        #[method(setAccessibilityValue:)]
        unsafe fn setAccessibilityValue(&self, accessibility_value: Option<&NSString>);

        #[method_id(@__retain_semantics Other accessibilityAttributedValue)]
        unsafe fn accessibilityAttributedValue(&self) -> Option<Id<NSAttributedString>>;

        #[method(setAccessibilityAttributedValue:)]
        unsafe fn setAccessibilityAttributedValue(
            &self,
            accessibility_attributed_value: Option<&NSAttributedString>,
        );

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(accessibilityTraits)]
        unsafe fn accessibilityTraits(&self) -> UIAccessibilityTraits;

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(setAccessibilityTraits:)]
        unsafe fn setAccessibilityTraits(&self, accessibility_traits: UIAccessibilityTraits);

        #[method(accessibilityFrame)]
        unsafe fn accessibilityFrame(&self) -> CGRect;

        #[method(setAccessibilityFrame:)]
        unsafe fn setAccessibilityFrame(&self, accessibility_frame: CGRect);

        #[cfg(feature = "UIBezierPath")]
        #[method_id(@__retain_semantics Other accessibilityPath)]
        unsafe fn accessibilityPath(&self) -> Option<Id<UIBezierPath>>;

        #[cfg(feature = "UIBezierPath")]
        #[method(setAccessibilityPath:)]
        unsafe fn setAccessibilityPath(&self, accessibility_path: Option<&UIBezierPath>);

        #[method(accessibilityActivationPoint)]
        unsafe fn accessibilityActivationPoint(&self) -> CGPoint;

        #[method(setAccessibilityActivationPoint:)]
        unsafe fn setAccessibilityActivationPoint(&self, accessibility_activation_point: CGPoint);

        #[method_id(@__retain_semantics Other accessibilityLanguage)]
        unsafe fn accessibilityLanguage(&self) -> Option<Id<NSString>>;

        #[method(setAccessibilityLanguage:)]
        unsafe fn setAccessibilityLanguage(&self, accessibility_language: Option<&NSString>);

        #[method(accessibilityElementsHidden)]
        unsafe fn accessibilityElementsHidden(&self) -> bool;

        #[method(setAccessibilityElementsHidden:)]
        unsafe fn setAccessibilityElementsHidden(&self, accessibility_elements_hidden: bool);

        #[method(accessibilityViewIsModal)]
        unsafe fn accessibilityViewIsModal(&self) -> bool;

        #[method(setAccessibilityViewIsModal:)]
        unsafe fn setAccessibilityViewIsModal(&self, accessibility_view_is_modal: bool);

        #[method(shouldGroupAccessibilityChildren)]
        unsafe fn shouldGroupAccessibilityChildren(&self) -> bool;

        #[method(setShouldGroupAccessibilityChildren:)]
        unsafe fn setShouldGroupAccessibilityChildren(
            &self,
            should_group_accessibility_children: bool,
        );

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(accessibilityNavigationStyle)]
        unsafe fn accessibilityNavigationStyle(&self) -> UIAccessibilityNavigationStyle;

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(setAccessibilityNavigationStyle:)]
        unsafe fn setAccessibilityNavigationStyle(
            &self,
            accessibility_navigation_style: UIAccessibilityNavigationStyle,
        );

        #[method(accessibilityRespondsToUserInteraction)]
        unsafe fn accessibilityRespondsToUserInteraction(&self) -> bool;

        #[method(setAccessibilityRespondsToUserInteraction:)]
        unsafe fn setAccessibilityRespondsToUserInteraction(
            &self,
            accessibility_responds_to_user_interaction: bool,
        );

        #[method_id(@__retain_semantics Other accessibilityUserInputLabels)]
        unsafe fn accessibilityUserInputLabels(&self) -> Option<Id<NSArray<NSString>>>;

        #[method(setAccessibilityUserInputLabels:)]
        unsafe fn setAccessibilityUserInputLabels(
            &self,
            accessibility_user_input_labels: Option<&NSArray<NSString>>,
        );

        #[method_id(@__retain_semantics Other accessibilityAttributedUserInputLabels)]
        unsafe fn accessibilityAttributedUserInputLabels(&self) -> Id<NSArray<NSAttributedString>>;

        #[method(setAccessibilityAttributedUserInputLabels:)]
        unsafe fn setAccessibilityAttributedUserInputLabels(
            &self,
            accessibility_attributed_user_input_labels: Option<&NSArray<NSAttributedString>>,
        );

        #[method_id(@__retain_semantics Other accessibilityHeaderElements)]
        unsafe fn accessibilityHeaderElements(&self) -> Option<Id<NSArray>>;

        #[method(setAccessibilityHeaderElements:)]
        unsafe fn setAccessibilityHeaderElements(
            &self,
            accessibility_header_elements: Option<&NSArray>,
        );

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method_id(@__retain_semantics Other accessibilityTextualContext)]
        unsafe fn accessibilityTextualContext(&self) -> Option<Id<UIAccessibilityTextualContext>>;

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(setAccessibilityTextualContext:)]
        unsafe fn setAccessibilityTextualContext(
            &self,
            accessibility_textual_context: Option<&UIAccessibilityTextualContext>,
        );

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(accessibilityDirectTouchOptions)]
        unsafe fn accessibilityDirectTouchOptions(&self) -> UIAccessibilityDirectTouchOptions;

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(setAccessibilityDirectTouchOptions:)]
        unsafe fn setAccessibilityDirectTouchOptions(
            &self,
            accessibility_direct_touch_options: UIAccessibilityDirectTouchOptions,
        );

        #[cfg(feature = "block2")]
        #[method(isAccessibilityElementBlock)]
        unsafe fn isAccessibilityElementBlock(&self) -> AXBoolReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setIsAccessibilityElementBlock:)]
        unsafe fn setIsAccessibilityElementBlock(
            &self,
            is_accessibility_element_block: AXBoolReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityLabelBlock)]
        unsafe fn accessibilityLabelBlock(&self) -> AXStringReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityLabelBlock:)]
        unsafe fn setAccessibilityLabelBlock(&self, accessibility_label_block: AXStringReturnBlock);

        #[cfg(feature = "block2")]
        #[method(accessibilityValueBlock)]
        unsafe fn accessibilityValueBlock(&self) -> AXStringReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityValueBlock:)]
        unsafe fn setAccessibilityValueBlock(&self, accessibility_value_block: AXStringReturnBlock);

        #[cfg(feature = "block2")]
        #[method(accessibilityHintBlock)]
        unsafe fn accessibilityHintBlock(&self) -> AXStringReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityHintBlock:)]
        unsafe fn setAccessibilityHintBlock(&self, accessibility_hint_block: AXStringReturnBlock);

        #[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
        #[method(accessibilityTraitsBlock)]
        unsafe fn accessibilityTraitsBlock(&self) -> AXTraitsReturnBlock;

        #[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
        #[method(setAccessibilityTraitsBlock:)]
        unsafe fn setAccessibilityTraitsBlock(
            &self,
            accessibility_traits_block: AXTraitsReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityIdentifierBlock)]
        unsafe fn accessibilityIdentifierBlock(&self) -> AXStringReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityIdentifierBlock:)]
        unsafe fn setAccessibilityIdentifierBlock(
            &self,
            accessibility_identifier_block: AXStringReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityHeaderElementsBlock)]
        unsafe fn accessibilityHeaderElementsBlock(&self) -> AXArrayReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityHeaderElementsBlock:)]
        unsafe fn setAccessibilityHeaderElementsBlock(
            &self,
            accessibility_header_elements_block: AXArrayReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityAttributedLabelBlock)]
        unsafe fn accessibilityAttributedLabelBlock(&self) -> AXAttributedStringReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityAttributedLabelBlock:)]
        unsafe fn setAccessibilityAttributedLabelBlock(
            &self,
            accessibility_attributed_label_block: AXAttributedStringReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityAttributedHintBlock)]
        unsafe fn accessibilityAttributedHintBlock(&self) -> AXAttributedStringReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityAttributedHintBlock:)]
        unsafe fn setAccessibilityAttributedHintBlock(
            &self,
            accessibility_attributed_hint_block: AXAttributedStringReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityLanguageBlock)]
        unsafe fn accessibilityLanguageBlock(&self) -> AXStringReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityLanguageBlock:)]
        unsafe fn setAccessibilityLanguageBlock(
            &self,
            accessibility_language_block: AXStringReturnBlock,
        );

        #[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
        #[method(accessibilityTextualContextBlock)]
        unsafe fn accessibilityTextualContextBlock(&self) -> AXTextualContextReturnBlock;

        #[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
        #[method(setAccessibilityTextualContextBlock:)]
        unsafe fn setAccessibilityTextualContextBlock(
            &self,
            accessibility_textual_context_block: AXTextualContextReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityUserInputLabelsBlock)]
        unsafe fn accessibilityUserInputLabelsBlock(&self) -> AXStringArrayReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityUserInputLabelsBlock:)]
        unsafe fn setAccessibilityUserInputLabelsBlock(
            &self,
            accessibility_user_input_labels_block: AXStringArrayReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityAttributedUserInputLabelsBlock)]
        unsafe fn accessibilityAttributedUserInputLabelsBlock(
            &self,
        ) -> AXAttributedStringArrayReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityAttributedUserInputLabelsBlock:)]
        unsafe fn setAccessibilityAttributedUserInputLabelsBlock(
            &self,
            accessibility_attributed_user_input_labels_block: AXAttributedStringArrayReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityAttributedValueBlock)]
        unsafe fn accessibilityAttributedValueBlock(&self) -> AXAttributedStringReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityAttributedValueBlock:)]
        unsafe fn setAccessibilityAttributedValueBlock(
            &self,
            accessibility_attributed_value_block: AXAttributedStringReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityElementsHiddenBlock)]
        unsafe fn accessibilityElementsHiddenBlock(&self) -> AXBoolReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityElementsHiddenBlock:)]
        unsafe fn setAccessibilityElementsHiddenBlock(
            &self,
            accessibility_elements_hidden_block: AXBoolReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityRespondsToUserInteractionBlock)]
        unsafe fn accessibilityRespondsToUserInteractionBlock(&self) -> AXBoolReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityRespondsToUserInteractionBlock:)]
        unsafe fn setAccessibilityRespondsToUserInteractionBlock(
            &self,
            accessibility_responds_to_user_interaction_block: AXBoolReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityViewIsModalBlock)]
        unsafe fn accessibilityViewIsModalBlock(&self) -> AXBoolReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityViewIsModalBlock:)]
        unsafe fn setAccessibilityViewIsModalBlock(
            &self,
            accessibility_view_is_modal_block: AXBoolReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityShouldGroupAccessibilityChildrenBlock)]
        unsafe fn accessibilityShouldGroupAccessibilityChildrenBlock(&self) -> AXBoolReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityShouldGroupAccessibilityChildrenBlock:)]
        unsafe fn setAccessibilityShouldGroupAccessibilityChildrenBlock(
            &self,
            accessibility_should_group_accessibility_children_block: AXBoolReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityElementsBlock)]
        unsafe fn accessibilityElementsBlock(&self) -> AXArrayReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityElementsBlock:)]
        unsafe fn setAccessibilityElementsBlock(
            &self,
            accessibility_elements_block: AXArrayReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(automationElementsBlock)]
        unsafe fn automationElementsBlock(&self) -> AXArrayReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAutomationElementsBlock:)]
        unsafe fn setAutomationElementsBlock(&self, automation_elements_block: AXArrayReturnBlock);

        #[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
        #[method(accessibilityContainerTypeBlock)]
        unsafe fn accessibilityContainerTypeBlock(&self) -> AXContainerTypeReturnBlock;

        #[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
        #[method(setAccessibilityContainerTypeBlock:)]
        unsafe fn setAccessibilityContainerTypeBlock(
            &self,
            accessibility_container_type_block: AXContainerTypeReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityActivationPointBlock)]
        unsafe fn accessibilityActivationPointBlock(&self) -> AXPointReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityActivationPointBlock:)]
        unsafe fn setAccessibilityActivationPointBlock(
            &self,
            accessibility_activation_point_block: AXPointReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityFrameBlock)]
        unsafe fn accessibilityFrameBlock(&self) -> AXRectReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityFrameBlock:)]
        unsafe fn setAccessibilityFrameBlock(&self, accessibility_frame_block: AXRectReturnBlock);

        #[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
        #[method(accessibilityNavigationStyleBlock)]
        unsafe fn accessibilityNavigationStyleBlock(&self) -> AXNavigationStyleReturnBlock;

        #[cfg(all(feature = "UIAccessibilityConstants", feature = "block2"))]
        #[method(setAccessibilityNavigationStyleBlock:)]
        unsafe fn setAccessibilityNavigationStyleBlock(
            &self,
            accessibility_navigation_style_block: AXNavigationStyleReturnBlock,
        );

        #[cfg(all(feature = "UIBezierPath", feature = "block2"))]
        #[method(accessibilityPathBlock)]
        unsafe fn accessibilityPathBlock(&self) -> AXPathReturnBlock;

        #[cfg(all(feature = "UIBezierPath", feature = "block2"))]
        #[method(setAccessibilityPathBlock:)]
        unsafe fn setAccessibilityPathBlock(&self, accessibility_path_block: AXPathReturnBlock);

        #[cfg(feature = "block2")]
        #[method(accessibilityActivateBlock)]
        unsafe fn accessibilityActivateBlock(&self) -> AXBoolReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityActivateBlock:)]
        unsafe fn setAccessibilityActivateBlock(
            &self,
            accessibility_activate_block: AXBoolReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityIncrementBlock)]
        unsafe fn accessibilityIncrementBlock(&self) -> AXVoidReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityIncrementBlock:)]
        unsafe fn setAccessibilityIncrementBlock(
            &self,
            accessibility_increment_block: AXVoidReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityDecrementBlock)]
        unsafe fn accessibilityDecrementBlock(&self) -> AXVoidReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityDecrementBlock:)]
        unsafe fn setAccessibilityDecrementBlock(
            &self,
            accessibility_decrement_block: AXVoidReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityPerformEscapeBlock)]
        unsafe fn accessibilityPerformEscapeBlock(&self) -> AXBoolReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityPerformEscapeBlock:)]
        unsafe fn setAccessibilityPerformEscapeBlock(
            &self,
            accessibility_perform_escape_block: AXBoolReturnBlock,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityMagicTapBlock)]
        unsafe fn accessibilityMagicTapBlock(&self) -> AXBoolReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityMagicTapBlock:)]
        unsafe fn setAccessibilityMagicTapBlock(
            &self,
            accessibility_magic_tap_block: AXBoolReturnBlock,
        );

        #[cfg(all(feature = "UIAccessibilityCustomAction", feature = "block2"))]
        #[method(accessibilityCustomActionsBlock)]
        unsafe fn accessibilityCustomActionsBlock(&self) -> AXCustomActionsReturnBlock;

        #[cfg(all(feature = "UIAccessibilityCustomAction", feature = "block2"))]
        #[method(setAccessibilityCustomActionsBlock:)]
        unsafe fn setAccessibilityCustomActionsBlock(
            &self,
            accessibility_custom_actions_block: AXCustomActionsReturnBlock,
        );
    }

    unsafe impl NSObjectUIAccessibility for NSObject {}
);

extern "C" {
    #[cfg(feature = "UIAccessibilityConstants")]
    pub fn UIAccessibilityFocusedElement(
        assistive_technology_identifier: Option<&UIAccessibilityAssistiveTechnologyIdentifier>,
    ) -> *mut AnyObject;
}

extern_category!(
    /// Category "UIAccessibilityFocus" on [`NSObject`].
    #[doc(alias = "UIAccessibilityFocus")]
    pub unsafe trait NSObjectUIAccessibilityFocus {
        #[method(accessibilityElementDidBecomeFocused)]
        unsafe fn accessibilityElementDidBecomeFocused(&self);

        #[method(accessibilityElementDidLoseFocus)]
        unsafe fn accessibilityElementDidLoseFocus(&self);

        #[method(accessibilityElementIsFocused)]
        unsafe fn accessibilityElementIsFocused(&self) -> bool;

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method_id(@__retain_semantics Other accessibilityAssistiveTechnologyFocusedIdentifiers)]
        unsafe fn accessibilityAssistiveTechnologyFocusedIdentifiers(
            &self,
        ) -> Option<Id<NSSet<UIAccessibilityAssistiveTechnologyIdentifier>>>;
    }

    unsafe impl NSObjectUIAccessibilityFocus for NSObject {}
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityScrollDirection(pub NSInteger);
impl UIAccessibilityScrollDirection {
    #[doc(alias = "UIAccessibilityScrollDirectionRight")]
    pub const Right: Self = Self(1);
    #[doc(alias = "UIAccessibilityScrollDirectionLeft")]
    pub const Left: Self = Self(2);
    #[doc(alias = "UIAccessibilityScrollDirectionUp")]
    pub const Up: Self = Self(3);
    #[doc(alias = "UIAccessibilityScrollDirectionDown")]
    pub const Down: Self = Self(4);
    #[doc(alias = "UIAccessibilityScrollDirectionNext")]
    pub const Next: Self = Self(5);
    #[doc(alias = "UIAccessibilityScrollDirectionPrevious")]
    pub const Previous: Self = Self(6);
}

unsafe impl Encode for UIAccessibilityScrollDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityScrollDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_category!(
    /// Category "UIAccessibilityAction" on [`NSObject`].
    #[doc(alias = "UIAccessibilityAction")]
    pub unsafe trait NSObjectUIAccessibilityAction {
        #[method(accessibilityActivate)]
        unsafe fn accessibilityActivate(&self) -> bool;

        #[method(accessibilityIncrement)]
        unsafe fn accessibilityIncrement(&self);

        #[method(accessibilityDecrement)]
        unsafe fn accessibilityDecrement(&self);

        #[method(accessibilityZoomInAtPoint:)]
        unsafe fn accessibilityZoomInAtPoint(&self, point: CGPoint) -> bool;

        #[method(accessibilityZoomOutAtPoint:)]
        unsafe fn accessibilityZoomOutAtPoint(&self, point: CGPoint) -> bool;

        #[method(accessibilityScroll:)]
        unsafe fn accessibilityScroll(&self, direction: UIAccessibilityScrollDirection) -> bool;

        #[method(accessibilityPerformEscape)]
        unsafe fn accessibilityPerformEscape(&self) -> bool;

        #[method(accessibilityPerformMagicTap)]
        unsafe fn accessibilityPerformMagicTap(&self) -> bool;

        #[cfg(feature = "UIAccessibilityCustomAction")]
        #[method_id(@__retain_semantics Other accessibilityCustomActions)]
        unsafe fn accessibilityCustomActions(
            &self,
        ) -> Option<Id<NSArray<UIAccessibilityCustomAction>>>;

        #[cfg(feature = "UIAccessibilityCustomAction")]
        #[method(setAccessibilityCustomActions:)]
        unsafe fn setAccessibilityCustomActions(
            &self,
            accessibility_custom_actions: Option<&NSArray<UIAccessibilityCustomAction>>,
        );
    }

    unsafe impl NSObjectUIAccessibilityAction for NSObject {}
);

extern_protocol!(
    pub unsafe trait UIAccessibilityReadingContent {
        #[method(accessibilityLineNumberForPoint:)]
        unsafe fn accessibilityLineNumberForPoint(&self, point: CGPoint) -> NSInteger;

        #[method_id(@__retain_semantics Other accessibilityContentForLineNumber:)]
        unsafe fn accessibilityContentForLineNumber(
            &self,
            line_number: NSInteger,
        ) -> Option<Id<NSString>>;

        #[method(accessibilityFrameForLineNumber:)]
        unsafe fn accessibilityFrameForLineNumber(&self, line_number: NSInteger) -> CGRect;

        #[method_id(@__retain_semantics Other accessibilityPageContent)]
        unsafe fn accessibilityPageContent(&self) -> Option<Id<NSString>>;

        #[optional]
        #[method_id(@__retain_semantics Other accessibilityAttributedContentForLineNumber:)]
        unsafe fn accessibilityAttributedContentForLineNumber(
            &self,
            line_number: NSInteger,
        ) -> Option<Id<NSAttributedString>>;

        #[optional]
        #[method_id(@__retain_semantics Other accessibilityAttributedPageContent)]
        unsafe fn accessibilityAttributedPageContent(&self) -> Option<Id<NSAttributedString>>;
    }

    unsafe impl ProtocolType for dyn UIAccessibilityReadingContent {}
);

extern_category!(
    /// Category "UIAccessibilityDragging" on [`NSObject`].
    #[doc(alias = "UIAccessibilityDragging")]
    pub unsafe trait NSObjectUIAccessibilityDragging {
        #[cfg(feature = "UIAccessibilityLocationDescriptor")]
        #[method_id(@__retain_semantics Other accessibilityDragSourceDescriptors)]
        unsafe fn accessibilityDragSourceDescriptors(
            &self,
        ) -> Option<Id<NSArray<UIAccessibilityLocationDescriptor>>>;

        #[cfg(feature = "UIAccessibilityLocationDescriptor")]
        #[method(setAccessibilityDragSourceDescriptors:)]
        unsafe fn setAccessibilityDragSourceDescriptors(
            &self,
            accessibility_drag_source_descriptors: Option<
                &NSArray<UIAccessibilityLocationDescriptor>,
            >,
        );

        #[cfg(feature = "UIAccessibilityLocationDescriptor")]
        #[method_id(@__retain_semantics Other accessibilityDropPointDescriptors)]
        unsafe fn accessibilityDropPointDescriptors(
            &self,
        ) -> Option<Id<NSArray<UIAccessibilityLocationDescriptor>>>;

        #[cfg(feature = "UIAccessibilityLocationDescriptor")]
        #[method(setAccessibilityDropPointDescriptors:)]
        unsafe fn setAccessibilityDropPointDescriptors(
            &self,
            accessibility_drop_point_descriptors: Option<
                &NSArray<UIAccessibilityLocationDescriptor>,
            >,
        );
    }

    unsafe impl NSObjectUIAccessibilityDragging for NSObject {}
);

extern "C" {
    #[cfg(feature = "UIAccessibilityConstants")]
    pub fn UIAccessibilityPostNotification(
        notification: UIAccessibilityNotifications,
        argument: Option<&AnyObject>,
    );
}

extern "C" {
    pub fn UIAccessibilityIsVoiceOverRunning() -> Bool;
}

extern "C" {
    pub static UIAccessibilityVoiceOverStatusChanged: &'static NSString;
}

extern "C" {
    pub static UIAccessibilityVoiceOverStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsMonoAudioEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityMonoAudioStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsClosedCaptioningEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityClosedCaptioningStatusDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsInvertColorsEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityInvertColorsStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsGuidedAccessEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityGuidedAccessStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsBoldTextEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityBoldTextStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityButtonShapesEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityButtonShapesEnabledStatusDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsGrayscaleEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityGrayscaleStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsReduceTransparencyEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityReduceTransparencyStatusDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsReduceMotionEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityReduceMotionStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityPrefersCrossFadeTransitions() -> Bool;
}

extern "C" {
    pub static UIAccessibilityPrefersCrossFadeTransitionsStatusDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsVideoAutoplayEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityVideoAutoplayStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityDarkerSystemColorsEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityDarkerSystemColorsStatusDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsSwitchControlRunning() -> Bool;
}

extern "C" {
    pub static UIAccessibilitySwitchControlStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsSpeakSelectionEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilitySpeakSelectionStatusDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsSpeakScreenEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilitySpeakScreenStatusDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsShakeToUndoEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityShakeToUndoDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsAssistiveTouchRunning() -> Bool;
}

extern "C" {
    pub static UIAccessibilityAssistiveTouchStatusDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityShouldDifferentiateWithoutColor() -> Bool;
}

extern "C" {
    pub static UIAccessibilityShouldDifferentiateWithoutColorDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub fn UIAccessibilityIsOnOffSwitchLabelsEnabled() -> Bool;
}

extern "C" {
    pub static UIAccessibilityOnOffSwitchLabelsDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(feature = "block2")]
    pub fn UIAccessibilityRequestGuidedAccessSession(
        enable: Bool,
        completion_handler: &block2::Block<dyn Fn(Bool)>,
    );
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityHearingDeviceEar(pub NSUInteger);
impl UIAccessibilityHearingDeviceEar {
    #[doc(alias = "UIAccessibilityHearingDeviceEarNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIAccessibilityHearingDeviceEarLeft")]
    pub const Left: Self = Self(1 << 1);
    #[doc(alias = "UIAccessibilityHearingDeviceEarRight")]
    pub const Right: Self = Self(1 << 2);
    #[doc(alias = "UIAccessibilityHearingDeviceEarBoth")]
    pub const Both: Self =
        Self(UIAccessibilityHearingDeviceEar::Left.0 | UIAccessibilityHearingDeviceEar::Right.0);
}

unsafe impl Encode for UIAccessibilityHearingDeviceEar {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityHearingDeviceEar {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub fn UIAccessibilityHearingDevicePairedEar() -> UIAccessibilityHearingDeviceEar;
}

extern "C" {
    pub static UIAccessibilityHearingDevicePairedEarDidChangeNotification:
        &'static NSNotificationName;
}
