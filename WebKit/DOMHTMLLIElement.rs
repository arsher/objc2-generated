//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLLIElement")]
    #[deprecated]
    pub struct DOMHTMLLIElement;

    #[cfg(feature = "WebKit_DOMHTMLLIElement")]
    unsafe impl ClassType for DOMHTMLLIElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLLIElement")]
unsafe impl DOMEventTarget for DOMHTMLLIElement {}

#[cfg(feature = "WebKit_DOMHTMLLIElement")]
unsafe impl NSCopying for DOMHTMLLIElement {}

#[cfg(feature = "WebKit_DOMHTMLLIElement")]
unsafe impl NSObjectProtocol for DOMHTMLLIElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLLIElement")]
    unsafe impl DOMHTMLLIElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[method(value)]
        pub unsafe fn value(&self) -> c_int;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_int);
    }
);
