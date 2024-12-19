//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileproviderrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileProviderRequest;
);

unsafe impl NSObjectProtocol for NSFileProviderRequest {}

extern_methods!(
    unsafe impl NSFileProviderRequest {
        #[method(isSystemRequest)]
        pub unsafe fn isSystemRequest(&self) -> bool;

        #[method(isFileViewerRequest)]
        pub unsafe fn isFileViewerRequest(&self) -> bool;

        #[method_id(@__retain_semantics Other requestingExecutable)]
        pub unsafe fn requestingExecutable(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSFileProviderDomain")]
        #[method_id(@__retain_semantics Other domainVersion)]
        pub unsafe fn domainVersion(&self) -> Option<Retained<NSFileProviderDomainVersion>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFileProviderRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
