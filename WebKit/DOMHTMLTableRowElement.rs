//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLTableRowElement")]
    #[deprecated]
    pub struct DOMHTMLTableRowElement;

    #[cfg(feature = "WebKit_DOMHTMLTableRowElement")]
    unsafe impl ClassType for DOMHTMLTableRowElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLTableRowElement")]
unsafe impl DOMEventTarget for DOMHTMLTableRowElement {}

#[cfg(feature = "WebKit_DOMHTMLTableRowElement")]
unsafe impl NSObjectProtocol for DOMHTMLTableRowElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLTableRowElement")]
    unsafe impl DOMHTMLTableRowElement {
        #[method(rowIndex)]
        pub unsafe fn rowIndex(&self) -> c_int;

        #[method(sectionRowIndex)]
        pub unsafe fn sectionRowIndex(&self) -> c_int;

        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[method_id(@__retain_semantics Other cells)]
        pub unsafe fn cells(&self) -> Option<Id<DOMHTMLCollection, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bgColor)]
        pub unsafe fn bgColor(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBgColor:)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ch)]
        pub unsafe fn ch(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCh:)]
        pub unsafe fn setCh(&self, ch: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other chOff)]
        pub unsafe fn chOff(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setChOff:)]
        pub unsafe fn setChOff(&self, ch_off: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other vAlign)]
        pub unsafe fn vAlign(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setVAlign:)]
        pub unsafe fn setVAlign(&self, v_align: Option<&NSString>);

        #[method_id(@__retain_semantics Other insertCell:)]
        pub unsafe fn insertCell(&self, index: c_int) -> Option<Id<DOMHTMLElement, Shared>>;

        #[method(deleteCell:)]
        pub unsafe fn deleteCell(&self, index: c_int);
    }
);
