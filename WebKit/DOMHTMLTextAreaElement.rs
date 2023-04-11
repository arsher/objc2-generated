//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLTextAreaElement")]
    #[deprecated]
    pub struct DOMHTMLTextAreaElement;

    #[cfg(feature = "WebKit_DOMHTMLTextAreaElement")]
    unsafe impl ClassType for DOMHTMLTextAreaElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLTextAreaElement")]
unsafe impl DOMEventTarget for DOMHTMLTextAreaElement {}

#[cfg(feature = "WebKit_DOMHTMLTextAreaElement")]
unsafe impl NSCopying for DOMHTMLTextAreaElement {}

#[cfg(feature = "WebKit_DOMHTMLTextAreaElement")]
unsafe impl NSObjectProtocol for DOMHTMLTextAreaElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLTextAreaElement")]
    unsafe impl DOMHTMLTextAreaElement {
        #[method(autofocus)]
        pub unsafe fn autofocus(&self) -> bool;

        #[method(setAutofocus:)]
        pub unsafe fn setAutofocus(&self, autofocus: bool);

        #[method(disabled)]
        pub unsafe fn disabled(&self) -> bool;

        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[cfg(feature = "WebKit_DOMHTMLFormElement")]
        #[method_id(@__retain_semantics Other form)]
        pub unsafe fn form(&self) -> Option<Id<DOMHTMLFormElement>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(readOnly)]
        pub unsafe fn readOnly(&self) -> bool;

        #[method(setReadOnly:)]
        pub unsafe fn setReadOnly(&self, read_only: bool);

        #[method(rows)]
        pub unsafe fn rows(&self) -> c_int;

        #[method(setRows:)]
        pub unsafe fn setRows(&self, rows: c_int);

        #[method(cols)]
        pub unsafe fn cols(&self) -> c_int;

        #[method(setCols:)]
        pub unsafe fn setCols(&self, cols: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultValue)]
        pub unsafe fn defaultValue(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDefaultValue:)]
        pub unsafe fn setDefaultValue(&self, default_value: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
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

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[method(select)]
        pub unsafe fn select(&self);

        #[method(setSelectionRange:end:)]
        pub unsafe fn setSelectionRange_end(&self, start: c_int, end: c_int);
    }
);
