//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialuserverificationpreference?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationPublicKeyCredentialUserVerificationPreference = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialuserverificationpreferencepreferred?language=objc)
    pub static ASAuthorizationPublicKeyCredentialUserVerificationPreferencePreferred:
        Option<&'static ASAuthorizationPublicKeyCredentialUserVerificationPreference>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialuserverificationpreferencerequired?language=objc)
    pub static ASAuthorizationPublicKeyCredentialUserVerificationPreferenceRequired:
        Option<&'static ASAuthorizationPublicKeyCredentialUserVerificationPreference>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialuserverificationpreferencediscouraged?language=objc)
    pub static ASAuthorizationPublicKeyCredentialUserVerificationPreferenceDiscouraged:
        Option<&'static ASAuthorizationPublicKeyCredentialUserVerificationPreference>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialattestationkind?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationPublicKeyCredentialAttestationKind = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialattestationkindnone?language=objc)
    pub static ASAuthorizationPublicKeyCredentialAttestationKindNone:
        Option<&'static ASAuthorizationPublicKeyCredentialAttestationKind>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialattestationkinddirect?language=objc)
    pub static ASAuthorizationPublicKeyCredentialAttestationKindDirect:
        Option<&'static ASAuthorizationPublicKeyCredentialAttestationKind>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialattestationkindindirect?language=objc)
    pub static ASAuthorizationPublicKeyCredentialAttestationKindIndirect:
        Option<&'static ASAuthorizationPublicKeyCredentialAttestationKind>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialattestationkindenterprise?language=objc)
    pub static ASAuthorizationPublicKeyCredentialAttestationKindEnterprise:
        Option<&'static ASAuthorizationPublicKeyCredentialAttestationKind>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialresidentkeypreference?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationPublicKeyCredentialResidentKeyPreference = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialresidentkeypreferencediscouraged?language=objc)
    pub static ASAuthorizationPublicKeyCredentialResidentKeyPreferenceDiscouraged:
        Option<&'static ASAuthorizationPublicKeyCredentialResidentKeyPreference>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialresidentkeypreferencepreferred?language=objc)
    pub static ASAuthorizationPublicKeyCredentialResidentKeyPreferencePreferred:
        Option<&'static ASAuthorizationPublicKeyCredentialResidentKeyPreference>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialresidentkeypreferencerequired?language=objc)
    pub static ASAuthorizationPublicKeyCredentialResidentKeyPreferenceRequired:
        Option<&'static ASAuthorizationPublicKeyCredentialResidentKeyPreference>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialattachment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationPublicKeyCredentialAttachment(pub NSInteger);
impl ASAuthorizationPublicKeyCredentialAttachment {
    #[doc(alias = "ASAuthorizationPublicKeyCredentialAttachmentPlatform")]
    pub const Platform: Self = Self(0);
    #[doc(alias = "ASAuthorizationPublicKeyCredentialAttachmentCrossPlatform")]
    pub const CrossPlatform: Self = Self(1);
}

unsafe impl Encode for ASAuthorizationPublicKeyCredentialAttachment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationPublicKeyCredentialAttachment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
