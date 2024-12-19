//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchtextfield?language=objc)
    #[unsafe(super(UITextField, UIControl, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "UIControl",
        feature = "UIResponder",
        feature = "UITextField",
        feature = "UIView"
    ))]
    pub struct UISearchTextField;
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
        pub unsafe fn tokens(&self) -> Retained<NSArray<UISearchToken>>;

        #[method(setTokens:)]
        pub unsafe fn setTokens(&self, tokens: &NSArray<UISearchToken>);

        #[method(insertToken:atIndex:)]
        pub unsafe fn insertToken_atIndex(&self, token: &UISearchToken, token_index: NSInteger);

        #[method(removeTokenAtIndex:)]
        pub unsafe fn removeTokenAtIndex(&self, token_index: NSInteger);

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other positionOfTokenAtIndex:)]
        pub unsafe fn positionOfTokenAtIndex(
            &self,
            token_index: NSInteger,
        ) -> Retained<UITextPosition>;

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other tokensInRange:)]
        pub unsafe fn tokensInRange(
            &self,
            text_range: &UITextRange,
        ) -> Retained<NSArray<UISearchToken>>;

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other textualRange)]
        pub unsafe fn textualRange(&self) -> Retained<UITextRange>;

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
        pub unsafe fn tokenBackgroundColor(&self) -> Option<Retained<UIColor>>;

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
        ) -> Option<Retained<NSArray<ProtocolObject<dyn UISearchSuggestion>>>>;

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
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "UIAction",
            feature = "UIMenuElement",
            feature = "objc2-core-foundation"
        ))]
        #[method_id(@__retain_semantics Init initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Retained<Self>;
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
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchtoken?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISearchToken;
);

unsafe impl NSObjectProtocol for UISearchToken {}

extern_methods!(
    unsafe impl UISearchToken {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other tokenWithIcon:text:)]
        pub unsafe fn tokenWithIcon_text(
            icon: Option<&UIImage>,
            text: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<UISearchToken>;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Retained<AnyObject>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchtextfielddelegate?language=objc)
    #[cfg(feature = "UITextField")]
    pub unsafe trait UISearchTextFieldDelegate:
        UITextFieldDelegate + MainThreadOnly
    {
        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method_id(@__retain_semantics Other searchTextField:itemProviderForCopyingToken:)]
        unsafe fn searchTextField_itemProviderForCopyingToken(
            &self,
            search_text_field: &UISearchTextField,
            token: &UISearchToken,
        ) -> Retained<NSItemProvider>;

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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchtextfieldpasteitem?language=objc)
    #[cfg(feature = "UITextPasteDelegate")]
    pub unsafe trait UISearchTextFieldPasteItem: UITextPasteItem + MainThreadOnly {
        #[method(setSearchTokenResult:)]
        unsafe fn setSearchTokenResult(&self, token: &UISearchToken);
    }

    #[cfg(feature = "UITextPasteDelegate")]
    unsafe impl ProtocolType for dyn UISearchTextFieldPasteItem {}
);
