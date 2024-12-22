//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/classkit/clserrorcodedomain?language=objc)
    pub static CLSErrorCodeDomain: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/classkit/clserrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLSErrorCode(pub NSInteger);
impl CLSErrorCode {
    #[doc(alias = "CLSErrorCodeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "CLSErrorCodeClassKitUnavailable")]
    pub const ClassKitUnavailable: Self = Self(1);
    #[doc(alias = "CLSErrorCodeInvalidArgument")]
    pub const InvalidArgument: Self = Self(2);
    #[doc(alias = "CLSErrorCodeInvalidModification")]
    pub const InvalidModification: Self = Self(3);
    #[doc(alias = "CLSErrorCodeAuthorizationDenied")]
    pub const AuthorizationDenied: Self = Self(4);
    #[doc(alias = "CLSErrorCodeDatabaseInaccessible")]
    pub const DatabaseInaccessible: Self = Self(5);
    #[doc(alias = "CLSErrorCodeLimits")]
    pub const Limits: Self = Self(6);
    #[doc(alias = "CLSErrorCodeInvalidCreate")]
    pub const InvalidCreate: Self = Self(7);
    #[doc(alias = "CLSErrorCodeInvalidUpdate")]
    pub const InvalidUpdate: Self = Self(8);
    #[doc(alias = "CLSErrorCodePartialFailure")]
    pub const PartialFailure: Self = Self(9);
    #[doc(alias = "CLSErrorCodeInvalidAccountCredentials")]
    pub const InvalidAccountCredentials: Self = Self(10);
}

unsafe impl Encode for CLSErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLSErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/classkit/clserroruserinfokey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type CLSErrorUserInfoKey = NSString;

extern "C" {
    /// Any object that caused a failure will be available in - [NSError userInfo]; under this key.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/classkit/clserrorobjectkey?language=objc)
    pub static CLSErrorObjectKey: Option<&'static CLSErrorUserInfoKey>;
}

extern "C" {
    /// If multiple objects cause errors we return an error with code `CLSErrorCodePartialFailure` which will contain an array of errors in - [NSError userInfo]; under this key.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/classkit/clserrorunderlyingerrorskey?language=objc)
    pub static CLSErrorUnderlyingErrorsKey: Option<&'static CLSErrorUserInfoKey>;
}

extern "C" {
    /// Errors with the code `CLSErrorCodePartialFailure` may contain an array of successful entities in - [NSError userInfo]; under this key.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/classkit/clserrorsuccessfulobjectskey?language=objc)
    pub static CLSErrorSuccessfulObjectsKey: Option<&'static CLSErrorUserInfoKey>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/classkit/clspredicatekeypath?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type CLSPredicateKeyPath = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/classkit/clspredicatekeypathdatecreated?language=objc)
    pub static CLSPredicateKeyPathDateCreated: Option<&'static CLSPredicateKeyPath>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/classkit/clspredicatekeypathidentifier?language=objc)
    pub static CLSPredicateKeyPathIdentifier: Option<&'static CLSPredicateKeyPath>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/classkit/clspredicatekeypathtitle?language=objc)
    pub static CLSPredicateKeyPathTitle: Option<&'static CLSPredicateKeyPath>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/classkit/clspredicatekeypathuniversallinkurl?language=objc)
    pub static CLSPredicateKeyPathUniversalLinkURL: Option<&'static CLSPredicateKeyPath>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/classkit/clspredicatekeypathtopic?language=objc)
    pub static CLSPredicateKeyPathTopic: Option<&'static CLSPredicateKeyPath>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/classkit/clspredicatekeypathparent?language=objc)
    pub static CLSPredicateKeyPathParent: Option<&'static CLSPredicateKeyPath>;
}
