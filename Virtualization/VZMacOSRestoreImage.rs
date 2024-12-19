//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacOSRestoreImage;
);

unsafe impl NSObjectProtocol for VZMacOSRestoreImage {}

extern_methods!(
    unsafe impl VZMacOSRestoreImage {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(loadFileURL:completionHandler:)]
        pub unsafe fn loadFileURL_completionHandler(
            file_url: &NSURL,
            completion_handler: &block2::Block<dyn Fn(*mut VZMacOSRestoreImage, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(fetchLatestSupportedWithCompletionHandler:)]
        pub unsafe fn fetchLatestSupportedWithCompletionHandler(
            completion_handler: &block2::Block<dyn Fn(*mut VZMacOSRestoreImage, *mut NSError)>,
        );

        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[method_id(@__retain_semantics Other buildVersion)]
        pub unsafe fn buildVersion(&self) -> Retained<NSString>;

        #[method(operatingSystemVersion)]
        pub unsafe fn operatingSystemVersion(&self) -> NSOperatingSystemVersion;

        #[cfg(feature = "VZMacOSConfigurationRequirements")]
        #[method_id(@__retain_semantics Other mostFeaturefulSupportedConfiguration)]
        pub unsafe fn mostFeaturefulSupportedConfiguration(
            &self,
        ) -> Option<Retained<VZMacOSConfigurationRequirements>>;
    }
);
