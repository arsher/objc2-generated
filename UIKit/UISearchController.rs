//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISearchControllerScopeBarActivation(pub NSInteger);
impl UISearchControllerScopeBarActivation {
    #[doc(alias = "UISearchControllerScopeBarActivationAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UISearchControllerScopeBarActivationManual")]
    pub const Manual: Self = Self(1);
    #[doc(alias = "UISearchControllerScopeBarActivationOnTextEntry")]
    pub const OnTextEntry: Self = Self(2);
    #[doc(alias = "UISearchControllerScopeBarActivationOnSearchActivation")]
    pub const OnSearchActivation: Self = Self(3);
}

unsafe impl Encode for UISearchControllerScopeBarActivation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISearchControllerScopeBarActivation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UISearchControllerDelegate:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(willPresentSearchController:)]
        unsafe fn willPresentSearchController(&self, search_controller: &UISearchController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(didPresentSearchController:)]
        unsafe fn didPresentSearchController(&self, search_controller: &UISearchController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(willDismissSearchController:)]
        unsafe fn willDismissSearchController(&self, search_controller: &UISearchController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(didDismissSearchController:)]
        unsafe fn didDismissSearchController(&self, search_controller: &UISearchController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(presentSearchController:)]
        unsafe fn presentSearchController(&self, search_controller: &UISearchController);

        #[cfg(all(
            feature = "UINavigationItem",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(searchController:willChangeToSearchBarPlacement:)]
        unsafe fn searchController_willChangeToSearchBarPlacement(
            &self,
            search_controller: &UISearchController,
            new_placement: UINavigationItemSearchBarPlacement,
        );

        #[cfg(all(
            feature = "UINavigationItem",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(searchController:didChangeFromSearchBarPlacement:)]
        unsafe fn searchController_didChangeFromSearchBarPlacement(
            &self,
            search_controller: &UISearchController,
            previous_placement: UINavigationItemSearchBarPlacement,
        );
    }

    unsafe impl ProtocolType for dyn UISearchControllerDelegate {}
);

