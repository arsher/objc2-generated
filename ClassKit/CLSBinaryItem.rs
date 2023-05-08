//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CLSBinaryValueType {
        CLSBinaryValueTypeTrueFalse = 0,
        CLSBinaryValueTypePassFail = 1,
        CLSBinaryValueTypeYesNo = 2,
        CLSBinaryValueTypeCorrectIncorrect = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSBinaryItem")]
    pub struct CLSBinaryItem;

    #[cfg(feature = "ClassKit_CLSBinaryItem")]
    unsafe impl ClassType for CLSBinaryItem {
        #[inherits(CLSObject, NSObject)]
        type Super = CLSActivityItem;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ClassKit_CLSBinaryItem")]
unsafe impl NSCoding for CLSBinaryItem {}

#[cfg(feature = "ClassKit_CLSBinaryItem")]
unsafe impl NSObjectProtocol for CLSBinaryItem {}

#[cfg(feature = "ClassKit_CLSBinaryItem")]
unsafe impl NSSecureCoding for CLSBinaryItem {}

extern_methods!(
    #[cfg(feature = "ClassKit_CLSBinaryItem")]
    unsafe impl CLSBinaryItem {
        #[method(value)]
        pub unsafe fn value(&self) -> bool;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: bool);

        #[method(valueType)]
        pub unsafe fn valueType(&self) -> CLSBinaryValueType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:title:type:)]
        pub unsafe fn initWithIdentifier_title_type(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            title: &NSString,
            value_type: CLSBinaryValueType,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLSActivityItem`
    #[cfg(feature = "ClassKit_CLSBinaryItem")]
    unsafe impl CLSBinaryItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
