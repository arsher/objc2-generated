//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odmappings?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ODMappings;
);

unsafe impl NSObjectProtocol for ODMappings {}

extern_methods!(
    unsafe impl ODMappings {
        #[method_id(@__retain_semantics Other comment)]
        pub unsafe fn comment(&self) -> Retained<NSString>;

        /// Setter for [`comment`][Self::comment].
        #[method(setComment:)]
        pub unsafe fn setComment(&self, comment: Option<&NSString>);

        #[method_id(@__retain_semantics Other templateName)]
        pub unsafe fn templateName(&self) -> Retained<NSString>;

        /// Setter for [`templateName`][Self::templateName].
        #[method(setTemplateName:)]
        pub unsafe fn setTemplateName(&self, template_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        /// Setter for [`identifier`][Self::identifier].
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[method_id(@__retain_semantics Other recordTypes)]
        pub unsafe fn recordTypes(&self) -> Retained<NSArray>;

        #[method_id(@__retain_semantics Other function)]
        pub unsafe fn function(&self) -> Retained<NSString>;

        /// Setter for [`function`][Self::function].
        #[method(setFunction:)]
        pub unsafe fn setFunction(&self, function: Option<&NSString>);

        #[method_id(@__retain_semantics Other functionAttributes)]
        pub unsafe fn functionAttributes(&self) -> Retained<NSArray>;

        /// Setter for [`functionAttributes`][Self::functionAttributes].
        #[method(setFunctionAttributes:)]
        pub unsafe fn setFunctionAttributes(&self, function_attributes: Option<&NSArray>);

        /// Returns an initialized and autoreleased ODMappings object.
        ///
        ///
        /// Returns an initialized and autoreleased ODMappings object.
        #[method_id(@__retain_semantics Other mappings)]
        pub unsafe fn mappings() -> Option<Retained<Self>>;

        #[cfg(feature = "ODRecordMap")]
        /// Returns an ODRecordMap associated with the provided recordtype.
        ///
        ///
        /// Returns an ODRecordMap associated with the provided recordtype.
        #[method_id(@__retain_semantics Other recordMapForStandardRecordType:)]
        pub unsafe fn recordMapForStandardRecordType(
            &self,
            std_type: Option<&NSString>,
        ) -> Option<Retained<ODRecordMap>>;

        #[cfg(feature = "ODRecordMap")]
        /// Sets a particular ODRecordMap for a given standard record type.
        ///
        ///
        /// Sets a particular ODRecordMap for a given standard record type.
        #[method(setRecordMap:forStandardRecordType:)]
        pub unsafe fn setRecordMap_forStandardRecordType(
            &self,
            map: Option<&ODRecordMap>,
            std_type: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ODMappings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
