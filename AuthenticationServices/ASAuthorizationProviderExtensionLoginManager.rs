//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationProviderExtensionKeyType(pub NSInteger);
impl ASAuthorizationProviderExtensionKeyType {
    #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeUserDeviceSigning")]
    pub const UserDeviceSigning: Self = Self(1);
    #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeUserDeviceEncryption")]
    pub const UserDeviceEncryption: Self = Self(2);
    #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeUserSecureEnclaveKey")]
    pub const UserSecureEnclaveKey: Self = Self(3);
    #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeSharedDeviceSigning")]
    pub const SharedDeviceSigning: Self = Self(4);
    #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeSharedDeviceEncryption")]
    pub const SharedDeviceEncryption: Self = Self(5);
    #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeCurrentDeviceSigning")]
    pub const CurrentDeviceSigning: Self = Self(10);
    #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeCurrentDeviceEncryption")]
    pub const CurrentDeviceEncryption: Self = Self(11);
    #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeUserSmartCard")]
    pub const UserSmartCard: Self = Self(20);
}

unsafe impl Encode for ASAuthorizationProviderExtensionKeyType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationProviderExtensionKeyType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationProviderExtensionLoginManager;

    unsafe impl ClassType for ASAuthorizationProviderExtensionLoginManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionLoginManager {}

extern_methods!(
    unsafe impl ASAuthorizationProviderExtensionLoginManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(isDeviceRegistered)]
        pub unsafe fn isDeviceRegistered(&self) -> bool;

        #[method(isUserRegistered)]
        pub unsafe fn isUserRegistered(&self) -> bool;

        #[method_id(@__retain_semantics Other registrationToken)]
        pub unsafe fn registrationToken(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other extensionData)]
        pub unsafe fn extensionData(&self) -> Id<NSDictionary>;

        #[deprecated]
        #[method_id(@__retain_semantics Other loginUserName)]
        pub unsafe fn loginUserName(&self) -> Option<Id<NSString>>;

        #[deprecated]
        #[method(setLoginUserName:)]
        pub unsafe fn setLoginUserName(&self, login_user_name: Option<&NSString>);

        #[cfg(feature = "ASAuthorizationProviderExtensionUserLoginConfiguration")]
        #[method_id(@__retain_semantics Other userLoginConfiguration)]
        pub unsafe fn userLoginConfiguration(
            &self,
        ) -> Option<Id<ASAuthorizationProviderExtensionUserLoginConfiguration>>;

        #[cfg(feature = "ASAuthorizationProviderExtensionUserLoginConfiguration")]
        #[method(saveUserLoginConfiguration:error:_)]
        pub unsafe fn saveUserLoginConfiguration_error(
            &self,
            user_login_configuration: &ASAuthorizationProviderExtensionUserLoginConfiguration,
        ) -> Result<(), Id<NSError>>;

        #[method_id(@__retain_semantics Other ssoTokens)]
        pub unsafe fn ssoTokens(&self) -> Option<Id<NSDictionary>>;

        #[method(setSsoTokens:)]
        pub unsafe fn setSsoTokens(&self, sso_tokens: Option<&NSDictionary>);

        #[cfg(feature = "ASAuthorizationProviderExtensionLoginConfiguration")]
        #[method_id(@__retain_semantics Other loginConfiguration)]
        pub unsafe fn loginConfiguration(
            &self,
        ) -> Option<Id<ASAuthorizationProviderExtensionLoginConfiguration>>;

        #[cfg(feature = "ASAuthorizationProviderExtensionLoginConfiguration")]
        #[method(saveLoginConfiguration:error:_)]
        pub unsafe fn saveLoginConfiguration_error(
            &self,
            login_configuration: &ASAuthorizationProviderExtensionLoginConfiguration,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "block2")]
        #[method(userNeedsReauthenticationWithCompletion:)]
        pub unsafe fn userNeedsReauthenticationWithCompletion(
            &self,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method(deviceRegistrationsNeedsRepair)]
        pub unsafe fn deviceRegistrationsNeedsRepair(&self);

        #[method(userRegistrationsNeedsRepair)]
        pub unsafe fn userRegistrationsNeedsRepair(&self);

        #[method(decryptionKeysNeedRepair)]
        pub unsafe fn decryptionKeysNeedRepair(&self);

        #[method(resetKeys)]
        pub unsafe fn resetKeys(&self);

        #[method(resetDeviceKeys)]
        pub unsafe fn resetDeviceKeys(&self);

        #[method(resetUserSecureEnclaveKey)]
        pub unsafe fn resetUserSecureEnclaveKey(&self);

        #[cfg(feature = "block2")]
        #[method(presentRegistrationViewControllerWithCompletion:)]
        pub unsafe fn presentRegistrationViewControllerWithCompletion(
            &self,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);
