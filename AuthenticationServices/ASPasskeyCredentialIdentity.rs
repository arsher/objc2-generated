//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An ASPasswordCredentialIdentity is used to describe an identity that can use a service upon successful passkey based authentication.
    /// Use this class to save entries into ASCredentialIdentityStore.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspasskeycredentialidentity?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyCredentialIdentity;
);

#[cfg(feature = "ASCredentialIdentity")]
unsafe impl ASCredentialIdentity for ASPasskeyCredentialIdentity {}

unsafe impl NSCoding for ASPasskeyCredentialIdentity {}

unsafe impl NSCopying for ASPasskeyCredentialIdentity {}

unsafe impl CopyingHelper for ASPasskeyCredentialIdentity {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasskeyCredentialIdentity {}

unsafe impl NSSecureCoding for ASPasskeyCredentialIdentity {}

extern_methods!(
    unsafe impl ASPasskeyCredentialIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Initialize an instance of ASPasskeyCredentialIdentity.
        ///
        /// Parameter `relyingPartyIdentifier`: relying party for this credential.
        ///
        /// Parameter `userName`: user name associated with this credential.
        ///
        /// Parameter `credentialID`: credential ID of this passkey credential.
        ///
        /// Parameter `userHandle`: user handle data of this passkey credential.
        ///
        /// Parameter `recordIdentifier`: identifier used by credential provider extension to identify this credential.
        #[method_id(@__retain_semantics Init initWithRelyingPartyIdentifier:userName:credentialID:userHandle:recordIdentifier:)]
        pub unsafe fn initWithRelyingPartyIdentifier_userName_credentialID_userHandle_recordIdentifier(
            this: Allocated<Self>,
            relying_party_identifier: &NSString,
            user_name: &NSString,
            credential_id: &NSData,
            user_handle: &NSData,
            record_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        /// Create and initialize an instance of ASPasskeyCredentialIdentity.
        ///
        /// Parameter `relyingPartyIdentifier`: relying party for this credential.
        ///
        /// Parameter `userName`: user name associated with this credential.
        ///
        /// Parameter `credentialID`: credential ID of this passkey credential.
        ///
        /// Parameter `userHandle`: user handle data of this passkey credential.
        ///
        /// Parameter `recordIdentifier`: identifier used by credential provider extension to identify this credential.
        #[method_id(@__retain_semantics Other identityWithRelyingPartyIdentifier:userName:credentialID:userHandle:recordIdentifier:)]
        pub unsafe fn identityWithRelyingPartyIdentifier_userName_credentialID_userHandle_recordIdentifier(
            relying_party_identifier: &NSString,
            user_name: &NSString,
            credential_id: &NSData,
            user_handle: &NSData,
            record_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        /// The relying party identifier of this passkey credential.
        ///
        /// This field is reported as the serviceIdentifier property of ASCredentialIdentity.
        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Retained<NSString>;

        /// The user name of this passkey credential.
        ///
        /// This field is reported as the user property of ASCredentialIdentity.
        #[method_id(@__retain_semantics Other userName)]
        pub unsafe fn userName(&self) -> Retained<NSString>;

        /// The credential ID of this passkey credential.
        ///
        /// This field is used to identify the correct credential to use based on relying party request parameters.
        #[method_id(@__retain_semantics Other credentialID)]
        pub unsafe fn credentialID(&self) -> Retained<NSData>;

        /// The user handle of this passkey credential.
        ///
        /// This field is used to identify the correct credential to use based on relying party request parameters.
        #[method_id(@__retain_semantics Other userHandle)]
        pub unsafe fn userHandle(&self) -> Retained<NSData>;

        /// Get the record identifier.
        ///
        /// Returns: The record identifier.
        ///
        /// You can utilize the record identifier to uniquely identify the credential identity in your local database.
        #[method_id(@__retain_semantics Other recordIdentifier)]
        pub unsafe fn recordIdentifier(&self) -> Option<Retained<NSString>>;

        /// Get or set the rank of the credential identity object.
        ///
        /// The system may utilize the rank to decide which credential identity precedes the other
        /// if two identities have the same service identifier. A credential identity with a larger rank value
        /// precedes one with a smaller value if both credential identities have the same service identifier.
        /// The default value of this property is 0.
        #[method(rank)]
        pub unsafe fn rank(&self) -> NSInteger;

        /// Setter for [`rank`][Self::rank].
        #[method(setRank:)]
        pub unsafe fn setRank(&self, rank: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasskeyCredentialIdentity {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
