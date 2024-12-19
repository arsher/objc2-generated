//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phassetresourcecreationoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHAssetResourceCreationOptions;
);

unsafe impl NSCopying for PHAssetResourceCreationOptions {}

unsafe impl CopyingHelper for PHAssetResourceCreationOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for PHAssetResourceCreationOptions {}

extern_methods!(
    unsafe impl PHAssetResourceCreationOptions {
        #[method_id(@__retain_semantics Other originalFilename)]
        pub unsafe fn originalFilename(&self) -> Option<Retained<NSString>>;

        #[method(setOriginalFilename:)]
        pub unsafe fn setOriginalFilename(&self, original_filename: Option<&NSString>);

        #[method_id(@__retain_semantics Other uniformTypeIdentifier)]
        pub unsafe fn uniformTypeIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setUniformTypeIdentifier:)]
        pub unsafe fn setUniformTypeIdentifier(&self, uniform_type_identifier: Option<&NSString>);

        #[method(shouldMoveFile)]
        pub unsafe fn shouldMoveFile(&self) -> bool;

        #[method(setShouldMoveFile:)]
        pub unsafe fn setShouldMoveFile(&self, should_move_file: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHAssetResourceCreationOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phassetcreationrequest?language=objc)
    #[unsafe(super(PHAssetChangeRequest, PHChangeRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "PHAssetChangeRequest", feature = "PHChangeRequest"))]
    pub struct PHAssetCreationRequest;
);

#[cfg(all(feature = "PHAssetChangeRequest", feature = "PHChangeRequest"))]
unsafe impl NSObjectProtocol for PHAssetCreationRequest {}

extern_methods!(
    #[cfg(all(feature = "PHAssetChangeRequest", feature = "PHChangeRequest"))]
    unsafe impl PHAssetCreationRequest {
        #[method_id(@__retain_semantics Other creationRequestForAsset)]
        pub unsafe fn creationRequestForAsset() -> Retained<Self>;

        #[method(supportsAssetResourceTypes:)]
        pub unsafe fn supportsAssetResourceTypes(types: &NSArray<NSNumber>) -> bool;

        #[cfg(feature = "PhotosTypes")]
        #[method(addResourceWithType:fileURL:options:)]
        pub unsafe fn addResourceWithType_fileURL_options(
            &self,
            r#type: PHAssetResourceType,
            file_url: &NSURL,
            options: Option<&PHAssetResourceCreationOptions>,
        );

        #[cfg(feature = "PhotosTypes")]
        #[method(addResourceWithType:data:options:)]
        pub unsafe fn addResourceWithType_data_options(
            &self,
            r#type: PHAssetResourceType,
            data: &NSData,
            options: Option<&PHAssetResourceCreationOptions>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `PHAssetChangeRequest`
    #[cfg(all(feature = "PHAssetChangeRequest", feature = "PHChangeRequest"))]
    unsafe impl PHAssetCreationRequest {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other creationRequestForAssetFromImage:)]
        pub unsafe fn creationRequestForAssetFromImage(image: &NSImage) -> Retained<Self>;

        #[method_id(@__retain_semantics Other creationRequestForAssetFromImageAtFileURL:)]
        pub unsafe fn creationRequestForAssetFromImageAtFileURL(
            file_url: &NSURL,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other creationRequestForAssetFromVideoAtFileURL:)]
        pub unsafe fn creationRequestForAssetFromVideoAtFileURL(
            file_url: &NSURL,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "PHAsset", feature = "PHObject"))]
        #[method_id(@__retain_semantics Other changeRequestForAsset:)]
        pub unsafe fn changeRequestForAsset(asset: &PHAsset) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "PHAssetChangeRequest", feature = "PHChangeRequest"))]
    unsafe impl PHAssetCreationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
