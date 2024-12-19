//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domxpathexpression?language=objc)
    #[unsafe(super(DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMXPathExpression;
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMXPathExpression {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMXPathExpression {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMXPathExpression {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMXPathExpression {
        #[cfg(all(feature = "DOMNode", feature = "DOMXPathResult"))]
        #[method_id(@__retain_semantics Other evaluate:type:inResult:)]
        pub unsafe fn evaluate_type_inResult(
            &self,
            context_node: Option<&DOMNode>,
            r#type: c_ushort,
            in_result: Option<&DOMXPathResult>,
        ) -> Option<Retained<DOMXPathResult>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMXPathExpression {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMXPathExpression {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMXPathExpressionDeprecated
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMXPathExpression {
        #[cfg(all(feature = "DOMNode", feature = "DOMXPathResult"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other evaluate:::)]
        pub unsafe fn evaluate(
            &self,
            context_node: Option<&DOMNode>,
            r#type: c_ushort,
            in_result: Option<&DOMXPathResult>,
        ) -> Option<Retained<DOMXPathResult>>;
    }
);
