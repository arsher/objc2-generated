//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextFormattingViewControllerFormattingStyle;

    unsafe impl ClassType for UITextFormattingViewControllerFormattingStyle {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for UITextFormattingViewControllerFormattingStyle {}

unsafe impl NSCopying for UITextFormattingViewControllerFormattingStyle {}

unsafe impl CopyingHelper for UITextFormattingViewControllerFormattingStyle {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UITextFormattingViewControllerFormattingStyle {}

unsafe impl NSSecureCoding for UITextFormattingViewControllerFormattingStyle {}

extern_methods!(
    unsafe impl UITextFormattingViewControllerFormattingStyle {
        #[method_id(@__retain_semantics Other styleKey)]
        pub unsafe fn styleKey(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self)
            -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method_id(@__retain_semantics Init initWithStyleKey:title:attributes:)]
        pub unsafe fn initWithStyleKey_title_attributes(
            this: Allocated<Self>,
            style_key: &NSString,
            string: &NSString,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextFormattingViewControllerFormattingStyle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
