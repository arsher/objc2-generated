//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMObject",
        feature = "DOMStyleSheet",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMCSSStyleSheet;

    #[cfg(all(
        feature = "DOMObject",
        feature = "DOMStyleSheet",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMCSSStyleSheet {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMStyleSheet;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMObject",
    feature = "DOMStyleSheet",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMCSSStyleSheet {}

#[cfg(all(
    feature = "DOMObject",
    feature = "DOMStyleSheet",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMCSSStyleSheet {}

extern_methods!(
    #[cfg(all(
        feature = "DOMObject",
        feature = "DOMStyleSheet",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSStyleSheet {
        #[cfg(feature = "DOMCSSRule")]
        #[deprecated]
        #[method_id(@__retain_semantics Other ownerRule)]
        pub unsafe fn ownerRule(&self) -> Option<Retained<DOMCSSRule>>;

        #[cfg(feature = "DOMCSSRuleList")]
        #[deprecated]
        #[method_id(@__retain_semantics Other cssRules)]
        pub unsafe fn cssRules(&self) -> Option<Retained<DOMCSSRuleList>>;

        #[cfg(feature = "DOMCSSRuleList")]
        #[method_id(@__retain_semantics Other rules)]
        pub unsafe fn rules(&self) -> Option<Retained<DOMCSSRuleList>>;

        #[method(insertRule:index:)]
        pub unsafe fn insertRule_index(&self, rule: Option<&NSString>, index: c_uint) -> c_uint;

        #[deprecated]
        #[method(deleteRule:)]
        pub unsafe fn deleteRule(&self, index: c_uint);

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
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMObject",
        feature = "DOMStyleSheet",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSStyleSheet {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMObject",
        feature = "DOMStyleSheet",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSStyleSheet {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMCSSStyleSheetDeprecated
    #[cfg(all(
        feature = "DOMObject",
        feature = "DOMStyleSheet",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSStyleSheet {
        #[deprecated]
        #[method(insertRule::)]
        pub unsafe fn insertRule(&self, rule: Option<&NSString>, index: c_uint) -> c_uint;
    }
);
