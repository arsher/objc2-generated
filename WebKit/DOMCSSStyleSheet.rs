//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSStyleSheet")]
    #[deprecated]
    pub struct DOMCSSStyleSheet;

    #[cfg(feature = "WebKit_DOMCSSStyleSheet")]
    unsafe impl ClassType for DOMCSSStyleSheet {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMStyleSheet;
    }
);

#[cfg(feature = "WebKit_DOMCSSStyleSheet")]
unsafe impl NSObjectProtocol for DOMCSSStyleSheet {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSStyleSheet")]
    unsafe impl DOMCSSStyleSheet {
        #[cfg(feature = "WebKit_DOMCSSRule")]
        #[method_id(@__retain_semantics Other ownerRule)]
        pub unsafe fn ownerRule(&self) -> Option<Id<DOMCSSRule, Shared>>;

        #[cfg(feature = "WebKit_DOMCSSRuleList")]
        #[method_id(@__retain_semantics Other cssRules)]
        pub unsafe fn cssRules(&self) -> Option<Id<DOMCSSRuleList, Shared>>;

        #[cfg(feature = "WebKit_DOMCSSRuleList")]
        #[method_id(@__retain_semantics Other rules)]
        pub unsafe fn rules(&self) -> Option<Id<DOMCSSRuleList, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(insertRule:index:)]
        pub unsafe fn insertRule_index(&self, rule: Option<&NSString>, index: c_uint) -> c_uint;

        #[method(deleteRule:)]
        pub unsafe fn deleteRule(&self, index: c_uint);

        #[cfg(feature = "Foundation_NSString")]
        #[method(addRule:style:index:)]
        pub unsafe fn addRule_style_index(
            &self,
            selector: Option<&NSString>,
            style: Option<&NSString>,
            index: c_uint,
        ) -> c_int;

        #[method(removeRule:)]
        pub unsafe fn removeRule(&self, index: c_uint);
    }
);

extern_methods!(
    /// DOMCSSStyleSheetDeprecated
    #[cfg(feature = "WebKit_DOMCSSStyleSheet")]
    unsafe impl DOMCSSStyleSheet {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(insertRule::)]
        pub unsafe fn insertRule(&self, rule: Option<&NSString>, index: c_uint) -> c_uint;
    }
);
