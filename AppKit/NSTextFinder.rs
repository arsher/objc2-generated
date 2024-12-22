//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfinderaction?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextFinderAction(pub NSInteger);
impl NSTextFinderAction {
    #[doc(alias = "NSTextFinderActionShowFindInterface")]
    pub const ShowFindInterface: Self = Self(1);
    #[doc(alias = "NSTextFinderActionNextMatch")]
    pub const NextMatch: Self = Self(2);
    #[doc(alias = "NSTextFinderActionPreviousMatch")]
    pub const PreviousMatch: Self = Self(3);
    #[doc(alias = "NSTextFinderActionReplaceAll")]
    pub const ReplaceAll: Self = Self(4);
    #[doc(alias = "NSTextFinderActionReplace")]
    pub const Replace: Self = Self(5);
    #[doc(alias = "NSTextFinderActionReplaceAndFind")]
    pub const ReplaceAndFind: Self = Self(6);
    #[doc(alias = "NSTextFinderActionSetSearchString")]
    pub const SetSearchString: Self = Self(7);
    #[doc(alias = "NSTextFinderActionReplaceAllInSelection")]
    pub const ReplaceAllInSelection: Self = Self(8);
    #[doc(alias = "NSTextFinderActionSelectAll")]
    pub const SelectAll: Self = Self(9);
    #[doc(alias = "NSTextFinderActionSelectAllInSelection")]
    pub const SelectAllInSelection: Self = Self(10);
    #[doc(alias = "NSTextFinderActionHideFindInterface")]
    pub const HideFindInterface: Self = Self(11);
    #[doc(alias = "NSTextFinderActionShowReplaceInterface")]
    pub const ShowReplaceInterface: Self = Self(12);
    #[doc(alias = "NSTextFinderActionHideReplaceInterface")]
    pub const HideReplaceInterface: Self = Self(13);
}

unsafe impl Encode for NSTextFinderAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextFinderAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypetextfinderoptionkey?language=objc)
// NS_TYPED_ENUM
pub type NSPasteboardTypeTextFinderOptionKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfindercaseinsensitivekey?language=objc)
    pub static NSTextFinderCaseInsensitiveKey: &'static NSPasteboardTypeTextFinderOptionKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfindermatchingtypekey?language=objc)
    pub static NSTextFinderMatchingTypeKey: &'static NSPasteboardTypeTextFinderOptionKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfindermatchingtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextFinderMatchingType(pub NSInteger);
impl NSTextFinderMatchingType {
    #[doc(alias = "NSTextFinderMatchingTypeContains")]
    pub const Contains: Self = Self(0);
    #[doc(alias = "NSTextFinderMatchingTypeStartsWith")]
    pub const StartsWith: Self = Self(1);
    #[doc(alias = "NSTextFinderMatchingTypeFullWord")]
    pub const FullWord: Self = Self(2);
    #[doc(alias = "NSTextFinderMatchingTypeEndsWith")]
    pub const EndsWith: Self = Self(3);
}

unsafe impl Encode for NSTextFinderMatchingType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextFinderMatchingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfinder?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextFinder;
);

unsafe impl NSCoding for NSTextFinder {}

unsafe impl NSObjectProtocol for NSTextFinder {}

