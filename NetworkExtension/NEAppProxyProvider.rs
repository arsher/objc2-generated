//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
    pub struct NEAppProxyProvider;

    #[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
    unsafe impl ClassType for NEAppProxyProvider {
        #[inherits(NEProvider, NSObject)]
        type Super = NETunnelProvider;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
unsafe impl NSObjectProtocol for NEAppProxyProvider {}

extern_methods!(
    #[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
    unsafe impl NEAppProxyProvider {
        #[cfg(feature = "block2")]
        #[method(startProxyWithOptions:completionHandler:)]
        pub unsafe fn startProxyWithOptions_completionHandler(
            &self,
            options: Option<&NSDictionary<NSString, AnyObject>>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(stopProxyWithReason:completionHandler:)]
        pub unsafe fn stopProxyWithReason_completionHandler(
            &self,
            reason: NEProviderStopReason,
            completion_handler: &block2::Block<dyn Fn()>,
        );

        #[method(cancelProxyWithError:)]
        pub unsafe fn cancelProxyWithError(&self, error: Option<&NSError>);

        #[cfg(feature = "NEAppProxyFlow")]
        #[method(handleNewFlow:)]
        pub unsafe fn handleNewFlow(&self, flow: &NEAppProxyFlow) -> bool;

        #[cfg(all(
            feature = "NEAppProxyFlow",
            feature = "NEAppProxyUDPFlow",
            feature = "NWEndpoint"
        ))]
        #[method(handleNewUDPFlow:initialRemoteEndpoint:)]
        pub unsafe fn handleNewUDPFlow_initialRemoteEndpoint(
            &self,
            flow: &NEAppProxyUDPFlow,
            remote_endpoint: &NWEndpoint,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NEProvider", feature = "NETunnelProvider"))]
    unsafe impl NEAppProxyProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
