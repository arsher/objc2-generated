//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[deprecated = "Not supported"]
pub const NSWindowsNTOperatingSystem: c_uint = 1;
#[deprecated = "Not supported"]
pub const NSWindows95OperatingSystem: c_uint = 2;
#[deprecated = "Not supported"]
pub const NSSolarisOperatingSystem: c_uint = 3;
#[deprecated = "Not supported"]
pub const NSHPUXOperatingSystem: c_uint = 4;
#[deprecated = "Not supported"]
pub const NSMACHOperatingSystem: c_uint = 5;
#[deprecated = "Not supported"]
pub const NSSunOSOperatingSystem: c_uint = 6;
#[deprecated = "Not supported"]
pub const NSOSF1OperatingSystem: c_uint = 7;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSOperatingSystemVersion {
    pub majorVersion: NSInteger,
    pub minorVersion: NSInteger,
    pub patchVersion: NSInteger,
}

unsafe impl Encode for NSOperatingSystemVersion {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <NSInteger>::ENCODING,
            <NSInteger>::ENCODING,
            <NSInteger>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for NSOperatingSystemVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for NSOperatingSystemVersion {}

unsafe impl Sync for NSOperatingSystemVersion {}

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSProcessInfo;

    unsafe impl ClassType for NSProcessInfo {
        type Super = NSObject;
    }
);

unsafe impl Send for NSProcessInfo {}

unsafe impl Sync for NSProcessInfo {}

unsafe impl NSObjectProtocol for NSProcessInfo {}

extern_methods!(
    unsafe impl NSProcessInfo {
        #[method_id(@__retain_semantics Other processInfo)]
        pub fn processInfo() -> Retained<NSProcessInfo>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other environment)]
        pub unsafe fn environment(&self) -> Retained<NSDictionary<NSString, NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other hostName)]
        pub unsafe fn hostName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other processName)]
        pub fn processName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method(setProcessName:)]
        pub unsafe fn setProcessName(&self, process_name: &NSString);

        #[method(processIdentifier)]
        pub unsafe fn processIdentifier(&self) -> c_int;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other globallyUniqueString)]
        pub unsafe fn globallyUniqueString(&self) -> Retained<NSString>;

        #[deprecated = "-operatingSystem always returns NSMACHOperatingSystem, use -operatingSystemVersion or -isOperatingSystemAtLeastVersion: instead"]
        #[method(operatingSystem)]
        pub unsafe fn operatingSystem(&self) -> NSUInteger;

        #[cfg(feature = "NSString")]
        #[deprecated = "-operatingSystemName always returns NSMACHOperatingSystem, use -operatingSystemVersionString instead"]
        #[method_id(@__retain_semantics Other operatingSystemName)]
        pub unsafe fn operatingSystemName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other operatingSystemVersionString)]
        pub unsafe fn operatingSystemVersionString(&self) -> Retained<NSString>;

        #[method(operatingSystemVersion)]
        pub fn operatingSystemVersion(&self) -> NSOperatingSystemVersion;

        #[method(processorCount)]
        pub unsafe fn processorCount(&self) -> NSUInteger;

        #[method(activeProcessorCount)]
        pub unsafe fn activeProcessorCount(&self) -> NSUInteger;

        #[method(physicalMemory)]
        pub unsafe fn physicalMemory(&self) -> c_ulonglong;

        #[method(isOperatingSystemAtLeastVersion:)]
        pub unsafe fn isOperatingSystemAtLeastVersion(
            &self,
            version: NSOperatingSystemVersion,
        ) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(systemUptime)]
        pub unsafe fn systemUptime(&self) -> NSTimeInterval;

        #[method(disableSuddenTermination)]
        pub unsafe fn disableSuddenTermination(&self);

        #[method(enableSuddenTermination)]
        pub unsafe fn enableSuddenTermination(&self);

        #[cfg(feature = "NSString")]
        #[method(disableAutomaticTermination:)]
        pub unsafe fn disableAutomaticTermination(&self, reason: &NSString);

        #[cfg(feature = "NSString")]
        #[method(enableAutomaticTermination:)]
        pub unsafe fn enableAutomaticTermination(&self, reason: &NSString);

        #[method(automaticTerminationSupportEnabled)]
        pub unsafe fn automaticTerminationSupportEnabled(&self) -> bool;

        #[method(setAutomaticTerminationSupportEnabled:)]
        pub unsafe fn setAutomaticTerminationSupportEnabled(
            &self,
            automatic_termination_support_enabled: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSProcessInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSActivityOptions(pub u64);
bitflags::bitflags! {
    impl NSActivityOptions: u64 {
        const NSActivityIdleDisplaySleepDisabled = 1<<40;
        const NSActivityIdleSystemSleepDisabled = 1<<20;
        const NSActivitySuddenTerminationDisabled = 1<<14;
        const NSActivityAutomaticTerminationDisabled = 1<<15;
        const NSActivityAnimationTrackingEnabled = 1<<45;
        const NSActivityTrackingEnabled = 1<<46;
        const NSActivityUserInitiated = 0x00FFFFFF|NSActivityOptions::NSActivityIdleSystemSleepDisabled.0;
        const NSActivityUserInitiatedAllowingIdleSystemSleep = NSActivityOptions::NSActivityUserInitiated.0&!NSActivityOptions::NSActivityIdleSystemSleepDisabled.0;
        const NSActivityBackground = 0x000000FF;
        const NSActivityLatencyCritical = 0xFF00000000;
        const NSActivityUserInteractive = NSActivityOptions::NSActivityUserInitiated.0|NSActivityOptions::NSActivityLatencyCritical.0;
    }
}

unsafe impl Encode for NSActivityOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for NSActivityOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSProcessInfoActivity
    unsafe impl NSProcessInfo {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other beginActivityWithOptions:reason:)]
        pub unsafe fn beginActivityWithOptions_reason(
            &self,
            options: NSActivityOptions,
            reason: &NSString,
        ) -> Retained<NSObject>;

        #[method(endActivity:)]
        pub unsafe fn endActivity(&self, activity: &NSObject);

        #[cfg(all(feature = "NSString", feature = "block2"))]
        #[method(performActivityWithOptions:reason:usingBlock:)]
        pub unsafe fn performActivityWithOptions_reason_usingBlock(
            &self,
            options: NSActivityOptions,
            reason: &NSString,
            block: &block2::Block<dyn Fn()>,
        );

        #[cfg(all(feature = "NSString", feature = "block2"))]
        #[method(performExpiringActivityWithReason:usingBlock:)]
        pub unsafe fn performExpiringActivityWithReason_usingBlock(
            &self,
            reason: &NSString,
            block: &block2::Block<dyn Fn(Bool)>,
        );
    }
);

