// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

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
#![allow(clippy::doc_lazy_continuation)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::unportable_markdown)]
#![allow(rustdoc::invalid_html_tags)]

#[link(name = "ServiceManagement", kind = "framework")]
extern "C" {}

#[cfg(feature = "SMAppService")]
#[path = "SMAppService.rs"]
mod __SMAppService;
#[cfg(feature = "SMErrors")]
#[path = "SMErrors.rs"]
mod __SMErrors;
#[cfg(feature = "SMLoginItem")]
#[path = "SMLoginItem.rs"]
mod __SMLoginItem;

#[cfg(feature = "SMAppService")]
pub use self::__SMAppService::SMAppService;
#[cfg(feature = "SMAppService")]
pub use self::__SMAppService::SMAppServiceErrorDomain;
#[cfg(feature = "SMAppService")]
pub use self::__SMAppService::SMAppServiceStatus;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorAlreadyRegistered;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorAuthorizationFailure;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorInternalFailure;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorInvalidPlist;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorInvalidSignature;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorJobMustBeEnabled;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorJobNotFound;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorJobPlistNotFound;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorLaunchDeniedByUser;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorServiceUnavailable;
#[cfg(feature = "SMErrors")]
pub use self::__SMErrors::kSMErrorToolNotValid;
