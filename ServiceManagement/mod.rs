// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `ServiceManagement` framework
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

#[link(name = "ServiceManagement", kind = "framework")]
extern "C" {}

#[path = "SMAppService.rs"]
mod __SMAppService;
#[path = "SMErrors.rs"]
mod __SMErrors;
#[path = "SMLoginItem.rs"]
mod __SMLoginItem;

#[cfg(feature = "ServiceManagement_SMAppService")]
pub use self::__SMAppService::SMAppService;
pub use self::__SMAppService::SMAppServiceStatus;
pub use self::__SMAppService::{
    SMAppServiceStatusEnabled, SMAppServiceStatusNotFound, SMAppServiceStatusNotRegistered,
    SMAppServiceStatusRequiresApproval,
};
pub use self::__SMErrors::{
    kSMErrorAlreadyRegistered, kSMErrorAuthorizationFailure, kSMErrorInternalFailure,
    kSMErrorInvalidPlist, kSMErrorInvalidSignature, kSMErrorJobMustBeEnabled, kSMErrorJobNotFound,
    kSMErrorJobPlistNotFound, kSMErrorLaunchDeniedByUser, kSMErrorServiceUnavailable,
    kSMErrorToolNotValid,
};
