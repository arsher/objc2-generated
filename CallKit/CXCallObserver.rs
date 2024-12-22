//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxcallobserverdelegate?language=objc)
    pub unsafe trait CXCallObserverDelegate: NSObjectProtocol {
        #[cfg(feature = "CXCall")]
        #[method(callObserver:callChanged:)]
        unsafe fn callObserver_callChanged(&self, call_observer: &CXCallObserver, call: &CXCall);
    }

    unsafe impl ProtocolType for dyn CXCallObserverDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxcallobserver?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXCallObserver;
);

unsafe impl NSObjectProtocol for CXCallObserver {}

extern_methods!(
    unsafe impl CXCallObserver {
        #[cfg(feature = "CXCall")]
        /// Retrieve the current call list, blocking on initial state retrieval if necessary
        #[method_id(@__retain_semantics Other calls)]
        pub unsafe fn calls(&self) -> Retained<NSArray<CXCall>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXCallObserver {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