extern_protocol!(
    pub unsafe trait UISearchResultsUpdating: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method(updateSearchResultsForSearchController:)]
        unsafe fn updateSearchResultsForSearchController(
            &self,
            search_controller: &UISearchController,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UISearchSuggestion",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(updateSearchResultsForSearchController:selectingSearchSuggestion:)]
        unsafe fn updateSearchResultsForSearchController_selectingSearchSuggestion(
            &self,
            search_controller: &UISearchController,
            search_suggestion: &ProtocolObject<dyn UISearchSuggestion>,
        );
    }

    unsafe impl ProtocolType for dyn UISearchResultsUpdating {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UISearchController;

    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl ClassType for UISearchController {
        #[inherits(UIResponder, NSObject)]
        type Super = UIViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UISearchController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UISearchController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UISearchController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UISearchController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UISearchController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UISearchController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UISearchController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIViewController",
    feature = "UIViewControllerTransitioning"
))]
unsafe impl UIViewControllerAnimatedTransitioning for UISearchController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIViewController",
    feature = "UIViewControllerTransitioning"
))]
unsafe impl UIViewControllerTransitioningDelegate for UISearchController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UISearchController {
        #[method_id(@__retain_semantics Init initWithSearchResultsController:)]
        pub unsafe fn initWithSearchResultsController(
            this: Allocated<Self>,
            search_results_controller: Option<&UIViewController>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other searchResultsUpdater)]
        pub unsafe fn searchResultsUpdater(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UISearchResultsUpdating>>>;

        #[method(setSearchResultsUpdater:)]
        pub unsafe fn setSearchResultsUpdater(
            &self,
            search_results_updater: Option<&ProtocolObject<dyn UISearchResultsUpdating>>,
        );

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Id<ProtocolObject<dyn UISearchControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UISearchControllerDelegate>>,
        );

        #[deprecated]
        #[method(dimsBackgroundDuringPresentation)]
        pub unsafe fn dimsBackgroundDuringPresentation(&self) -> bool;

        #[deprecated]
        #[method(setDimsBackgroundDuringPresentation:)]
        pub unsafe fn setDimsBackgroundDuringPresentation(
            &self,
            dims_background_during_presentation: bool,
        );

        #[method(obscuresBackgroundDuringPresentation)]
        pub unsafe fn obscuresBackgroundDuringPresentation(&self) -> bool;

        #[method(setObscuresBackgroundDuringPresentation:)]
        pub unsafe fn setObscuresBackgroundDuringPresentation(
            &self,
            obscures_background_during_presentation: bool,
        );

        #[method(hidesNavigationBarDuringPresentation)]
        pub unsafe fn hidesNavigationBarDuringPresentation(&self) -> bool;

        #[method(setHidesNavigationBarDuringPresentation:)]
        pub unsafe fn setHidesNavigationBarDuringPresentation(
            &self,
            hides_navigation_bar_during_presentation: bool,
        );

        #[method_id(@__retain_semantics Other searchResultsController)]
        pub unsafe fn searchResultsController(&self) -> Option<Id<UIViewController>>;

        #[cfg(all(feature = "UISearchBar", feature = "UIView"))]
        #[method_id(@__retain_semantics Other searchBar)]
        pub unsafe fn searchBar(&self) -> Id<UISearchBar>;

        #[cfg(feature = "UINavigationItem")]
        #[method(searchBarPlacement)]
        pub unsafe fn searchBarPlacement(&self) -> UINavigationItemSearchBarPlacement;

        #[method(automaticallyShowsSearchResultsController)]
        pub unsafe fn automaticallyShowsSearchResultsController(&self) -> bool;

        #[method(setAutomaticallyShowsSearchResultsController:)]
        pub unsafe fn setAutomaticallyShowsSearchResultsController(
            &self,
            automatically_shows_search_results_controller: bool,
        );

        #[method(showsSearchResultsController)]
        pub unsafe fn showsSearchResultsController(&self) -> bool;

        #[method(setShowsSearchResultsController:)]
        pub unsafe fn setShowsSearchResultsController(&self, shows_search_results_controller: bool);

        #[method(automaticallyShowsCancelButton)]
        pub unsafe fn automaticallyShowsCancelButton(&self) -> bool;

        #[method(setAutomaticallyShowsCancelButton:)]
        pub unsafe fn setAutomaticallyShowsCancelButton(
            &self,
            automatically_shows_cancel_button: bool,
        );

        #[deprecated = "Use scopeBarActivation instead"]
        #[method(automaticallyShowsScopeBar)]
        pub unsafe fn automaticallyShowsScopeBar(&self) -> bool;

        #[deprecated = "Use scopeBarActivation instead"]
        #[method(setAutomaticallyShowsScopeBar:)]
        pub unsafe fn setAutomaticallyShowsScopeBar(&self, automatically_shows_scope_bar: bool);

        #[method(scopeBarActivation)]
        pub unsafe fn scopeBarActivation(&self) -> UISearchControllerScopeBarActivation;

        #[method(setScopeBarActivation:)]
        pub unsafe fn setScopeBarActivation(
            &self,
            scope_bar_activation: UISearchControllerScopeBarActivation,
        );

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

        #[method(ignoresSearchSuggestionsForSearchBarPlacementStacked)]
        pub unsafe fn ignoresSearchSuggestionsForSearchBarPlacementStacked(&self) -> bool;

        #[method(setIgnoresSearchSuggestionsForSearchBarPlacementStacked:)]
        pub unsafe fn setIgnoresSearchSuggestionsForSearchBarPlacementStacked(
            &self,
            ignores_search_suggestions_for_search_bar_placement_stacked: bool,
        );

        #[cfg(all(feature = "UIScrollView", feature = "UIView"))]
        #[deprecated = "Use -[UIViewController setContentScrollView:forEdge:] on the searchResultsController instead."]
        #[method_id(@__retain_semantics Other searchControllerObservedScrollView)]
        pub unsafe fn searchControllerObservedScrollView(&self) -> Option<Id<UIScrollView>>;

        #[cfg(all(feature = "UIScrollView", feature = "UIView"))]
        #[deprecated = "Use -[UIViewController setContentScrollView:forEdge:] on the searchResultsController instead."]
        #[method(setSearchControllerObservedScrollView:)]
        pub unsafe fn setSearchControllerObservedScrollView(
            &self,
            search_controller_observed_scroll_view: Option<&UIScrollView>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UISearchController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
