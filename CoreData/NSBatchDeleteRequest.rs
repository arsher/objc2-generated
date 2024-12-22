//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsbatchdeleterequest?language=objc)
    #[unsafe(super(NSPersistentStoreRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStoreRequest")]
    pub struct NSBatchDeleteRequest;
);

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSCopying for NSBatchDeleteRequest {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl CopyingHelper for NSBatchDeleteRequest {
    type Result = Self;
}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSObjectProtocol for NSBatchDeleteRequest {}

extern_methods!(
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSBatchDeleteRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSFetchRequest")]
        #[method_id(@__retain_semantics Init initWithFetchRequest:)]
        pub unsafe fn initWithFetchRequest(
            this: Allocated<Self>,
            fetch: &NSFetchRequest,
        ) -> Retained<Self>;

        #[cfg(feature = "NSManagedObjectID")]
        #[method_id(@__retain_semantics Init initWithObjectIDs:)]
        pub unsafe fn initWithObjectIDs(
            this: Allocated<Self>,
            objects: &NSArray<NSManagedObjectID>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType;

        #[cfg(feature = "NSPersistentStoreResult")]
        /// Setter for [`resultType`][Self::resultType].
        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSBatchDeleteRequestResultType);

        #[cfg(feature = "NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Retained<NSFetchRequest>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSBatchDeleteRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
