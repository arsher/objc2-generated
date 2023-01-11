//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextInputTraitType {
        NSTextInputTraitTypeDefault = 0,
        NSTextInputTraitTypeNo = 1,
        NSTextInputTraitTypeYes = 2,
    }
);

extern_protocol!(
    pub struct NSTextInputTraits;

    unsafe impl ProtocolType for NSTextInputTraits {
        #[optional]
        #[method(autocorrectionType)]
        pub unsafe fn autocorrectionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setAutocorrectionType:)]
        pub unsafe fn setAutocorrectionType(&self, autocorrectionType: NSTextInputTraitType);

        #[optional]
        #[method(spellCheckingType)]
        pub unsafe fn spellCheckingType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setSpellCheckingType:)]
        pub unsafe fn setSpellCheckingType(&self, spellCheckingType: NSTextInputTraitType);

        #[optional]
        #[method(grammarCheckingType)]
        pub unsafe fn grammarCheckingType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setGrammarCheckingType:)]
        pub unsafe fn setGrammarCheckingType(&self, grammarCheckingType: NSTextInputTraitType);

        #[optional]
        #[method(smartQuotesType)]
        pub unsafe fn smartQuotesType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setSmartQuotesType:)]
        pub unsafe fn setSmartQuotesType(&self, smartQuotesType: NSTextInputTraitType);

        #[optional]
        #[method(smartDashesType)]
        pub unsafe fn smartDashesType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setSmartDashesType:)]
        pub unsafe fn setSmartDashesType(&self, smartDashesType: NSTextInputTraitType);

        #[optional]
        #[method(smartInsertDeleteType)]
        pub unsafe fn smartInsertDeleteType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setSmartInsertDeleteType:)]
        pub unsafe fn setSmartInsertDeleteType(&self, smartInsertDeleteType: NSTextInputTraitType);

        #[optional]
        #[method(textReplacementType)]
        pub unsafe fn textReplacementType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setTextReplacementType:)]
        pub unsafe fn setTextReplacementType(&self, textReplacementType: NSTextInputTraitType);

        #[optional]
        #[method(dataDetectionType)]
        pub unsafe fn dataDetectionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setDataDetectionType:)]
        pub unsafe fn setDataDetectionType(&self, dataDetectionType: NSTextInputTraitType);

        #[optional]
        #[method(linkDetectionType)]
        pub unsafe fn linkDetectionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setLinkDetectionType:)]
        pub unsafe fn setLinkDetectionType(&self, linkDetectionType: NSTextInputTraitType);

        #[optional]
        #[method(textCompletionType)]
        pub unsafe fn textCompletionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setTextCompletionType:)]
        pub unsafe fn setTextCompletionType(&self, textCompletionType: NSTextInputTraitType);
    }
);

extern_protocol!(
    pub struct NSTextCheckingClient;

    unsafe impl ProtocolType for NSTextCheckingClient {
        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other annotatedSubstringForProposedRange:actualRange:)]
        pub unsafe fn annotatedSubstringForProposedRange_actualRange(
            &self,
            range: NSRange,
            actualRange: NSRangePointer,
        ) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setAnnotations:range:)]
        pub unsafe fn setAnnotations_range(
            &self,
            annotations: &NSDictionary<NSAttributedStringKey, NSString>,
            range: NSRange,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(addAnnotations:range:)]
        pub unsafe fn addAnnotations_range(
            &self,
            annotations: &NSDictionary<NSAttributedStringKey, NSString>,
            range: NSRange,
        );

        #[method(removeAnnotation:range:)]
        pub unsafe fn removeAnnotation_range(
            &self,
            annotationName: &NSAttributedStringKey,
            range: NSRange,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(replaceCharactersInRange:withAnnotatedString:)]
        pub unsafe fn replaceCharactersInRange_withAnnotatedString(
            &self,
            range: NSRange,
            annotatedString: &NSAttributedString,
        );

        #[method(selectAndShowRange:)]
        pub unsafe fn selectAndShowRange(&self, range: NSRange);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other viewForRange:firstRect:actualRange:)]
        pub unsafe fn viewForRange_firstRect_actualRange(
            &self,
            range: NSRange,
            firstRect: NSRectPointer,
            actualRange: NSRangePointer,
        ) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
        #[method_id(@__retain_semantics Other candidateListTouchBarItem)]
        pub unsafe fn candidateListTouchBarItem(
            &self,
        ) -> Option<Id<NSCandidateListTouchBarItem, Shared>>;
    }
);
