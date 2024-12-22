//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// This protocol represents the sender of an
    /// authentication challenge. It has methods to provide a credential,
    /// to continue without any credential, getting whatever failure
    /// result would happen in that case, cancel a challenge, perform the default
    /// action as defined by the system, or reject the currently supplied protection-space
    /// in the challenge.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlauthenticationchallengesender?language=objc)
    pub unsafe trait NSURLAuthenticationChallengeSender: NSObjectProtocol {
        #[cfg(feature = "NSURLCredential")]
        #[method(useCredential:forAuthenticationChallenge:)]
        unsafe fn useCredential_forAuthenticationChallenge(
            &self,
            credential: &NSURLCredential,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[method(continueWithoutCredentialForAuthenticationChallenge:)]
        unsafe fn continueWithoutCredentialForAuthenticationChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[method(cancelAuthenticationChallenge:)]
        unsafe fn cancelAuthenticationChallenge(&self, challenge: &NSURLAuthenticationChallenge);

        #[optional]
        #[method(performDefaultHandlingForAuthenticationChallenge:)]
        unsafe fn performDefaultHandlingForAuthenticationChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[optional]
        #[method(rejectProtectionSpaceAndContinueWithChallenge:)]
        unsafe fn rejectProtectionSpaceAndContinueWithChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );
    }

    unsafe impl ProtocolType for dyn NSURLAuthenticationChallengeSender {}
);

extern_class!(
    /// This class represents an authentication challenge. It
    /// provides all the information about the challenge, and has a method
    /// to indicate when it's done.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlauthenticationchallenge?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLAuthenticationChallenge;
);

unsafe impl Send for NSURLAuthenticationChallenge {}

unsafe impl Sync for NSURLAuthenticationChallenge {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSURLAuthenticationChallenge {}

unsafe impl NSObjectProtocol for NSURLAuthenticationChallenge {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSURLAuthenticationChallenge {}

extern_methods!(
    unsafe impl NSURLAuthenticationChallenge {
        #[cfg(all(
            feature = "NSError",
            feature = "NSURLCredential",
            feature = "NSURLProtectionSpace",
            feature = "NSURLResponse"
        ))]
        /// Initialize an authentication challenge
        ///
        /// Parameter `space`: The NSURLProtectionSpace to use
        ///
        /// Parameter `credential`: The proposed NSURLCredential for this challenge, or nil
        ///
        /// Parameter `previousFailureCount`: A count of previous failures attempting access.
        ///
        /// Parameter `response`: The NSURLResponse for the authentication failure, if applicable, else nil
        ///
        /// Parameter `error`: The NSError for the authentication failure, if applicable, else nil
        ///
        /// Returns: An authentication challenge initialized with the specified parameters
        #[method_id(@__retain_semantics Init initWithProtectionSpace:proposedCredential:previousFailureCount:failureResponse:error:sender:)]
        pub unsafe fn initWithProtectionSpace_proposedCredential_previousFailureCount_failureResponse_error_sender(
            this: Allocated<Self>,
            space: &NSURLProtectionSpace,
            credential: Option<&NSURLCredential>,
            previous_failure_count: NSInteger,
            response: Option<&NSURLResponse>,
            error: Option<&NSError>,
            sender: &ProtocolObject<dyn NSURLAuthenticationChallengeSender>,
        ) -> Retained<Self>;

        /// Initialize an authentication challenge copying all parameters from another one.
        ///
        /// Returns: A new challenge initialized with the parameters from the passed in challenge
        ///
        /// This initializer may be useful to subclassers that want to proxy
        /// one type of authentication challenge to look like another type.
        #[method_id(@__retain_semantics Init initWithAuthenticationChallenge:sender:)]
        pub unsafe fn initWithAuthenticationChallenge_sender(
            this: Allocated<Self>,
            challenge: &NSURLAuthenticationChallenge,
            sender: &ProtocolObject<dyn NSURLAuthenticationChallengeSender>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURLProtectionSpace")]
        /// Get a description of the protection space that requires authentication
        ///
        /// Returns: The protection space that needs authentication
        #[method_id(@__retain_semantics Other protectionSpace)]
        pub unsafe fn protectionSpace(&self) -> Retained<NSURLProtectionSpace>;

        #[cfg(feature = "NSURLCredential")]
        /// Get the proposed credential for this challenge
        ///
        /// Returns: The proposed credential
        ///
        /// proposedCredential may be nil, if there is no default
        /// credential to use for this challenge (either stored or in the
        /// URL). If the credential is not nil and returns YES for
        /// hasPassword, this means the NSURLConnection thinks the credential
        /// is ready to use as-is. If it returns NO for hasPassword, then the
        /// credential is not ready to use as-is, but provides a default
        /// username the client could use when prompting.
        #[method_id(@__retain_semantics Other proposedCredential)]
        pub unsafe fn proposedCredential(&self) -> Option<Retained<NSURLCredential>>;

        /// Get count of previous failed authentication attempts
        ///
        /// Returns: The count of previous failures
        #[method(previousFailureCount)]
        pub unsafe fn previousFailureCount(&self) -> NSInteger;

        #[cfg(feature = "NSURLResponse")]
        /// Get the response representing authentication failure.
        ///
        /// Returns: The failure response or nil
        ///
        /// If there was a previous authentication failure, and
        /// this protocol uses responses to indicate authentication failure,
        /// then this method will return the response. Otherwise it will
        /// return nil.
        #[method_id(@__retain_semantics Other failureResponse)]
        pub unsafe fn failureResponse(&self) -> Option<Retained<NSURLResponse>>;

        #[cfg(feature = "NSError")]
        /// Get the error representing authentication failure.
        ///
        /// If there was a previous authentication failure, and
        /// this protocol uses errors to indicate authentication failure,
        /// then this method will return the error. Otherwise it will
        /// return nil.
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;

        /// Get the sender of this challenge
        ///
        /// Returns: The sender of the challenge
        ///
        /// The sender is the object you should reply to when done processing the challenge.
        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSURLAuthenticationChallengeSender>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLAuthenticationChallenge {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
