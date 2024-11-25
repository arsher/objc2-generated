//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchindexdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFetchIndexDescription;
);

unsafe impl NSCoding for NSFetchIndexDescription {}

unsafe impl NSCopying for NSFetchIndexDescription {}

unsafe impl CopyingHelper for NSFetchIndexDescription {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSFetchIndexDescription {}

extern_methods!(
    unsafe impl NSFetchIndexDescription {
        #[cfg(feature = "NSFetchIndexElementDescription")]
        #[method_id(@__retain_semantics Init initWithName:elements:)]
        pub unsafe fn initWithName_elements(
            this: Allocated<Self>,
            name: &NSString,
            elements: Option<&NSArray<NSFetchIndexElementDescription>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[cfg(feature = "NSFetchIndexElementDescription")]
        #[method_id(@__retain_semantics Other elements)]
        pub unsafe fn elements(&self) -> Retained<NSArray<NSFetchIndexElementDescription>>;

        #[cfg(feature = "NSFetchIndexElementDescription")]
        #[method(setElements:)]
        pub unsafe fn setElements(&self, elements: &NSArray<NSFetchIndexElementDescription>);

        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Option<Retained<NSEntityDescription>>;

        #[method_id(@__retain_semantics Other partialIndexPredicate)]
        pub unsafe fn partialIndexPredicate(&self) -> Option<Retained<NSPredicate>>;

        #[method(setPartialIndexPredicate:)]
        pub unsafe fn setPartialIndexPredicate(
            &self,
            partial_index_predicate: Option<&NSPredicate>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFetchIndexDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
