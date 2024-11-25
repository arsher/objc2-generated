//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmanagedobjectid?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSManagedObjectID;
);

unsafe impl Send for NSManagedObjectID {}

unsafe impl Sync for NSManagedObjectID {}

unsafe impl NSCopying for NSManagedObjectID {}

unsafe impl CopyingHelper for NSManagedObjectID {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSManagedObjectID {}

extern_methods!(
    unsafe impl NSManagedObjectID {
        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Retained<NSEntityDescription>;

        #[cfg(feature = "NSPersistentStore")]
        #[method_id(@__retain_semantics Other persistentStore)]
        pub unsafe fn persistentStore(&self) -> Option<Retained<NSPersistentStore>>;

        #[method(isTemporaryID)]
        pub unsafe fn isTemporaryID(&self) -> bool;

        #[method_id(@__retain_semantics Other URIRepresentation)]
        pub unsafe fn URIRepresentation(&self) -> Retained<NSURL>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSManagedObjectID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
