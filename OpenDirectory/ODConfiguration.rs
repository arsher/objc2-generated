//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odpacketsigningdisabled?language=objc)
pub const ODPacketSigningDisabled: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odpacketsigningallow?language=objc)
pub const ODPacketSigningAllow: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odpacketsigningrequired?language=objc)
pub const ODPacketSigningRequired: c_uint = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odpacketencryptiondisabled?language=objc)
pub const ODPacketEncryptionDisabled: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odpacketencryptionallow?language=objc)
pub const ODPacketEncryptionAllow: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odpacketencryptionrequired?language=objc)
pub const ODPacketEncryptionRequired: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odpacketencryptionssl?language=objc)
pub const ODPacketEncryptionSSL: c_uint = 3;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odtrusttypejoined?language=objc)
    pub static ODTrustTypeJoined: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odtrusttypeusingcredentials?language=objc)
    pub static ODTrustTypeUsingCredentials: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odtrusttypeanonymous?language=objc)
    pub static ODTrustTypeAnonymous: Option<&'static NSString>;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ODConfiguration;
);

unsafe impl NSObjectProtocol for ODConfiguration {}

extern_methods!(
    unsafe impl ODConfiguration {
        #[method_id(@__retain_semantics Other nodeName)]
        pub unsafe fn nodeName(&self) -> Retained<NSString>;

        #[method(setNodeName:)]
        pub unsafe fn setNodeName(&self, node_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other comment)]
        pub unsafe fn comment(&self) -> Retained<NSString>;

        #[method(setComment:)]
        pub unsafe fn setComment(&self, comment: Option<&NSString>);

        #[cfg(feature = "ODMappings")]
        #[method_id(@__retain_semantics Other defaultMappings)]
        pub unsafe fn defaultMappings(&self) -> Option<Retained<ODMappings>>;

        #[cfg(feature = "ODMappings")]
        #[method(setDefaultMappings:)]
        pub unsafe fn setDefaultMappings(&self, default_mappings: Option<&ODMappings>);

        #[method_id(@__retain_semantics Other templateName)]
        pub unsafe fn templateName(&self) -> Retained<NSString>;

        #[method(setTemplateName:)]
        pub unsafe fn setTemplateName(&self, template_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other virtualSubnodes)]
        pub unsafe fn virtualSubnodes(&self) -> Retained<NSArray>;

        #[method(setVirtualSubnodes:)]
        pub unsafe fn setVirtualSubnodes(&self, virtual_subnodes: Option<&NSArray>);

        #[method(hideRegistration)]
        pub unsafe fn hideRegistration(&self) -> bool;

        #[method(setHideRegistration:)]
        pub unsafe fn setHideRegistration(&self, hide_registration: bool);

        #[method_id(@__retain_semantics Other preferredDestinationHostName)]
        pub unsafe fn preferredDestinationHostName(&self) -> Retained<NSString>;

        #[method(setPreferredDestinationHostName:)]
        pub unsafe fn setPreferredDestinationHostName(
            &self,
            preferred_destination_host_name: Option<&NSString>,
        );

        #[method(preferredDestinationHostPort)]
        pub unsafe fn preferredDestinationHostPort(&self) -> u16;

        #[method(setPreferredDestinationHostPort:)]
        pub unsafe fn setPreferredDestinationHostPort(&self, preferred_destination_host_port: u16);

        #[method_id(@__retain_semantics Other trustAccount)]
        pub unsafe fn trustAccount(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other trustMetaAccount)]
        pub unsafe fn trustMetaAccount(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other trustKerberosPrincipal)]
        pub unsafe fn trustKerberosPrincipal(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other trustType)]
        pub unsafe fn trustType(&self) -> Retained<NSString>;

        #[method(trustUsesMutualAuthentication)]
        pub unsafe fn trustUsesMutualAuthentication(&self) -> bool;

        #[method(trustUsesKerberosKeytab)]
        pub unsafe fn trustUsesKerberosKeytab(&self) -> bool;

        #[method(trustUsesSystemKeychain)]
        pub unsafe fn trustUsesSystemKeychain(&self) -> bool;

        #[method(packetSigning)]
        pub unsafe fn packetSigning(&self) -> NSInteger;

        #[method(setPacketSigning:)]
        pub unsafe fn setPacketSigning(&self, packet_signing: NSInteger);

        #[method(packetEncryption)]
        pub unsafe fn packetEncryption(&self) -> NSInteger;

        #[method(setPacketEncryption:)]
        pub unsafe fn setPacketEncryption(&self, packet_encryption: NSInteger);

        #[method(manInTheMiddleProtection)]
        pub unsafe fn manInTheMiddleProtection(&self) -> bool;

        #[method(setManInTheMiddleProtection:)]
        pub unsafe fn setManInTheMiddleProtection(&self, man_in_the_middle_protection: bool);

        #[method(queryTimeoutInSeconds)]
        pub unsafe fn queryTimeoutInSeconds(&self) -> NSInteger;

        #[method(setQueryTimeoutInSeconds:)]
        pub unsafe fn setQueryTimeoutInSeconds(&self, query_timeout_in_seconds: NSInteger);

        #[method(connectionSetupTimeoutInSeconds)]
        pub unsafe fn connectionSetupTimeoutInSeconds(&self) -> NSInteger;

        #[method(setConnectionSetupTimeoutInSeconds:)]
        pub unsafe fn setConnectionSetupTimeoutInSeconds(
            &self,
            connection_setup_timeout_in_seconds: NSInteger,
        );

        #[method(connectionIdleTimeoutInSeconds)]
        pub unsafe fn connectionIdleTimeoutInSeconds(&self) -> NSInteger;

        #[method(setConnectionIdleTimeoutInSeconds:)]
        pub unsafe fn setConnectionIdleTimeoutInSeconds(
            &self,
            connection_idle_timeout_in_seconds: NSInteger,
        );

        #[method_id(@__retain_semantics Other defaultModuleEntries)]
        pub unsafe fn defaultModuleEntries(&self) -> Retained<NSArray>;

        #[method(setDefaultModuleEntries:)]
        pub unsafe fn setDefaultModuleEntries(&self, default_module_entries: Option<&NSArray>);

        #[method_id(@__retain_semantics Other authenticationModuleEntries)]
        pub unsafe fn authenticationModuleEntries(&self) -> Retained<NSArray>;

        #[method(setAuthenticationModuleEntries:)]
        pub unsafe fn setAuthenticationModuleEntries(
            &self,
            authentication_module_entries: Option<&NSArray>,
        );

        #[method_id(@__retain_semantics Other discoveryModuleEntries)]
        pub unsafe fn discoveryModuleEntries(&self) -> Retained<NSArray>;

        #[method(setDiscoveryModuleEntries:)]
        pub unsafe fn setDiscoveryModuleEntries(&self, discovery_module_entries: Option<&NSArray>);

        #[method_id(@__retain_semantics Other generalModuleEntries)]
        pub unsafe fn generalModuleEntries(&self) -> Retained<NSArray>;

        #[method(setGeneralModuleEntries:)]
        pub unsafe fn setGeneralModuleEntries(&self, general_module_entries: Option<&NSArray>);

        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration() -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other suggestedTrustAccount:)]
        pub unsafe fn suggestedTrustAccount(
            hostname: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other suggestedTrustPassword:)]
        pub unsafe fn suggestedTrustPassword(length: usize) -> Option<Retained<NSString>>;

        #[method(addTrustType:trustAccount:trustPassword:username:password:joinExisting:error:)]
        pub unsafe fn addTrustType_trustAccount_trustPassword_username_password_joinExisting_error(
            &self,
            trust_type: Option<&NSString>,
            account: Option<&NSString>,
            account_password: Option<&NSString>,
            username: Option<&NSString>,
            password: Option<&NSString>,
            join: bool,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;

        #[method(removeTrustUsingUsername:password:deleteTrustAccount:error:)]
        pub unsafe fn removeTrustUsingUsername_password_deleteTrustAccount_error(
            &self,
            username: Option<&NSString>,
            password: Option<&NSString>,
            delete_account: bool,
            error: Option<&mut Option<Retained<NSError>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ODConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
