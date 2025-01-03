//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "NSItemSourceInfo" on [`NSItemProvider`].
    #[doc(alias = "NSItemSourceInfo")]
    pub unsafe trait NSItemProviderNSItemSourceInfo {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(sourceFrame)]
        unsafe fn sourceFrame(&self) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(containerFrame)]
        unsafe fn containerFrame(&self) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredPresentationSize)]
        unsafe fn preferredPresentationSize(&self) -> NSSize;
    }

    unsafe impl NSItemProviderNSItemSourceInfo for NSItemProvider {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstypeidentifierdatetext?language=objc)
    pub static NSTypeIdentifierDateText: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstypeidentifieraddresstext?language=objc)
    pub static NSTypeIdentifierAddressText: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstypeidentifierphonenumbertext?language=objc)
    pub static NSTypeIdentifierPhoneNumberText: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstypeidentifiertransitinformationtext?language=objc)
    pub static NSTypeIdentifierTransitInformationText: &'static NSString;
}
