//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-photos")]
#[cfg(not(target_os = "watchos"))]
use objc2_photos::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickerconfigurationassetrepresentationmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHPickerConfigurationAssetRepresentationMode(pub NSInteger);
impl PHPickerConfigurationAssetRepresentationMode {
    #[doc(alias = "PHPickerConfigurationAssetRepresentationModeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "PHPickerConfigurationAssetRepresentationModeCurrent")]
    pub const Current: Self = Self(1);
    #[doc(alias = "PHPickerConfigurationAssetRepresentationModeCompatible")]
    pub const Compatible: Self = Self(2);
}

unsafe impl Encode for PHPickerConfigurationAssetRepresentationMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHPickerConfigurationAssetRepresentationMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for PHPickerConfigurationAssetRepresentationMode {}

unsafe impl Sync for PHPickerConfigurationAssetRepresentationMode {}

/// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickerconfigurationselection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHPickerConfigurationSelection(pub NSInteger);
impl PHPickerConfigurationSelection {
    #[doc(alias = "PHPickerConfigurationSelectionDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "PHPickerConfigurationSelectionOrdered")]
    pub const Ordered: Self = Self(1);
    #[doc(alias = "PHPickerConfigurationSelectionContinuous")]
    pub const Continuous: Self = Self(2);
    #[doc(alias = "PHPickerConfigurationSelectionContinuousAndOrdered")]
    pub const ContinuousAndOrdered: Self = Self(3);
}

unsafe impl Encode for PHPickerConfigurationSelection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHPickerConfigurationSelection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for PHPickerConfigurationSelection {}

unsafe impl Sync for PHPickerConfigurationSelection {}

/// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHPickerMode(pub NSInteger);
impl PHPickerMode {
    #[doc(alias = "PHPickerModeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "PHPickerModeCompact")]
    pub const Compact: Self = Self(1);
}

unsafe impl Encode for PHPickerMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHPickerMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickercapabilities?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHPickerCapabilities(pub NSUInteger);
bitflags::bitflags! {
    impl PHPickerCapabilities: NSUInteger {
        #[doc(alias = "PHPickerCapabilitiesNone")]
        const None = 0;
        #[doc(alias = "PHPickerCapabilitiesSearch")]
        const Search = 1<<0;
        #[doc(alias = "PHPickerCapabilitiesStagingArea")]
        const StagingArea = 1<<1;
        #[doc(alias = "PHPickerCapabilitiesCollectionNavigation")]
        const CollectionNavigation = 1<<2;
        #[doc(alias = "PHPickerCapabilitiesSelectionActions")]
        const SelectionActions = 1<<3;
        #[doc(alias = "PHPickerCapabilitiesSensitivityAnalysisIntervention")]
        const SensitivityAnalysisIntervention = 1<<4;
    }
}

unsafe impl Encode for PHPickerCapabilities {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for PHPickerCapabilities {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickerfilter?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPickerFilter;
);

unsafe impl NSCopying for PHPickerFilter {}

unsafe impl CopyingHelper for PHPickerFilter {
    type Result = Self;
}

unsafe impl NSObjectProtocol for PHPickerFilter {}

extern_methods!(
    unsafe impl PHPickerFilter {
        #[method_id(@__retain_semantics Other imagesFilter)]
        pub unsafe fn imagesFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other videosFilter)]
        pub unsafe fn videosFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other livePhotosFilter)]
        pub unsafe fn livePhotosFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other depthEffectPhotosFilter)]
        pub unsafe fn depthEffectPhotosFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other burstsFilter)]
        pub unsafe fn burstsFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other panoramasFilter)]
        pub unsafe fn panoramasFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other screenshotsFilter)]
        pub unsafe fn screenshotsFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other screenRecordingsFilter)]
        pub unsafe fn screenRecordingsFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other cinematicVideosFilter)]
        pub unsafe fn cinematicVideosFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other slomoVideosFilter)]
        pub unsafe fn slomoVideosFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other timelapseVideosFilter)]
        pub unsafe fn timelapseVideosFilter() -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other spatialMediaFilter)]
        pub unsafe fn spatialMediaFilter() -> Retained<PHPickerFilter>;

        #[cfg(feature = "objc2-photos")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other playbackStyleFilter:)]
        pub unsafe fn playbackStyleFilter(
            playback_style: PHAssetPlaybackStyle,
        ) -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other anyFilterMatchingSubfilters:)]
        pub unsafe fn anyFilterMatchingSubfilters(
            subfilters: &NSArray<PHPickerFilter>,
        ) -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other allFilterMatchingSubfilters:)]
        pub unsafe fn allFilterMatchingSubfilters(
            subfilters: &NSArray<PHPickerFilter>,
        ) -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics Other notFilterOfSubfilter:)]
        pub unsafe fn notFilterOfSubfilter(subfilter: &PHPickerFilter) -> Retained<PHPickerFilter>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickerupdateconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPickerUpdateConfiguration;
);

unsafe impl NSCopying for PHPickerUpdateConfiguration {}

unsafe impl CopyingHelper for PHPickerUpdateConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for PHPickerUpdateConfiguration {}

