//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NENetworkRuleProtocol(pub NSInteger);
impl NENetworkRuleProtocol {
    #[doc(alias = "NENetworkRuleProtocolAny")]
    pub const Any: Self = Self(0);
    #[doc(alias = "NENetworkRuleProtocolTCP")]
    pub const TCP: Self = Self(1);
    #[doc(alias = "NENetworkRuleProtocolUDP")]
    pub const UDP: Self = Self(2);
}

unsafe impl Encode for NENetworkRuleProtocol {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NENetworkRuleProtocol {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NETrafficDirection(pub NSInteger);
impl NETrafficDirection {
    #[doc(alias = "NETrafficDirectionAny")]
    pub const Any: Self = Self(0);
    #[doc(alias = "NETrafficDirectionInbound")]
    pub const Inbound: Self = Self(1);
    #[doc(alias = "NETrafficDirectionOutbound")]
    pub const Outbound: Self = Self(2);
}

unsafe impl Encode for NETrafficDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NETrafficDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NENetworkRule;

    unsafe impl ClassType for NENetworkRule {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for NENetworkRule {}

unsafe impl NSCopying for NENetworkRule {}

unsafe impl CopyingHelper for NENetworkRule {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NENetworkRule {}

unsafe impl NSSecureCoding for NENetworkRule {}

extern_methods!(
    unsafe impl NENetworkRule {
        #[cfg(all(feature = "NWEndpoint", feature = "NWHostEndpoint"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithDestinationNetwork:prefix:protocol:)]
        pub unsafe fn initWithDestinationNetwork_prefix_protocol(
            this: Allocated<Self>,
            network_endpoint: &NWHostEndpoint,
            destination_prefix: NSUInteger,
            protocol: NENetworkRuleProtocol,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NWEndpoint", feature = "NWHostEndpoint"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithDestinationHost:protocol:)]
        pub unsafe fn initWithDestinationHost_protocol(
            this: Allocated<Self>,
            host_endpoint: &NWHostEndpoint,
            protocol: NENetworkRuleProtocol,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NWEndpoint", feature = "NWHostEndpoint"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithRemoteNetwork:remotePrefix:localNetwork:localPrefix:protocol:direction:)]
        pub unsafe fn initWithRemoteNetwork_remotePrefix_localNetwork_localPrefix_protocol_direction(
            this: Allocated<Self>,
            remote_network: Option<&NWHostEndpoint>,
            remote_prefix: NSUInteger,
            local_network: Option<&NWHostEndpoint>,
            local_prefix: NSUInteger,
            protocol: NENetworkRuleProtocol,
            direction: NETrafficDirection,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NWEndpoint", feature = "NWHostEndpoint"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other matchRemoteEndpoint)]
        pub unsafe fn matchRemoteEndpoint(&self) -> Option<Retained<NWHostEndpoint>>;

        #[method(matchRemotePrefix)]
        pub unsafe fn matchRemotePrefix(&self) -> NSUInteger;

        #[cfg(all(feature = "NWEndpoint", feature = "NWHostEndpoint"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other matchLocalNetwork)]
        pub unsafe fn matchLocalNetwork(&self) -> Option<Retained<NWHostEndpoint>>;

        #[method(matchLocalPrefix)]
        pub unsafe fn matchLocalPrefix(&self) -> NSUInteger;

        #[method(matchProtocol)]
        pub unsafe fn matchProtocol(&self) -> NENetworkRuleProtocol;

        #[method(matchDirection)]
        pub unsafe fn matchDirection(&self) -> NETrafficDirection;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NENetworkRule {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
