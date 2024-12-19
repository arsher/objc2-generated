//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnlabeledvalue?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNLabeledValue<ValueType: ?Sized = AnyObject>;
);

unsafe impl<ValueType: ?Sized + NSCoding> NSCoding for CNLabeledValue<ValueType> {}

unsafe impl<ValueType: ?Sized> NSCopying for CNLabeledValue<ValueType> {}

unsafe impl<ValueType: ?Sized + Message> CopyingHelper for CNLabeledValue<ValueType> {
    type Result = Self;
}

unsafe impl<ValueType: ?Sized> NSObjectProtocol for CNLabeledValue<ValueType> {}

unsafe impl<ValueType: ?Sized + NSSecureCoding> NSSecureCoding for CNLabeledValue<ValueType> {}

extern_methods!(
    unsafe impl<ValueType: Message> CNLabeledValue<ValueType> {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<ValueType>;

        #[method_id(@__retain_semantics Other labeledValueWithLabel:value:)]
        pub unsafe fn labeledValueWithLabel_value(
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLabel:value:)]
        pub unsafe fn initWithLabel_value(
            this: Allocated<Self>,
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other labeledValueBySettingLabel:)]
        pub unsafe fn labeledValueBySettingLabel(&self, label: Option<&NSString>)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Other labeledValueBySettingValue:)]
        pub unsafe fn labeledValueBySettingValue(&self, value: &ValueType) -> Retained<Self>;

        #[method_id(@__retain_semantics Other labeledValueBySettingLabel:value:)]
        pub unsafe fn labeledValueBySettingLabel_value(
            &self,
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other localizedStringForLabel:)]
        pub unsafe fn localizedStringForLabel(label: &NSString) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ValueType: Message> CNLabeledValue<ValueType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnlabelhome?language=objc)
    pub static CNLabelHome: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnlabelwork?language=objc)
    pub static CNLabelWork: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnlabelschool?language=objc)
    pub static CNLabelSchool: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnlabelother?language=objc)
    pub static CNLabelOther: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnlabelemailicloud?language=objc)
    pub static CNLabelEmailiCloud: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnlabelurladdresshomepage?language=objc)
    pub static CNLabelURLAddressHomePage: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnlabeldateanniversary?language=objc)
    pub static CNLabelDateAnniversary: &'static NSString;
}
