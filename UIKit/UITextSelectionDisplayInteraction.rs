//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextselectiondisplayinteractiondelegate?language=objc)
    pub unsafe trait UITextSelectionDisplayInteractionDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// If different than the view that the interaction is installed onto, one can return a container view here for
        /// selection views that draw _below_ text. Includes selection highlight view, etc. The default is assumed
        /// that all views are to be installed onto the interaction's view.
        #[optional]
        #[method_id(@__retain_semantics Other selectionContainerViewBelowTextForSelectionDisplayInteraction:)]
        unsafe fn selectionContainerViewBelowTextForSelectionDisplayInteraction(
            &self,
            interaction: &UITextSelectionDisplayInteraction,
        ) -> Option<Retained<UIView>>;
    }

    unsafe impl ProtocolType for dyn UITextSelectionDisplayInteractionDelegate {}
);

extern_class!(
    /// Manages a collection of selection views (cursor, highlight, range adjustment) for a particular UITextInput object.
    ///
    /// This is the component that
    /// `UITextInteraction`generally talks to in order to accomplish all selection display
    /// using a collection of "managed subviews", i.e., selection view components that actually manage the display of the selection
    /// and the various affordances for changing the selection.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextselectiondisplayinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextSelectionDisplayInteraction;
);

unsafe impl NSObjectProtocol for UITextSelectionDisplayInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UITextSelectionDisplayInteraction {}

extern_methods!(
    unsafe impl UITextSelectionDisplayInteraction {
        /// Controls both the hidden sate of contained selection views as well as interactions that follow.
        #[method(isActivated)]
        pub unsafe fn isActivated(&self) -> bool;

        /// Setter for [`isActivated`][Self::isActivated].
        #[method(setActivated:)]
        pub unsafe fn setActivated(&self, activated: bool);

        #[cfg(all(feature = "UITextInput", feature = "UITextInputTraits"))]
        /// The object the selection is being managed for.
        #[method_id(@__retain_semantics Other textInput)]
        pub unsafe fn textInput(&self) -> Option<Retained<ProtocolObject<dyn UITextInput>>>;

        /// See
        /// `UITextSelectionDisplayInteractionDelegate.`
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITextSelectionDisplayInteractionDelegate>>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextCursorView",
            feature = "UIView"
        ))]
        /// The cursor view (also known as "caret" view). Shown when the selection is not ranged.
        #[method_id(@__retain_semantics Other cursorView)]
        pub unsafe fn cursorView(&self) -> Retained<UIView>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextCursorView",
            feature = "UIView"
        ))]
        /// Setter for [`cursorView`][Self::cursorView].
        #[method(setCursorView:)]
        pub unsafe fn setCursorView(&self, cursor_view: &UIView);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextSelectionHighlightView",
            feature = "UIView"
        ))]
        /// The highlight view. This is the blue/tinted highlight drawn behind the rendered text.
        #[method_id(@__retain_semantics Other highlightView)]
        pub unsafe fn highlightView(&self) -> Retained<UIView>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextSelectionHighlightView",
            feature = "UIView"
        ))]
        /// Setter for [`highlightView`][Self::highlightView].
        #[method(setHighlightView:)]
        pub unsafe fn setHighlightView(&self, highlight_view: &UIView);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextSelectionHandleView",
            feature = "UIView"
        ))]
        /// The selection handles, shown adjacent to the highlight view's
        /// `selectionRects`when the selection is ranged.
        ///
        /// If you are replacing these system-provided handle views with your own, you must provide exactly two handle views, one to be used as the leading handle,
        /// and another to be used as the trailing handle.
        #[method_id(@__retain_semantics Other handleViews)]
        pub unsafe fn handleViews(&self) -> Retained<NSArray<UIView>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextSelectionHandleView",
            feature = "UIView"
        ))]
        /// Setter for [`handleViews`][Self::handleViews].
        #[method(setHandleViews:)]
        pub unsafe fn setHandleViews(&self, handle_views: &NSArray<UIView>);

        #[cfg(all(feature = "UITextInput", feature = "UITextInputTraits"))]
        /// Creates a UITextSelectionDisplayInteractionDelegate for a given object that implements the UITextInput protocol.
        /// `textInput` may be the same as the view this interaction is installed onto.
        #[method_id(@__retain_semantics Init initWithTextInput:delegate:)]
        pub unsafe fn initWithTextInput_delegate(
            this: Allocated<Self>,
            text_input: &ProtocolObject<dyn UITextInput>,
            delegate: &ProtocolObject<dyn UITextSelectionDisplayInteractionDelegate>,
        ) -> Retained<Self>;

        /// Loads the selection from `-[UITextInput selectedTextRange]` and applies the selection to all managed subviews.
        #[method(layoutManagedSubviews)]
        pub unsafe fn layoutManagedSubviews(&self);

        /// Call this whenever the selection changes, or needs to be re-laid out.
        #[method(setNeedsSelectionUpdate)]
        pub unsafe fn setNeedsSelectionUpdate(&self);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
