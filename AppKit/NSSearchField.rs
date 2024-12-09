//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssearchfieldrecentsautosavename?language=objc)
pub type NSSearchFieldRecentsAutosaveName = NSString;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssearchfielddelegate?language=objc)
    #[cfg(all(feature = "NSControl", feature = "NSTextField"))]
    pub unsafe trait NSSearchFieldDelegate: NSTextFieldDelegate + MainThreadOnly {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(searchFieldDidStartSearching:)]
        unsafe fn searchFieldDidStartSearching(&self, sender: &NSSearchField);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(searchFieldDidEndSearching:)]
        unsafe fn searchFieldDidEndSearching(&self, sender: &NSSearchField);
    }

    #[cfg(all(feature = "NSControl", feature = "NSTextField"))]
    unsafe impl ProtocolType for dyn NSSearchFieldDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssearchfield?language=objc)
    #[unsafe(super(NSTextField, NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    pub struct NSSearchField;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSSearchField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSSearchField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityNavigableStaticText for NSSearchField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityStaticText for NSSearchField {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSSearchField {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSSearchField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSCoding for NSSearchField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSSearchField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSObjectProtocol for NSSearchField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextContent",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSTextContent for NSSearchField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSearchField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSUserInterfaceValidation",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceValidations for NSSearchField {}

extern_methods!(
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSearchField {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(searchTextBounds)]
        pub unsafe fn searchTextBounds(&self) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(searchButtonBounds)]
        pub unsafe fn searchButtonBounds(&self) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(cancelButtonBounds)]
        pub unsafe fn cancelButtonBounds(&self) -> NSRect;

        #[method_id(@__retain_semantics Other recentSearches)]
        pub unsafe fn recentSearches(&self) -> Retained<NSArray<NSString>>;

        #[method(setRecentSearches:)]
        pub unsafe fn setRecentSearches(&self, recent_searches: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other recentsAutosaveName)]
        pub unsafe fn recentsAutosaveName(
            &self,
        ) -> Option<Retained<NSSearchFieldRecentsAutosaveName>>;

        #[method(setRecentsAutosaveName:)]
        pub unsafe fn setRecentsAutosaveName(
            &self,
            recents_autosave_name: Option<&NSSearchFieldRecentsAutosaveName>,
        );

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other searchMenuTemplate)]
        pub unsafe fn searchMenuTemplate(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setSearchMenuTemplate:)]
        pub unsafe fn setSearchMenuTemplate(&self, search_menu_template: Option<&NSMenu>);

        #[method(sendsWholeSearchString)]
        pub unsafe fn sendsWholeSearchString(&self) -> bool;

        #[method(setSendsWholeSearchString:)]
        pub unsafe fn setSendsWholeSearchString(&self, sends_whole_search_string: bool);

        #[method(maximumRecents)]
        pub unsafe fn maximumRecents(&self) -> NSInteger;

        #[method(setMaximumRecents:)]
        pub unsafe fn setMaximumRecents(&self, maximum_recents: NSInteger);

        #[method(sendsSearchStringImmediately)]
        pub unsafe fn sendsSearchStringImmediately(&self) -> bool;

        #[method(setSendsSearchStringImmediately:)]
        pub unsafe fn setSendsSearchStringImmediately(&self, sends_search_string_immediately: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSSearchFieldDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSearchFieldDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSearchField {
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
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSearchField {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSearchField {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSSearchField_Deprecated
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSSearchField {
        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated]
        #[method(rectForSearchTextWhenCentered:)]
        pub unsafe fn rectForSearchTextWhenCentered(&self, is_centered: bool) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated]
        #[method(rectForSearchButtonWhenCentered:)]
        pub unsafe fn rectForSearchButtonWhenCentered(&self, is_centered: bool) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated]
        #[method(rectForCancelButtonWhenCentered:)]
        pub unsafe fn rectForCancelButtonWhenCentered(&self, is_centered: bool) -> NSRect;

        #[deprecated = "The placeholder centering UI design is no longer available. Setting this property is no-op."]
        #[method(centersPlaceholder)]
        pub unsafe fn centersPlaceholder(&self) -> bool;

        #[deprecated = "The placeholder centering UI design is no longer available. Setting this property is no-op."]
        #[method(setCentersPlaceholder:)]
        pub unsafe fn setCentersPlaceholder(&self, centers_placeholder: bool);
    }
);
