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

#[link(name = "DataDetection", kind = "framework")]
extern "C" {}

#[cfg(feature = "DDMatch")]
#[path = "DDMatch.rs"]
mod __DDMatch;
#[cfg(feature = "DataDetectionBase")]
#[path = "DataDetectionBase.rs"]
mod __DataDetectionBase;

#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatch;
#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatchCalendarEvent;
#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatchEmailAddress;
#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatchFlightNumber;
#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatchLink;
#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatchMoneyAmount;
#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatchPhoneNumber;
#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatchPostalAddress;
#[cfg(feature = "DDMatch")]
pub use self::__DDMatch::DDMatchShipmentTrackingNumber;
