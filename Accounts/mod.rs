// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "Accounts", kind = "framework")]
extern "C" {}

#[cfg(feature = "ACAccount")]
#[path = "ACAccount.rs"]
mod __ACAccount;
#[cfg(feature = "ACAccountCredential")]
#[path = "ACAccountCredential.rs"]
mod __ACAccountCredential;
#[cfg(feature = "ACAccountStore")]
#[path = "ACAccountStore.rs"]
mod __ACAccountStore;
#[cfg(feature = "ACAccountType")]
#[path = "ACAccountType.rs"]
mod __ACAccountType;
#[cfg(feature = "ACError")]
#[path = "ACError.rs"]
mod __ACError;
#[cfg(feature = "AccountsDefines")]
#[path = "AccountsDefines.rs"]
mod __AccountsDefines;

#[cfg(feature = "ACAccount")]
pub use self::__ACAccount::ACAccount;
#[cfg(feature = "ACAccountCredential")]
pub use self::__ACAccountCredential::ACAccountCredential;
#[cfg(feature = "ACAccountStore")]
pub use self::__ACAccountStore::ACAccountCredentialRenewResult;
#[cfg(feature = "ACAccountStore")]
pub use self::__ACAccountStore::ACAccountStore;
#[cfg(all(feature = "ACAccountStore", feature = "block2"))]
pub use self::__ACAccountStore::ACAccountStoreCredentialRenewalHandler;
#[cfg(feature = "ACAccountStore")]
pub use self::__ACAccountStore::ACAccountStoreDidChangeNotification;
#[cfg(all(feature = "ACAccountStore", feature = "block2"))]
pub use self::__ACAccountStore::ACAccountStoreRemoveCompletionHandler;
#[cfg(all(feature = "ACAccountStore", feature = "block2"))]
pub use self::__ACAccountStore::ACAccountStoreRequestAccessCompletionHandler;
#[cfg(all(feature = "ACAccountStore", feature = "block2"))]
pub use self::__ACAccountStore::ACAccountStoreSaveCompletionHandler;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACAccountType;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACAccountTypeIdentifierFacebook;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACAccountTypeIdentifierLinkedIn;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACAccountTypeIdentifierSinaWeibo;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACAccountTypeIdentifierTencentWeibo;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACAccountTypeIdentifierTwitter;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACFacebookAppIdKey;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACFacebookAudienceEveryone;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACFacebookAudienceFriends;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACFacebookAudienceKey;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACFacebookAudienceOnlyMe;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACFacebookPermissionsKey;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACLinkedInAppIdKey;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACLinkedInPermissionsKey;
#[cfg(feature = "ACAccountType")]
pub use self::__ACAccountType::ACTencentWeiboAppIdKey;
#[cfg(feature = "ACError")]
pub use self::__ACError::ACErrorCode;
#[cfg(feature = "ACError")]
pub use self::__ACError::ACErrorDomain;
