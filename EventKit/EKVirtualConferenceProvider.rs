//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekvirtualconferenceprovider?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EKVirtualConferenceProvider;
);

unsafe impl NSExtensionRequestHandling for EKVirtualConferenceProvider {}

unsafe impl NSObjectProtocol for EKVirtualConferenceProvider {}

extern_methods!(
    unsafe impl EKVirtualConferenceProvider {
        #[cfg(all(feature = "EKVirtualConferenceDescriptor", feature = "block2"))]
        #[method(fetchAvailableRoomTypesWithCompletionHandler:)]
        pub unsafe fn fetchAvailableRoomTypesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<EKVirtualConferenceRoomTypeDescriptor>, *mut NSError),
            >,
        );

        #[cfg(all(feature = "EKVirtualConferenceDescriptor", feature = "block2"))]
        #[method(fetchVirtualConferenceForIdentifier:completionHandler:)]
        pub unsafe fn fetchVirtualConferenceForIdentifier_completionHandler(
            &self,
            identifier: &EKVirtualConferenceRoomTypeIdentifier,
            completion_handler: &block2::Block<
                dyn Fn(*mut EKVirtualConferenceDescriptor, *mut NSError),
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EKVirtualConferenceProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
