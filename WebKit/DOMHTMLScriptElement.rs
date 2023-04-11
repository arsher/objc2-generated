//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLScriptElement")]
    #[deprecated]
    pub struct DOMHTMLScriptElement;

    #[cfg(feature = "WebKit_DOMHTMLScriptElement")]
    unsafe impl ClassType for DOMHTMLScriptElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLScriptElement")]
unsafe impl DOMEventTarget for DOMHTMLScriptElement {}

#[cfg(feature = "WebKit_DOMHTMLScriptElement")]
unsafe impl NSCopying for DOMHTMLScriptElement {}

#[cfg(feature = "WebKit_DOMHTMLScriptElement")]
unsafe impl NSObjectProtocol for DOMHTMLScriptElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLScriptElement")]
    unsafe impl DOMHTMLScriptElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other htmlFor)]
        pub unsafe fn htmlFor(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHtmlFor:)]
        pub unsafe fn setHtmlFor(&self, html_for: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other event)]
        pub unsafe fn event(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setEvent:)]
        pub unsafe fn setEvent(&self, event: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other charset)]
        pub unsafe fn charset(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCharset:)]
        pub unsafe fn setCharset(&self, charset: Option<&NSString>);

        #[method(defer)]
        pub unsafe fn defer(&self) -> bool;

        #[method(setDefer:)]
        pub unsafe fn setDefer(&self, defer: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other src)]
        pub unsafe fn src(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSrc:)]
        pub unsafe fn setSrc(&self, src: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);
    }
);
