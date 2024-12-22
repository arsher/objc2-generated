//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A class that contains miscellaneous metadata about an associated payload.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxmetadata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXMetaData;
);

unsafe impl NSCoding for MXMetaData {}

unsafe impl NSObjectProtocol for MXMetaData {}

unsafe impl NSSecureCoding for MXMetaData {}

extern_methods!(
    unsafe impl MXMetaData {
        /// An NSString designating the region format associated with the application.
        #[method_id(@__retain_semantics Other regionFormat)]
        pub unsafe fn regionFormat(&self) -> Retained<NSString>;

        /// An NSString designating the OS version associated with the device.
        #[method_id(@__retain_semantics Other osVersion)]
        pub unsafe fn osVersion(&self) -> Retained<NSString>;

        /// An NSString designating the device type associated with this device.
        #[method_id(@__retain_semantics Other deviceType)]
        pub unsafe fn deviceType(&self) -> Retained<NSString>;

        /// An NSString designating the app build version.
        #[method_id(@__retain_semantics Other applicationBuildVersion)]
        pub unsafe fn applicationBuildVersion(&self) -> Retained<NSString>;

        /// An NSString designating the current architecture.
        #[method_id(@__retain_semantics Other platformArchitecture)]
        pub unsafe fn platformArchitecture(&self) -> Retained<NSString>;

        /// A boolean representing low power mode enablement on device
        #[method(lowPowerModeEnabled)]
        pub unsafe fn lowPowerModeEnabled(&self) -> bool;

        /// A boolean representing if the app is registered as a testFlightApp
        #[method(isTestFlightApp)]
        pub unsafe fn isTestFlightApp(&self) -> bool;

        #[cfg(feature = "libc")]
        /// pid of the process
        ///
        /// Note: A value of -1 indicates that the PID was unavailable for the containing payload.
        #[method(pid)]
        pub unsafe fn pid(&self) -> libc::pid_t;

        /// Convenience method to return a JSON representation of this metadata.
        ///
        /// Returns: An NSData object containing the JSON representation
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Retained<NSData>;

        /// Convenience method to return a NSDictionary representation of this metadata.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[deprecated]
        #[method_id(@__retain_semantics Other DictionaryRepresentation)]
        pub unsafe fn DictionaryRepresentation(&self) -> Retained<NSDictionary>;

        /// Convenience method to return a NSDictionary representation of this metadata.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Retained<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXMetaData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
