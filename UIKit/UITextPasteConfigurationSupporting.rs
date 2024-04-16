//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    #[cfg(feature = "UIPasteConfigurationSupporting")]
    pub unsafe trait UITextPasteConfigurationSupporting:
        UIPasteConfigurationSupporting + IsMainThreadOnly
    {
        #[cfg(feature = "UITextPasteDelegate")]
        #[method_id(@__retain_semantics Other pasteDelegate)]
        unsafe fn pasteDelegate(&self) -> Option<Id<ProtocolObject<dyn UITextPasteDelegate>>>;

        #[cfg(feature = "UITextPasteDelegate")]
        #[method(setPasteDelegate:)]
        unsafe fn setPasteDelegate(
            &self,
            paste_delegate: Option<&ProtocolObject<dyn UITextPasteDelegate>>,
        );
    }

    #[cfg(feature = "UIPasteConfigurationSupporting")]
    unsafe impl ProtocolType for dyn UITextPasteConfigurationSupporting {}
);
