//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nwtcpconnectionstate?language=objc)
// NS_ENUM
#[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NWTCPConnectionState(pub NSInteger);
impl NWTCPConnectionState {
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
    #[doc(alias = "NWTCPConnectionStateInvalid")]
    pub const Invalid: Self = Self(0);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
    #[doc(alias = "NWTCPConnectionStateConnecting")]
    pub const Connecting: Self = Self(1);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
    #[doc(alias = "NWTCPConnectionStateWaiting")]
    pub const Waiting: Self = Self(2);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
    #[doc(alias = "NWTCPConnectionStateConnected")]
    pub const Connected: Self = Self(3);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
    #[doc(alias = "NWTCPConnectionStateDisconnected")]
    pub const Disconnected: Self = Self(4);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
    #[doc(alias = "NWTCPConnectionStateCancelled")]
    pub const Cancelled: Self = Self(5);
}

unsafe impl Encode for NWTCPConnectionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NWTCPConnectionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nwtcpconnection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use `nw_connection_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
    pub struct NWTCPConnection;
);

unsafe impl NSObjectProtocol for NWTCPConnection {}

extern_methods!(
    unsafe impl NWTCPConnection {
        #[deprecated = "Use `nw_connection_create` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method_id(@__retain_semantics Init initWithUpgradeForConnection:)]
        pub unsafe fn initWithUpgradeForConnection(
            this: Allocated<Self>,
            connection: &NWTCPConnection,
        ) -> Retained<Self>;

        #[deprecated = "Use `nw_connection_set_state_changed_handler` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method(state)]
        pub unsafe fn state(&self) -> NWTCPConnectionState;

        #[deprecated = "Use `nw_connection_set_viability_changed_handler` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method(isViable)]
        pub unsafe fn isViable(&self) -> bool;

        #[deprecated = "Use `nw_connection_set_better_path_available_handler` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method(hasBetterPath)]
        pub unsafe fn hasBetterPath(&self) -> bool;

        #[cfg(feature = "NWEndpoint")]
        #[deprecated = "Use `nw_connection_copy_endpoint` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method_id(@__retain_semantics Other endpoint)]
        pub unsafe fn endpoint(&self) -> Retained<NWEndpoint>;

        #[cfg(feature = "NWPath")]
        #[deprecated = "Use `nw_connection_copy_current_path` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method_id(@__retain_semantics Other connectedPath)]
        pub unsafe fn connectedPath(&self) -> Option<Retained<NWPath>>;

        #[cfg(feature = "NWEndpoint")]
        #[deprecated = "Use `nw_path_copy_effective_local_endpoint` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method_id(@__retain_semantics Other localAddress)]
        pub unsafe fn localAddress(&self) -> Option<Retained<NWEndpoint>>;

        #[cfg(feature = "NWEndpoint")]
        #[deprecated = "Use `nw_path_copy_effective_remote_endpoint` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method_id(@__retain_semantics Other remoteAddress)]
        pub unsafe fn remoteAddress(&self) -> Option<Retained<NWEndpoint>>;

        #[deprecated = "Use `nw_endpoint_copy_txt_record` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method_id(@__retain_semantics Other txtRecord)]
        pub unsafe fn txtRecord(&self) -> Option<Retained<NSData>>;

        #[deprecated = "Use `nw_connection_set_state_changed_handler` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;

        #[deprecated = "Use `nw_connection_cancel` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[cfg(feature = "block2")]
        #[deprecated = "Use `nw_connection_receive` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method(readLength:completionHandler:)]
        pub unsafe fn readLength_completionHandler(
            &self,
            length: NSUInteger,
            completion: &block2::Block<dyn Fn(*mut NSData, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[deprecated = "Use `nw_connection_receive` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method(readMinimumLength:maximumLength:completionHandler:)]
        pub unsafe fn readMinimumLength_maximumLength_completionHandler(
            &self,
            minimum: NSUInteger,
            maximum: NSUInteger,
            completion: &block2::Block<dyn Fn(*mut NSData, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[deprecated = "Use `nw_connection_send` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method(write:completionHandler:)]
        pub unsafe fn write_completionHandler(
            &self,
            data: &NSData,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[deprecated = "Use `nw_connection_send` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[method(writeClose)]
        pub unsafe fn writeClose(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NWTCPConnection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nwtcpconnectionauthenticationdelegate?language=objc)
    #[deprecated = "Use `sec_protocol_options_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
    pub unsafe trait NWTCPConnectionAuthenticationDelegate: NSObjectProtocol {
        #[deprecated = "Use `sec_protocol_options_set_challenge_block` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[optional]
        #[method(shouldProvideIdentityForConnection:)]
        unsafe fn shouldProvideIdentityForConnection(&self, connection: &NWTCPConnection) -> bool;

        #[deprecated = "Use `sec_protocol_options_set_verify_block` in Network framework instead, see deprecation notice in <NetworkExtension/NWTCPConnection.h>"]
        #[optional]
        #[method(shouldEvaluateTrustForConnection:)]
        unsafe fn shouldEvaluateTrustForConnection(&self, connection: &NWTCPConnection) -> bool;
    }

    unsafe impl ProtocolType for dyn NWTCPConnectionAuthenticationDelegate {}
);
