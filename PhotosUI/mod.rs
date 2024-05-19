// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

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

#[link(name = "PhotosUI", kind = "framework")]
extern "C" {}

#[cfg(feature = "PHContentEditingController")]
#[path = "PHContentEditingController.rs"]
mod __PHContentEditingController;
#[cfg(feature = "PHLivePhotoView")]
#[path = "PHLivePhotoView.rs"]
mod __PHLivePhotoView;
#[cfg(feature = "PHPicker")]
#[path = "PHPicker.rs"]
mod __PHPicker;
#[cfg(feature = "PHProjectExtensionContext")]
#[path = "PHProjectExtensionContext.rs"]
mod __PHProjectExtensionContext;
#[cfg(feature = "PHProjectExtensionController")]
#[path = "PHProjectExtensionController.rs"]
mod __PHProjectExtensionController;
#[cfg(feature = "PHProjectInfo")]
#[path = "PHProjectInfo.rs"]
mod __PHProjectInfo;
#[cfg(feature = "PHProjectTypeDescription")]
#[path = "PHProjectTypeDescription.rs"]
mod __PHProjectTypeDescription;
#[cfg(feature = "PHProjectTypeDescriptionDataSource")]
#[path = "PHProjectTypeDescriptionDataSource.rs"]
mod __PHProjectTypeDescriptionDataSource;
#[cfg(feature = "PhotosUITypes")]
#[path = "PhotosUITypes.rs"]
mod __PhotosUITypes;

#[cfg(feature = "PHContentEditingController")]
pub use self::__PHContentEditingController::PHContentEditingController;
#[cfg(all(feature = "PHLivePhotoView", feature = "objc2-app-kit"))]
pub use self::__PHLivePhotoView::PHLivePhotoView;
#[cfg(feature = "PHLivePhotoView")]
pub use self::__PHLivePhotoView::PHLivePhotoViewContentMode;
#[cfg(feature = "PHLivePhotoView")]
pub use self::__PHLivePhotoView::PHLivePhotoViewDelegate;
#[cfg(feature = "PHLivePhotoView")]
pub use self::__PHLivePhotoView::PHLivePhotoViewPlaybackStyle;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerCapabilities;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerConfiguration;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerConfigurationAssetRepresentationMode;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerConfigurationSelection;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerFilter;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerMode;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerResult;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerUpdateConfiguration;
#[cfg(all(feature = "PHPicker", feature = "objc2-app-kit"))]
pub use self::__PHPicker::PHPickerViewController;
#[cfg(feature = "PHPicker")]
pub use self::__PHPicker::PHPickerViewControllerDelegate;
#[cfg(feature = "PHProjectExtensionContext")]
pub use self::__PHProjectExtensionContext::PHProjectExtensionContext;
#[cfg(feature = "PHProjectExtensionController")]
pub use self::__PHProjectExtensionController::PHProjectExtensionController;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectAssetElement;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectCreationSource;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectElement;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectInfo;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectJournalEntryElement;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectMapElement;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectRegionOfInterest;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectRegionOfInterestIdentifier;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectSection;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectSectionContent;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectSectionType;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectTextElement;
#[cfg(feature = "PHProjectInfo")]
pub use self::__PHProjectInfo::PHProjectTextElementType;
#[cfg(feature = "PHProjectTypeDescription")]
pub use self::__PHProjectTypeDescription::PHProjectTypeDescription;
#[cfg(feature = "PHProjectTypeDescriptionDataSource")]
pub use self::__PHProjectTypeDescriptionDataSource::PHProjectTypeDescriptionDataSource;
#[cfg(feature = "PHProjectTypeDescriptionDataSource")]
pub use self::__PHProjectTypeDescriptionDataSource::PHProjectTypeDescriptionInvalidator;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategory;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategoryBook;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategoryCalendar;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategoryCard;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategoryOther;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategoryPrints;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategorySlideshow;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategoryUndefined;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectCategoryWallDecor;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectType;
#[cfg(feature = "PhotosUITypes")]
pub use self::__PhotosUITypes::PHProjectTypeUndefined;
