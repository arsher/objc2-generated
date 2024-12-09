//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotorequestid?language=objc)
pub type PHLivePhotoRequestID = i32;

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotorequestidinvalid?language=objc)
pub static PHLivePhotoRequestIDInvalid: PHLivePhotoRequestID = 0;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoinfoerrorkey?language=objc)
    pub static PHLivePhotoInfoErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoinfoisdegradedkey?language=objc)
    pub static PHLivePhotoInfoIsDegradedKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephotoinfocancelledkey?language=objc)
    pub static PHLivePhotoInfoCancelledKey: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phlivephoto?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHLivePhoto;
);

unsafe impl NSCoding for PHLivePhoto {}

unsafe impl NSCopying for PHLivePhoto {}

unsafe impl CopyingHelper for PHLivePhoto {
    type Result = Self;
}

unsafe impl NSObjectProtocol for PHLivePhoto {}

unsafe impl NSSecureCoding for PHLivePhoto {}

extern_methods!(
    unsafe impl PHLivePhoto {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[cfg(all(
            feature = "PhotosTypes",
            feature = "block2",
            feature = "objc2-app-kit",
            feature = "objc2-core-foundation"
        ))]
        #[cfg(target_os = "macos")]
        #[method(requestLivePhotoWithResourceFileURLs:placeholderImage:targetSize:contentMode:resultHandler:)]
        pub unsafe fn requestLivePhotoWithResourceFileURLs_placeholderImage_targetSize_contentMode_resultHandler(
            file_ur_ls: &NSArray<NSURL>,
            image: Option<&NSImage>,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            result_handler: &block2::Block<dyn Fn(*mut PHLivePhoto, NonNull<NSDictionary>)>,
        ) -> PHLivePhotoRequestID;

        #[method(cancelLivePhotoRequestWithRequestID:)]
        pub unsafe fn cancelLivePhotoRequestWithRequestID(request_id: PHLivePhotoRequestID);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHLivePhoto {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSItemProvider
    unsafe impl PHLivePhoto {}
);

unsafe impl NSItemProviderReading for PHLivePhoto {}
