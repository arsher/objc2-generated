//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LAEnvironmentMechanism")]
    pub struct LAEnvironmentMechanismCompanion;

    #[cfg(feature = "LAEnvironmentMechanism")]
    unsafe impl ClassType for LAEnvironmentMechanismCompanion {
        #[inherits(NSObject)]
        type Super = LAEnvironmentMechanism;
    }
);

#[cfg(feature = "LAEnvironmentMechanism")]
unsafe impl Send for LAEnvironmentMechanismCompanion {}

#[cfg(feature = "LAEnvironmentMechanism")]
unsafe impl Sync for LAEnvironmentMechanismCompanion {}

#[cfg(feature = "LAEnvironmentMechanism")]
unsafe impl NSObjectProtocol for LAEnvironmentMechanismCompanion {}

extern_methods!(
    #[cfg(feature = "LAEnvironmentMechanism")]
    unsafe impl LAEnvironmentMechanismCompanion {
        #[cfg(feature = "LACompanionType")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> LACompanionType;

        #[method_id(@__retain_semantics Other stateHash)]
        pub unsafe fn stateHash(&self) -> Option<Retained<NSData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `LAEnvironmentMechanism`
    #[cfg(feature = "LAEnvironmentMechanism")]
    unsafe impl LAEnvironmentMechanismCompanion {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
