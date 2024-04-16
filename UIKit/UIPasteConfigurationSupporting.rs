//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UIPasteConfigurationSupporting:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(feature = "UIPasteConfiguration")]
        #[method_id(@__retain_semantics Other pasteConfiguration)]
        unsafe fn pasteConfiguration(&self) -> Option<Id<UIPasteConfiguration>>;

        #[cfg(feature = "UIPasteConfiguration")]
        #[method(setPasteConfiguration:)]
        unsafe fn setPasteConfiguration(&self, paste_configuration: Option<&UIPasteConfiguration>);

        #[optional]
        #[method(pasteItemProviders:)]
        unsafe fn pasteItemProviders(&self, item_providers: &NSArray<NSItemProvider>);

        #[optional]
        #[method(canPasteItemProviders:)]
        unsafe fn canPasteItemProviders(&self, item_providers: &NSArray<NSItemProvider>) -> bool;
    }

    unsafe impl ProtocolType for dyn UIPasteConfigurationSupporting {}
);
