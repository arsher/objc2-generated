//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlayoutmanagersegmenttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutManagerSegmentType(pub NSInteger);
impl NSTextLayoutManagerSegmentType {
    #[doc(alias = "NSTextLayoutManagerSegmentTypeStandard")]
    pub const Standard: Self = Self(0);
    #[doc(alias = "NSTextLayoutManagerSegmentTypeSelection")]
    pub const Selection: Self = Self(1);
    #[doc(alias = "NSTextLayoutManagerSegmentTypeHighlight")]
    pub const Highlight: Self = Self(2);
}

unsafe impl Encode for NSTextLayoutManagerSegmentType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextLayoutManagerSegmentType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlayoutmanagersegmentoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutManagerSegmentOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTextLayoutManagerSegmentOptions: NSUInteger {
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsNone")]
        const None = 0;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsRangeNotRequired")]
        const RangeNotRequired = 1<<0;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsMiddleFragmentsExcluded")]
        const MiddleFragmentsExcluded = 1<<1;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsHeadSegmentExtended")]
        const HeadSegmentExtended = 1<<2;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsTailSegmentExtended")]
        const TailSegmentExtended = 1<<3;
        #[doc(alias = "NSTextLayoutManagerSegmentOptionsUpstreamAffinity")]
        const UpstreamAffinity = 1<<4;
    }
}

unsafe impl Encode for NSTextLayoutManagerSegmentOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextLayoutManagerSegmentOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlayoutmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextLayoutManager;
);

unsafe impl NSCoding for NSTextLayoutManager {}

unsafe impl NSObjectProtocol for NSTextLayoutManager {}

unsafe impl NSSecureCoding for NSTextLayoutManager {}

#[cfg(feature = "NSTextSelectionNavigation")]
unsafe impl NSTextSelectionDataSource for NSTextLayoutManager {}

