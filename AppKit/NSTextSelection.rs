//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionGranularity {
        NSTextSelectionGranularityCharacter = 0,
        NSTextSelectionGranularityWord = 1,
        NSTextSelectionGranularityParagraph = 2,
        NSTextSelectionGranularityLine = 3,
        NSTextSelectionGranularitySentence = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionAffinity {
        NSTextSelectionAffinityUpstream = 0,
        NSTextSelectionAffinityDownstream = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextSelection;

    unsafe impl ClassType for NSTextSelection {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextSelection")]
    unsafe impl NSTextSelection {
        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithRanges:affinity:granularity:)]
        pub unsafe fn initWithRanges_affinity_granularity(
            this: Option<Allocated<Self>>,
            textRanges: &Foundation::NSArray<AppKit::NSTextRange>,
            affinity: NSTextSelectionAffinity,
            granularity: NSTextSelectionGranularity,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Init initWithRange:affinity:granularity:)]
        pub unsafe fn initWithRange_affinity_granularity(
            this: Option<Allocated<Self>>,
            range: &AppKit::NSTextRange,
            affinity: NSTextSelectionAffinity,
            granularity: NSTextSelectionGranularity,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLocation:affinity:)]
        pub unsafe fn initWithLocation_affinity(
            this: Option<Allocated<Self>>,
            location: &AppKit::NSTextLocation,
            affinity: NSTextSelectionAffinity,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textRanges)]
        pub unsafe fn textRanges(&self) -> Id<Foundation::NSArray<AppKit::NSTextRange>, Shared>;

        #[method(granularity)]
        pub unsafe fn granularity(&self) -> NSTextSelectionGranularity;

        #[method(affinity)]
        pub unsafe fn affinity(&self) -> NSTextSelectionAffinity;

        #[method(isTransient)]
        pub unsafe fn isTransient(&self) -> bool;

        #[method(anchorPositionOffset)]
        pub unsafe fn anchorPositionOffset(&self) -> CGFloat;

        #[method(setAnchorPositionOffset:)]
        pub unsafe fn setAnchorPositionOffset(&self, anchorPositionOffset: CGFloat);

        #[method(isLogical)]
        pub unsafe fn isLogical(&self) -> bool;

        #[method(setLogical:)]
        pub unsafe fn setLogical(&self, logical: bool);

        #[method_id(@__retain_semantics Other secondarySelectionLocation)]
        pub unsafe fn secondarySelectionLocation(
            &self,
        ) -> Option<Id<AppKit::NSTextLocation, Shared>>;

        #[method(setSecondarySelectionLocation:)]
        pub unsafe fn setSecondarySelectionLocation(
            &self,
            secondarySelectionLocation: Option<&AppKit::NSTextLocation>,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other typingAttributes)]
        pub unsafe fn typingAttributes(
            &self,
        ) -> Id<Foundation::NSDictionary<Foundation::NSAttributedStringKey, Object>, Shared>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setTypingAttributes:)]
        pub unsafe fn setTypingAttributes(
            &self,
            typingAttributes: &Foundation::NSDictionary<Foundation::NSAttributedStringKey, Object>,
        );

        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textSelectionWithTextRanges:)]
        pub unsafe fn textSelectionWithTextRanges(
            &self,
            textRanges: &Foundation::NSArray<AppKit::NSTextRange>,
        ) -> Id<AppKit::NSTextSelection, Shared>;
    }
);
