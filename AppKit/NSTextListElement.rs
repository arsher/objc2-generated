//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextListElement")]
    pub struct NSTextListElement;

    #[cfg(feature = "AppKit_NSTextListElement")]
    unsafe impl ClassType for NSTextListElement {
        #[inherits(NSTextElement, NSObject)]
        type Super = NSTextParagraph;
    }
);

#[cfg(feature = "AppKit_NSTextListElement")]
unsafe impl NSObjectProtocol for NSTextListElement {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextListElement")]
    unsafe impl NSTextListElement {
        #[cfg(all(
            feature = "AppKit_NSTextList",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Init initWithParentElement:textList:contents:markerAttributes:childElements:)]
        pub unsafe fn initWithParentElement_textList_contents_markerAttributes_childElements(
            this: Option<Allocated<Self>>,
            parent: Option<&NSTextListElement>,
            text_list: &NSTextList,
            contents: Option<&NSAttributedString>,
            marker_attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
            children: Option<&NSArray<NSTextListElement>>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            this: Option<Allocated<Self>>,
            attributed_string: Option<&NSAttributedString>,
        ) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "AppKit_NSTextList",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other textListElementWithContents:markerAttributes:textList:childElements:)]
        pub unsafe fn textListElementWithContents_markerAttributes_textList_childElements(
            contents: &NSAttributedString,
            marker_attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
            text_list: &NSTextList,
            children: Option<&NSArray<NSTextListElement>>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSTextList", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textListElementWithChildElements:textList:nestingLevel:)]
        pub unsafe fn textListElementWithChildElements_textList_nestingLevel(
            children: &NSArray<NSTextListElement>,
            text_list: &NSTextList,
            nesting_level: NSInteger,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "AppKit_NSTextList")]
        #[method_id(@__retain_semantics Other textList)]
        pub unsafe fn textList(&self) -> Id<NSTextList, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other markerAttributes)]
        pub unsafe fn markerAttributes(
            &self,
        ) -> Option<Id<NSDictionary<NSAttributedStringKey, Object>, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other childElements)]
        pub unsafe fn childElements(&self) -> Id<NSArray<NSTextListElement>, Shared>;

        #[method_id(@__retain_semantics Other parentElement)]
        pub unsafe fn parentElement(&self) -> Option<Id<NSTextListElement, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextElement`
    #[cfg(feature = "AppKit_NSTextListElement")]
    unsafe impl NSTextListElement {
        #[cfg(feature = "AppKit_NSTextContentManager")]
        #[method_id(@__retain_semantics Init initWithTextContentManager:)]
        pub unsafe fn initWithTextContentManager(
            this: Option<Allocated<Self>>,
            text_content_manager: Option<&NSTextContentManager>,
        ) -> Id<Self, Shared>;
    }
);