extern_methods!(
    unsafe impl NSTextFinder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Retained<ProtocolObject<dyn NSTextFinderClient>>>;

        /// Setter for [`client`][Self::client].
        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&ProtocolObject<dyn NSTextFinderClient>>);

        #[method(performAction:)]
        pub unsafe fn performAction(&self, op: NSTextFinderAction);

        #[method(validateAction:)]
        pub unsafe fn validateAction(&self, op: NSTextFinderAction) -> bool;

        #[method_id(@__retain_semantics Other findBarContainer)]
        pub unsafe fn findBarContainer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextFinderBarContainer>>>;

        /// Setter for [`findBarContainer`][Self::findBarContainer].
        #[method(setFindBarContainer:)]
        pub unsafe fn setFindBarContainer(
            &self,
            find_bar_container: Option<&ProtocolObject<dyn NSTextFinderBarContainer>>,
        );

        #[method(cancelFindIndicator)]
        pub unsafe fn cancelFindIndicator(&self);

        #[method(findIndicatorNeedsUpdate)]
        pub unsafe fn findIndicatorNeedsUpdate(&self) -> bool;

        /// Setter for [`findIndicatorNeedsUpdate`][Self::findIndicatorNeedsUpdate].
        #[method(setFindIndicatorNeedsUpdate:)]
        pub unsafe fn setFindIndicatorNeedsUpdate(&self, find_indicator_needs_update: bool);

        #[method(isIncrementalSearchingEnabled)]
        pub unsafe fn isIncrementalSearchingEnabled(&self) -> bool;

        /// Setter for [`isIncrementalSearchingEnabled`][Self::isIncrementalSearchingEnabled].
        #[method(setIncrementalSearchingEnabled:)]
        pub unsafe fn setIncrementalSearchingEnabled(&self, incremental_searching_enabled: bool);

        #[method(incrementalSearchingShouldDimContentView)]
        pub unsafe fn incrementalSearchingShouldDimContentView(&self) -> bool;

        /// Setter for [`incrementalSearchingShouldDimContentView`][Self::incrementalSearchingShouldDimContentView].
        #[method(setIncrementalSearchingShouldDimContentView:)]
        pub unsafe fn setIncrementalSearchingShouldDimContentView(
            &self,
            incremental_searching_should_dim_content_view: bool,
        );

        #[method_id(@__retain_semantics Other incrementalMatchRanges)]
        pub unsafe fn incrementalMatchRanges(&self) -> Retained<NSArray<NSValue>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawIncrementalMatchHighlightInRect:)]
        pub unsafe fn drawIncrementalMatchHighlightInRect(rect: NSRect);

        #[method(noteClientStringWillChange)]
        pub unsafe fn noteClientStringWillChange(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextFinder {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfinderclient?language=objc)
    pub unsafe trait NSTextFinderClient: NSObjectProtocol {
        #[optional]
        #[method(isSelectable)]
        unsafe fn isSelectable(&self) -> bool;

        #[optional]
        #[method(allowsMultipleSelection)]
        unsafe fn allowsMultipleSelection(&self) -> bool;

        #[optional]
        #[method(isEditable)]
        unsafe fn isEditable(&self) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other string)]
        unsafe fn string(&self) -> Retained<NSString>;

        #[optional]
        #[method_id(@__retain_semantics Other stringAtIndex:effectiveRange:endsWithSearchBoundary:)]
        unsafe fn stringAtIndex_effectiveRange_endsWithSearchBoundary(
            &self,
            character_index: NSUInteger,
            out_range: NSRangePointer,
            out_flag: NonNull<Bool>,
        ) -> Retained<NSString>;

        #[optional]
        #[method(stringLength)]
        unsafe fn stringLength(&self) -> NSUInteger;

        #[optional]
        #[method(firstSelectedRange)]
        unsafe fn firstSelectedRange(&self) -> NSRange;

        #[optional]
        #[method_id(@__retain_semantics Other selectedRanges)]
        unsafe fn selectedRanges(&self) -> Retained<NSArray<NSValue>>;

        /// Setter for [`selectedRanges`][Self::selectedRanges].
        #[optional]
        #[method(setSelectedRanges:)]
        unsafe fn setSelectedRanges(&self, selected_ranges: &NSArray<NSValue>);

        #[optional]
        #[method(scrollRangeToVisible:)]
        unsafe fn scrollRangeToVisible(&self, range: NSRange);

        #[optional]
        #[method(shouldReplaceCharactersInRanges:withStrings:)]
        unsafe fn shouldReplaceCharactersInRanges_withStrings(
            &self,
            ranges: &NSArray<NSValue>,
            strings: &NSArray<NSString>,
        ) -> bool;

        #[optional]
        #[method(replaceCharactersInRange:withString:)]
        unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, string: &NSString);

        #[optional]
        #[method(didReplaceCharacters)]
        unsafe fn didReplaceCharacters(&self);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method_id(@__retain_semantics Other contentViewAtIndex:effectiveCharacterRange:)]
        unsafe fn contentViewAtIndex_effectiveCharacterRange(
            &self,
            index: NSUInteger,
            out_range: NSRangePointer,
            mtm: MainThreadMarker,
        ) -> Retained<NSView>;

        #[optional]
        #[method_id(@__retain_semantics Other rectsForCharacterRange:)]
        unsafe fn rectsForCharacterRange(
            &self,
            range: NSRange,
        ) -> Option<Retained<NSArray<NSValue>>>;

        #[optional]
        #[method_id(@__retain_semantics Other visibleCharacterRanges)]
        unsafe fn visibleCharacterRanges(&self) -> Retained<NSArray<NSValue>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(drawCharactersInRange:forContentView:)]
        unsafe fn drawCharactersInRange_forContentView(&self, range: NSRange, view: &NSView);
    }

    unsafe impl ProtocolType for dyn NSTextFinderClient {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextfinderbarcontainer?language=objc)
    pub unsafe trait NSTextFinderBarContainer: NSObjectProtocol {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other findBarView)]
        unsafe fn findBarView(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// Setter for [`findBarView`][Self::findBarView].
        #[method(setFindBarView:)]
        unsafe fn setFindBarView(&self, find_bar_view: Option<&NSView>);

        #[method(isFindBarVisible)]
        unsafe fn isFindBarVisible(&self) -> bool;

        /// Setter for [`isFindBarVisible`][Self::isFindBarVisible].
        #[method(setFindBarVisible:)]
        unsafe fn setFindBarVisible(&self, find_bar_visible: bool);

        #[method(findBarViewDidChangeHeight)]
        unsafe fn findBarViewDidChangeHeight(&self);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method_id(@__retain_semantics Other contentView)]
        unsafe fn contentView(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;
    }

    unsafe impl ProtocolType for dyn NSTextFinderBarContainer {}
);
