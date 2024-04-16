//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UITextPasteDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(all(
            feature = "UIPasteConfigurationSupporting",
            feature = "UITextPasteConfigurationSupporting"
        ))]
        #[optional]
        #[method(textPasteConfigurationSupporting:transformPasteItem:)]
        unsafe fn textPasteConfigurationSupporting_transformPasteItem(
            &self,
            text_paste_configuration_supporting: &ProtocolObject<
                dyn UITextPasteConfigurationSupporting,
            >,
            item: &ProtocolObject<dyn UITextPasteItem>,
        );

        #[cfg(all(
            feature = "UIPasteConfigurationSupporting",
            feature = "UITextInput",
            feature = "UITextPasteConfigurationSupporting"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textPasteConfigurationSupporting:combineItemAttributedStrings:forRange:)]
        unsafe fn textPasteConfigurationSupporting_combineItemAttributedStrings_forRange(
            &self,
            text_paste_configuration_supporting: &ProtocolObject<
                dyn UITextPasteConfigurationSupporting,
            >,
            item_strings: &NSArray<NSAttributedString>,
            text_range: &UITextRange,
        ) -> Id<NSAttributedString>;

        #[cfg(all(
            feature = "UIPasteConfigurationSupporting",
            feature = "UITextInput",
            feature = "UITextPasteConfigurationSupporting"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textPasteConfigurationSupporting:performPasteOfAttributedString:toRange:)]
        unsafe fn textPasteConfigurationSupporting_performPasteOfAttributedString_toRange(
            &self,
            text_paste_configuration_supporting: &ProtocolObject<
                dyn UITextPasteConfigurationSupporting,
            >,
            attributed_string: &NSAttributedString,
            text_range: &UITextRange,
        ) -> Id<UITextRange>;

        #[cfg(all(
            feature = "UIPasteConfigurationSupporting",
            feature = "UITextInput",
            feature = "UITextPasteConfigurationSupporting"
        ))]
        #[optional]
        #[method(textPasteConfigurationSupporting:shouldAnimatePasteOfAttributedString:toRange:)]
        unsafe fn textPasteConfigurationSupporting_shouldAnimatePasteOfAttributedString_toRange(
            &self,
            text_paste_configuration_supporting: &ProtocolObject<
                dyn UITextPasteConfigurationSupporting,
            >,
            attributed_string: &NSAttributedString,
            text_range: &UITextRange,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UITextPasteDelegate {}
);

extern_protocol!(
    pub unsafe trait UITextPasteItem: NSObjectProtocol + IsMainThreadOnly {
        #[method_id(@__retain_semantics Other itemProvider)]
        unsafe fn itemProvider(&self) -> Id<NSItemProvider>;

        #[method_id(@__retain_semantics Other localObject)]
        unsafe fn localObject(&self) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other defaultAttributes)]
        unsafe fn defaultAttributes(&self) -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method(setStringResult:)]
        unsafe fn setStringResult(&self, string: &NSString);

        #[method(setAttributedStringResult:)]
        unsafe fn setAttributedStringResult(&self, string: &NSAttributedString);

        #[cfg(feature = "NSTextAttachment")]
        #[method(setAttachmentResult:)]
        unsafe fn setAttachmentResult(&self, text_attachment: &NSTextAttachment);

        #[method(setNoResult)]
        unsafe fn setNoResult(&self);

        #[method(setDefaultResult)]
        unsafe fn setDefaultResult(&self);
    }

    unsafe impl ProtocolType for dyn UITextPasteItem {}
);
