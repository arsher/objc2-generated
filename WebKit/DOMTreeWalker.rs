//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domtreewalker?language=objc)
    #[unsafe(super(DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMTreeWalker;
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMTreeWalker {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMTreeWalker {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMTreeWalker {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMTreeWalker {
        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other root)]
        pub unsafe fn root(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(whatToShow)]
        pub unsafe fn whatToShow(&self) -> c_uint;

        #[cfg(feature = "DOMNodeFilter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Retained<ProtocolObject<dyn DOMNodeFilter>>>;

        #[deprecated]
        #[method(expandEntityReferences)]
        pub unsafe fn expandEntityReferences(&self) -> bool;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other currentNode)]
        pub unsafe fn currentNode(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method(setCurrentNode:)]
        pub unsafe fn setCurrentNode(&self, current_node: Option<&DOMNode>);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other parentNode)]
        pub unsafe fn parentNode(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other firstChild)]
        pub unsafe fn firstChild(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other lastChild)]
        pub unsafe fn lastChild(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other previousSibling)]
        pub unsafe fn previousSibling(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other nextSibling)]
        pub unsafe fn nextSibling(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other previousNode)]
        pub unsafe fn previousNode(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other nextNode)]
        pub unsafe fn nextNode(&self) -> Option<Retained<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMTreeWalker {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMTreeWalker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
