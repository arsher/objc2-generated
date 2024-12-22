//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Only AND compound predicates are allowed.
    ///
    /// Key names must begin with either an upper or lower case character ([a-zA-Z]) and may be followed by characters, numbers, or underscores ([0-9a-zA-Z_]). Keypaths may only resolve to the currently evaluated object, so the '.' character is not allowed in key names.
    ///
    /// A limited subset of classes are allowed as predicate arguments:
    /// - NSString
    /// - NSDate
    /// - NSData
    /// - NSNumber
    /// - NSArray
    /// - CKReference
    /// - CKRecord
    /// - CLLocation
    ///
    /// Any other class as an argument will result in an error when executing the query.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckquery?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKQuery;
);

unsafe impl NSCoding for CKQuery {}

unsafe impl NSCopying for CKQuery {}

unsafe impl CopyingHelper for CKQuery {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKQuery {}

unsafe impl NSSecureCoding for CKQuery {}

extern_methods!(
    unsafe impl CKQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, a_decoder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "CKRecord")]
        /// Use
        ///
        /// ```text
        ///  [NSPredicate predicateWithValue:YES] / NSPredicate(value: true)
        /// ```
        ///
        /// if you want to query for all records of a given type.
        #[method_id(@__retain_semantics Init initWithRecordType:predicate:)]
        pub unsafe fn initWithRecordType_predicate(
            this: Allocated<Self>,
            record_type: &CKRecordType,
            predicate: &NSPredicate,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Retained<CKRecordType>;

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Retained<NSArray<NSSortDescriptor>>>;

        /// Setter for [`sortDescriptors`][Self::sortDescriptors].
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(
            &self,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
        );
    }
);
