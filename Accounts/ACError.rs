//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static ACErrorDomain: Option<&'static NSString>;
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ACErrorCode(pub c_uint);
impl ACErrorCode {
    pub const ACErrorUnknown: Self = Self(1);
    pub const ACErrorAccountMissingRequiredProperty: Self = Self(2);
    pub const ACErrorAccountAuthenticationFailed: Self = Self(3);
    pub const ACErrorAccountTypeInvalid: Self = Self(4);
    pub const ACErrorAccountAlreadyExists: Self = Self(5);
    pub const ACErrorAccountNotFound: Self = Self(6);
    pub const ACErrorPermissionDenied: Self = Self(7);
    pub const ACErrorAccessInfoInvalid: Self = Self(8);
    pub const ACErrorClientPermissionDenied: Self = Self(9);
    pub const ACErrorAccessDeniedByProtectionPolicy: Self = Self(10);
    pub const ACErrorCredentialNotFound: Self = Self(11);
    pub const ACErrorFetchCredentialFailed: Self = Self(12);
    pub const ACErrorStoreCredentialFailed: Self = Self(13);
    pub const ACErrorRemoveCredentialFailed: Self = Self(14);
    pub const ACErrorUpdatingNonexistentAccount: Self = Self(15);
    pub const ACErrorInvalidClientBundleID: Self = Self(16);
    pub const ACErrorDeniedByPlugin: Self = Self(17);
    pub const ACErrorCoreDataSaveFailed: Self = Self(18);
    pub const ACErrorFailedSerializingAccountInfo: Self = Self(19);
    pub const ACErrorInvalidCommand: Self = Self(20);
    pub const ACErrorMissingTransportMessageID: Self = Self(21);
    pub const ACErrorCredentialItemNotFound: Self = Self(22);
    pub const ACErrorCredentialItemNotExpired: Self = Self(23);
}

unsafe impl Encode for ACErrorCode {
    const ENCODING: Encoding = c_uint::ENCODING;
}

unsafe impl RefEncode for ACErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
