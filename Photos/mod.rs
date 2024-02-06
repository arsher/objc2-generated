// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `Photos` framework
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "Photos", kind = "framework")]
extern "C" {}

#[path = "PHAdjustmentData.rs"]
mod __PHAdjustmentData;
#[path = "PHAsset.rs"]
mod __PHAsset;
#[path = "PHAssetChangeRequest.rs"]
mod __PHAssetChangeRequest;
#[path = "PHAssetCollectionChangeRequest.rs"]
mod __PHAssetCollectionChangeRequest;
#[path = "PHAssetCreationRequest.rs"]
mod __PHAssetCreationRequest;
#[path = "PHAssetResource.rs"]
mod __PHAssetResource;
#[path = "PHAssetResourceManager.rs"]
mod __PHAssetResourceManager;
#[path = "PHChange.rs"]
mod __PHChange;
#[path = "PHChangeRequest.rs"]
mod __PHChangeRequest;
#[path = "PHCloudIdentifier.rs"]
mod __PHCloudIdentifier;
#[path = "PHCollection.rs"]
mod __PHCollection;
#[path = "PHCollectionListChangeRequest.rs"]
mod __PHCollectionListChangeRequest;
#[path = "PHContentEditingInput.rs"]
mod __PHContentEditingInput;
#[path = "PHContentEditingOutput.rs"]
mod __PHContentEditingOutput;
#[path = "PHError.rs"]
mod __PHError;
#[path = "PHFetchOptions.rs"]
mod __PHFetchOptions;
#[path = "PHFetchResult.rs"]
mod __PHFetchResult;
#[path = "PHImageManager.rs"]
mod __PHImageManager;
#[path = "PHLivePhoto.rs"]
mod __PHLivePhoto;
#[path = "PHLivePhotoEditingContext.rs"]
mod __PHLivePhotoEditingContext;
#[path = "PHObject.rs"]
mod __PHObject;
#[path = "PHPersistentChange.rs"]
mod __PHPersistentChange;
#[path = "PHPersistentChangeFetchResult.rs"]
mod __PHPersistentChangeFetchResult;
#[path = "PHPersistentChangeToken.rs"]
mod __PHPersistentChangeToken;
#[path = "PHPersistentObjectChangeDetails.rs"]
mod __PHPersistentObjectChangeDetails;
#[path = "PHPhotoLibrary.rs"]
mod __PHPhotoLibrary;
#[path = "PHProject.rs"]
mod __PHProject;
#[path = "PHProjectChangeRequest.rs"]
mod __PHProjectChangeRequest;
#[path = "PhotosTypes.rs"]
mod __PhotosTypes;

