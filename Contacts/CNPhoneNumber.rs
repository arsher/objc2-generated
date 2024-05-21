//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNPhoneNumber;

    unsafe impl ClassType for CNPhoneNumber {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CNPhoneNumber {}

unsafe impl NSCopying for CNPhoneNumber {}

unsafe impl NSObjectProtocol for CNPhoneNumber {}

unsafe impl NSSecureCoding for CNPhoneNumber {}

extern_methods!(
    unsafe impl CNPhoneNumber {
        #[method_id(@__retain_semantics Other phoneNumberWithStringValue:)]
        pub unsafe fn phoneNumberWithStringValue(string_value: &NSString)
            -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithStringValue:)]
        pub unsafe fn initWithStringValue(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[deprecated = "Use initWithStringValue:"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated = "Use phoneNumberWithStringValue:"]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Retained<NSString>;
    }
);

extern "C" {
    pub static CNLabelPhoneNumberiPhone: &'static NSString;
}

extern "C" {
    pub static CNLabelPhoneNumberAppleWatch: &'static NSString;
}

extern "C" {
    pub static CNLabelPhoneNumberMobile: &'static NSString;
}

extern "C" {
    pub static CNLabelPhoneNumberMain: &'static NSString;
}

extern "C" {
    pub static CNLabelPhoneNumberHomeFax: &'static NSString;
}

extern "C" {
    pub static CNLabelPhoneNumberWorkFax: &'static NSString;
}

extern "C" {
    pub static CNLabelPhoneNumberOtherFax: &'static NSString;
}

extern "C" {
    pub static CNLabelPhoneNumberPager: &'static NSString;
}
