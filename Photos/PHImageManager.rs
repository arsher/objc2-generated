//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHImageRequestOptionsVersion(pub NSInteger);
impl PHImageRequestOptionsVersion {
    #[doc(alias = "PHImageRequestOptionsVersionCurrent")]
    pub const Current: Self = Self(0);
    #[doc(alias = "PHImageRequestOptionsVersionUnadjusted")]
    pub const Unadjusted: Self = Self(1);
    #[doc(alias = "PHImageRequestOptionsVersionOriginal")]
    pub const Original: Self = Self(2);
}

unsafe impl Encode for PHImageRequestOptionsVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHImageRequestOptionsVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHImageRequestOptionsDeliveryMode(pub NSInteger);
impl PHImageRequestOptionsDeliveryMode {
    #[doc(alias = "PHImageRequestOptionsDeliveryModeOpportunistic")]
    pub const Opportunistic: Self = Self(0);
    #[doc(alias = "PHImageRequestOptionsDeliveryModeHighQualityFormat")]
    pub const HighQualityFormat: Self = Self(1);
    #[doc(alias = "PHImageRequestOptionsDeliveryModeFastFormat")]
    pub const FastFormat: Self = Self(2);
}

unsafe impl Encode for PHImageRequestOptionsDeliveryMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHImageRequestOptionsDeliveryMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHImageRequestOptionsResizeMode(pub NSInteger);
impl PHImageRequestOptionsResizeMode {
    #[doc(alias = "PHImageRequestOptionsResizeModeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "PHImageRequestOptionsResizeModeFast")]
    pub const Fast: Self = Self(1);
    #[doc(alias = "PHImageRequestOptionsResizeModeExact")]
    pub const Exact: Self = Self(2);
}

unsafe impl Encode for PHImageRequestOptionsResizeMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHImageRequestOptionsResizeMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "block2")]
pub type PHAssetImageProgressHandler =
    *mut block2::Block<dyn Fn(c_double, *mut NSError, NonNull<Bool>, *mut NSDictionary)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHImageRequestOptions;

    unsafe impl ClassType for PHImageRequestOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for PHImageRequestOptions {}

unsafe impl NSObjectProtocol for PHImageRequestOptions {}

