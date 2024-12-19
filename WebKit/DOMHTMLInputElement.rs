//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domhtmlinputelement?language=objc)
    #[unsafe(super(
        DOMHTMLElement,
        DOMElement,
        DOMNode,
        DOMObject,
        WebScriptObject,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLInputElement;
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLInputElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLInputElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLInputElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLInputElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLInputElement {
        #[deprecated]
        #[method_id(@__retain_semantics Other accept)]
        pub unsafe fn accept(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAccept:)]
        pub unsafe fn setAccept(&self, accept: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other alt)]
        pub unsafe fn alt(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAlt:)]
        pub unsafe fn setAlt(&self, alt: Option<&NSString>);

        #[method(autofocus)]
        pub unsafe fn autofocus(&self) -> bool;

        #[method(setAutofocus:)]
        pub unsafe fn setAutofocus(&self, autofocus: bool);

        #[deprecated]
        #[method(defaultChecked)]
        pub unsafe fn defaultChecked(&self) -> bool;

        #[deprecated]
        #[method(setDefaultChecked:)]
        pub unsafe fn setDefaultChecked(&self, default_checked: bool);

        #[deprecated]
        #[method(checked)]
        pub unsafe fn checked(&self) -> bool;

        #[deprecated]
        #[method(setChecked:)]
        pub unsafe fn setChecked(&self, checked: bool);

        #[deprecated]
        #[method(disabled)]
        pub unsafe fn disabled(&self) -> bool;

        #[deprecated]
        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[cfg(feature = "DOMHTMLFormElement")]
        #[deprecated]
        #[method_id(@__retain_semantics Other form)]
        pub unsafe fn form(&self) -> Option<Retained<DOMHTMLFormElement>>;

        #[cfg(feature = "DOMFileList")]
        #[method_id(@__retain_semantics Other files)]
        pub unsafe fn files(&self) -> Option<Retained<DOMFileList>>;

        #[cfg(feature = "DOMFileList")]
        #[method(setFiles:)]
        pub unsafe fn setFiles(&self, files: Option<&DOMFileList>);

        #[method(indeterminate)]
        pub unsafe fn indeterminate(&self) -> bool;

        #[method(setIndeterminate:)]
        pub unsafe fn setIndeterminate(&self, indeterminate: bool);

        #[deprecated]
        #[method(maxLength)]
        pub unsafe fn maxLength(&self) -> c_int;

        #[deprecated]
        #[method(setMaxLength:)]
        pub unsafe fn setMaxLength(&self, max_length: c_int);

        #[method(multiple)]
        pub unsafe fn multiple(&self) -> bool;

        #[method(setMultiple:)]
        pub unsafe fn setMultiple(&self, multiple: bool);

        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[deprecated]
        #[method(readOnly)]
        pub unsafe fn readOnly(&self) -> bool;

        #[deprecated]
        #[method(setReadOnly:)]
        pub unsafe fn setReadOnly(&self, read_only: bool);

        #[deprecated]
        #[method_id(@__retain_semantics Other size)]
        pub unsafe fn size(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other src)]
        pub unsafe fn src(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setSrc:)]
        pub unsafe fn setSrc(&self, src: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other defaultValue)]
        pub unsafe fn defaultValue(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setDefaultValue:)]
        pub unsafe fn setDefaultValue(&self, default_value: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&NSString>);

        #[method(willValidate)]
        pub unsafe fn willValidate(&self) -> bool;

        #[method(selectionStart)]
        pub unsafe fn selectionStart(&self) -> c_int;

        #[method(setSelectionStart:)]
        pub unsafe fn setSelectionStart(&self, selection_start: c_int);

        #[method(selectionEnd)]
        pub unsafe fn selectionEnd(&self) -> c_int;

        #[method(setSelectionEnd:)]
        pub unsafe fn setSelectionEnd(&self, selection_end: c_int);

        #[deprecated]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other useMap)]
        pub unsafe fn useMap(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setUseMap:)]
        pub unsafe fn setUseMap(&self, use_map: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[method_id(@__retain_semantics Other altDisplayString)]
        pub unsafe fn altDisplayString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other absoluteImageURL)]
        pub unsafe fn absoluteImageURL(&self) -> Retained<NSURL>;

        #[deprecated]
        #[method(select)]
        pub unsafe fn select(&self);

        #[method(setSelectionRange:end:)]
        pub unsafe fn setSelectionRange_end(&self, start: c_int, end: c_int);

        #[deprecated]
        #[method(click)]
        pub unsafe fn click(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLInputElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLInputElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
