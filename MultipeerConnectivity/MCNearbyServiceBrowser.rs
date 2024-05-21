//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MCNearbyServiceBrowser;

    unsafe impl ClassType for MCNearbyServiceBrowser {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MCNearbyServiceBrowser {}

extern_methods!(
    unsafe impl MCNearbyServiceBrowser {
        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Init initWithPeer:serviceType:)]
        pub unsafe fn initWithPeer_serviceType(
            this: Allocated<Self>,
            my_peer_id: &MCPeerID,
            service_type: &NSString,
        ) -> Retained<Self>;

        #[method(startBrowsingForPeers)]
        pub unsafe fn startBrowsingForPeers(&self);

        #[method(stopBrowsingForPeers)]
        pub unsafe fn stopBrowsingForPeers(&self);

        #[cfg(all(feature = "MCPeerID", feature = "MCSession"))]
        #[method(invitePeer:toSession:withContext:timeout:)]
        pub unsafe fn invitePeer_toSession_withContext_timeout(
            &self,
            peer_id: &MCPeerID,
            session: &MCSession,
            context: Option<&NSData>,
            timeout: NSTimeInterval,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MCNearbyServiceBrowserDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MCNearbyServiceBrowserDelegate>>,
        );

        #[cfg(feature = "MCPeerID")]
        #[method_id(@__retain_semantics Other myPeerID)]
        pub unsafe fn myPeerID(&self) -> Retained<MCPeerID>;

        #[method_id(@__retain_semantics Other serviceType)]
        pub unsafe fn serviceType(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MCNearbyServiceBrowser {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MCNearbyServiceBrowserDelegate: NSObjectProtocol {
        #[cfg(feature = "MCPeerID")]
        #[method(browser:foundPeer:withDiscoveryInfo:)]
        unsafe fn browser_foundPeer_withDiscoveryInfo(
            &self,
            browser: &MCNearbyServiceBrowser,
            peer_id: &MCPeerID,
            info: Option<&NSDictionary<NSString, NSString>>,
        );

        #[cfg(feature = "MCPeerID")]
        #[method(browser:lostPeer:)]
        unsafe fn browser_lostPeer(&self, browser: &MCNearbyServiceBrowser, peer_id: &MCPeerID);

        #[optional]
        #[method(browser:didNotStartBrowsingForPeers:)]
        unsafe fn browser_didNotStartBrowsingForPeers(
            &self,
            browser: &MCNearbyServiceBrowser,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn MCNearbyServiceBrowserDelegate {}
);
