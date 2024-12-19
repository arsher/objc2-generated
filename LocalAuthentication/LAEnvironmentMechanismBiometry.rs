//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/laenvironmentmechanismbiometry?language=objc)
    #[unsafe(super(LAEnvironmentMechanism, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LAEnvironmentMechanism")]
    pub struct LAEnvironmentMechanismBiometry;
);

#[cfg(feature = "LAEnvironmentMechanism")]
unsafe impl Send for LAEnvironmentMechanismBiometry {}

#[cfg(feature = "LAEnvironmentMechanism")]
unsafe impl Sync for LAEnvironmentMechanismBiometry {}

#[cfg(feature = "LAEnvironmentMechanism")]
unsafe impl NSObjectProtocol for LAEnvironmentMechanismBiometry {}

extern_methods!(
    #[cfg(feature = "LAEnvironmentMechanism")]
    unsafe impl LAEnvironmentMechanismBiometry {
        #[cfg(feature = "LABiometryType")]
        #[method(biometryType)]
        pub unsafe fn biometryType(&self) -> LABiometryType;

        #[method(isEnrolled)]
        pub unsafe fn isEnrolled(&self) -> bool;

        #[method(isLockedOut)]
        pub unsafe fn isLockedOut(&self) -> bool;

        #[method_id(@__retain_semantics Other stateHash)]
        pub unsafe fn stateHash(&self) -> Retained<NSData>;

        #[method(builtInSensorInaccessible)]
        pub unsafe fn builtInSensorInaccessible(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `LAEnvironmentMechanism`
    #[cfg(feature = "LAEnvironmentMechanism")]
    unsafe impl LAEnvironmentMechanismBiometry {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
