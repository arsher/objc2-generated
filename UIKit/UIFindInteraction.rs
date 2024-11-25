//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifindinteractiondelegate?language=objc)
    pub unsafe trait UIFindInteractionDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "UIFindSession", feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other findInteraction:sessionForView:)]
        unsafe fn findInteraction_sessionForView(
            &self,
            interaction: &UIFindInteraction,
            view: &UIView,
        ) -> Option<Retained<UIFindSession>>;

        #[cfg(feature = "UIFindSession")]
        #[optional]
        #[method(findInteraction:didBeginFindSession:)]
        unsafe fn findInteraction_didBeginFindSession(
            &self,
            interaction: &UIFindInteraction,
            session: &UIFindSession,
        );

        #[cfg(feature = "UIFindSession")]
        #[optional]
        #[method(findInteraction:didEndFindSession:)]
        unsafe fn findInteraction_didEndFindSession(
            &self,
            interaction: &UIFindInteraction,
            session: &UIFindSession,
        );
    }

    unsafe impl ProtocolType for dyn UIFindInteractionDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifindinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFindInteraction;
);

unsafe impl NSObjectProtocol for UIFindInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIFindInteraction {}

extern_methods!(
    unsafe impl UIFindInteraction {
        #[method(isFindNavigatorVisible)]
        pub unsafe fn isFindNavigatorVisible(&self) -> bool;

        #[cfg(feature = "UIFindSession")]
        #[method_id(@__retain_semantics Other activeFindSession)]
        pub unsafe fn activeFindSession(&self) -> Option<Retained<UIFindSession>>;

        #[method_id(@__retain_semantics Other searchText)]
        pub unsafe fn searchText(&self) -> Option<Retained<NSString>>;

        #[method(setSearchText:)]
        pub unsafe fn setSearchText(&self, search_text: Option<&NSString>);

        #[method_id(@__retain_semantics Other replacementText)]
        pub unsafe fn replacementText(&self) -> Option<Retained<NSString>>;

        #[method(setReplacementText:)]
        pub unsafe fn setReplacementText(&self, replacement_text: Option<&NSString>);

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement", feature = "block2"))]
        #[method(optionsMenuProvider)]
        pub unsafe fn optionsMenuProvider(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<NSArray<UIMenuElement>>) -> *mut UIMenu>;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement", feature = "block2"))]
        #[method(setOptionsMenuProvider:)]
        pub unsafe fn setOptionsMenuProvider(
            &self,
            options_menu_provider: Option<
                &block2::Block<dyn Fn(NonNull<NSArray<UIMenuElement>>) -> *mut UIMenu>,
            >,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIFindInteractionDelegate>>>;

        #[method_id(@__retain_semantics Init initWithSessionDelegate:)]
        pub unsafe fn initWithSessionDelegate(
            this: Allocated<Self>,
            session_delegate: &ProtocolObject<dyn UIFindInteractionDelegate>,
        ) -> Retained<Self>;

        #[method(presentFindNavigatorShowingReplace:)]
        pub unsafe fn presentFindNavigatorShowingReplace(&self, showing_replace: bool);

        #[method(dismissFindNavigator)]
        pub unsafe fn dismissFindNavigator(&self);

        #[method(findNext)]
        pub unsafe fn findNext(&self);

        #[method(findPrevious)]
        pub unsafe fn findPrevious(&self);

        #[method(updateResultCount)]
        pub unsafe fn updateResultCount(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
