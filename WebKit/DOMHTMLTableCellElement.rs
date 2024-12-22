//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domhtmltablecellelement?language=objc)
    #[unsafe(super(
        DOMHTMLElement,
        DOMElement,
        DOMNode,
        DOMObject,
        WebScriptObject,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLTableCellElement;
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLTableCellElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLTableCellElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLTableCellElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLTableCellElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLTableCellElement {
        #[deprecated]
        #[method(cellIndex)]
        pub unsafe fn cellIndex(&self) -> c_int;

        #[deprecated]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Retained<NSString>;

        /// Setter for [`align`][Self::align].
        #[deprecated]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other axis)]
        pub unsafe fn axis(&self) -> Retained<NSString>;

        /// Setter for [`axis`][Self::axis].
        #[deprecated]
        #[method(setAxis:)]
        pub unsafe fn setAxis(&self, axis: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other bgColor)]
        pub unsafe fn bgColor(&self) -> Retained<NSString>;

        /// Setter for [`bgColor`][Self::bgColor].
        #[deprecated]
        #[method(setBgColor:)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other ch)]
        pub unsafe fn ch(&self) -> Retained<NSString>;

        /// Setter for [`ch`][Self::ch].
        #[deprecated]
        #[method(setCh:)]
        pub unsafe fn setCh(&self, ch: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other chOff)]
        pub unsafe fn chOff(&self) -> Retained<NSString>;

        /// Setter for [`chOff`][Self::chOff].
        #[deprecated]
        #[method(setChOff:)]
        pub unsafe fn setChOff(&self, ch_off: Option<&NSString>);

        #[deprecated]
        #[method(colSpan)]
        pub unsafe fn colSpan(&self) -> c_int;

        /// Setter for [`colSpan`][Self::colSpan].
        #[deprecated]
        #[method(setColSpan:)]
        pub unsafe fn setColSpan(&self, col_span: c_int);

        #[deprecated]
        #[method(rowSpan)]
        pub unsafe fn rowSpan(&self) -> c_int;

        /// Setter for [`rowSpan`][Self::rowSpan].
        #[deprecated]
        #[method(setRowSpan:)]
        pub unsafe fn setRowSpan(&self, row_span: c_int);

        #[deprecated]
        #[method_id(@__retain_semantics Other headers)]
        pub unsafe fn headers(&self) -> Retained<NSString>;

        /// Setter for [`headers`][Self::headers].
        #[deprecated]
        #[method(setHeaders:)]
        pub unsafe fn setHeaders(&self, headers: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other height)]
        pub unsafe fn height(&self) -> Retained<NSString>;

        /// Setter for [`height`][Self::height].
        #[deprecated]
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: Option<&NSString>);

        #[deprecated]
        #[method(noWrap)]
        pub unsafe fn noWrap(&self) -> bool;

        /// Setter for [`noWrap`][Self::noWrap].
        #[deprecated]
        #[method(setNoWrap:)]
        pub unsafe fn setNoWrap(&self, no_wrap: bool);

        #[deprecated]
        #[method_id(@__retain_semantics Other vAlign)]
        pub unsafe fn vAlign(&self) -> Retained<NSString>;

        /// Setter for [`vAlign`][Self::vAlign].
        #[deprecated]
        #[method(setVAlign:)]
        pub unsafe fn setVAlign(&self, v_align: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Retained<NSString>;

        /// Setter for [`width`][Self::width].
        #[deprecated]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other abbr)]
        pub unsafe fn abbr(&self) -> Retained<NSString>;

        /// Setter for [`abbr`][Self::abbr].
        #[deprecated]
        #[method(setAbbr:)]
        pub unsafe fn setAbbr(&self, abbr: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other scope)]
        pub unsafe fn scope(&self) -> Retained<NSString>;

        /// Setter for [`scope`][Self::scope].
        #[deprecated]
        #[method(setScope:)]
        pub unsafe fn setScope(&self, scope: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLTableCellElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLTableCellElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
