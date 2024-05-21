//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "UIControl",
        feature = "UIResponder",
        feature = "UITextField",
        feature = "UIView"
    ))]
    pub struct UISearchTextField;

    #[cfg(all(
        feature = "UIControl",
        feature = "UIResponder",
        feature = "UITextField",
        feature = "UIView"
    ))]
    unsafe impl ClassType for UISearchTextField {
        #[inherits(UIControl, UIView, UIResponder, NSObject)]
        type Super = UITextField;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl NSCoding for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl NSObjectProtocol for UISearchTextField {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UIAppearance for UISearchTextField {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UISearchTextField {}

#[cfg(all(
    feature = "UIContentSizeCategoryAdjusting",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UIContentSizeCategoryAdjusting for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UICoordinateSpace for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
unsafe impl UIKeyInput for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UILetterformAwareAdjusting",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UILetterformAwareAdjusting for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UIView"
))]
unsafe impl UIResponderStandardEditActions for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
unsafe impl UITextInput for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
unsafe impl UITextInputTraits for UISearchTextField {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextField",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UISearchTextField {}

extern_methods!(
    #[cfg(all(
        feature = "UIControl",
        feature = "UIResponder",
        feature = "UITextField",
        feature = "UIView"
    ))]
    unsafe impl UISearchTextField {
        #[method_id(@__retain_semantics Other tokens)]
        pub unsafe fn tokens(&self) -> Id<NSArray<UISearchToken>>;

        #[method(setTokens:)]
        pub unsafe fn setTokens(&self, tokens: &NSArray<UISearchToken>);

        #[method(insertToken:atIndex:)]
        pub unsafe fn insertToken_atIndex(&self, token: &UISearchToken, token_index: NSInteger);

        #[method(removeTokenAtIndex:)]
        pub unsafe fn removeTokenAtIndex(&self, token_index: NSInteger);

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other positionOfTokenAtIndex:)]
        pub unsafe fn positionOfTokenAtIndex(&self, token_index: NSInteger) -> Id<UITextPosition>;

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other tokensInRange:)]
        pub unsafe fn tokensInRange(&self, text_range: &UITextRange) -> Id<NSArray<UISearchToken>>;

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other textualRange)]
        pub unsafe fn textualRange(&self) -> Id<UITextRange>;

        #[cfg(feature = "UITextInput")]
        #[method(replaceTextualPortionOfRange:withToken:atIndex:)]
        pub unsafe fn replaceTextualPortionOfRange_withToken_atIndex(
            &self,
            text_range: &UITextRange,
            token: &UISearchToken,
            token_index: NSUInteger,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tokenBackgroundColor)]
        pub unsafe fn tokenBackgroundColor(&self) -> Option<Id<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTokenBackgroundColor:)]
        pub unsafe fn setTokenBackgroundColor(&self, token_background_color: Option<&UIColor>);

        #[method(allowsDeletingTokens)]
        pub unsafe fn allowsDeletingTokens(&self) -> bool;

        #[method(setAllowsDeletingTokens:)]
        pub unsafe fn setAllowsDeletingTokens(&self, allows_deleting_tokens: bool);

        #[method(allowsCopyingTokens)]
        pub unsafe fn allowsCopyingTokens(&self) -> bool;

        #[method(setAllowsCopyingTokens:)]
        pub unsafe fn setAllowsCopyingTokens(&self, allows_copying_tokens: bool);

        #[cfg(feature = "UISearchSuggestion")]
        #[method_id(@__retain_semantics Other searchSuggestions)]
        pub unsafe fn searchSuggestions(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn UISearchSuggestion>>>>;

        #[cfg(feature = "UISearchSuggestion")]
        #[method(setSearchSuggestions:)]
        pub unsafe fn setSearchSuggestions(
            &self,
            search_suggestions: Option<&NSArray<ProtocolObject<dyn UISearchSuggestion>>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIControl`
    #[cfg(all(
        feature = "UIControl",
        feature = "UIResponder",
        feature = "UITextField",
        feature = "UIView"
    ))]
    unsafe impl UISearchTextField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Init initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "UIControl",
        feature = "UIResponder",
        feature = "UITextField",
        feature = "UIView"
    ))]
    unsafe impl UISearchTextField {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISearchToken;

    unsafe impl ClassType for UISearchToken {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UISearchToken {}

extern_methods!(
    unsafe impl UISearchToken {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other tokenWithIcon:text:)]
        pub unsafe fn tokenWithIcon_text(
            icon: Option<&UIImage>,
            text: &NSString,
            mtm: MainThreadMarker,
        ) -> Id<UISearchToken>;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<AnyObject>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);
    }
);

extern_protocol!(
    #[cfg(feature = "UITextField")]
    pub unsafe trait UISearchTextFieldDelegate:
        UITextFieldDelegate + IsMainThreadOnly
    {
        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method_id(@__retain_semantics Other searchTextField:itemProviderForCopyingToken:)]
        unsafe fn searchTextField_itemProviderForCopyingToken(
            &self,
            search_text_field: &UISearchTextField,
            token: &UISearchToken,
        ) -> Id<NSItemProvider>;

        #[cfg(all(
            feature = "UIControl",
            feature = "UIResponder",
            feature = "UISearchSuggestion",
            feature = "UIView"
        ))]
        #[optional]
        #[method(searchTextField:didSelectSuggestion:)]
        unsafe fn searchTextField_didSelectSuggestion(
            &self,
            search_text_field: &UISearchTextField,
            suggestion: &ProtocolObject<dyn UISearchSuggestion>,
        );
    }

    #[cfg(feature = "UITextField")]
    unsafe impl ProtocolType for dyn UISearchTextFieldDelegate {}
);

extern_protocol!(
    #[cfg(feature = "UITextPasteDelegate")]
    pub unsafe trait UISearchTextFieldPasteItem: UITextPasteItem + IsMainThreadOnly {
        #[method(setSearchTokenResult:)]
        unsafe fn setSearchTokenResult(&self, token: &UISearchToken);
    }

    #[cfg(feature = "UITextPasteDelegate")]
    unsafe impl ProtocolType for dyn UISearchTextFieldPasteItem {}
);
