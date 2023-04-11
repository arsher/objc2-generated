//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait CXCallObserverDelegate: NSObjectProtocol {
        #[cfg(all(feature = "CallKit_CXCall", feature = "CallKit_CXCallObserver"))]
        #[method(callObserver:callChanged:)]
        unsafe fn callObserver_callChanged(&self, call_observer: &CXCallObserver, call: &CXCall);
    }

    unsafe impl ProtocolType for dyn CXCallObserverDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXCallObserver")]
    pub struct CXCallObserver;

    #[cfg(feature = "CallKit_CXCallObserver")]
    unsafe impl ClassType for CXCallObserver {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXCallObserver")]
unsafe impl NSObjectProtocol for CXCallObserver {}

extern_methods!(
    #[cfg(feature = "CallKit_CXCallObserver")]
    unsafe impl CXCallObserver {
        #[cfg(all(feature = "CallKit_CXCall", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other calls)]
        pub unsafe fn calls(&self) -> Id<NSArray<CXCall>>;
    }
);
