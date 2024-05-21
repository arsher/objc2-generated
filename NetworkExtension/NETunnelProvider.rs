//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NETunnelProviderError(pub NSInteger);
impl NETunnelProviderError {
    #[doc(alias = "NETunnelProviderErrorNetworkSettingsInvalid")]
    pub const NetworkSettingsInvalid: Self = Self(1);
    #[doc(alias = "NETunnelProviderErrorNetworkSettingsCanceled")]
    pub const NetworkSettingsCanceled: Self = Self(2);
    #[doc(alias = "NETunnelProviderErrorNetworkSettingsFailed")]
    pub const NetworkSettingsFailed: Self = Self(3);
}

unsafe impl Encode for NETunnelProviderError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NETunnelProviderError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NETunnelProviderRoutingMethod(pub NSInteger);
impl NETunnelProviderRoutingMethod {
    #[doc(alias = "NETunnelProviderRoutingMethodDestinationIP")]
    pub const DestinationIP: Self = Self(1);
    #[doc(alias = "NETunnelProviderRoutingMethodSourceApplication")]
    pub const SourceApplication: Self = Self(2);
    #[doc(alias = "NETunnelProviderRoutingMethodNetworkRule")]
    pub const NetworkRule: Self = Self(3);
}

unsafe impl Encode for NETunnelProviderRoutingMethod {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NETunnelProviderRoutingMethod {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static NETunnelProviderErrorDomain: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NEProvider")]
    pub struct NETunnelProvider;

    #[cfg(feature = "NEProvider")]
    unsafe impl ClassType for NETunnelProvider {
        #[inherits(NSObject)]
        type Super = NEProvider;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NEProvider")]
unsafe impl NSObjectProtocol for NETunnelProvider {}

extern_methods!(
    #[cfg(feature = "NEProvider")]
    unsafe impl NETunnelProvider {
        #[cfg(feature = "block2")]
        #[method(handleAppMessage:completionHandler:)]
        pub unsafe fn handleAppMessage_completionHandler(
            &self,
            message_data: &NSData,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSData)>>,
        );

        #[cfg(all(feature = "NETunnelNetworkSettings", feature = "block2"))]
        #[method(setTunnelNetworkSettings:completionHandler:)]
        pub unsafe fn setTunnelNetworkSettings_completionHandler(
            &self,
            tunnel_network_settings: Option<&NETunnelNetworkSettings>,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "NEVPNProtocol")]
        #[method_id(@__retain_semantics Other protocolConfiguration)]
        pub unsafe fn protocolConfiguration(&self) -> Retained<NEVPNProtocol>;

        #[cfg(feature = "NEAppRule")]
        #[method_id(@__retain_semantics Other appRules)]
        pub unsafe fn appRules(&self) -> Option<Retained<NSArray<NEAppRule>>>;

        #[method(routingMethod)]
        pub unsafe fn routingMethod(&self) -> NETunnelProviderRoutingMethod;

        #[method(reasserting)]
        pub unsafe fn reasserting(&self) -> bool;

        #[method(setReasserting:)]
        pub unsafe fn setReasserting(&self, reasserting: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NEProvider")]
    unsafe impl NETunnelProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
