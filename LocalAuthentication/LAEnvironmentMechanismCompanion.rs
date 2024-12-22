//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/laenvironmentmechanismcompanion?language=objc)
    #[unsafe(super(LAEnvironmentMechanism, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LAEnvironmentMechanism")]
    pub struct LAEnvironmentMechanismCompanion;
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
        /// Type of the companion.
        #[method(type)]
        pub unsafe fn r#type(&self) -> LACompanionType;

        /// Hash of the current companion pairing as returned by
        /// `LAContext.domainState.companion.stateHash(for:)`
        /// If no companion are paired for this companion type,
        /// `stateHash`property is
        /// `nil.`If at least one companion is paired for this companion type,
        /// `stateHash`is not
        /// `nil`and
        /// it changes whenever the set of paired companions of this type is changed.
        #[method_id(@__retain_semantics Other stateHash)]
        pub unsafe fn stateHash(&self) -> Option<Retained<NSData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `LAEnvironmentMechanism`
    #[cfg(feature = "LAEnvironmentMechanism")]
    unsafe impl LAEnvironmentMechanismCompanion {
        /// Clients should only consume environment mechanisms..
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The Clients should only consume environment mechanisms..
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
