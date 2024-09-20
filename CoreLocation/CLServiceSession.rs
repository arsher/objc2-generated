//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLServiceSessionAuthorizationRequirement(pub NSInteger);
impl CLServiceSessionAuthorizationRequirement {
    #[doc(alias = "CLServiceSessionAuthorizationRequirementNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "CLServiceSessionAuthorizationRequirementWhenInUse")]
    pub const WhenInUse: Self = Self(1);
}

unsafe impl Encode for CLServiceSessionAuthorizationRequirement {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLServiceSessionAuthorizationRequirement {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLServiceSessionDiagnostic;

    unsafe impl ClassType for CLServiceSessionDiagnostic {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for CLServiceSessionDiagnostic {}

extern_methods!(
    unsafe impl CLServiceSessionDiagnostic {
        #[method(authorizationDenied)]
        pub unsafe fn authorizationDenied(&self) -> bool;

        #[method(authorizationDeniedGlobally)]
        pub unsafe fn authorizationDeniedGlobally(&self) -> bool;

        #[method(authorizationRestricted)]
        pub unsafe fn authorizationRestricted(&self) -> bool;

        #[method(insufficientlyInUse)]
        pub unsafe fn insufficientlyInUse(&self) -> bool;

        #[method(serviceSessionRequired)]
        pub unsafe fn serviceSessionRequired(&self) -> bool;

        #[method(fullAccuracyDenied)]
        pub unsafe fn fullAccuracyDenied(&self) -> bool;

        #[method(alwaysAuthorizationDenied)]
        pub unsafe fn alwaysAuthorizationDenied(&self) -> bool;

        #[method(authorizationRequestInProgress)]
        pub unsafe fn authorizationRequestInProgress(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLServiceSessionDiagnostic {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLServiceSession;

    unsafe impl ClassType for CLServiceSession {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for CLServiceSession {}

extern_methods!(
    unsafe impl CLServiceSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other sessionRequiringAuthorization:)]
        pub unsafe fn sessionRequiringAuthorization(
            authorization_requirement: CLServiceSessionAuthorizationRequirement,
        ) -> Retained<CLServiceSession>;

        #[method_id(@__retain_semantics Other sessionRequiringAuthorization:fullAccuracyPurposeKey:)]
        pub unsafe fn sessionRequiringAuthorization_fullAccuracyPurposeKey(
            authorization_requirement: CLServiceSessionAuthorizationRequirement,
            purpose_key: &NSString,
        ) -> Retained<CLServiceSession>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
    }
);
