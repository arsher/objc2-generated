//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// CIImageProvider
    #[cfg(feature = "CIImage")]
    unsafe impl CIImage {
        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other imageWithImageProvider:size::format:colorSpace:options:)]
        pub unsafe fn imageWithImageProvider_size__format_colorSpace_options(
            p: &AnyObject,
            width: usize,
            height: usize,
            f: CIFormat,
            cs: CGColorSpaceRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Init initWithImageProvider:size::format:colorSpace:options:)]
        pub unsafe fn initWithImageProvider_size__format_colorSpace_options(
            this: Allocated<Self>,
            p: &AnyObject,
            width: usize,
            height: usize,
            f: CIFormat,
            cs: CGColorSpaceRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<Self>;
    }
);

extern_category!(
    /// Category "CIImageProvider" on [`NSObject`].
    #[doc(alias = "CIImageProvider")]
    pub unsafe trait NSObjectCIImageProvider {
        #[method(provideImageData:bytesPerRow:origin::size::userInfo:)]
        unsafe fn provideImageData_bytesPerRow_origin__size__userInfo(
            &self,
            data: NonNull<c_void>,
            rowbytes: usize,
            x: usize,
            y: usize,
            width: usize,
            height: usize,
            info: Option<&AnyObject>,
        );
    }

    unsafe impl NSObjectCIImageProvider for NSObject {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageprovidertilesize?language=objc)
    #[cfg(feature = "CIImage")]
    pub static kCIImageProviderTileSize: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageprovideruserinfo?language=objc)
    #[cfg(feature = "CIImage")]
    pub static kCIImageProviderUserInfo: &'static CIImageOption;
}
