//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxcrashdiagnosticobjectivecexceptionreason?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXCrashDiagnosticObjectiveCExceptionReason;
);

unsafe impl NSCoding for MXCrashDiagnosticObjectiveCExceptionReason {}

unsafe impl NSObjectProtocol for MXCrashDiagnosticObjectiveCExceptionReason {}

unsafe impl NSSecureCoding for MXCrashDiagnosticObjectiveCExceptionReason {}

extern_methods!(
    unsafe impl MXCrashDiagnosticObjectiveCExceptionReason {
        #[method_id(@__retain_semantics Other composedMessage)]
        pub unsafe fn composedMessage(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other formatString)]
        pub unsafe fn formatString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other exceptionType)]
        pub unsafe fn exceptionType(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other className)]
        pub unsafe fn className(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other exceptionName)]
        pub unsafe fn exceptionName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Retained<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXCrashDiagnosticObjectiveCExceptionReason {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
