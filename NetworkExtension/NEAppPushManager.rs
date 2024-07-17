//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static NEAppPushErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEAppPushManagerError(pub NSInteger);
impl NEAppPushManagerError {
    #[doc(alias = "NEAppPushManagerErrorConfigurationInvalid")]
    pub const ConfigurationInvalid: Self = Self(1);
    #[doc(alias = "NEAppPushManagerErrorConfigurationNotLoaded")]
    pub const ConfigurationNotLoaded: Self = Self(2);
    #[doc(alias = "NEAppPushManagerErrorInternalError")]
    pub const InternalError: Self = Self(3);
    #[doc(alias = "NEAppPushManagerErrorInactiveSession")]
    pub const InactiveSession: Self = Self(4);
}

unsafe impl Encode for NEAppPushManagerError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEAppPushManagerError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEPrivateLTENetwork;

    unsafe impl ClassType for NEPrivateLTENetwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NEPrivateLTENetwork {}

unsafe impl NSCopying for NEPrivateLTENetwork {}

unsafe impl NSObjectProtocol for NEPrivateLTENetwork {}

unsafe impl NSSecureCoding for NEPrivateLTENetwork {}

extern_methods!(
    unsafe impl NEPrivateLTENetwork {
        #[method_id(@__retain_semantics Other mobileCountryCode)]
        pub unsafe fn mobileCountryCode(&self) -> Retained<NSString>;

        #[method(setMobileCountryCode:)]
        pub unsafe fn setMobileCountryCode(&self, mobile_country_code: &NSString);

        #[method_id(@__retain_semantics Other mobileNetworkCode)]
        pub unsafe fn mobileNetworkCode(&self) -> Retained<NSString>;

        #[method(setMobileNetworkCode:)]
        pub unsafe fn setMobileNetworkCode(&self, mobile_network_code: &NSString);

        #[method_id(@__retain_semantics Other trackingAreaCode)]
        pub unsafe fn trackingAreaCode(&self) -> Option<Retained<NSString>>;

        #[method(setTrackingAreaCode:)]
        pub unsafe fn setTrackingAreaCode(&self, tracking_area_code: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEPrivateLTENetwork {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEAppPushManager;

    unsafe impl ClassType for NEAppPushManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NEAppPushManager {}

extern_methods!(
    unsafe impl NEAppPushManager {
        #[method_id(@__retain_semantics Other matchSSIDs)]
        pub unsafe fn matchSSIDs(&self) -> Retained<NSArray<NSString>>;

        #[method(setMatchSSIDs:)]
        pub unsafe fn setMatchSSIDs(&self, match_ssi_ds: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other matchPrivateLTENetworks)]
        pub unsafe fn matchPrivateLTENetworks(&self) -> Retained<NSArray<NEPrivateLTENetwork>>;

        #[method(setMatchPrivateLTENetworks:)]
        pub unsafe fn setMatchPrivateLTENetworks(
            &self,
            match_private_lte_networks: &NSArray<NEPrivateLTENetwork>,
        );

        #[method_id(@__retain_semantics Other providerConfiguration)]
        pub unsafe fn providerConfiguration(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[method(setProviderConfiguration:)]
        pub unsafe fn setProviderConfiguration(
            &self,
            provider_configuration: &NSDictionary<NSString, AnyObject>,
        );

        #[method_id(@__retain_semantics Other providerBundleIdentifier)]
        pub unsafe fn providerBundleIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setProviderBundleIdentifier:)]
        pub unsafe fn setProviderBundleIdentifier(
            &self,
            provider_bundle_identifier: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NEAppPushDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NEAppPushDelegate>>);

        #[cfg(feature = "block2")]
        #[method(loadAllFromPreferencesWithCompletionHandler:)]
        pub unsafe fn loadAllFromPreferencesWithCompletionHandler(
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<NEAppPushManager>, *mut NSError),
            >,
        );

        #[cfg(feature = "block2")]
        #[method(loadFromPreferencesWithCompletionHandler:)]
        pub unsafe fn loadFromPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(removeFromPreferencesWithCompletionHandler:)]
        pub unsafe fn removeFromPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(saveToPreferencesWithCompletionHandler:)]
        pub unsafe fn saveToPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Option<Retained<NSString>>;

        #[method(setLocalizedDescription:)]
        pub unsafe fn setLocalizedDescription(&self, localized_description: Option<&NSString>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEAppPushManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NEAppPushDelegate: NSObjectProtocol {
        #[method(appPushManager:didReceiveIncomingCallWithUserInfo:)]
        unsafe fn appPushManager_didReceiveIncomingCallWithUserInfo(
            &self,
            manager: &NEAppPushManager,
            user_info: &NSDictionary,
        );
    }

    unsafe impl ProtocolType for dyn NEAppPushDelegate {}
);