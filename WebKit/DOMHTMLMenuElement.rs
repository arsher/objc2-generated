//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLMenuElement")]
    #[deprecated]
    pub struct DOMHTMLMenuElement;

    #[cfg(feature = "WebKit_DOMHTMLMenuElement")]
    unsafe impl ClassType for DOMHTMLMenuElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLMenuElement")]
unsafe impl DOMEventTarget for DOMHTMLMenuElement {}

#[cfg(feature = "WebKit_DOMHTMLMenuElement")]
unsafe impl NSObjectProtocol for DOMHTMLMenuElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLMenuElement")]
    unsafe impl DOMHTMLMenuElement {
        #[method(compact)]
        pub unsafe fn compact(&self) -> bool;

        #[method(setCompact:)]
        pub unsafe fn setCompact(&self, compact: bool);
    }
);
