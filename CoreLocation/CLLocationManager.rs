//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cldeviceorientation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLDeviceOrientation(pub c_int);
impl CLDeviceOrientation {
    #[doc(alias = "CLDeviceOrientationUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CLDeviceOrientationPortrait")]
    pub const Portrait: Self = Self(1);
    #[doc(alias = "CLDeviceOrientationPortraitUpsideDown")]
    pub const PortraitUpsideDown: Self = Self(2);
    #[doc(alias = "CLDeviceOrientationLandscapeLeft")]
    pub const LandscapeLeft: Self = Self(3);
    #[doc(alias = "CLDeviceOrientationLandscapeRight")]
    pub const LandscapeRight: Self = Self(4);
    #[doc(alias = "CLDeviceOrientationFaceUp")]
    pub const FaceUp: Self = Self(5);
    #[doc(alias = "CLDeviceOrientationFaceDown")]
    pub const FaceDown: Self = Self(6);
}

unsafe impl Encode for CLDeviceOrientation {
    const ENCODING: Encoding = c_int::ENCODING;
}

unsafe impl RefEncode for CLDeviceOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLAuthorizationStatus(pub c_int);
impl CLAuthorizationStatus {
    pub const kCLAuthorizationStatusNotDetermined: Self = Self(0);
    pub const kCLAuthorizationStatusRestricted: Self = Self(1);
    pub const kCLAuthorizationStatusDenied: Self = Self(2);
    pub const kCLAuthorizationStatusAuthorizedAlways: Self = Self(3);
    pub const kCLAuthorizationStatusAuthorizedWhenInUse: Self = Self(4);
    #[deprecated = "Use kCLAuthorizationStatusAuthorizedAlways"]
    pub const kCLAuthorizationStatusAuthorized: Self =
        Self(CLAuthorizationStatus::kCLAuthorizationStatusAuthorizedAlways.0);
}

unsafe impl Encode for CLAuthorizationStatus {
    const ENCODING: Encoding = c_int::ENCODING;
}

unsafe impl RefEncode for CLAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/claccuracyauthorization?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLAccuracyAuthorization(pub NSInteger);
impl CLAccuracyAuthorization {
    #[doc(alias = "CLAccuracyAuthorizationFullAccuracy")]
    pub const FullAccuracy: Self = Self(0);
    #[doc(alias = "CLAccuracyAuthorizationReducedAccuracy")]
    pub const ReducedAccuracy: Self = Self(1);
}

