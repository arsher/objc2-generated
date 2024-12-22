//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/classkit/clsprogressreportingcapabilitykind?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLSProgressReportingCapabilityKind(pub NSInteger);
impl CLSProgressReportingCapabilityKind {
    #[doc(alias = "CLSProgressReportingCapabilityKindDuration")]
    pub const Duration: Self = Self(0);
    #[doc(alias = "CLSProgressReportingCapabilityKindPercent")]
    pub const Percent: Self = Self(1);
    #[doc(alias = "CLSProgressReportingCapabilityKindBinary")]
    pub const Binary: Self = Self(2);
    #[doc(alias = "CLSProgressReportingCapabilityKindQuantity")]
    pub const Quantity: Self = Self(3);
    #[doc(alias = "CLSProgressReportingCapabilityKindScore")]
    pub const Score: Self = Self(4);
}

unsafe impl Encode for CLSProgressReportingCapabilityKind {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLSProgressReportingCapabilityKind {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// This class specifies progress reporting capability of a ClassKit client app
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/classkit/clsprogressreportingcapability?language=objc)
    #[unsafe(super(CLSObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CLSObject")]
    pub struct CLSProgressReportingCapability;
);

#[cfg(feature = "CLSObject")]
unsafe impl NSCoding for CLSProgressReportingCapability {}

#[cfg(feature = "CLSObject")]
unsafe impl NSObjectProtocol for CLSProgressReportingCapability {}

#[cfg(feature = "CLSObject")]
unsafe impl NSSecureCoding for CLSProgressReportingCapability {}

extern_methods!(
    #[cfg(feature = "CLSObject")]
    unsafe impl CLSProgressReportingCapability {
        /// Returns the kind of progress reporting capability
        #[method(kind)]
        pub unsafe fn kind(&self) -> CLSProgressReportingCapabilityKind;

        /// Returns progress reporting details
        #[method_id(@__retain_semantics Other details)]
        pub unsafe fn details(&self) -> Option<Retained<NSString>>;

        /// Initialize and configure the type of progress reporting capability
        ///
        /// Parameter `kind`: The kind of progress reporting capability
        ///
        /// Parameter `details`: An optional localized string describing the capability. For example: "Reports percentage of progress", "Reports overall score". Schoolwork will use an appropriate default string if one is not provided.
        #[method_id(@__retain_semantics Init initWithKind:details:)]
        pub unsafe fn initWithKind_details(
            this: Allocated<Self>,
            kind: CLSProgressReportingCapabilityKind,
            details: Option<&NSString>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLSObject`
    #[cfg(feature = "CLSObject")]
    unsafe impl CLSProgressReportingCapability {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
