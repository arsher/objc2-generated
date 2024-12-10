//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caremotelayerclient?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CARemoteLayerClient;
);

unsafe impl NSObjectProtocol for CARemoteLayerClient {}

extern_methods!(
    unsafe impl CARemoteLayerClient {
        #[cfg(feature = "libc")]
        #[method_id(@__retain_semantics Init initWithServerPort:)]
        pub unsafe fn initWithServerPort(
            this: Allocated<Self>,
            port: libc::mach_port_t,
        ) -> Retained<Self>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(clientId)]
        pub unsafe fn clientId(&self) -> u32;

        #[cfg(feature = "CALayer")]
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer(&self) -> Option<Retained<CALayer>>;

        #[cfg(feature = "CALayer")]
        #[method(setLayer:)]
        pub unsafe fn setLayer(&self, layer: Option<&CALayer>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CARemoteLayerClient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
