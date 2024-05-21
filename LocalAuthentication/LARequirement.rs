//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAAuthenticationRequirement;

    unsafe impl ClassType for LAAuthenticationRequirement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LAAuthenticationRequirement {}

extern_methods!(
    unsafe impl LAAuthenticationRequirement {
        #[method_id(@__retain_semantics Other defaultRequirement)]
        pub unsafe fn defaultRequirement() -> Retained<LAAuthenticationRequirement>;

        #[method_id(@__retain_semantics Other biometryRequirement)]
        pub unsafe fn biometryRequirement() -> Retained<LAAuthenticationRequirement>;

        #[method_id(@__retain_semantics Other biometryCurrentSetRequirement)]
        pub unsafe fn biometryCurrentSetRequirement() -> Retained<LAAuthenticationRequirement>;

        #[method_id(@__retain_semantics Other biometryRequirementWithFallback:)]
        pub unsafe fn biometryRequirementWithFallback(
            fallback: &LABiometryFallbackRequirement,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LAAuthenticationRequirement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LABiometryFallbackRequirement;

    unsafe impl ClassType for LABiometryFallbackRequirement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LABiometryFallbackRequirement {}

extern_methods!(
    unsafe impl LABiometryFallbackRequirement {
        #[method_id(@__retain_semantics Other defaultRequirement)]
        pub unsafe fn defaultRequirement() -> Retained<LABiometryFallbackRequirement>;

        #[method_id(@__retain_semantics Other devicePasscodeRequirement)]
        pub unsafe fn devicePasscodeRequirement() -> Retained<LABiometryFallbackRequirement>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LABiometryFallbackRequirement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
