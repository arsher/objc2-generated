//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLIFrameElement")]
    #[deprecated]
    pub struct DOMHTMLIFrameElement;

    #[cfg(feature = "WebKit_DOMHTMLIFrameElement")]
    unsafe impl ClassType for DOMHTMLIFrameElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLIFrameElement")]
unsafe impl DOMEventTarget for DOMHTMLIFrameElement {}

#[cfg(feature = "WebKit_DOMHTMLIFrameElement")]
unsafe impl NSObjectProtocol for DOMHTMLIFrameElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLIFrameElement")]
    unsafe impl DOMHTMLIFrameElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other frameBorder)]
        pub unsafe fn frameBorder(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFrameBorder:)]
        pub unsafe fn setFrameBorder(&self, frame_border: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other height)]
        pub unsafe fn height(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other longDesc)]
        pub unsafe fn longDesc(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLongDesc:)]
        pub unsafe fn setLongDesc(&self, long_desc: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other marginHeight)]
        pub unsafe fn marginHeight(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMarginHeight:)]
        pub unsafe fn setMarginHeight(&self, margin_height: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other marginWidth)]
        pub unsafe fn marginWidth(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMarginWidth:)]
        pub unsafe fn setMarginWidth(&self, margin_width: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other scrolling)]
        pub unsafe fn scrolling(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setScrolling:)]
        pub unsafe fn setScrolling(&self, scrolling: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other src)]
        pub unsafe fn src(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSrc:)]
        pub unsafe fn setSrc(&self, src: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMDocument")]
        #[method_id(@__retain_semantics Other contentDocument)]
        pub unsafe fn contentDocument(&self) -> Option<Id<DOMDocument, Shared>>;

        #[cfg(feature = "WebKit_DOMAbstractView")]
        #[method_id(@__retain_semantics Other contentWindow)]
        pub unsafe fn contentWindow(&self) -> Option<Id<DOMAbstractView, Shared>>;
    }
);
