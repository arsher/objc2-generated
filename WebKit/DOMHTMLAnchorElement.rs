//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLAnchorElement")]
    #[deprecated]
    pub struct DOMHTMLAnchorElement;

    #[cfg(feature = "WebKit_DOMHTMLAnchorElement")]
    unsafe impl ClassType for DOMHTMLAnchorElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLAnchorElement")]
unsafe impl DOMEventTarget for DOMHTMLAnchorElement {}

#[cfg(feature = "WebKit_DOMHTMLAnchorElement")]
unsafe impl NSCopying for DOMHTMLAnchorElement {}

#[cfg(feature = "WebKit_DOMHTMLAnchorElement")]
unsafe impl NSObjectProtocol for DOMHTMLAnchorElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLAnchorElement")]
    unsafe impl DOMHTMLAnchorElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other charset)]
        pub unsafe fn charset(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCharset:)]
        pub unsafe fn setCharset(&self, charset: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other coords)]
        pub unsafe fn coords(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCoords:)]
        pub unsafe fn setCoords(&self, coords: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hreflang)]
        pub unsafe fn hreflang(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHreflang:)]
        pub unsafe fn setHreflang(&self, hreflang: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other rel)]
        pub unsafe fn rel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setRel:)]
        pub unsafe fn setRel(&self, rel: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other rev)]
        pub unsafe fn rev(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setRev:)]
        pub unsafe fn setRev(&self, rev: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setShape:)]
        pub unsafe fn setShape(&self, shape: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other absoluteLinkURL)]
        pub unsafe fn absoluteLinkURL(&self) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other href)]
        pub unsafe fn href(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHref:)]
        pub unsafe fn setHref(&self, href: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hostname)]
        pub unsafe fn hostname(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other port)]
        pub unsafe fn port(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathname)]
        pub unsafe fn pathname(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other search)]
        pub unsafe fn search(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hashName)]
        pub unsafe fn hashName(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLAnchorElement")]
    unsafe impl DOMHTMLAnchorElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLAnchorElement")]
    unsafe impl DOMHTMLAnchorElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