extern_methods!(
    unsafe impl PHImageRequestOptions {
        #[method(version)]
        pub unsafe fn version(&self) -> PHImageRequestOptionsVersion;

        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: PHImageRequestOptionsVersion);

        #[method(deliveryMode)]
        pub unsafe fn deliveryMode(&self) -> PHImageRequestOptionsDeliveryMode;

        #[method(setDeliveryMode:)]
        pub unsafe fn setDeliveryMode(&self, delivery_mode: PHImageRequestOptionsDeliveryMode);

        #[method(resizeMode)]
        pub unsafe fn resizeMode(&self) -> PHImageRequestOptionsResizeMode;

        #[method(setResizeMode:)]
        pub unsafe fn setResizeMode(&self, resize_mode: PHImageRequestOptionsResizeMode);

        #[method(normalizedCropRect)]
        pub unsafe fn normalizedCropRect(&self) -> CGRect;

        #[method(setNormalizedCropRect:)]
        pub unsafe fn setNormalizedCropRect(&self, normalized_crop_rect: CGRect);

        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[method(isSynchronous)]
        pub unsafe fn isSynchronous(&self) -> bool;

        #[method(setSynchronous:)]
        pub unsafe fn setSynchronous(&self, synchronous: bool);

        #[cfg(feature = "block2")]
        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> PHAssetImageProgressHandler;

        #[cfg(feature = "block2")]
        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(&self, progress_handler: PHAssetImageProgressHandler);

        #[method(allowSecondaryDegradedImage)]
        pub unsafe fn allowSecondaryDegradedImage(&self) -> bool;

        #[method(setAllowSecondaryDegradedImage:)]
        pub unsafe fn setAllowSecondaryDegradedImage(&self, allow_secondary_degraded_image: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHImageRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHLivePhotoRequestOptions;

    unsafe impl ClassType for PHLivePhotoRequestOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for PHLivePhotoRequestOptions {}

unsafe impl NSObjectProtocol for PHLivePhotoRequestOptions {}

extern_methods!(
    unsafe impl PHLivePhotoRequestOptions {
        #[method(version)]
        pub unsafe fn version(&self) -> PHImageRequestOptionsVersion;

        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: PHImageRequestOptionsVersion);

        #[method(deliveryMode)]
        pub unsafe fn deliveryMode(&self) -> PHImageRequestOptionsDeliveryMode;

        #[method(setDeliveryMode:)]
        pub unsafe fn setDeliveryMode(&self, delivery_mode: PHImageRequestOptionsDeliveryMode);

        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[cfg(feature = "block2")]
        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> PHAssetImageProgressHandler;

        #[cfg(feature = "block2")]
        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(&self, progress_handler: PHAssetImageProgressHandler);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHLivePhotoRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHVideoRequestOptionsVersion(pub NSInteger);
impl PHVideoRequestOptionsVersion {
    #[doc(alias = "PHVideoRequestOptionsVersionCurrent")]
    pub const Current: Self = Self(0);
    #[doc(alias = "PHVideoRequestOptionsVersionOriginal")]
    pub const Original: Self = Self(1);
}

unsafe impl Encode for PHVideoRequestOptionsVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHVideoRequestOptionsVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHVideoRequestOptionsDeliveryMode(pub NSInteger);
impl PHVideoRequestOptionsDeliveryMode {
    #[doc(alias = "PHVideoRequestOptionsDeliveryModeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "PHVideoRequestOptionsDeliveryModeHighQualityFormat")]
    pub const HighQualityFormat: Self = Self(1);
    #[doc(alias = "PHVideoRequestOptionsDeliveryModeMediumQualityFormat")]
    pub const MediumQualityFormat: Self = Self(2);
    #[doc(alias = "PHVideoRequestOptionsDeliveryModeFastFormat")]
    pub const FastFormat: Self = Self(3);
}

unsafe impl Encode for PHVideoRequestOptionsDeliveryMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHVideoRequestOptionsDeliveryMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "block2")]
pub type PHAssetVideoProgressHandler =
    *mut block2::Block<dyn Fn(c_double, *mut NSError, NonNull<Bool>, *mut NSDictionary)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHVideoRequestOptions;

    unsafe impl ClassType for PHVideoRequestOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for PHVideoRequestOptions {}

unsafe impl NSObjectProtocol for PHVideoRequestOptions {}

extern_methods!(
    unsafe impl PHVideoRequestOptions {
        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[method(version)]
        pub unsafe fn version(&self) -> PHVideoRequestOptionsVersion;

        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: PHVideoRequestOptionsVersion);

        #[method(deliveryMode)]
        pub unsafe fn deliveryMode(&self) -> PHVideoRequestOptionsDeliveryMode;

        #[method(setDeliveryMode:)]
        pub unsafe fn setDeliveryMode(&self, delivery_mode: PHVideoRequestOptionsDeliveryMode);

        #[cfg(feature = "block2")]
        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> PHAssetVideoProgressHandler;

        #[cfg(feature = "block2")]
        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(&self, progress_handler: PHAssetVideoProgressHandler);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHVideoRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

pub type PHImageRequestID = i32;

pub static PHInvalidImageRequestID: PHImageRequestID = 0;

extern "C" {
    pub static PHImageManagerMaximumSize: CGSize;
}

extern "C" {
    pub static PHImageResultIsInCloudKey: &'static NSString;
}

extern "C" {
    pub static PHImageResultIsDegradedKey: &'static NSString;
}

extern "C" {
    pub static PHImageResultRequestIDKey: &'static NSString;
}

extern "C" {
    pub static PHImageCancelledKey: &'static NSString;
}

extern "C" {
    pub static PHImageErrorKey: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHImageManager;

    unsafe impl ClassType for PHImageManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for PHImageManager {}

extern_methods!(
    unsafe impl PHImageManager {
        #[method_id(@__retain_semantics Other defaultManager)]
        pub unsafe fn defaultManager() -> Id<PHImageManager>;

        #[cfg(all(
            feature = "PHAsset",
            feature = "PHObject",
            feature = "PhotosTypes",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[method(requestImageForAsset:targetSize:contentMode:options:resultHandler:)]
        pub unsafe fn requestImageForAsset_targetSize_contentMode_options_resultHandler(
            &self,
            asset: &PHAsset,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            options: Option<&PHImageRequestOptions>,
            result_handler: &block2::Block<dyn Fn(*mut NSImage, *mut NSDictionary)>,
        ) -> PHImageRequestID;

        #[method(cancelImageRequest:)]
        pub unsafe fn cancelImageRequest(&self, request_id: PHImageRequestID);

        #[cfg(all(
            feature = "PHAsset",
            feature = "PHLivePhoto",
            feature = "PHObject",
            feature = "PhotosTypes",
            feature = "block2"
        ))]
        #[method(requestLivePhotoForAsset:targetSize:contentMode:options:resultHandler:)]
        pub unsafe fn requestLivePhotoForAsset_targetSize_contentMode_options_resultHandler(
            &self,
            asset: &PHAsset,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            options: Option<&PHLivePhotoRequestOptions>,
            result_handler: &block2::Block<dyn Fn(*mut PHLivePhoto, *mut NSDictionary)>,
        ) -> PHImageRequestID;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHImageManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHCachingImageManager;

    unsafe impl ClassType for PHCachingImageManager {
        #[inherits(NSObject)]
        type Super = PHImageManager;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for PHCachingImageManager {}

extern_methods!(
    unsafe impl PHCachingImageManager {
        #[method(allowsCachingHighQualityImages)]
        pub unsafe fn allowsCachingHighQualityImages(&self) -> bool;

        #[method(setAllowsCachingHighQualityImages:)]
        pub unsafe fn setAllowsCachingHighQualityImages(
            &self,
            allows_caching_high_quality_images: bool,
        );

        #[cfg(all(feature = "PHAsset", feature = "PHObject", feature = "PhotosTypes"))]
        #[method(startCachingImagesForAssets:targetSize:contentMode:options:)]
        pub unsafe fn startCachingImagesForAssets_targetSize_contentMode_options(
            &self,
            assets: &NSArray<PHAsset>,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            options: Option<&PHImageRequestOptions>,
        );

        #[cfg(all(feature = "PHAsset", feature = "PHObject", feature = "PhotosTypes"))]
        #[method(stopCachingImagesForAssets:targetSize:contentMode:options:)]
        pub unsafe fn stopCachingImagesForAssets_targetSize_contentMode_options(
            &self,
            assets: &NSArray<PHAsset>,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            options: Option<&PHImageRequestOptions>,
        );

        #[method(stopCachingImagesForAllAssets)]
        pub unsafe fn stopCachingImagesForAllAssets(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHCachingImageManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