extern_methods!(
    /// NSUserInformation
    unsafe impl NSProcessInfo {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other userName)]
        pub unsafe fn userName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other fullUserName)]
        pub unsafe fn fullUserName(&self) -> Retained<NSString>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSProcessInfoThermalState(pub NSInteger);
impl NSProcessInfoThermalState {
    #[doc(alias = "NSProcessInfoThermalStateNominal")]
    pub const Nominal: Self = Self(0);
    #[doc(alias = "NSProcessInfoThermalStateFair")]
    pub const Fair: Self = Self(1);
    #[doc(alias = "NSProcessInfoThermalStateSerious")]
    pub const Serious: Self = Self(2);
    #[doc(alias = "NSProcessInfoThermalStateCritical")]
    pub const Critical: Self = Self(3);
}

unsafe impl Encode for NSProcessInfoThermalState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSProcessInfoThermalState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSProcessInfoThermalState
    unsafe impl NSProcessInfo {
        #[method(thermalState)]
        pub unsafe fn thermalState(&self) -> NSProcessInfoThermalState;
    }
);

extern_methods!(
    /// NSProcessInfoPowerState
    unsafe impl NSProcessInfo {
        #[method(isLowPowerModeEnabled)]
        pub unsafe fn isLowPowerModeEnabled(&self) -> bool;
    }
);

extern "C" {
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSProcessInfoThermalStateDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSProcessInfoPowerStateDidChangeNotification: &'static NSNotificationName;
}

extern_methods!(
    /// NSProcessInfoPlatform
    unsafe impl NSProcessInfo {
        #[method(isMacCatalystApp)]
        pub unsafe fn isMacCatalystApp(&self) -> bool;

        #[method(isiOSAppOnMac)]
        pub unsafe fn isiOSAppOnMac(&self) -> bool;
    }
);
