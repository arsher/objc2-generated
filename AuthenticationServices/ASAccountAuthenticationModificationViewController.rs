//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
unsafe impl NSCoding for ASAccountAuthenticationModificationViewController {}

#[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
unsafe impl NSEditor for ASAccountAuthenticationModificationViewController {}

#[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
unsafe impl NSObjectProtocol for ASAccountAuthenticationModificationViewController {}

#[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
unsafe impl NSSeguePerforming for ASAccountAuthenticationModificationViewController {}

#[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
unsafe impl NSUserInterfaceItemIdentification
    for ASAccountAuthenticationModificationViewController
{
}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
    unsafe impl ASAccountAuthenticationModificationViewController {
        #[cfg(
            feature = "AuthenticationServices_ASAccountAuthenticationModificationExtensionContext"
        )]
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(
            &self,
        ) -> Id<ASAccountAuthenticationModificationExtensionContext>;

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "AuthenticationServices_ASPasswordCredential",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(convertAccountToSignInWithAppleWithoutUserInteractionForServiceIdentifier:existingCredential:userInfo:)]
        pub unsafe fn convertAccountToSignInWithAppleWithoutUserInteractionForServiceIdentifier_existingCredential_userInfo(
            &self,
            service_identifier: &ASCredentialServiceIdentifier,
            existing_credential: &ASPasswordCredential,
            user_info: Option<&NSDictionary>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "AuthenticationServices_ASPasswordCredential",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(prepareInterfaceToConvertAccountToSignInWithAppleForServiceIdentifier:existingCredential:userInfo:)]
        pub unsafe fn prepareInterfaceToConvertAccountToSignInWithAppleForServiceIdentifier_existingCredential_userInfo(
            &self,
            service_identifier: &ASCredentialServiceIdentifier,
            existing_credential: &ASPasswordCredential,
            user_info: Option<&NSDictionary>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "AuthenticationServices_ASPasswordCredential",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(changePasswordWithoutUserInteractionForServiceIdentifier:existingCredential:newPassword:userInfo:)]
        pub unsafe fn changePasswordWithoutUserInteractionForServiceIdentifier_existingCredential_newPassword_userInfo(
            &self,
            service_identifier: &ASCredentialServiceIdentifier,
            existing_credential: &ASPasswordCredential,
            new_password: &NSString,
            user_info: Option<&NSDictionary>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "AuthenticationServices_ASPasswordCredential",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(prepareInterfaceToChangePasswordForServiceIdentifier:existingCredential:newPassword:userInfo:)]
        pub unsafe fn prepareInterfaceToChangePasswordForServiceIdentifier_existingCredential_newPassword_userInfo(
            &self,
            service_identifier: &ASCredentialServiceIdentifier,
            existing_credential: &ASPasswordCredential,
            new_password: &NSString,
            user_info: Option<&NSDictionary>,
        );

        #[method(cancelRequest)]
        pub unsafe fn cancelRequest(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
    unsafe impl ASAccountAuthenticationModificationViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);