extern_methods!(
    unsafe impl NSTextLayoutManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLayoutManagerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextLayoutManagerDelegate>>,
        );

        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;

        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, uses_font_leading: bool);

        #[method(limitsLayoutForSuspiciousContents)]
        pub unsafe fn limitsLayoutForSuspiciousContents(&self) -> bool;

        #[method(setLimitsLayoutForSuspiciousContents:)]
        pub unsafe fn setLimitsLayoutForSuspiciousContents(
            &self,
            limits_layout_for_suspicious_contents: bool,
        );

        #[method(usesHyphenation)]
        pub unsafe fn usesHyphenation(&self) -> bool;

        #[method(setUsesHyphenation:)]
        pub unsafe fn setUsesHyphenation(&self, uses_hyphenation: bool);

        #[cfg(feature = "NSTextContentManager")]
        #[method_id(@__retain_semantics Other textContentManager)]
        pub unsafe fn textContentManager(&self) -> Option<Retained<NSTextContentManager>>;

        #[cfg(feature = "NSTextContentManager")]
        #[method(replaceTextContentManager:)]
        pub unsafe fn replaceTextContentManager(&self, text_content_manager: &NSTextContentManager);

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other textContainer)]
        pub unsafe fn textContainer(&self) -> Option<Retained<NSTextContainer>>;

        #[cfg(feature = "NSTextContainer")]
        #[method(setTextContainer:)]
        pub unsafe fn setTextContainer(&self, text_container: Option<&NSTextContainer>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(usageBoundsForTextContainer)]
        pub unsafe fn usageBoundsForTextContainer(&self) -> CGRect;

        #[cfg(feature = "NSTextViewportLayoutController")]
        #[method_id(@__retain_semantics Other textViewportLayoutController)]
        pub unsafe fn textViewportLayoutController(
            &self,
        ) -> Retained<NSTextViewportLayoutController>;

        #[method_id(@__retain_semantics Other layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Retained<NSOperationQueue>>;

        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layout_queue: Option<&NSOperationQueue>);

        #[cfg(feature = "NSTextRange")]
        #[method(ensureLayoutForRange:)]
        pub unsafe fn ensureLayoutForRange(&self, range: &NSTextRange);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(ensureLayoutForBounds:)]
        pub unsafe fn ensureLayoutForBounds(&self, bounds: CGRect);

        #[cfg(feature = "NSTextRange")]
        #[method(invalidateLayoutForRange:)]
        pub unsafe fn invalidateLayoutForRange(&self, range: &NSTextRange);

        #[cfg(all(feature = "NSTextLayoutFragment", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Other textLayoutFragmentForPosition:)]
        pub unsafe fn textLayoutFragmentForPosition(
            &self,
            position: CGPoint,
        ) -> Option<Retained<NSTextLayoutFragment>>;

        #[cfg(all(feature = "NSTextLayoutFragment", feature = "NSTextRange"))]
        #[method_id(@__retain_semantics Other textLayoutFragmentForLocation:)]
        pub unsafe fn textLayoutFragmentForLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Retained<NSTextLayoutFragment>>;

        #[cfg(all(
            feature = "NSTextLayoutFragment",
            feature = "NSTextRange",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other enumerateTextLayoutFragmentsFromLocation:options:usingBlock:)]
        pub unsafe fn enumerateTextLayoutFragmentsFromLocation_options_usingBlock(
            &self,
            location: Option<&ProtocolObject<dyn NSTextLocation>>,
            options: NSTextLayoutFragmentEnumerationOptions,
            block: &block2::Block<dyn Fn(NonNull<NSTextLayoutFragment>) -> Bool + '_>,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(feature = "NSTextSelection")]
        #[method_id(@__retain_semantics Other textSelections)]
        pub unsafe fn textSelections(&self) -> Retained<NSArray<NSTextSelection>>;

        #[cfg(feature = "NSTextSelection")]
        #[method(setTextSelections:)]
        pub unsafe fn setTextSelections(&self, text_selections: &NSArray<NSTextSelection>);

        #[cfg(feature = "NSTextSelectionNavigation")]
        #[method_id(@__retain_semantics Other textSelectionNavigation)]
        pub unsafe fn textSelectionNavigation(&self) -> Retained<NSTextSelectionNavigation>;

        #[cfg(feature = "NSTextSelectionNavigation")]
        #[method(setTextSelectionNavigation:)]
        pub unsafe fn setTextSelectionNavigation(
            &self,
            text_selection_navigation: &NSTextSelectionNavigation,
        );

        #[cfg(all(feature = "NSTextRange", feature = "block2"))]
        #[method(enumerateRenderingAttributesFromLocation:reverse:usingBlock:)]
        pub unsafe fn enumerateRenderingAttributesFromLocation_reverse_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            reverse: bool,
            block: &block2::Block<
                dyn Fn(
                        NonNull<NSTextLayoutManager>,
                        NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
                        NonNull<NSTextRange>,
                    ) -> Bool
                    + '_,
            >,
        );

        #[cfg(feature = "NSTextRange")]
        #[method(setRenderingAttributes:forTextRange:)]
        pub unsafe fn setRenderingAttributes_forTextRange(
            &self,
            rendering_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "NSTextRange")]
        #[method(addRenderingAttribute:value:forTextRange:)]
        pub unsafe fn addRenderingAttribute_value_forTextRange(
            &self,
            rendering_attribute: &NSAttributedStringKey,
            value: Option<&AnyObject>,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "NSTextRange")]
        #[method(removeRenderingAttribute:forTextRange:)]
        pub unsafe fn removeRenderingAttribute_forTextRange(
            &self,
            rendering_attribute: &NSAttributedStringKey,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "NSTextRange")]
        #[method(invalidateRenderingAttributesForTextRange:)]
        pub unsafe fn invalidateRenderingAttributesForTextRange(&self, text_range: &NSTextRange);

        #[cfg(all(feature = "NSTextLayoutFragment", feature = "block2"))]
        #[method(renderingAttributesValidator)]
        pub unsafe fn renderingAttributesValidator(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<NSTextLayoutManager>, NonNull<NSTextLayoutFragment>)>;

        #[cfg(all(feature = "NSTextLayoutFragment", feature = "block2"))]
        #[method(setRenderingAttributesValidator:)]
        pub unsafe fn setRenderingAttributesValidator(
            &self,
            rendering_attributes_validator: Option<
                &block2::Block<dyn Fn(NonNull<NSTextLayoutManager>, NonNull<NSTextLayoutFragment>)>,
            >,
        );

        #[method_id(@__retain_semantics Other linkRenderingAttributes)]
        pub unsafe fn linkRenderingAttributes(
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(feature = "NSTextRange")]
        #[method_id(@__retain_semantics Other renderingAttributesForLink:atLocation:)]
        pub unsafe fn renderingAttributesForLink_atLocation(
            &self,
            link: &AnyObject,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(all(
            feature = "NSTextContainer",
            feature = "NSTextRange",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        #[method(enumerateTextSegmentsInRange:type:options:usingBlock:)]
        pub unsafe fn enumerateTextSegmentsInRange_type_options_usingBlock(
            &self,
            text_range: &NSTextRange,
            r#type: NSTextLayoutManagerSegmentType,
            options: NSTextLayoutManagerSegmentOptions,
            block: &block2::Block<
                dyn Fn(*mut NSTextRange, CGRect, CGFloat, NonNull<NSTextContainer>) -> Bool + '_,
            >,
        );

        #[cfg(all(feature = "NSTextElement", feature = "NSTextRange"))]
        #[method(replaceContentsInRange:withTextElements:)]
        pub unsafe fn replaceContentsInRange_withTextElements(
            &self,
            range: &NSTextRange,
            text_elements: &NSArray<NSTextElement>,
        );

        #[cfg(feature = "NSTextRange")]
        #[method(replaceContentsInRange:withAttributedString:)]
        pub unsafe fn replaceContentsInRange_withAttributedString(
            &self,
            range: &NSTextRange,
            attributed_string: &NSAttributedString,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextLayoutManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlayoutmanagerdelegate?language=objc)
    pub unsafe trait NSTextLayoutManagerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "NSTextElement",
            feature = "NSTextLayoutFragment",
            feature = "NSTextRange"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textLayoutManager:textLayoutFragmentForLocation:inTextElement:)]
        unsafe fn textLayoutManager_textLayoutFragmentForLocation_inTextElement(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_element: &NSTextElement,
        ) -> Retained<NSTextLayoutFragment>;

        #[cfg(feature = "NSTextRange")]
        #[optional]
        #[method(textLayoutManager:shouldBreakLineBeforeLocation:hyphenating:)]
        unsafe fn textLayoutManager_shouldBreakLineBeforeLocation_hyphenating(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            location: &ProtocolObject<dyn NSTextLocation>,
            hyphenating: bool,
        ) -> bool;

        #[cfg(feature = "NSTextRange")]
        #[optional]
        #[method_id(@__retain_semantics Other textLayoutManager:renderingAttributesForLink:atLocation:defaultAttributes:)]
        unsafe fn textLayoutManager_renderingAttributesForLink_atLocation_defaultAttributes(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            link: &AnyObject,
            location: &ProtocolObject<dyn NSTextLocation>,
            rendering_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;
    }

    unsafe impl ProtocolType for dyn NSTextLayoutManagerDelegate {}
);
