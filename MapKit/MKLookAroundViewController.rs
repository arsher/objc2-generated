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

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklookaroundbadgeposition?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKLookAroundBadgePosition(pub NSInteger);
impl MKLookAroundBadgePosition {
    #[doc(alias = "MKLookAroundBadgePositionTopLeading")]
    pub const TopLeading: Self = Self(0);
    #[doc(alias = "MKLookAroundBadgePositionTopTrailing")]
    pub const TopTrailing: Self = Self(1);
    #[doc(alias = "MKLookAroundBadgePositionBottomTrailing")]
    pub const BottomTrailing: Self = Self(2);
}

unsafe impl Encode for MKLookAroundBadgePosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKLookAroundBadgePosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklookaroundviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct MKLookAroundViewController;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for MKLookAroundViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for MKLookAroundViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for MKLookAroundViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSSecureCoding for MKLookAroundViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for MKLookAroundViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for MKLookAroundViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKLookAroundViewController {
        #[cfg(feature = "MKLookAroundScene")]
        #[method_id(@__retain_semantics Init initWithScene:)]
        pub unsafe fn initWithScene(
            this: Allocated<Self>,
            scene: &MKLookAroundScene,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MKLookAroundViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MKLookAroundViewControllerDelegate>>,
        );

        #[cfg(feature = "MKLookAroundScene")]
        #[method_id(@__retain_semantics Other scene)]
        pub unsafe fn scene(&self) -> Option<Retained<MKLookAroundScene>>;

        #[cfg(feature = "MKLookAroundScene")]
        /// Setter for [`scene`][Self::scene].
        #[method(setScene:)]
        pub unsafe fn setScene(&self, scene: Option<&MKLookAroundScene>);

        #[method(isNavigationEnabled)]
        pub unsafe fn isNavigationEnabled(&self) -> bool;

        /// Setter for [`isNavigationEnabled`][Self::isNavigationEnabled].
        #[method(setNavigationEnabled:)]
        pub unsafe fn setNavigationEnabled(&self, navigation_enabled: bool);

        #[method(showsRoadLabels)]
        pub unsafe fn showsRoadLabels(&self) -> bool;

        /// Setter for [`showsRoadLabels`][Self::showsRoadLabels].
        #[method(setShowsRoadLabels:)]
        pub unsafe fn setShowsRoadLabels(&self, shows_road_labels: bool);

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        /// Setter for [`pointOfInterestFilter`][Self::pointOfInterestFilter].
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method(badgePosition)]
        pub unsafe fn badgePosition(&self) -> MKLookAroundBadgePosition;

        /// Setter for [`badgePosition`][Self::badgePosition].
        #[method(setBadgePosition:)]
        pub unsafe fn setBadgePosition(&self, badge_position: MKLookAroundBadgePosition);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKLookAroundViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKLookAroundViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklookaroundviewcontrollerdelegate?language=objc)
    pub unsafe trait MKLookAroundViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(lookAroundViewControllerWillUpdateScene:)]
        unsafe fn lookAroundViewControllerWillUpdateScene(
            &self,
            view_controller: &MKLookAroundViewController,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(lookAroundViewControllerDidUpdateScene:)]
        unsafe fn lookAroundViewControllerDidUpdateScene(
            &self,
            view_controller: &MKLookAroundViewController,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(lookAroundViewControllerWillPresentFullScreen:)]
        unsafe fn lookAroundViewControllerWillPresentFullScreen(
            &self,
            view_controller: &MKLookAroundViewController,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(lookAroundViewControllerDidPresentFullScreen:)]
        unsafe fn lookAroundViewControllerDidPresentFullScreen(
            &self,
            view_controller: &MKLookAroundViewController,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(lookAroundViewControllerWillDismissFullScreen:)]
        unsafe fn lookAroundViewControllerWillDismissFullScreen(
            &self,
            view_controller: &MKLookAroundViewController,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(lookAroundViewControllerDidDismissFullScreen:)]
        unsafe fn lookAroundViewControllerDidDismissFullScreen(
            &self,
            view_controller: &MKLookAroundViewController,
        );
    }

    unsafe impl ProtocolType for dyn MKLookAroundViewControllerDelegate {}
);
