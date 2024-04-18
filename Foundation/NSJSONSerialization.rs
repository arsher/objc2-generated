//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSJSONReadingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSJSONReadingOptions: NSUInteger {
        const NSJSONReadingMutableContainers = 1<<0;
        const NSJSONReadingMutableLeaves = 1<<1;
        const NSJSONReadingFragmentsAllowed = 1<<2;
        const NSJSONReadingJSON5Allowed = 1<<3;
        const NSJSONReadingTopLevelDictionaryAssumed = 1<<4;
#[deprecated]
        const NSJSONReadingAllowFragments = NSJSONReadingOptions::NSJSONReadingFragmentsAllowed.0;
    }
}

unsafe impl Encode for NSJSONReadingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSJSONReadingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSJSONWritingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSJSONWritingOptions: NSUInteger {
        const NSJSONWritingPrettyPrinted = 1<<0;
        const NSJSONWritingSortedKeys = 1<<1;
        const NSJSONWritingFragmentsAllowed = 1<<2;
        const NSJSONWritingWithoutEscapingSlashes = 1<<3;
    }
}

unsafe impl Encode for NSJSONWritingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSJSONWritingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSJSONSerialization;

    unsafe impl ClassType for NSJSONSerialization {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSJSONSerialization {}

extern_methods!(
    unsafe impl NSJSONSerialization {
        #[method(isValidJSONObject:)]
        pub unsafe fn isValidJSONObject(obj: &AnyObject) -> bool;

        #[cfg(all(feature = "NSData", feature = "NSError"))]
        #[method_id(@__retain_semantics Other dataWithJSONObject:options:error:_)]
        pub unsafe fn dataWithJSONObject_options_error(
            obj: &AnyObject,
            opt: NSJSONWritingOptions,
        ) -> Result<Id<NSData>, Id<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSError"))]
        #[method_id(@__retain_semantics Other JSONObjectWithData:options:error:_)]
        pub unsafe fn JSONObjectWithData_options_error(
            data: &NSData,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSStream"))]
        #[method_id(@__retain_semantics Other JSONObjectWithStream:options:error:_)]
        pub unsafe fn JSONObjectWithStream_options_error(
            stream: &NSInputStream,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<AnyObject>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSJSONSerialization {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
