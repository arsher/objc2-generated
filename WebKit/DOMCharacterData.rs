//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domcharacterdata?language=objc)
    #[unsafe(super(DOMNode, DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMCharacterData;
);

#[cfg(all(
    feature = "DOMEventTarget",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMCharacterData {}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMCharacterData {}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMCharacterData {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMCharacterData {}

extern_methods!(
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCharacterData {
        #[deprecated]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSString>;

        /// Setter for [`data`][Self::data].
        #[deprecated]
        #[method(setData:)]
        pub unsafe fn setData(&self, data: Option<&NSString>);

        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[method_id(@__retain_semantics Other substringData:length:)]
        pub unsafe fn substringData_length(
            &self,
            offset: c_uint,
            length: c_uint,
        ) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method(appendData:)]
        pub unsafe fn appendData(&self, data: Option<&NSString>);

        #[method(insertData:data:)]
        pub unsafe fn insertData_data(&self, offset: c_uint, data: Option<&NSString>);

        #[method(deleteData:length:)]
        pub unsafe fn deleteData_length(&self, offset: c_uint, length: c_uint);

        #[method(replaceData:length:data:)]
        pub unsafe fn replaceData_length_data(
            &self,
            offset: c_uint,
            length: c_uint,
            data: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCharacterData {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCharacterData {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMCharacterDataDeprecated
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCharacterData {
        #[deprecated]
        #[method_id(@__retain_semantics Other substringData::)]
        pub unsafe fn substringData(
            &self,
            offset: c_uint,
            length: c_uint,
        ) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method(insertData::)]
        pub unsafe fn insertData(&self, offset: c_uint, data: Option<&NSString>);

        #[deprecated]
        #[method(deleteData::)]
        pub unsafe fn deleteData(&self, offset: c_uint, length: c_uint);

        #[deprecated]
        #[method(replaceData:::)]
        pub unsafe fn replaceData(&self, offset: c_uint, length: c_uint, data: Option<&NSString>);
    }
);
