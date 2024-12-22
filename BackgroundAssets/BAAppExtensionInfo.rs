//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/backgroundassets/baappextensioninfo?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BAAppExtensionInfo;
);

unsafe impl Send for BAAppExtensionInfo {}

unsafe impl Sync for BAAppExtensionInfo {}

unsafe impl NSCoding for BAAppExtensionInfo {}

unsafe impl NSObjectProtocol for BAAppExtensionInfo {}

unsafe impl NSSecureCoding for BAAppExtensionInfo {}

extern_methods!(
    unsafe impl BAAppExtensionInfo {
        /// The number of bytes remaining that can be scheduled if the total download size is restricted.
        ///
        /// When a download is restricted, your extension can only schedule up to its `BADownloadAllowance`
        /// defined in your app's `Info.plist`. This result tells you the number of bytes remaining that can be scheduled
        /// before the application is launched. Once the application is launched, this restriction is removed.
        ///
        /// Returns: The result is `nil` if downloads are not restricted. It returns a valid number with the remaining available download size otherwise.
        #[method_id(@__retain_semantics Other restrictedDownloadSizeRemaining)]
        pub unsafe fn restrictedDownloadSizeRemaining(&self) -> Option<Retained<NSNumber>>;

        /// The number of bytes remaining that can be scheduled if the total download size of optional assets is restricted.
        ///
        /// When a download is restricted, your extension can only schedule up to its `BAEssentialDownloadAllowance`
        /// defined in your app's `Info.plist`. This result tells you the number of bytes remaining that can be scheduled
        /// before the application is launched. Once the application is launched, this restriction is removed.
        ///
        /// Returns: The result is `nil` if downloads are not restricted. It returns a valid number with the remaining available download size otherwise.
        #[method_id(@__retain_semantics Other restrictedEssentialDownloadSizeRemaining)]
        pub unsafe fn restrictedEssentialDownloadSizeRemaining(&self)
            -> Option<Retained<NSNumber>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
