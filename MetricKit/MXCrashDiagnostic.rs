//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXDiagnostic")]
    pub struct MXCrashDiagnostic;

    #[cfg(feature = "MXDiagnostic")]
    unsafe impl ClassType for MXCrashDiagnostic {
        #[inherits(NSObject)]
        type Super = MXDiagnostic;
        type Mutability = InteriorMutable;
    }
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
        #[method_id(@__retain_semantics Other callStackTree)]
        pub unsafe fn callStackTree(&self) -> Retained<MXCallStackTree>;

        #[method_id(@__retain_semantics Other terminationReason)]
        pub unsafe fn terminationReason(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other virtualMemoryRegionInfo)]
        pub unsafe fn virtualMemoryRegionInfo(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other exceptionType)]
        pub unsafe fn exceptionType(&self) -> Option<Retained<NSNumber>>;

        #[method_id(@__retain_semantics Other exceptionCode)]
        pub unsafe fn exceptionCode(&self) -> Option<Retained<NSNumber>>;

        #[method_id(@__retain_semantics Other signal)]
        pub unsafe fn signal(&self) -> Option<Retained<NSNumber>>;

        #[cfg(feature = "MXCrashDiagnosticObjectiveCExceptionReason")]
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