#[cfg(feature = "Photos_PHAdjustmentData")]
pub use self::__PHAdjustmentData::PHAdjustmentData;
#[cfg(feature = "Photos_PHAsset")]
pub use self::__PHAsset::PHAsset;
#[cfg(feature = "Photos_PHAssetChangeRequest")]
pub use self::__PHAssetChangeRequest::PHAssetChangeRequest;
pub use self::__PHAssetChangeRequest::PHContentEditingInputCancelledKey;
pub use self::__PHAssetChangeRequest::PHContentEditingInputErrorKey;
pub use self::__PHAssetChangeRequest::PHContentEditingInputRequestID;
#[cfg(feature = "Photos_PHContentEditingInputRequestOptions")]
pub use self::__PHAssetChangeRequest::PHContentEditingInputRequestOptions;
pub use self::__PHAssetChangeRequest::PHContentEditingInputResultIsInCloudKey;
#[cfg(feature = "Photos_PHAssetCollectionChangeRequest")]
pub use self::__PHAssetCollectionChangeRequest::PHAssetCollectionChangeRequest;
#[cfg(feature = "Photos_PHAssetCreationRequest")]
pub use self::__PHAssetCreationRequest::PHAssetCreationRequest;
#[cfg(feature = "Photos_PHAssetResourceCreationOptions")]
pub use self::__PHAssetCreationRequest::PHAssetResourceCreationOptions;
#[cfg(feature = "Photos_PHAssetResource")]
pub use self::__PHAssetResource::PHAssetResource;
pub use self::__PHAssetResourceManager::PHAssetResourceDataRequestID;
#[cfg(feature = "Photos_PHAssetResourceManager")]
pub use self::__PHAssetResourceManager::PHAssetResourceManager;
pub use self::__PHAssetResourceManager::PHAssetResourceProgressHandler;
#[cfg(feature = "Photos_PHAssetResourceRequestOptions")]
pub use self::__PHAssetResourceManager::PHAssetResourceRequestOptions;
pub use self::__PHAssetResourceManager::PHInvalidAssetResourceDataRequestID;
#[cfg(feature = "Photos_PHChange")]
pub use self::__PHChange::PHChange;
#[cfg(feature = "Photos_PHFetchResultChangeDetails")]
pub use self::__PHChange::PHFetchResultChangeDetails;
#[cfg(feature = "Photos_PHObjectChangeDetails")]
pub use self::__PHChange::PHObjectChangeDetails;
#[cfg(feature = "Photos_PHChangeRequest")]
pub use self::__PHChangeRequest::PHChangeRequest;
#[cfg(feature = "Photos_PHCloudIdentifier")]
pub use self::__PHCloudIdentifier::PHCloudIdentifier;
#[cfg(feature = "Photos_PHCloudIdentifierMapping")]
pub use self::__PHCloudIdentifier::PHCloudIdentifierMapping;
#[cfg(feature = "Photos_PHLocalIdentifierMapping")]
pub use self::__PHCloudIdentifier::PHLocalIdentifierMapping;
pub use self::__PHCloudIdentifier::PHLocalIdentifierNotFound;
#[cfg(feature = "Photos_PHAssetCollection")]
pub use self::__PHCollection::PHAssetCollection;
#[cfg(feature = "Photos_PHCollection")]
pub use self::__PHCollection::PHCollection;
#[cfg(feature = "Photos_PHCollectionList")]
pub use self::__PHCollection::PHCollectionList;
#[cfg(feature = "Photos_PHCollectionListChangeRequest")]
pub use self::__PHCollectionListChangeRequest::PHCollectionListChangeRequest;
#[cfg(feature = "Photos_PHContentEditingInput")]
pub use self::__PHContentEditingInput::PHContentEditingInput;
#[cfg(feature = "Photos_PHContentEditingOutput")]
pub use self::__PHContentEditingOutput::PHContentEditingOutput;
pub use self::__PHError::PHLocalIdentifiersErrorKey;
pub use self::__PHError::PHPhotosError;
pub use self::__PHError::PHPhotosErrorDomain;
pub use self::__PHError::{
    PHPhotosErrorAccessRestricted, PHPhotosErrorAccessUserDenied, PHPhotosErrorChangeNotSupported,
    PHPhotosErrorIdentifierNotFound, PHPhotosErrorInternalError, PHPhotosErrorInvalid,
    PHPhotosErrorInvalidResource, PHPhotosErrorLibraryInFileProviderSyncRoot,
    PHPhotosErrorLibraryVolumeOffline, PHPhotosErrorMissingResource,
    PHPhotosErrorMultipleIdentifiersFound, PHPhotosErrorNetworkAccessRequired,
    PHPhotosErrorNetworkError, PHPhotosErrorNotEnoughSpace, PHPhotosErrorOperationInterrupted,
    PHPhotosErrorPersistentChangeDetailsUnavailable, PHPhotosErrorPersistentChangeTokenExpired,
    PHPhotosErrorRelinquishingLibraryBundleToWriter, PHPhotosErrorRequestNotSupportedForAsset,
    PHPhotosErrorSwitchingSystemPhotoLibrary, PHPhotosErrorUserCancelled,
};
#[cfg(feature = "Photos_PHFetchOptions")]
pub use self::__PHFetchOptions::PHFetchOptions;
#[cfg(feature = "Photos_PHFetchResult")]
pub use self::__PHFetchResult::PHFetchResult;
pub use self::__PHImageManager::PHAssetImageProgressHandler;
pub use self::__PHImageManager::PHAssetVideoProgressHandler;
#[cfg(feature = "Photos_PHCachingImageManager")]
pub use self::__PHImageManager::PHCachingImageManager;
pub use self::__PHImageManager::PHImageCancelledKey;
pub use self::__PHImageManager::PHImageErrorKey;
#[cfg(feature = "Photos_PHImageManager")]
pub use self::__PHImageManager::PHImageManager;
pub use self::__PHImageManager::PHImageManagerMaximumSize;
pub use self::__PHImageManager::PHImageRequestID;
#[cfg(feature = "Photos_PHImageRequestOptions")]
pub use self::__PHImageManager::PHImageRequestOptions;
pub use self::__PHImageManager::PHImageRequestOptionsDeliveryMode;
pub use self::__PHImageManager::PHImageRequestOptionsResizeMode;
pub use self::__PHImageManager::PHImageRequestOptionsVersion;
pub use self::__PHImageManager::PHImageResultIsDegradedKey;
pub use self::__PHImageManager::PHImageResultIsInCloudKey;
pub use self::__PHImageManager::PHImageResultRequestIDKey;
pub use self::__PHImageManager::PHInvalidImageRequestID;
#[cfg(feature = "Photos_PHLivePhotoRequestOptions")]
pub use self::__PHImageManager::PHLivePhotoRequestOptions;
#[cfg(feature = "Photos_PHVideoRequestOptions")]
pub use self::__PHImageManager::PHVideoRequestOptions;
pub use self::__PHImageManager::PHVideoRequestOptionsDeliveryMode;
pub use self::__PHImageManager::PHVideoRequestOptionsVersion;
pub use self::__PHImageManager::{
    PHImageRequestOptionsDeliveryModeFastFormat,
    PHImageRequestOptionsDeliveryModeHighQualityFormat,
    PHImageRequestOptionsDeliveryModeOpportunistic,
};
pub use self::__PHImageManager::{
    PHImageRequestOptionsResizeModeExact, PHImageRequestOptionsResizeModeFast,
    PHImageRequestOptionsResizeModeNone,
};
pub use self::__PHImageManager::{
    PHImageRequestOptionsVersionCurrent, PHImageRequestOptionsVersionOriginal,
    PHImageRequestOptionsVersionUnadjusted,
};
pub use self::__PHImageManager::{
    PHVideoRequestOptionsDeliveryModeAutomatic, PHVideoRequestOptionsDeliveryModeFastFormat,
    PHVideoRequestOptionsDeliveryModeHighQualityFormat,
    PHVideoRequestOptionsDeliveryModeMediumQualityFormat,
};
pub use self::__PHImageManager::{
    PHVideoRequestOptionsVersionCurrent, PHVideoRequestOptionsVersionOriginal,
};
#[cfg(feature = "Photos_PHLivePhoto")]
pub use self::__PHLivePhoto::PHLivePhoto;
pub use self::__PHLivePhoto::PHLivePhotoInfoCancelledKey;
pub use self::__PHLivePhoto::PHLivePhotoInfoErrorKey;
pub use self::__PHLivePhoto::PHLivePhotoInfoIsDegradedKey;
pub use self::__PHLivePhoto::PHLivePhotoRequestID;
pub use self::__PHLivePhoto::PHLivePhotoRequestIDInvalid;
#[cfg(feature = "Photos_PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingContext;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingErrorCode;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingErrorDomain;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingOption;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrame;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrameType;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoShouldRenderAtPlaybackTime;
pub use self::__PHLivePhotoEditingContext::{
    PHLivePhotoEditingErrorCodeAborted, PHLivePhotoEditingErrorCodeUnknown,
};
pub use self::__PHLivePhotoEditingContext::{PHLivePhotoFrameTypePhoto, PHLivePhotoFrameTypeVideo};
#[cfg(feature = "Photos_PHObject")]
pub use self::__PHObject::PHObject;
#[cfg(feature = "Photos_PHObjectPlaceholder")]
pub use self::__PHObject::PHObjectPlaceholder;
#[cfg(feature = "Photos_PHPersistentChange")]
pub use self::__PHPersistentChange::PHPersistentChange;
#[cfg(feature = "Photos_PHPersistentChangeFetchResult")]
pub use self::__PHPersistentChangeFetchResult::PHPersistentChangeFetchResult;
#[cfg(feature = "Photos_PHPersistentChangeToken")]
pub use self::__PHPersistentChangeToken::PHPersistentChangeToken;
#[cfg(feature = "Photos_PHPersistentObjectChangeDetails")]
pub use self::__PHPersistentObjectChangeDetails::PHPersistentObjectChangeDetails;
pub use self::__PHPhotoLibrary::PHAccessLevel;
pub use self::__PHPhotoLibrary::PHAuthorizationStatus;
#[cfg(feature = "Photos_PHPhotoLibrary")]
pub use self::__PHPhotoLibrary::PHPhotoLibrary;
pub use self::__PHPhotoLibrary::PHPhotoLibraryAvailabilityObserver;
pub use self::__PHPhotoLibrary::PHPhotoLibraryChangeObserver;
pub use self::__PHPhotoLibrary::{PHAccessLevelAddOnly, PHAccessLevelReadWrite};
pub use self::__PHPhotoLibrary::{
    PHAuthorizationStatusAuthorized, PHAuthorizationStatusDenied, PHAuthorizationStatusLimited,
    PHAuthorizationStatusNotDetermined, PHAuthorizationStatusRestricted,
};
#[cfg(feature = "Photos_PHProject")]
pub use self::__PHProject::PHProject;
#[cfg(feature = "Photos_PHProjectChangeRequest")]
pub use self::__PHProjectChangeRequest::PHProjectChangeRequest;
pub use self::__PhotosTypes::PHAssetBurstSelectionType;
pub use self::__PhotosTypes::PHAssetCollectionSubtype;
pub use self::__PhotosTypes::PHAssetCollectionType;
pub use self::__PhotosTypes::PHAssetEditOperation;
pub use self::__PhotosTypes::PHAssetMediaSubtype;
pub use self::__PhotosTypes::PHAssetMediaType;
pub use self::__PhotosTypes::PHAssetPlaybackStyle;
pub use self::__PhotosTypes::PHAssetResourceType;
pub use self::__PhotosTypes::PHAssetSourceType;
pub use self::__PhotosTypes::PHCollectionEditOperation;
pub use self::__PhotosTypes::PHCollectionListSubtype;
pub use self::__PhotosTypes::PHCollectionListType;
pub use self::__PhotosTypes::PHImageContentMode;
pub use self::__PhotosTypes::PHObjectType;
pub use self::__PhotosTypes::{
    PHAssetBurstSelectionTypeAutoPick, PHAssetBurstSelectionTypeNone,
    PHAssetBurstSelectionTypeUserPick,
};
pub use self::__PhotosTypes::{
    PHAssetCollectionSubtypeAlbumCloudShared, PHAssetCollectionSubtypeAlbumImported,
    PHAssetCollectionSubtypeAlbumMyPhotoStream, PHAssetCollectionSubtypeAlbumRegular,
    PHAssetCollectionSubtypeAlbumSyncedAlbum, PHAssetCollectionSubtypeAlbumSyncedEvent,
    PHAssetCollectionSubtypeAlbumSyncedFaces, PHAssetCollectionSubtypeAny,
    PHAssetCollectionSubtypeSmartAlbumAllHidden, PHAssetCollectionSubtypeSmartAlbumAnimated,
    PHAssetCollectionSubtypeSmartAlbumBursts, PHAssetCollectionSubtypeSmartAlbumCinematic,
    PHAssetCollectionSubtypeSmartAlbumDepthEffect, PHAssetCollectionSubtypeSmartAlbumFavorites,
    PHAssetCollectionSubtypeSmartAlbumGeneric, PHAssetCollectionSubtypeSmartAlbumLivePhotos,
    PHAssetCollectionSubtypeSmartAlbumLongExposures, PHAssetCollectionSubtypeSmartAlbumPanoramas,
    PHAssetCollectionSubtypeSmartAlbumRAW, PHAssetCollectionSubtypeSmartAlbumRecentlyAdded,
    PHAssetCollectionSubtypeSmartAlbumScreenshots, PHAssetCollectionSubtypeSmartAlbumSelfPortraits,
    PHAssetCollectionSubtypeSmartAlbumSlomoVideos, PHAssetCollectionSubtypeSmartAlbumTimelapses,
    PHAssetCollectionSubtypeSmartAlbumUnableToUpload,
    PHAssetCollectionSubtypeSmartAlbumUserLibrary, PHAssetCollectionSubtypeSmartAlbumVideos,
};
pub use self::__PhotosTypes::{
    PHAssetCollectionTypeAlbum, PHAssetCollectionTypeMoment, PHAssetCollectionTypeSmartAlbum,
};
pub use self::__PhotosTypes::{
    PHAssetEditOperationContent, PHAssetEditOperationDelete, PHAssetEditOperationProperties,
};
pub use self::__PhotosTypes::{
    PHAssetMediaSubtypeNone, PHAssetMediaSubtypePhotoDepthEffect, PHAssetMediaSubtypePhotoHDR,
    PHAssetMediaSubtypePhotoLive, PHAssetMediaSubtypePhotoPanorama,
    PHAssetMediaSubtypePhotoScreenshot, PHAssetMediaSubtypeVideoCinematic,
    PHAssetMediaSubtypeVideoHighFrameRate, PHAssetMediaSubtypeVideoStreamed,
    PHAssetMediaSubtypeVideoTimelapse,
};
pub use self::__PhotosTypes::{
    PHAssetMediaTypeAudio, PHAssetMediaTypeImage, PHAssetMediaTypeUnknown, PHAssetMediaTypeVideo,
};
pub use self::__PhotosTypes::{
    PHAssetPlaybackStyleImage, PHAssetPlaybackStyleImageAnimated, PHAssetPlaybackStyleLivePhoto,
    PHAssetPlaybackStyleUnsupported, PHAssetPlaybackStyleVideo, PHAssetPlaybackStyleVideoLooping,
};
pub use self::__PhotosTypes::{
    PHAssetResourceTypeAdjustmentBasePairedVideo, PHAssetResourceTypeAdjustmentBasePhoto,
    PHAssetResourceTypeAdjustmentBaseVideo, PHAssetResourceTypeAdjustmentData,
    PHAssetResourceTypeAlternatePhoto, PHAssetResourceTypeAudio,
    PHAssetResourceTypeFullSizePairedVideo, PHAssetResourceTypeFullSizePhoto,
    PHAssetResourceTypeFullSizeVideo, PHAssetResourceTypePairedVideo, PHAssetResourceTypePhoto,
    PHAssetResourceTypePhotoProxy, PHAssetResourceTypeVideo,
};
pub use self::__PhotosTypes::{
    PHAssetSourceTypeCloudShared, PHAssetSourceTypeNone, PHAssetSourceTypeUserLibrary,
    PHAssetSourceTypeiTunesSynced,
};
pub use self::__PhotosTypes::{
    PHCollectionEditOperationAddContent, PHCollectionEditOperationCreateContent,
    PHCollectionEditOperationDelete, PHCollectionEditOperationDeleteContent,
    PHCollectionEditOperationRearrangeContent, PHCollectionEditOperationRemoveContent,
    PHCollectionEditOperationRename,
};
pub use self::__PhotosTypes::{
    PHCollectionListSubtypeAny, PHCollectionListSubtypeMomentListCluster,
    PHCollectionListSubtypeMomentListYear, PHCollectionListSubtypeRegularFolder,
    PHCollectionListSubtypeSmartFolderEvents, PHCollectionListSubtypeSmartFolderFaces,
};
pub use self::__PhotosTypes::{
    PHCollectionListTypeFolder, PHCollectionListTypeMomentList, PHCollectionListTypeSmartFolder,
};
pub use self::__PhotosTypes::{
    PHImageContentModeAspectFill, PHImageContentModeAspectFit, PHImageContentModeDefault,
};
pub use self::__PhotosTypes::{
    PHObjectTypeAsset, PHObjectTypeAssetCollection, PHObjectTypeCollectionList,
};
