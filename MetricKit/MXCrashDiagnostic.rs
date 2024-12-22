//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An MXDiagnostic subclass that encapsulates crash reports.
    ///
    /// See "Analyzing a Crash Report" for more information on crash diagnostics.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxcrashdiagnostic?language=objc)
    #[unsafe(super(MXDiagnostic, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXDiagnostic")]
    pub struct MXCrashDiagnostic;
);

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSCoding for MXCrashDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSObjectProtocol for MXCrashDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSSecureCoding for MXCrashDiagnostic {}

extern_methods!(
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXCrashDiagnostic {
        #[cfg(feature = "MXCallStackTree")]
        /// The application call stack tree associated with this crash.
        ///
        /// This call stack tree includes those stack frames present at the time of the crash.
        #[method_id(@__retain_semantics Other callStackTree)]
        pub unsafe fn callStackTree(&self) -> Retained<MXCallStackTree>;

        /// The termination reason associated with this crash.
        ///
        /// Exit reason information specified when a process is terminated. Key system components, both inside and outside of a process, will terminate the process upon encountering a fatal error (e.g. a bad code signature, a missing dependent library, or accessing privacy sensitive information without the proper entitlement).
        #[method_id(@__retain_semantics Other terminationReason)]
        pub unsafe fn terminationReason(&self) -> Option<Retained<NSString>>;

        /// Details about memory that the app incorrectly accessed in relation to other sections of the app’s virtual memory address space.
        ///
        /// This property is set when a bad memory access crash occurs.
        #[method_id(@__retain_semantics Other virtualMemoryRegionInfo)]
        pub unsafe fn virtualMemoryRegionInfo(&self) -> Option<Retained<NSString>>;

        /// The name of the Mach exception that terminated the app.
        ///
        /// See: sys/exception_types.h
        #[method_id(@__retain_semantics Other exceptionType)]
        pub unsafe fn exceptionType(&self) -> Option<Retained<NSNumber>>;

        /// Processor specific information about the exception encoded into one or more 64-bit hexadecimal numbers
        ///
        /// See: sys/exception_types.h
        #[method_id(@__retain_semantics Other exceptionCode)]
        pub unsafe fn exceptionCode(&self) -> Option<Retained<NSNumber>>;

        /// The signal associated with this crash.
        ///
        /// See: sys/signal.h
        #[method_id(@__retain_semantics Other signal)]
        pub unsafe fn signal(&self) -> Option<Retained<NSNumber>>;

        #[cfg(feature = "MXCrashDiagnosticObjectiveCExceptionReason")]
        /// The MXCrashDiagnosticObjectiveCExceptionReason object associated with this crash.
        ///
        /// See: <MetricKit
        /// /MXCrashDiagnosticObjectiveCExceptionReason.h>
        #[method_id(@__retain_semantics Other exceptionReason)]
        pub unsafe fn exceptionReason(
            &self,
        ) -> Option<Retained<MXCrashDiagnosticObjectiveCExceptionReason>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXCrashDiagnostic {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
