//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidevicebatterystate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDeviceBatteryState(pub NSInteger);
impl UIDeviceBatteryState {
    #[doc(alias = "UIDeviceBatteryStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "UIDeviceBatteryStateUnplugged")]
    pub const Unplugged: Self = Self(1);
    #[doc(alias = "UIDeviceBatteryStateCharging")]
    pub const Charging: Self = Self(2);
    #[doc(alias = "UIDeviceBatteryStateFull")]
    pub const Full: Self = Self(3);
}

unsafe impl Encode for UIDeviceBatteryState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDeviceBatteryState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiuserinterfaceidiom?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserInterfaceIdiom(pub NSInteger);
impl UIUserInterfaceIdiom {
    #[doc(alias = "UIUserInterfaceIdiomUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UIUserInterfaceIdiomPhone")]
    pub const Phone: Self = Self(0);
    #[doc(alias = "UIUserInterfaceIdiomPad")]
    pub const Pad: Self = Self(1);
    #[doc(alias = "UIUserInterfaceIdiomTV")]
    pub const TV: Self = Self(2);
    #[doc(alias = "UIUserInterfaceIdiomCarPlay")]
    pub const CarPlay: Self = Self(3);
    #[doc(alias = "UIUserInterfaceIdiomMac")]
    pub const Mac: Self = Self(5);
    #[doc(alias = "UIUserInterfaceIdiomVision")]
    pub const Vision: Self = Self(6);
}

unsafe impl Encode for UIUserInterfaceIdiom {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIUserInterfaceIdiom {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidevice?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDevice;
);

unsafe impl NSObjectProtocol for UIDevice {}

extern_methods!(
    unsafe impl UIDevice {
        #[method_id(@__retain_semantics Other currentDevice)]
        pub fn currentDevice(mtm: MainThreadMarker) -> Retained<UIDevice>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other model)]
        pub unsafe fn model(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedModel)]
        pub unsafe fn localizedModel(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other systemName)]
        pub unsafe fn systemName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other systemVersion)]
        pub unsafe fn systemVersion(&self) -> Retained<NSString>;

        #[cfg(feature = "UIOrientation")]
        #[method(orientation)]
        pub unsafe fn orientation(&self) -> UIDeviceOrientation;

        #[method_id(@__retain_semantics Other identifierForVendor)]
        pub unsafe fn identifierForVendor(&self) -> Option<Retained<NSUUID>>;

        #[method(isGeneratingDeviceOrientationNotifications)]
        pub unsafe fn isGeneratingDeviceOrientationNotifications(&self) -> bool;

        #[method(beginGeneratingDeviceOrientationNotifications)]
        pub unsafe fn beginGeneratingDeviceOrientationNotifications(&self);

        #[method(endGeneratingDeviceOrientationNotifications)]
        pub unsafe fn endGeneratingDeviceOrientationNotifications(&self);

        #[method(isBatteryMonitoringEnabled)]
        pub unsafe fn isBatteryMonitoringEnabled(&self) -> bool;

        /// Setter for [`isBatteryMonitoringEnabled`][Self::isBatteryMonitoringEnabled].
        #[method(setBatteryMonitoringEnabled:)]
        pub unsafe fn setBatteryMonitoringEnabled(&self, battery_monitoring_enabled: bool);

        #[method(batteryState)]
        pub unsafe fn batteryState(&self) -> UIDeviceBatteryState;

        #[method(batteryLevel)]
        pub unsafe fn batteryLevel(&self) -> c_float;

        #[method(isProximityMonitoringEnabled)]
        pub unsafe fn isProximityMonitoringEnabled(&self) -> bool;

        /// Setter for [`isProximityMonitoringEnabled`][Self::isProximityMonitoringEnabled].
        #[method(setProximityMonitoringEnabled:)]
        pub unsafe fn setProximityMonitoringEnabled(&self, proximity_monitoring_enabled: bool);

        #[method(proximityState)]
        pub unsafe fn proximityState(&self) -> bool;

        #[method(isMultitaskingSupported)]
        pub unsafe fn isMultitaskingSupported(&self) -> bool;

        #[method(userInterfaceIdiom)]
        pub fn userInterfaceIdiom(&self) -> UIUserInterfaceIdiom;

        #[method(playInputClick)]
        pub unsafe fn playInputClick(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDevice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiinputviewaudiofeedback?language=objc)
    pub unsafe trait UIInputViewAudioFeedback: NSObjectProtocol + MainThreadOnly {
        #[optional]
        #[method(enableInputClicksWhenVisible)]
        unsafe fn enableInputClicksWhenVisible(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn UIInputViewAudioFeedback {}
);

// TODO: pub fn UI_USER_INTERFACE_IDIOM() -> UIUserInterfaceIdiom;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uideviceorientationdidchangenotification?language=objc)
    pub static UIDeviceOrientationDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidevicebatterystatedidchangenotification?language=objc)
    pub static UIDeviceBatteryStateDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidevicebatteryleveldidchangenotification?language=objc)
    pub static UIDeviceBatteryLevelDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uideviceproximitystatedidchangenotification?language=objc)
    pub static UIDeviceProximityStateDidChangeNotification: &'static NSNotificationName;
}
