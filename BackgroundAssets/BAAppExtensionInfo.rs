//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BAAppExtensionInfo;

    unsafe impl ClassType for BAAppExtensionInfo {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for BAAppExtensionInfo {}

unsafe impl Sync for BAAppExtensionInfo {}

unsafe impl NSCoding for BAAppExtensionInfo {}

unsafe impl NSObjectProtocol for BAAppExtensionInfo {}

unsafe impl NSSecureCoding for BAAppExtensionInfo {}

extern_methods!(
    unsafe impl BAAppExtensionInfo {
        #[method_id(@__retain_semantics Other restrictedDownloadSizeRemaining)]
        pub unsafe fn restrictedDownloadSizeRemaining(&self) -> Option<Retained<NSNumber>>;

        #[method_id(@__retain_semantics Other restrictedEssentialDownloadSizeRemaining)]
        pub unsafe fn restrictedEssentialDownloadSizeRemaining(&self)
            -> Option<Retained<NSNumber>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
