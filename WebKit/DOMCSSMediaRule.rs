//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSMediaRule")]
    #[deprecated]
    pub struct DOMCSSMediaRule;

    #[cfg(feature = "WebKit_DOMCSSMediaRule")]
    unsafe impl ClassType for DOMCSSMediaRule {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSRule;
    }
);

#[cfg(feature = "WebKit_DOMCSSMediaRule")]
unsafe impl NSObjectProtocol for DOMCSSMediaRule {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSMediaRule")]
    unsafe impl DOMCSSMediaRule {
        #[cfg(feature = "WebKit_DOMMediaList")]
        #[method_id(@__retain_semantics Other media)]
        pub unsafe fn media(&self) -> Option<Id<DOMMediaList, Shared>>;

        #[cfg(feature = "WebKit_DOMCSSRuleList")]
        #[method_id(@__retain_semantics Other cssRules)]
        pub unsafe fn cssRules(&self) -> Option<Id<DOMCSSRuleList, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(insertRule:index:)]
        pub unsafe fn insertRule_index(&self, rule: Option<&NSString>, index: c_uint) -> c_uint;

        #[method(deleteRule:)]
        pub unsafe fn deleteRule(&self, index: c_uint);
    }
);

extern_methods!(
    /// DOMCSSMediaRuleDeprecated
    #[cfg(feature = "WebKit_DOMCSSMediaRule")]
    unsafe impl DOMCSSMediaRule {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(insertRule::)]
        pub unsafe fn insertRule(&self, rule: Option<&NSString>, index: c_uint) -> c_uint;
    }
);
