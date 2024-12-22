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

#[link(name = "ExceptionHandling", kind = "framework")]
extern "C" {}

#[cfg(feature = "ExceptionHandlingDefines")]
#[path = "ExceptionHandlingDefines.rs"]
mod __ExceptionHandlingDefines;
#[cfg(feature = "NSExceptionHandler")]
#[path = "NSExceptionHandler.rs"]
mod __NSExceptionHandler;

#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSExceptionHandler;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSExceptionHandlerResume;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHandleOtherExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHandleTopLevelExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHandleUncaughtExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHandleUncaughtRuntimeErrorMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHandleUncaughtSystemExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHangOnOtherExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHangOnTopLevelExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHangOnUncaughtExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHangOnUncaughtRuntimeErrorMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSHangOnUncaughtSystemExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSLogOtherExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSLogTopLevelExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSLogUncaughtExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSLogUncaughtRuntimeErrorMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSLogUncaughtSystemExceptionMask;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSObjectNSExceptionHandlerDelegate;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSStackTraceKey;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSUncaughtRuntimeErrorException;
#[cfg(feature = "NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSUncaughtSystemExceptionException;
