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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchbaricon?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISearchBarIcon(pub NSInteger);
impl UISearchBarIcon {
    #[doc(alias = "UISearchBarIconSearch")]
    pub const Search: Self = Self(0);
    #[doc(alias = "UISearchBarIconClear")]
    pub const Clear: Self = Self(1);
    #[doc(alias = "UISearchBarIconBookmark")]
    pub const Bookmark: Self = Self(2);
    #[doc(alias = "UISearchBarIconResultsList")]
    pub const ResultsList: Self = Self(3);
}

unsafe impl Encode for UISearchBarIcon {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISearchBarIcon {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchbarstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISearchBarStyle(pub NSUInteger);
impl UISearchBarStyle {
    #[doc(alias = "UISearchBarStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UISearchBarStyleProminent")]
    pub const Prominent: Self = Self(1);
    #[doc(alias = "UISearchBarStyleMinimal")]
    pub const Minimal: Self = Self(2);
}

unsafe impl Encode for UISearchBarStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UISearchBarStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilooktodictatecapable?language=objc)
    pub unsafe trait UILookToDictateCapable: NSObjectProtocol {
        /// Enable or disable 'Look To Dictate' on the receiver.
        #[method(isLookToDictateEnabled)]
        unsafe fn isLookToDictateEnabled(&self) -> bool;

        /// Setter for [`isLookToDictateEnabled`][Self::isLookToDictateEnabled].
        #[method(setLookToDictateEnabled:)]
        unsafe fn setLookToDictateEnabled(&self, look_to_dictate_enabled: bool);
    }

    unsafe impl ProtocolType for dyn UILookToDictateCapable {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchbar?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UISearchBar;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UISearchBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UISearchBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UISearchBar {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UISearchBar {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UISearchBar {}

#[cfg(all(feature = "UIBarCommon", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIBarPositioning for UISearchBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UISearchBar {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UISearchBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UISearchBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UISearchBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UISearchBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UILookToDictateCapable for UISearchBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UISearchBar {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
unsafe impl UITextInputTraits for UISearchBar {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UISearchBar {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UISearchBar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "UIInterface")]
        #[method(barStyle)]
        pub unsafe fn barStyle(&self) -> UIBarStyle;

        #[cfg(feature = "UIInterface")]
        /// Setter for [`barStyle`][Self::barStyle].
        #[method(setBarStyle:)]
        pub unsafe fn setBarStyle(&self, bar_style: UIBarStyle);

        #[cfg(feature = "UIBarCommon")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn UISearchBarDelegate>>>;

        #[cfg(feature = "UIBarCommon")]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UISearchBarDelegate>>,
        );

        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Option<Retained<NSString>>;

        /// Setter for [`text`][Self::text].
        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[method_id(@__retain_semantics Other prompt)]
        pub unsafe fn prompt(&self) -> Option<Retained<NSString>>;

        /// Setter for [`prompt`][Self::prompt].
        #[method(setPrompt:)]
        pub unsafe fn setPrompt(&self, prompt: Option<&NSString>);

        #[method_id(@__retain_semantics Other placeholder)]
        pub unsafe fn placeholder(&self) -> Option<Retained<NSString>>;

        /// Setter for [`placeholder`][Self::placeholder].
        #[method(setPlaceholder:)]
        pub unsafe fn setPlaceholder(&self, placeholder: Option<&NSString>);

        #[method(showsBookmarkButton)]
        pub unsafe fn showsBookmarkButton(&self) -> bool;

        /// Setter for [`showsBookmarkButton`][Self::showsBookmarkButton].
        #[method(setShowsBookmarkButton:)]
        pub unsafe fn setShowsBookmarkButton(&self, shows_bookmark_button: bool);

