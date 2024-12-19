//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odrecordmap?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ODRecordMap;
);

unsafe impl NSObjectProtocol for ODRecordMap {}

extern_methods!(
    unsafe impl ODRecordMap {
        #[method_id(@__retain_semantics Other native)]
        pub unsafe fn native(&self) -> Retained<NSString>;

        #[method(setNative:)]
        pub unsafe fn setNative(&self, native: Option<&NSString>);

        #[method_id(@__retain_semantics Other odPredicate)]
        pub unsafe fn odPredicate(&self) -> Retained<NSDictionary>;

        #[method(setOdPredicate:)]
        pub unsafe fn setOdPredicate(&self, od_predicate: Option<&NSDictionary>);

        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Retained<NSDictionary>;

        #[method_id(@__retain_semantics Other standardAttributeTypes)]
        pub unsafe fn standardAttributeTypes(&self) -> Retained<NSArray>;

        #[method_id(@__retain_semantics Other recordMap)]
        pub unsafe fn recordMap() -> Option<Retained<Self>>;

        #[cfg(feature = "ODAttributeMap")]
        #[method_id(@__retain_semantics Other attributeMapForStandardAttribute:)]
        pub unsafe fn attributeMapForStandardAttribute(
            &self,
            standard_attribute: Option<&NSString>,
        ) -> Option<Retained<ODAttributeMap>>;

        #[cfg(feature = "ODAttributeMap")]
        #[method(setAttributeMap:forStandardAttribute:)]
        pub unsafe fn setAttributeMap_forStandardAttribute(
            &self,
            attribute_map: Option<&ODAttributeMap>,
            standard_attribute: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ODRecordMap {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