extern_methods!(
    unsafe impl PHPickerUpdateConfiguration {
        #[method(selectionLimit)]
        pub unsafe fn selectionLimit(&self) -> NSInteger;

        #[method(setSelectionLimit:)]
        pub unsafe fn setSelectionLimit(&self, selection_limit: NSInteger);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(edgesWithoutContentMargins)]
        pub unsafe fn edgesWithoutContentMargins(&self) -> NSDirectionalRectEdge;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(setEdgesWithoutContentMargins:)]
        pub unsafe fn setEdgesWithoutContentMargins(
            &self,
            edges_without_content_margins: NSDirectionalRectEdge,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHPickerUpdateConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickerconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPickerConfiguration;
);

unsafe impl NSCopying for PHPickerConfiguration {}

unsafe impl CopyingHelper for PHPickerConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for PHPickerConfiguration {}

extern_methods!(
    unsafe impl PHPickerConfiguration {
        #[method(preferredAssetRepresentationMode)]
        pub unsafe fn preferredAssetRepresentationMode(
            &self,
        ) -> PHPickerConfigurationAssetRepresentationMode;

        #[method(setPreferredAssetRepresentationMode:)]
        pub unsafe fn setPreferredAssetRepresentationMode(
            &self,
            preferred_asset_representation_mode: PHPickerConfigurationAssetRepresentationMode,
        );

        #[method(selection)]
        pub unsafe fn selection(&self) -> PHPickerConfigurationSelection;

        #[method(setSelection:)]
        pub unsafe fn setSelection(&self, selection: PHPickerConfigurationSelection);

        #[method(selectionLimit)]
        pub unsafe fn selectionLimit(&self) -> NSInteger;

        #[method(setSelectionLimit:)]
        pub unsafe fn setSelectionLimit(&self, selection_limit: NSInteger);

        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Retained<PHPickerFilter>>;

        #[method(setFilter:)]
        pub unsafe fn setFilter(&self, filter: Option<&PHPickerFilter>);

        #[method_id(@__retain_semantics Other preselectedAssetIdentifiers)]
        pub unsafe fn preselectedAssetIdentifiers(&self) -> Retained<NSArray<NSString>>;

        #[method(setPreselectedAssetIdentifiers:)]
        pub unsafe fn setPreselectedAssetIdentifiers(
            &self,
            preselected_asset_identifiers: &NSArray<NSString>,
        );

        #[method(mode)]
        pub unsafe fn mode(&self) -> PHPickerMode;

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: PHPickerMode);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(edgesWithoutContentMargins)]
        pub unsafe fn edgesWithoutContentMargins(&self) -> NSDirectionalRectEdge;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(setEdgesWithoutContentMargins:)]
        pub unsafe fn setEdgesWithoutContentMargins(
            &self,
            edges_without_content_margins: NSDirectionalRectEdge,
        );

        #[method(disabledCapabilities)]
        pub unsafe fn disabledCapabilities(&self) -> PHPickerCapabilities;

        #[method(setDisabledCapabilities:)]
        pub unsafe fn setDisabledCapabilities(&self, disabled_capabilities: PHPickerCapabilities);

        #[cfg(feature = "objc2-photos")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Init initWithPhotoLibrary:)]
        pub unsafe fn initWithPhotoLibrary(
            this: Allocated<Self>,
            photo_library: &PHPhotoLibrary,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHPickerConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickerresult?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPickerResult;
);

unsafe impl NSObjectProtocol for PHPickerResult {}

extern_methods!(
    unsafe impl PHPickerResult {
        #[method_id(@__retain_semantics Other itemProvider)]
        pub unsafe fn itemProvider(&self) -> Retained<NSItemProvider>;

        #[method_id(@__retain_semantics Other assetIdentifier)]
        pub unsafe fn assetIdentifier(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickerviewcontrollerdelegate?language=objc)
    pub unsafe trait PHPickerViewControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(picker:didFinishPicking:)]
        unsafe fn picker_didFinishPicking(
            &self,
            picker: &PHPickerViewController,
            results: &NSArray<PHPickerResult>,
        );
    }

    unsafe impl ProtocolType for dyn PHPickerViewControllerDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phpickerviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct PHPickerViewController;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for PHPickerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for PHPickerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for PHPickerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for PHPickerViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for PHPickerViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl PHPickerViewController {
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Retained<PHPickerConfiguration>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn PHPickerViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn PHPickerViewControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &PHPickerConfiguration,
        ) -> Retained<Self>;

        #[method(updatePickerUsingConfiguration:)]
        pub unsafe fn updatePickerUsingConfiguration(
            &self,
            configuration: &PHPickerUpdateConfiguration,
        );

        #[method(deselectAssetsWithIdentifiers:)]
        pub unsafe fn deselectAssetsWithIdentifiers(&self, identifiers: &NSArray<NSString>);

        #[method(moveAssetWithIdentifier:afterAssetWithIdentifier:)]
        pub unsafe fn moveAssetWithIdentifier_afterAssetWithIdentifier(
            &self,
            identifier: &NSString,
            after_identifier: Option<&NSString>,
        );

        #[method(scrollToInitialPosition)]
        pub unsafe fn scrollToInitialPosition(&self);

        #[method(zoomIn)]
        pub unsafe fn zoomIn(&self);

        #[method(zoomOut)]
        pub unsafe fn zoomOut(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);