unsafe impl Encode for CLAccuracyAuthorization {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLAccuracyAuthorization {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clactivitytype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLActivityType(pub NSInteger);
impl CLActivityType {
    #[doc(alias = "CLActivityTypeOther")]
    pub const Other: Self = Self(1);
    #[doc(alias = "CLActivityTypeAutomotiveNavigation")]
    pub const AutomotiveNavigation: Self = Self(2);
    #[doc(alias = "CLActivityTypeFitness")]
    pub const Fitness: Self = Self(3);
    #[doc(alias = "CLActivityTypeOtherNavigation")]
    pub const OtherNavigation: Self = Self(4);
    #[doc(alias = "CLActivityTypeAirborne")]
    pub const Airborne: Self = Self(5);
}

unsafe impl Encode for CLActivityType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLActivityType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocationManager;
);

unsafe impl NSObjectProtocol for CLLocationManager {}

extern_methods!(
    unsafe impl CLLocationManager {
        #[method(locationServicesEnabled)]
        pub unsafe fn locationServicesEnabled_class() -> bool;

        #[method(headingAvailable)]
        pub unsafe fn headingAvailable_class() -> bool;

        #[method(significantLocationChangeMonitoringAvailable)]
        pub unsafe fn significantLocationChangeMonitoringAvailable() -> bool;

        #[method(isMonitoringAvailableForClass:)]
        pub unsafe fn isMonitoringAvailableForClass(region_class: &AnyClass) -> bool;

        #[deprecated]
        #[method(regionMonitoringAvailable)]
        pub unsafe fn regionMonitoringAvailable() -> bool;

        #[deprecated = "Use +isMonitoringAvailableForClass: and -authorizationStatus instead"]
        #[method(regionMonitoringEnabled)]
        pub unsafe fn regionMonitoringEnabled() -> bool;

        #[method(isRangingAvailable)]
        pub unsafe fn isRangingAvailable() -> bool;

        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus(&self) -> CLAuthorizationStatus;

        #[deprecated]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus_class() -> CLAuthorizationStatus;

        #[method(accuracyAuthorization)]
        pub unsafe fn accuracyAuthorization(&self) -> CLAccuracyAuthorization;

        #[method(isAuthorizedForWidgetUpdates)]
        pub unsafe fn isAuthorizedForWidgetUpdates(&self) -> bool;

        #[cfg(feature = "CLLocationManagerDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn CLLocationManagerDelegate>>>;

        #[cfg(feature = "CLLocationManagerDelegate")]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CLLocationManagerDelegate>>,
        );

        #[deprecated]
        #[method(locationServicesEnabled)]
        pub unsafe fn locationServicesEnabled(&self) -> bool;

        #[deprecated = "Set the purpose string in Info.plist using key NSLocationUsageDescription"]
        #[method_id(@__retain_semantics Other purpose)]
        pub unsafe fn purpose(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Set the purpose string in Info.plist using key NSLocationUsageDescription"]
        #[method(setPurpose:)]
        pub unsafe fn setPurpose(&self, purpose: Option<&NSString>);

        #[method(activityType)]
        pub unsafe fn activityType(&self) -> CLActivityType;

        #[method(setActivityType:)]
        pub unsafe fn setActivityType(&self, activity_type: CLActivityType);

        #[cfg(feature = "CLLocation")]
        #[method(distanceFilter)]
        pub unsafe fn distanceFilter(&self) -> CLLocationDistance;

        #[cfg(feature = "CLLocation")]
        #[method(setDistanceFilter:)]
        pub unsafe fn setDistanceFilter(&self, distance_filter: CLLocationDistance);

        #[cfg(feature = "CLLocation")]
        #[method(desiredAccuracy)]
        pub unsafe fn desiredAccuracy(&self) -> CLLocationAccuracy;

        #[cfg(feature = "CLLocation")]
        #[method(setDesiredAccuracy:)]
        pub unsafe fn setDesiredAccuracy(&self, desired_accuracy: CLLocationAccuracy);

        #[method(pausesLocationUpdatesAutomatically)]
        pub unsafe fn pausesLocationUpdatesAutomatically(&self) -> bool;

        #[method(setPausesLocationUpdatesAutomatically:)]
        pub unsafe fn setPausesLocationUpdatesAutomatically(
            &self,
            pauses_location_updates_automatically: bool,
        );

        #[method(allowsBackgroundLocationUpdates)]
        pub unsafe fn allowsBackgroundLocationUpdates(&self) -> bool;

        #[method(setAllowsBackgroundLocationUpdates:)]
        pub unsafe fn setAllowsBackgroundLocationUpdates(
            &self,
            allows_background_location_updates: bool,
        );

        #[method(showsBackgroundLocationIndicator)]
        pub unsafe fn showsBackgroundLocationIndicator(&self) -> bool;

        #[method(setShowsBackgroundLocationIndicator:)]
        pub unsafe fn setShowsBackgroundLocationIndicator(
            &self,
            shows_background_location_indicator: bool,
        );

        #[cfg(feature = "CLLocation")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Retained<CLLocation>>;

        #[deprecated]
        #[method(headingAvailable)]
        pub unsafe fn headingAvailable(&self) -> bool;

        #[cfg(feature = "CLLocation")]
        #[method(headingFilter)]
        pub unsafe fn headingFilter(&self) -> CLLocationDegrees;

        #[cfg(feature = "CLLocation")]
        #[method(setHeadingFilter:)]
        pub unsafe fn setHeadingFilter(&self, heading_filter: CLLocationDegrees);

        #[method(headingOrientation)]
        pub unsafe fn headingOrientation(&self) -> CLDeviceOrientation;

        #[method(setHeadingOrientation:)]
        pub unsafe fn setHeadingOrientation(&self, heading_orientation: CLDeviceOrientation);

        #[cfg(feature = "CLHeading")]
        #[method_id(@__retain_semantics Other heading)]
        pub unsafe fn heading(&self) -> Option<Retained<CLHeading>>;

        #[cfg(feature = "CLLocation")]
        #[method(maximumRegionMonitoringDistance)]
        pub unsafe fn maximumRegionMonitoringDistance(&self) -> CLLocationDistance;

        #[cfg(feature = "CLRegion")]
        #[method_id(@__retain_semantics Other monitoredRegions)]
        pub unsafe fn monitoredRegions(&self) -> Retained<NSSet<CLRegion>>;

        #[cfg(feature = "CLRegion")]
        #[deprecated = "Use -rangedBeaconConstraints"]
        #[method_id(@__retain_semantics Other rangedRegions)]
        pub unsafe fn rangedRegions(&self) -> Retained<NSSet<CLRegion>>;

        #[cfg(all(
            feature = "CLBeaconIdentityCondition",
            feature = "CLBeaconIdentityConstraint",
            feature = "CLCondition"
        ))]
        #[method_id(@__retain_semantics Other rangedBeaconConstraints)]
        pub unsafe fn rangedBeaconConstraints(&self)
            -> Retained<NSSet<CLBeaconIdentityConstraint>>;

        #[method(requestWhenInUseAuthorization)]
        pub unsafe fn requestWhenInUseAuthorization(&self);

        #[method(requestAlwaysAuthorization)]
        pub unsafe fn requestAlwaysAuthorization(&self);

        #[cfg(feature = "block2")]
        #[method(requestTemporaryFullAccuracyAuthorizationWithPurposeKey:completion:)]
        pub unsafe fn requestTemporaryFullAccuracyAuthorizationWithPurposeKey_completion(
            &self,
            purpose_key: &NSString,
            completion: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[method(requestTemporaryFullAccuracyAuthorizationWithPurposeKey:)]
        pub unsafe fn requestTemporaryFullAccuracyAuthorizationWithPurposeKey(
            &self,
            purpose_key: &NSString,
        );

        #[method(startUpdatingLocation)]
        pub unsafe fn startUpdatingLocation(&self);

        #[method(stopUpdatingLocation)]
        pub unsafe fn stopUpdatingLocation(&self);

        #[method(requestLocation)]
        pub unsafe fn requestLocation(&self);

        #[method(startUpdatingHeading)]
        pub unsafe fn startUpdatingHeading(&self);

        #[method(stopUpdatingHeading)]
        pub unsafe fn stopUpdatingHeading(&self);

        #[method(dismissHeadingCalibrationDisplay)]
        pub unsafe fn dismissHeadingCalibrationDisplay(&self);

        #[method(startMonitoringSignificantLocationChanges)]
        pub unsafe fn startMonitoringSignificantLocationChanges(&self);

        #[method(stopMonitoringSignificantLocationChanges)]
        pub unsafe fn stopMonitoringSignificantLocationChanges(&self);

        #[cfg(feature = "block2")]
        #[method(startMonitoringLocationPushesWithCompletion:)]
        pub unsafe fn startMonitoringLocationPushesWithCompletion(
            &self,
            completion: Option<&block2::Block<dyn Fn(*mut NSData, *mut NSError)>>,
        );

        #[method(stopMonitoringLocationPushes)]
        pub unsafe fn stopMonitoringLocationPushes(&self);

        #[cfg(all(feature = "CLLocation", feature = "CLRegion"))]
        #[deprecated]
        #[method(startMonitoringForRegion:desiredAccuracy:)]
        pub unsafe fn startMonitoringForRegion_desiredAccuracy(
            &self,
            region: &CLRegion,
            accuracy: CLLocationAccuracy,
        );

        #[cfg(feature = "CLRegion")]
        #[deprecated]
        #[method(stopMonitoringForRegion:)]
        pub unsafe fn stopMonitoringForRegion(&self, region: &CLRegion);

        #[cfg(feature = "CLRegion")]
        #[deprecated]
        #[method(startMonitoringForRegion:)]
        pub unsafe fn startMonitoringForRegion(&self, region: &CLRegion);

        #[cfg(feature = "CLRegion")]
        #[deprecated]
        #[method(requestStateForRegion:)]
        pub unsafe fn requestStateForRegion(&self, region: &CLRegion);

        #[cfg(all(feature = "CLBeaconRegion", feature = "CLRegion"))]
        #[deprecated = "Use -startRangingBeaconsSatisfyingConstraint:"]
        #[method(startRangingBeaconsInRegion:)]
        pub unsafe fn startRangingBeaconsInRegion(&self, region: &CLBeaconRegion);

        #[cfg(all(feature = "CLBeaconRegion", feature = "CLRegion"))]
        #[deprecated = "Use -stopRangingBeaconsSatisfyingConstraint:"]
        #[method(stopRangingBeaconsInRegion:)]
        pub unsafe fn stopRangingBeaconsInRegion(&self, region: &CLBeaconRegion);

        #[cfg(all(
            feature = "CLBeaconIdentityCondition",
            feature = "CLBeaconIdentityConstraint",
            feature = "CLCondition"
        ))]
        #[method(startRangingBeaconsSatisfyingConstraint:)]
        pub unsafe fn startRangingBeaconsSatisfyingConstraint(
            &self,
            constraint: &CLBeaconIdentityConstraint,
        );

        #[cfg(all(
            feature = "CLBeaconIdentityCondition",
            feature = "CLBeaconIdentityConstraint",
            feature = "CLCondition"
        ))]
        #[method(stopRangingBeaconsSatisfyingConstraint:)]
        pub unsafe fn stopRangingBeaconsSatisfyingConstraint(
            &self,
            constraint: &CLBeaconIdentityConstraint,
        );

        #[cfg(feature = "CLLocation")]
        #[deprecated = "You can remove calls to this method"]
        #[method(allowDeferredLocationUpdatesUntilTraveled:timeout:)]
        pub unsafe fn allowDeferredLocationUpdatesUntilTraveled_timeout(
            &self,
            distance: CLLocationDistance,
            timeout: NSTimeInterval,
        );

        #[deprecated = "You can remove calls to this method"]
        #[method(disallowDeferredLocationUpdates)]
        pub unsafe fn disallowDeferredLocationUpdates(&self);

        #[deprecated = "You can remove calls to this method"]
        #[method(deferredLocationUpdatesAvailable)]
        pub unsafe fn deferredLocationUpdatesAvailable() -> bool;

        #[cfg(all(feature = "CLLocation", feature = "block2"))]
        #[method(requestHistoricalLocationsWithPurposeKey:sampleCount:completionHandler:)]
        pub unsafe fn requestHistoricalLocationsWithPurposeKey_sampleCount_completionHandler(
            &self,
            purpose_key: &NSString,
            sample_count: NSInteger,
            handler: &block2::Block<dyn Fn(NonNull<NSArray<CLLocation>>, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLLocationManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
