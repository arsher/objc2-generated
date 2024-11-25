//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nwudpsessionstate?language=objc)
// NS_ENUM
#[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NWUDPSessionState(pub NSInteger);
impl NWUDPSessionState {
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
    #[doc(alias = "NWUDPSessionStateInvalid")]
    pub const Invalid: Self = Self(0);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
    #[doc(alias = "NWUDPSessionStateWaiting")]
    pub const Waiting: Self = Self(1);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
    #[doc(alias = "NWUDPSessionStatePreparing")]
    pub const Preparing: Self = Self(2);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
    #[doc(alias = "NWUDPSessionStateReady")]
    pub const Ready: Self = Self(3);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
    #[doc(alias = "NWUDPSessionStateFailed")]
    pub const Failed: Self = Self(4);
    #[deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
    #[doc(alias = "NWUDPSessionStateCancelled")]
    pub const Cancelled: Self = Self(5);
}

unsafe impl Encode for NWUDPSessionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NWUDPSessionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nwudpsession?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use `nw_connection_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
    pub struct NWUDPSession;
);

unsafe impl NSObjectProtocol for NWUDPSession {}

extern_methods!(
    unsafe impl NWUDPSession {
        #[deprecated = "Use `nw_connection_create` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method_id(@__retain_semantics Init initWithUpgradeForSession:)]
        pub unsafe fn initWithUpgradeForSession(
            this: Allocated<Self>,
            session: &NWUDPSession,
        ) -> Retained<Self>;

        #[deprecated = "Use `nw_connection_set_state_changed_handler` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(state)]
        pub unsafe fn state(&self) -> NWUDPSessionState;

        #[cfg(feature = "NWEndpoint")]
        #[deprecated = "Use `nw_connection_copy_endpoint` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method_id(@__retain_semantics Other endpoint)]
        pub unsafe fn endpoint(&self) -> Retained<NWEndpoint>;

        #[cfg(feature = "NWEndpoint")]
        #[deprecated = "Use `nw_connection_copy_current_path` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method_id(@__retain_semantics Other resolvedEndpoint)]
        pub unsafe fn resolvedEndpoint(&self) -> Option<Retained<NWEndpoint>>;

        #[deprecated = "Use `nw_connection_set_viability_changed_handler` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(isViable)]
        pub unsafe fn isViable(&self) -> bool;

        #[deprecated = "Use `nw_connection_set_better_path_available_handler` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(hasBetterPath)]
        pub unsafe fn hasBetterPath(&self) -> bool;

        #[cfg(feature = "NWPath")]
        #[deprecated = "Use `nw_connection_copy_current_path` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method_id(@__retain_semantics Other currentPath)]
        pub unsafe fn currentPath(&self) -> Option<Retained<NWPath>>;

        #[deprecated = "Use `nw_connection_cancel_current_endpoint` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(tryNextResolvedEndpoint)]
        pub unsafe fn tryNextResolvedEndpoint(&self);

        #[deprecated = "Use `nw_connection_get_maximum_datagram_size` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(maximumDatagramLength)]
        pub unsafe fn maximumDatagramLength(&self) -> NSUInteger;

        #[cfg(feature = "block2")]
        #[deprecated = "Use `nw_connection_receive` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(setReadHandler:maxDatagrams:)]
        pub unsafe fn setReadHandler_maxDatagrams(
            &self,
            handler: &block2::Block<dyn Fn(*mut NSArray<NSData>, *mut NSError)>,
            max_datagrams: NSUInteger,
        );

        #[cfg(feature = "block2")]
        #[deprecated = "Use `nw_connection_send` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(writeMultipleDatagrams:completionHandler:)]
        pub unsafe fn writeMultipleDatagrams_completionHandler(
            &self,
            datagram_array: &NSArray<NSData>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[deprecated = "Use `nw_connection_send` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(writeDatagram:completionHandler:)]
        pub unsafe fn writeDatagram_completionHandler(
            &self,
            datagram: &NSData,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[deprecated = "Use `nw_connection_cancel` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"]
        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NWUDPSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