        #[cfg(all(
            feature = "UIControl",
            feature = "UISearchTextField",
            feature = "UITextField"
        ))]
        #[method_id(@__retain_semantics Other searchTextField)]
        pub unsafe fn searchTextField(&self) -> Retained<UISearchTextField>;

        #[method(showsCancelButton)]
        pub unsafe fn showsCancelButton(&self) -> bool;

        /// Setter for [`showsCancelButton`][Self::showsCancelButton].
        #[method(setShowsCancelButton:)]
        pub unsafe fn setShowsCancelButton(&self, shows_cancel_button: bool);

        #[method(showsSearchResultsButton)]
        pub unsafe fn showsSearchResultsButton(&self) -> bool;

        /// Setter for [`showsSearchResultsButton`][Self::showsSearchResultsButton].
        #[method(setShowsSearchResultsButton:)]
        pub unsafe fn setShowsSearchResultsButton(&self, shows_search_results_button: bool);

        #[method(isSearchResultsButtonSelected)]
        pub unsafe fn isSearchResultsButtonSelected(&self) -> bool;

        /// Setter for [`isSearchResultsButtonSelected`][Self::isSearchResultsButtonSelected].
        #[method(setSearchResultsButtonSelected:)]
        pub unsafe fn setSearchResultsButtonSelected(&self, search_results_button_selected: bool);

        #[method(setShowsCancelButton:animated:)]
        pub unsafe fn setShowsCancelButton_animated(
            &self,
            shows_cancel_button: bool,
            animated: bool,
        );

        #[cfg(feature = "UITextInput")]
        /// Use this method to modify the contents of the Unified Content Bar, shown on top of the keyboard when search is engaged.
        /// You may modify the returned inputAssistantItem to add to or replace the existing items on the bar.
        /// Modifications made to the returned UITextInputAssistantItem are reflected automatically.
        #[method_id(@__retain_semantics Other inputAssistantItem)]
        pub unsafe fn inputAssistantItem(&self) -> Retained<UITextInputAssistantItem>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`tintColor`][Self::tintColor].
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other barTintColor)]
        pub unsafe fn barTintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`barTintColor`][Self::barTintColor].
        #[method(setBarTintColor:)]
        pub unsafe fn setBarTintColor(&self, bar_tint_color: Option<&UIColor>);

        #[method(searchBarStyle)]
        pub unsafe fn searchBarStyle(&self) -> UISearchBarStyle;

        /// Setter for [`searchBarStyle`][Self::searchBarStyle].
        #[method(setSearchBarStyle:)]
        pub unsafe fn setSearchBarStyle(&self, search_bar_style: UISearchBarStyle);

        #[method(isTranslucent)]
        pub unsafe fn isTranslucent(&self) -> bool;

        /// Setter for [`isTranslucent`][Self::isTranslucent].
        #[method(setTranslucent:)]
        pub unsafe fn setTranslucent(&self, translucent: bool);

        #[method_id(@__retain_semantics Other scopeButtonTitles)]
        pub unsafe fn scopeButtonTitles(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`scopeButtonTitles`][Self::scopeButtonTitles].
        #[method(setScopeButtonTitles:)]
        pub unsafe fn setScopeButtonTitles(&self, scope_button_titles: Option<&NSArray<NSString>>);

        #[method(selectedScopeButtonIndex)]
        pub unsafe fn selectedScopeButtonIndex(&self) -> NSInteger;

        /// Setter for [`selectedScopeButtonIndex`][Self::selectedScopeButtonIndex].
        #[method(setSelectedScopeButtonIndex:)]
        pub unsafe fn setSelectedScopeButtonIndex(&self, selected_scope_button_index: NSInteger);

        #[method(showsScopeBar)]
        pub unsafe fn showsScopeBar(&self) -> bool;

        /// Setter for [`showsScopeBar`][Self::showsScopeBar].
        #[method(setShowsScopeBar:)]
        pub unsafe fn setShowsScopeBar(&self, shows_scope_bar: bool);

        #[method(setShowsScopeBar:animated:)]
        pub unsafe fn setShowsScopeBar_animated(&self, show: bool, animate: bool);

        #[method_id(@__retain_semantics Other inputAccessoryView)]
        pub unsafe fn inputAccessoryView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`inputAccessoryView`][Self::inputAccessoryView].
        #[method(setInputAccessoryView:)]
        pub unsafe fn setInputAccessoryView(&self, input_accessory_view: Option<&UIView>);

        /// When set to false, user interaction will be prevented and the search bar will take on a disabled appearance
        /// If the search bar is associated with a UINavigationItem with `UINavigationItemSearchBarPlacementInline`,
        /// then the minimized (icon-only) UISearchBar will not grow to the text field while `enabled` is false.
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other backgroundImage)]
        pub unsafe fn backgroundImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`backgroundImage`][Self::backgroundImage].
        #[method(setBackgroundImage:)]
        pub unsafe fn setBackgroundImage(&self, background_image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other scopeBarBackgroundImage)]
        pub unsafe fn scopeBarBackgroundImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`scopeBarBackgroundImage`][Self::scopeBarBackgroundImage].
        #[method(setScopeBarBackgroundImage:)]
        pub unsafe fn setScopeBarBackgroundImage(
            &self,
            scope_bar_background_image: Option<&UIImage>,
        );

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method(setBackgroundImage:forBarPosition:barMetrics:)]
        pub unsafe fn setBackgroundImage_forBarPosition_barMetrics(
            &self,
            background_image: Option<&UIImage>,
            bar_position: UIBarPosition,
            bar_metrics: UIBarMetrics,
        );

        #[cfg(all(feature = "UIBarCommon", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other backgroundImageForBarPosition:barMetrics:)]
        pub unsafe fn backgroundImageForBarPosition_barMetrics(
            &self,
            bar_position: UIBarPosition,
            bar_metrics: UIBarMetrics,
        ) -> Option<Retained<UIImage>>;

        #[cfg(all(feature = "UIControl", feature = "UIImage"))]
        #[method(setSearchFieldBackgroundImage:forState:)]
        pub unsafe fn setSearchFieldBackgroundImage_forState(
            &self,
            background_image: Option<&UIImage>,
            state: UIControlState,
        );

        #[cfg(all(feature = "UIControl", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other searchFieldBackgroundImageForState:)]
        pub unsafe fn searchFieldBackgroundImageForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<UIImage>>;

        #[cfg(all(feature = "UIControl", feature = "UIImage"))]
        #[method(setImage:forSearchBarIcon:state:)]
        pub unsafe fn setImage_forSearchBarIcon_state(
            &self,
            icon_image: Option<&UIImage>,
            icon: UISearchBarIcon,
            state: UIControlState,
        );

        #[cfg(all(feature = "UIControl", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other imageForSearchBarIcon:state:)]
        pub unsafe fn imageForSearchBarIcon_state(
            &self,
            icon: UISearchBarIcon,
            state: UIControlState,
        ) -> Option<Retained<UIImage>>;

        #[cfg(all(feature = "UIControl", feature = "UIImage"))]
        #[method(setScopeBarButtonBackgroundImage:forState:)]
        pub unsafe fn setScopeBarButtonBackgroundImage_forState(
            &self,
            background_image: Option<&UIImage>,
            state: UIControlState,
        );

        #[cfg(all(feature = "UIControl", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other scopeBarButtonBackgroundImageForState:)]
        pub unsafe fn scopeBarButtonBackgroundImageForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<UIImage>>;

        #[cfg(all(feature = "UIControl", feature = "UIImage"))]
        #[method(setScopeBarButtonDividerImage:forLeftSegmentState:rightSegmentState:)]
        pub unsafe fn setScopeBarButtonDividerImage_forLeftSegmentState_rightSegmentState(
            &self,
            divider_image: Option<&UIImage>,
            left_state: UIControlState,
            right_state: UIControlState,
        );

        #[cfg(all(feature = "UIControl", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other scopeBarButtonDividerImageForLeftSegmentState:rightSegmentState:)]
        pub unsafe fn scopeBarButtonDividerImageForLeftSegmentState_rightSegmentState(
            &self,
            left_state: UIControlState,
            right_state: UIControlState,
        ) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIControl")]
        #[method(setScopeBarButtonTitleTextAttributes:forState:)]
        pub unsafe fn setScopeBarButtonTitleTextAttributes_forState(
            &self,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            state: UIControlState,
        );

        #[cfg(feature = "UIControl")]
        #[method_id(@__retain_semantics Other scopeBarButtonTitleTextAttributesForState:)]
        pub unsafe fn scopeBarButtonTitleTextAttributesForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(searchFieldBackgroundPositionAdjustment)]
        pub unsafe fn searchFieldBackgroundPositionAdjustment(&self) -> UIOffset;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`searchFieldBackgroundPositionAdjustment`][Self::searchFieldBackgroundPositionAdjustment].
        #[method(setSearchFieldBackgroundPositionAdjustment:)]
        pub unsafe fn setSearchFieldBackgroundPositionAdjustment(
            &self,
            search_field_background_position_adjustment: UIOffset,
        );

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(searchTextPositionAdjustment)]
        pub unsafe fn searchTextPositionAdjustment(&self) -> UIOffset;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`searchTextPositionAdjustment`][Self::searchTextPositionAdjustment].
        #[method(setSearchTextPositionAdjustment:)]
        pub unsafe fn setSearchTextPositionAdjustment(
            &self,
            search_text_position_adjustment: UIOffset,
        );

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(setPositionAdjustment:forSearchBarIcon:)]
        pub unsafe fn setPositionAdjustment_forSearchBarIcon(
            &self,
            adjustment: UIOffset,
            icon: UISearchBarIcon,
        );

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(positionAdjustmentForSearchBarIcon:)]
        pub unsafe fn positionAdjustmentForSearchBarIcon(&self, icon: UISearchBarIcon) -> UIOffset;

        #[method(isLookToDictateEnabled)]
        pub unsafe fn isLookToDictateEnabled(&self) -> bool;

        /// Setter for [`isLookToDictateEnabled`][Self::isLookToDictateEnabled].
        #[method(setLookToDictateEnabled:)]
        pub unsafe fn setLookToDictateEnabled(&self, look_to_dictate_enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UISearchBar {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchbardelegate?language=objc)
    #[cfg(feature = "UIBarCommon")]
    pub unsafe trait UISearchBarDelegate: UIBarPositioningDelegate + MainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBarShouldBeginEditing:)]
        unsafe fn searchBarShouldBeginEditing(&self, search_bar: &UISearchBar) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBarTextDidBeginEditing:)]
        unsafe fn searchBarTextDidBeginEditing(&self, search_bar: &UISearchBar);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBarShouldEndEditing:)]
        unsafe fn searchBarShouldEndEditing(&self, search_bar: &UISearchBar) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBarTextDidEndEditing:)]
        unsafe fn searchBarTextDidEndEditing(&self, search_bar: &UISearchBar);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBar:textDidChange:)]
        unsafe fn searchBar_textDidChange(&self, search_bar: &UISearchBar, search_text: &NSString);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBar:shouldChangeTextInRange:replacementText:)]
        unsafe fn searchBar_shouldChangeTextInRange_replacementText(
            &self,
            search_bar: &UISearchBar,
            range: NSRange,
            text: &NSString,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBarSearchButtonClicked:)]
        unsafe fn searchBarSearchButtonClicked(&self, search_bar: &UISearchBar);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBarBookmarkButtonClicked:)]
        unsafe fn searchBarBookmarkButtonClicked(&self, search_bar: &UISearchBar);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBarCancelButtonClicked:)]
        unsafe fn searchBarCancelButtonClicked(&self, search_bar: &UISearchBar);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBarResultsListButtonClicked:)]
        unsafe fn searchBarResultsListButtonClicked(&self, search_bar: &UISearchBar);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(searchBar:selectedScopeButtonIndexDidChange:)]
        unsafe fn searchBar_selectedScopeButtonIndexDidChange(
            &self,
            search_bar: &UISearchBar,
            selected_scope: NSInteger,
        );
    }

    #[cfg(feature = "UIBarCommon")]
    unsafe impl ProtocolType for dyn UISearchBarDelegate {}
);
