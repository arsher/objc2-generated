//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/lapolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LAPolicy(pub NSInteger);
impl LAPolicy {
    #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithBiometrics")]
    pub const DeviceOwnerAuthenticationWithBiometrics: Self = Self(1);
    #[doc(alias = "LAPolicyDeviceOwnerAuthentication")]
    pub const DeviceOwnerAuthentication: Self = Self(2);
    #[deprecated]
    #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithWatch")]
    pub const DeviceOwnerAuthenticationWithWatch: Self = Self(3);
    #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithCompanion")]
    pub const DeviceOwnerAuthenticationWithCompanion: Self = Self(3);
    #[deprecated]
    #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithBiometricsOrWatch")]
    pub const DeviceOwnerAuthenticationWithBiometricsOrWatch: Self = Self(4);
    #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithBiometricsOrCompanion")]
    pub const DeviceOwnerAuthenticationWithBiometricsOrCompanion: Self = Self(4);
    #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithWristDetection")]
    pub const DeviceOwnerAuthenticationWithWristDetection: Self = Self(5);
}

unsafe impl Encode for LAPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for LAPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/latouchidauthenticationmaximumallowablereuseduration?language=objc)
    pub static LATouchIDAuthenticationMaximumAllowableReuseDuration: NSTimeInterval;
}

/// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/lacredentialtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LACredentialType(pub NSInteger);
impl LACredentialType {
    #[doc(alias = "LACredentialTypeApplicationPassword")]
    pub const ApplicationPassword: Self = Self(0);
    #[doc(alias = "LACredentialTypeSmartCardPIN")]
    pub const SmartCardPIN: Self = Self(-3);
}

unsafe impl Encode for LACredentialType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for LACredentialType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/laaccesscontroloperation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LAAccessControlOperation(pub NSInteger);
impl LAAccessControlOperation {
    #[doc(alias = "LAAccessControlOperationCreateItem")]
    pub const CreateItem: Self = Self(0);
    #[doc(alias = "LAAccessControlOperationUseItem")]
    pub const UseItem: Self = Self(1);
    #[doc(alias = "LAAccessControlOperationCreateKey")]
    pub const CreateKey: Self = Self(2);
    #[doc(alias = "LAAccessControlOperationUseKeySign")]
    pub const UseKeySign: Self = Self(3);
    #[doc(alias = "LAAccessControlOperationUseKeyDecrypt")]
    pub const UseKeyDecrypt: Self = Self(4);
    #[doc(alias = "LAAccessControlOperationUseKeyKeyExchange")]
    pub const UseKeyKeyExchange: Self = Self(5);
}

unsafe impl Encode for LAAccessControlOperation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for LAAccessControlOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/lacontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAContext;
);

unsafe impl NSObjectProtocol for LAContext {}

extern_methods!(
    unsafe impl LAContext {
        #[method(canEvaluatePolicy:error:_)]
        pub unsafe fn canEvaluatePolicy_error(
            &self,
            policy: LAPolicy,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method(evaluatePolicy:localizedReason:reply:)]
        pub unsafe fn evaluatePolicy_localizedReason_reply(
            &self,
            policy: LAPolicy,
            localized_reason: &NSString,
            reply: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(setCredential:type:)]
        pub unsafe fn setCredential_type(
            &self,
            credential: Option<&NSData>,
            r#type: LACredentialType,
        ) -> bool;

        #[method(isCredentialSet:)]
        pub unsafe fn isCredentialSet(&self, r#type: LACredentialType) -> bool;

        #[method_id(@__retain_semantics Other localizedFallbackTitle)]
        pub unsafe fn localizedFallbackTitle(&self) -> Option<Retained<NSString>>;

        #[method(setLocalizedFallbackTitle:)]
        pub unsafe fn setLocalizedFallbackTitle(&self, localized_fallback_title: Option<&NSString>);

        #[deprecated = "No longer supported"]
        #[method_id(@__retain_semantics Other maxBiometryFailures)]
        pub unsafe fn maxBiometryFailures(&self) -> Option<Retained<NSNumber>>;

        #[deprecated = "No longer supported"]
        #[method(setMaxBiometryFailures:)]
        pub unsafe fn setMaxBiometryFailures(&self, max_biometry_failures: Option<&NSNumber>);

        #[method_id(@__retain_semantics Other localizedCancelTitle)]
        pub unsafe fn localizedCancelTitle(&self) -> Option<Retained<NSString>>;

        #[method(setLocalizedCancelTitle:)]
        pub unsafe fn setLocalizedCancelTitle(&self, localized_cancel_title: Option<&NSString>);

        #[method(touchIDAuthenticationAllowableReuseDuration)]
        pub unsafe fn touchIDAuthenticationAllowableReuseDuration(&self) -> NSTimeInterval;

        #[method(setTouchIDAuthenticationAllowableReuseDuration:)]
        pub unsafe fn setTouchIDAuthenticationAllowableReuseDuration(
            &self,
            touch_id_authentication_allowable_reuse_duration: NSTimeInterval,
        );

        #[method_id(@__retain_semantics Other localizedReason)]
        pub unsafe fn localizedReason(&self) -> Retained<NSString>;

        #[method(setLocalizedReason:)]
        pub unsafe fn setLocalizedReason(&self, localized_reason: &NSString);

        #[method(interactionNotAllowed)]
        pub unsafe fn interactionNotAllowed(&self) -> bool;

        #[method(setInteractionNotAllowed:)]
        pub unsafe fn setInteractionNotAllowed(&self, interaction_not_allowed: bool);

        #[cfg(feature = "LABiometryType")]
        #[method(biometryType)]
        pub unsafe fn biometryType(&self) -> LABiometryType;

        #[deprecated]
        #[method_id(@__retain_semantics Other evaluatedPolicyDomainState)]
        pub unsafe fn evaluatedPolicyDomainState(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "LADomainState")]
        #[method_id(@__retain_semantics Other domainState)]
        pub unsafe fn domainState(&self) -> Retained<LADomainState>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LAContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
