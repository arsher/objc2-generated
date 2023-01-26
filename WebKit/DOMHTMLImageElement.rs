//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLImageElement")]
    #[deprecated]
    pub struct DOMHTMLImageElement;

    #[cfg(feature = "WebKit_DOMHTMLImageElement")]
    unsafe impl ClassType for DOMHTMLImageElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLImageElement")]
unsafe impl DOMEventTarget for DOMHTMLImageElement {}

#[cfg(feature = "WebKit_DOMHTMLImageElement")]
unsafe impl NSObjectProtocol for DOMHTMLImageElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLImageElement")]
    unsafe impl DOMHTMLImageElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alt)]
        pub unsafe fn alt(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlt:)]
        pub unsafe fn setAlt(&self, alt: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other border)]
        pub unsafe fn border(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBorder:)]
        pub unsafe fn setBorder(&self, border: Option<&NSString>);

        #[method(height)]
        pub unsafe fn height(&self) -> c_int;

        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: c_int);

        #[method(hspace)]
        pub unsafe fn hspace(&self) -> c_int;

        #[method(setHspace:)]
        pub unsafe fn setHspace(&self, hspace: c_int);

        #[method(isMap)]
        pub unsafe fn isMap(&self) -> bool;

        #[method(setIsMap:)]
        pub unsafe fn setIsMap(&self, is_map: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other longDesc)]
        pub unsafe fn longDesc(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLongDesc:)]
        pub unsafe fn setLongDesc(&self, long_desc: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other src)]
        pub unsafe fn src(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSrc:)]
        pub unsafe fn setSrc(&self, src: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other useMap)]
        pub unsafe fn useMap(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUseMap:)]
        pub unsafe fn setUseMap(&self, use_map: Option<&NSString>);

        #[method(vspace)]
        pub unsafe fn vspace(&self) -> c_int;

        #[method(setVspace:)]
        pub unsafe fn setVspace(&self, vspace: c_int);

        #[method(width)]
        pub unsafe fn width(&self) -> c_int;

        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: c_int);

        #[method(complete)]
        pub unsafe fn complete(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lowsrc)]
        pub unsafe fn lowsrc(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLowsrc:)]
        pub unsafe fn setLowsrc(&self, lowsrc: Option<&NSString>);

        #[method(naturalHeight)]
        pub unsafe fn naturalHeight(&self) -> c_int;

        #[method(naturalWidth)]
        pub unsafe fn naturalWidth(&self) -> c_int;

        #[method(x)]
        pub unsafe fn x(&self) -> c_int;

        #[method(y)]
        pub unsafe fn y(&self) -> c_int;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other altDisplayString)]
        pub unsafe fn altDisplayString(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other absoluteImageURL)]
        pub unsafe fn absoluteImageURL(&self) -> Id<NSURL, Shared>;
    }
);
