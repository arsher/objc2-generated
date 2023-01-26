//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLAreaElement")]
    #[deprecated]
    pub struct DOMHTMLAreaElement;

    #[cfg(feature = "WebKit_DOMHTMLAreaElement")]
    unsafe impl ClassType for DOMHTMLAreaElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLAreaElement")]
unsafe impl DOMEventTarget for DOMHTMLAreaElement {}

#[cfg(feature = "WebKit_DOMHTMLAreaElement")]
unsafe impl NSObjectProtocol for DOMHTMLAreaElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLAreaElement")]
    unsafe impl DOMHTMLAreaElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alt)]
        pub unsafe fn alt(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlt:)]
        pub unsafe fn setAlt(&self, alt: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other coords)]
        pub unsafe fn coords(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCoords:)]
        pub unsafe fn setCoords(&self, coords: Option<&NSString>);

        #[method(noHref)]
        pub unsafe fn noHref(&self) -> bool;

        #[method(setNoHref:)]
        pub unsafe fn setNoHref(&self, no_href: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setShape:)]
        pub unsafe fn setShape(&self, shape: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other absoluteLinkURL)]
        pub unsafe fn absoluteLinkURL(&self) -> Id<NSURL, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other href)]
        pub unsafe fn href(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHref:)]
        pub unsafe fn setHref(&self, href: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hostname)]
        pub unsafe fn hostname(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other port)]
        pub unsafe fn port(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathname)]
        pub unsafe fn pathname(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other search)]
        pub unsafe fn search(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hashName)]
        pub unsafe fn hashName(&self) -> Id<NSString, Shared>;
    }
);
