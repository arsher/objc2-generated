//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NEAppProxyProvider",
        feature = "NEProvider",
        feature = "NETunnelProvider"
    ))]
    pub struct NETransparentProxyProvider;

    #[cfg(all(
        feature = "NEAppProxyProvider",
        feature = "NEProvider",
        feature = "NETunnelProvider"
    ))]
    unsafe impl ClassType for NETransparentProxyProvider {
        #[inherits(NETunnelProvider, NEProvider, NSObject)]
        type Super = NEAppProxyProvider;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "NEAppProxyProvider",
    feature = "NEProvider",
    feature = "NETunnelProvider"
))]
unsafe impl NSObjectProtocol for NETransparentProxyProvider {}

extern_methods!(
    #[cfg(all(
        feature = "NEAppProxyProvider",
        feature = "NEProvider",
        feature = "NETunnelProvider"
    ))]
    unsafe impl NETransparentProxyProvider {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NEAppProxyProvider",
        feature = "NEProvider",
        feature = "NETunnelProvider"
    ))]
    unsafe impl NETransparentProxyProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
