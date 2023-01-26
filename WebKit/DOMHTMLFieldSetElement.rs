//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLFieldSetElement")]
    #[deprecated]
    pub struct DOMHTMLFieldSetElement;

    #[cfg(feature = "WebKit_DOMHTMLFieldSetElement")]
    unsafe impl ClassType for DOMHTMLFieldSetElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLFieldSetElement")]
unsafe impl DOMEventTarget for DOMHTMLFieldSetElement {}

#[cfg(feature = "WebKit_DOMHTMLFieldSetElement")]
unsafe impl NSObjectProtocol for DOMHTMLFieldSetElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLFieldSetElement")]
    unsafe impl DOMHTMLFieldSetElement {
        #[cfg(feature = "WebKit_DOMHTMLFormElement")]
        #[method_id(@__retain_semantics Other form)]
        pub unsafe fn form(&self) -> Option<Id<DOMHTMLFormElement, Shared>>;
    }
);
