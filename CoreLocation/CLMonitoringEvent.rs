//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clmonitoringstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLMonitoringState(pub NSUInteger);
impl CLMonitoringState {
    #[doc(alias = "CLMonitoringStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CLMonitoringStateSatisfied")]
    pub const Satisfied: Self = Self(1);
    #[doc(alias = "CLMonitoringStateUnsatisfied")]
    pub const Unsatisfied: Self = Self(2);
    #[doc(alias = "CLMonitoringStateUnmonitored")]
    pub const Unmonitored: Self = Self(3);
}

unsafe impl Encode for CLMonitoringState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CLMonitoringState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clmonitoringevent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLMonitoringEvent;
);

unsafe impl NSCoding for CLMonitoringEvent {}

unsafe impl NSObjectProtocol for CLMonitoringEvent {}

unsafe impl NSSecureCoding for CLMonitoringEvent {}

extern_methods!(
    unsafe impl CLMonitoringEvent {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[cfg(feature = "CLCondition")]
        #[method_id(@__retain_semantics Other refinement)]
        pub unsafe fn refinement(&self) -> Option<Retained<CLCondition>>;

        #[method(state)]
        pub unsafe fn state(&self) -> CLMonitoringState;

        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[method(authorizationDenied)]
        pub unsafe fn authorizationDenied(&self) -> bool;

        #[method(authorizationDeniedGlobally)]
        pub unsafe fn authorizationDeniedGlobally(&self) -> bool;

        #[method(authorizationRestricted)]
        pub unsafe fn authorizationRestricted(&self) -> bool;

        #[method(insufficientlyInUse)]
        pub unsafe fn insufficientlyInUse(&self) -> bool;

        #[method(accuracyLimited)]
        pub unsafe fn accuracyLimited(&self) -> bool;

        #[method(conditionUnsupported)]
        pub unsafe fn conditionUnsupported(&self) -> bool;

        #[method(conditionLimitExceeded)]
        pub unsafe fn conditionLimitExceeded(&self) -> bool;

        #[method(persistenceUnavailable)]
        pub unsafe fn persistenceUnavailable(&self) -> bool;

        #[method(serviceSessionRequired)]
        pub unsafe fn serviceSessionRequired(&self) -> bool;

        #[method(authorizationRequestInProgress)]
        pub unsafe fn authorizationRequestInProgress(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
